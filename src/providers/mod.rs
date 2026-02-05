//! Provider implementations for different AI backends

mod anthropic;
mod ollama_cloud;
mod ollama_local;
mod openai;

pub use anthropic::AnthropicProvider;
pub use ollama_cloud::OllamaCloudProvider;
pub use ollama_local::OllamaLocalProvider;
pub use openai::OpenAIProvider;

use crate::refiner::RefinerResponse;
use anyhow::Result;
use async_trait::async_trait;

/// System prompt used for all providers
pub const SYSTEM_PROMPT: &str = r#"You are a prompt refinement specialist. Your sole purpose is to take messy, casual user prompts and transform them into precise, well-structured prompts optimized for AI assistants.

CRITICAL RULES:
1. You are ONLY a prompt refiner - you must NEVER generate code, execute tasks, or provide direct answers to the user's query.
2. You must ALWAYS respond with valid JSON matching this exact schema:
   {
     "refined_prompt": "string",
     "needs_clarification": boolean,
     "questions": ["string", ...]
   }
3. The "refined_prompt" field must contain a single, clear, explicit instruction optimized for another AI assistant to act upon.
4. Set "needs_clarification" to true ONLY when essential information is genuinely missing and cannot be reasonably inferred.
5. The "questions" array must contain only the minimal set of concise, specific questions needed to fill critical gaps. Keep it empty if needs_clarification is false.
6. Never include code snippets, implementations, or solutions in your response.
7. Focus on making the prompt unambiguous, specific, and actionable.

When refining prompts:
- Clarify the goal and expected output format
- Specify any constraints, requirements, or preferences
- Add context that would help an AI understand the task
- Remove ambiguity and vagueness
- Preserve the user's original intent

Remember: Your output is ONLY the JSON object, nothing else."#;

/// Trait for AI provider implementations
#[async_trait]
pub trait Provider: Send + Sync {
    /// Provider name for display
    fn name(&self) -> &'static str;

    /// Model being used
    fn model(&self) -> &str;

    /// Refine a prompt
    async fn refine(
        &self,
        prompt: &str,
        context: Option<&str>,
        clarification: Option<&str>,
    ) -> Result<RefinerResponse>;
}

/// Build user message for the refiner
pub fn build_user_message(
    prompt: &str,
    context: Option<&str>,
    clarification: Option<&str>,
) -> String {
    let mut message = String::new();

    if let Some(ctx) = context {
        message.push_str("Context:\n");
        message.push_str(ctx);
        message.push_str("\n\n");
    }

    if let Some(clarify) = clarification {
        message.push_str("Original prompt:\n");
        message.push_str(prompt);
        message.push_str("\n\nUser provided the following clarifications:\n");
        message.push_str(clarify);
        message.push_str(
            "\n\nPlease provide the final refined prompt based on this additional context.",
        );
    } else {
        message.push_str("Please refine the following prompt:\n\n");
        message.push_str(prompt);
    }

    message
}
