use super::{build_user_message, Provider, SYSTEM_PROMPT};
use crate::refiner::RefinerResponse;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAIProvider {
    client: Client,
    endpoint: String,
    model: String,
    api_key: String,
}

impl OpenAIProvider {
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
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    response_format: ResponseFormat,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAI"
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

        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: user_message,
                },
            ],
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
            temperature: 0.7,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    anyhow::anyhow!("Could not connect to OpenAI API at {}", self.endpoint)
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
                    "Authentication failed. Check your OPENAI_API_KEY environment variable."
                );
            } else if status.as_u16() == 429 {
                anyhow::bail!("Rate limited by OpenAI. Please wait and try again.");
            }

            anyhow::bail!("OpenAI returned error {}: {}", status, body);
        }

        let openai_response: OpenAIResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        let choice = openai_response
            .choices
            .first()
            .context("No response from OpenAI")?;

        let refiner_response: RefinerResponse = serde_json::from_str(&choice.message.content)
            .with_context(|| {
                format!(
                    "Failed to parse refiner response as JSON. Raw content:\n{}",
                    choice.message.content
                )
            })?;

        if refiner_response.refined_prompt.is_empty() {
            anyhow::bail!("Refiner returned an empty refined_prompt");
        }

        Ok(refiner_response)
    }
}
