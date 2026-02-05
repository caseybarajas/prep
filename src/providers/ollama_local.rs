use super::{build_user_message, Provider, SYSTEM_PROMPT};
use crate::refiner::RefinerResponse;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaLocalProvider {
    client: Client,
    endpoint: String,
    model: String,
}

impl OllamaLocalProvider {
    pub fn new(endpoint: String, model: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            endpoint,
            model,
        }
    }
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    format: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
}

#[async_trait]
impl Provider for OllamaLocalProvider {
    fn name(&self) -> &'static str {
        "Ollama (Local)"
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

        let request = OllamaRequest {
            model: self.model.clone(),
            messages: vec![
                OllamaMessage {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                OllamaMessage {
                    role: "user".to_string(),
                    content: user_message,
                },
            ],
            stream: false,
            format: "json".to_string(),
        };

        let response = self
            .client
            .post(format!("{}/api/chat", self.endpoint))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    anyhow::anyhow!(
                        "Could not connect to Ollama at {}. Is Ollama running? Try: ollama serve",
                        self.endpoint
                    )
                } else if e.is_timeout() {
                    anyhow::anyhow!(
                        "Request timed out. The model might be loading or the prompt is very long."
                    )
                } else {
                    anyhow::anyhow!("HTTP request failed: {}", e)
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Ollama returned error {}: {}", status, body);
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        let refiner_response: RefinerResponse =
            serde_json::from_str(&ollama_response.message.content).with_context(|| {
                format!(
                    "Failed to parse refiner response as JSON. Raw content:\n{}",
                    ollama_response.message.content
                )
            })?;

        if refiner_response.refined_prompt.is_empty() {
            anyhow::bail!("Refiner returned an empty refined_prompt");
        }

        Ok(refiner_response)
    }
}
