//! Core refiner logic

use serde::{Deserialize, Serialize};

/// Response from the prompt refiner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefinerResponse {
    pub refined_prompt: String,
    #[serde(default)]
    pub needs_clarification: bool,
    #[serde(default)]
    pub questions: Vec<String>,
}

/// Build clarification summary from Q&A pairs
pub fn build_clarification_summary(questions: &[String], answers: &[String]) -> String {
    let mut summary = String::new();
    for (i, (q, a)) in questions.iter().zip(answers.iter()).enumerate() {
        summary.push_str(&format!("Q{}: {} → Answer: {}\n", i + 1, q, a));
    }
    summary
}

/// Refiner struct for managing the refinement flow
pub struct Refiner;

impl Refiner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Refiner {
    fn default() -> Self {
        Self::new()
    }
}
