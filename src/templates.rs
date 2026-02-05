//! Built-in prompt templates

use std::collections::HashMap;

/// A prompt template
#[derive(Debug, Clone)]
pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
    pub prefix: &'static str,
    pub suffix: &'static str,
}

impl Template {
    /// Apply the template to a prompt
    pub fn apply(&self, prompt: &str) -> String {
        format!("{}{}{}", self.prefix, prompt, self.suffix)
    }
}

/// Get all built-in templates
pub fn get_templates() -> HashMap<&'static str, Template> {
    let mut templates = HashMap::new();

    templates.insert(
        "code",
        Template {
            name: "code",
            description: "Optimize for code generation requests",
            prefix: "[Code Generation Request]\n\n",
            suffix: "\n\nPlease provide clean, well-documented, production-ready code with proper error handling.",
        },
    );

    templates.insert(
        "explain",
        Template {
            name: "explain",
            description: "Optimize for explanation requests",
            prefix: "[Explanation Request]\n\n",
            suffix: "\n\nProvide a clear, structured explanation suitable for someone learning this concept.",
        },
    );

    templates.insert(
        "debug",
        Template {
            name: "debug",
            description: "Optimize for debugging assistance",
            prefix: "[Debugging Assistance Request]\n\n",
            suffix: "\n\nAnalyze the issue, identify the root cause, and suggest specific fixes with explanations.",
        },
    );

    templates.insert(
        "review",
        Template {
            name: "review",
            description: "Optimize for code review requests",
            prefix: "[Code Review Request]\n\n",
            suffix: "\n\nProvide a thorough code review covering: correctness, performance, security, readability, and best practices.",
        },
    );

    templates.insert(
        "docs",
        Template {
            name: "docs",
            description: "Optimize for documentation requests",
            prefix: "[Documentation Request]\n\n",
            suffix: "\n\nCreate clear, comprehensive documentation following best practices for the target audience.",
        },
    );

    templates.insert(
        "refactor",
        Template {
            name: "refactor",
            description: "Optimize for refactoring requests",
            prefix: "[Refactoring Request]\n\n",
            suffix: "\n\nRefactor the code to improve maintainability, readability, and adherence to SOLID principles while preserving functionality.",
        },
    );

    templates.insert(
        "test",
        Template {
            name: "test",
            description: "Optimize for test writing requests",
            prefix: "[Test Writing Request]\n\n",
            suffix: "\n\nWrite comprehensive tests covering edge cases, error scenarios, and happy paths with clear test descriptions.",
        },
    );

    templates.insert(
        "api",
        Template {
            name: "api",
            description: "Optimize for API design requests",
            prefix: "[API Design Request]\n\n",
            suffix: "\n\nDesign a RESTful API following best practices with proper status codes, validation, and documentation.",
        },
    );

    templates.insert(
        "security",
        Template {
            name: "security",
            description: "Optimize for security-focused requests",
            prefix: "[Security Analysis Request]\n\n",
            suffix: "\n\nAnalyze for security vulnerabilities including OWASP Top 10 issues and provide specific remediation steps.",
        },
    );

    templates.insert(
        "architecture",
        Template {
            name: "architecture",
            description: "Optimize for architecture design requests",
            prefix: "[Architecture Design Request]\n\n",
            suffix: "\n\nDesign a scalable, maintainable architecture considering performance, reliability, and future extensibility.",
        },
    );

    templates
}

/// Get a specific template by name
pub fn get_template(name: &str) -> Option<Template> {
    get_templates().remove(name)
}

/// List all template names with descriptions
pub fn list_templates() -> Vec<(&'static str, &'static str)> {
    let templates = get_templates();
    let mut list: Vec<_> = templates
        .iter()
        .map(|(name, t)| (*name, t.description))
        .collect();
    list.sort_by_key(|(name, _)| *name);
    list
}
