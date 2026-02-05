use super::{build_user_message, Provider, SYSTEM_PROMPT};
use crate::refiner::RefinerResponse;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct AnthropicProvider {
    client: Client,
    endpoint: String,
    model: String,
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(endpoint: String, model: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            endpoint,
            model,
            api_key,
        }
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &'static str {
        "Anthropic"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn refine(
        &self,
        prompt: &str,
        context: Option<&str>,
        clarification: Option<&str>,
    ) -> Result<RefinerResponse> {
        let user_message = build_user_message(prompt, context, clarification);

        // Anthropic requires specific JSON instruction in the prompt
        let json_system = format!(
            "{}\n\nIMPORTANT: Respond with ONLY a valid JSON object. No markdown code blocks, no explanation, just the raw JSON.",
            SYSTEM_PROMPT
        );

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            system: json_system,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: user_message,
            }],
        };

        let response = self
            .client
            .post(format!("{}/messages", self.endpoint))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    anyhow::anyhow!("Could not connect to Anthropic API at {}", self.endpoint)
                } else if e.is_timeout() {
                    anyhow::anyhow!("Request timed out")
                } else {
                    anyhow::anyhow!("HTTP request failed: {}", e)
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            if status.as_u16() == 401 {
                anyhow::bail!(
                    "Authentication failed. Check your ANTHROPIC_API_KEY environment variable."
                );
            } else if status.as_u16() == 429 {
                anyhow::bail!("Rate limited by Anthropic. Please wait and try again.");
            }

            anyhow::bail!("Anthropic returned error {}: {}", status, body);
        }

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        let content = anthropic_response
            .content
            .first()
            .context("No response from Anthropic")?;

        // Clean up potential markdown code blocks
        let json_text = content.text.trim();
        let json_text = json_text
            .strip_prefix("```json")
            .or_else(|| json_text.strip_prefix("```"))
            .unwrap_or(json_text);
        let json_text = json_text.strip_suffix("```").unwrap_or(json_text).trim();

        let refiner_response: RefinerResponse =
            serde_json::from_str(json_text).with_context(|| {
                format!(
                    "Failed to parse refiner response as JSON. Raw content:\n{}",
                    content.text
                )
            })?;

        if refiner_response.refined_prompt.is_empty() {
            anyhow::bail!("Refiner returned an empty refined_prompt");
        }

        Ok(refiner_response)
    }
}
