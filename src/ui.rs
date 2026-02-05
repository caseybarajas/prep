//! Beautiful terminal UI components

use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Terminal output helper
pub struct UI {
    color_enabled: bool,
    spinner_enabled: bool,
}

impl UI {
    pub fn new(color_enabled: bool, spinner_enabled: bool) -> Self {
        Self {
            color_enabled,
            spinner_enabled,
        }
    }

    /// Create a spinner for long-running operations
    pub fn spinner(&self, message: &str) -> Option<ProgressBar> {
        if !self.spinner_enabled {
            self.status(message);
            return None;
        }

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));
        Some(pb)
    }

    /// Print a success message
    pub fn success(&self, message: &str) {
        if self.color_enabled {
            eprintln!("{} {}", "✓".green().bold(), message.green());
        } else {
            eprintln!("✓ {}", message);
        }
    }

    /// Print an error message
    pub fn error(&self, message: &str) {
        if self.color_enabled {
            eprintln!("{} {}", "✗".red().bold(), message.red());
        } else {
            eprintln!("✗ {}", message);
        }
    }

    /// Print a warning message
    pub fn warning(&self, message: &str) {
        if self.color_enabled {
            eprintln!("{} {}", "⚠".yellow().bold(), message.yellow());
        } else {
            eprintln!("⚠ {}", message);
        }
    }

    /// Print an info message
    pub fn info(&self, message: &str) {
        if self.color_enabled {
            eprintln!("{} {}", "ℹ".cyan().bold(), message.cyan());
        } else {
            eprintln!("ℹ {}", message);
        }
    }

    /// Print a status message (verbose)
    pub fn status(&self, message: &str) {
        if self.color_enabled {
            eprintln!("{} {}", "→".bright_black(), message.bright_black());
        } else {
            eprintln!("→ {}", message);
        }
    }

    /// Print a header
    pub fn header(&self, title: &str) {
        if self.color_enabled {
            eprintln!("\n{}", title.bright_cyan().bold());
            eprintln!("{}", "─".repeat(title.len()).bright_black());
        } else {
            eprintln!("\n{}", title);
            eprintln!("{}", "─".repeat(title.len()));
        }
    }

    /// Print a section with a box
    pub fn boxed(&self, content: &str, title: Option<&str>) {
        let lines: Vec<&str> = content.lines().collect();
        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0).max(40);

        if self.color_enabled {
            if let Some(t) = title {
                eprintln!(
                    "{} {} {}",
                    "┌─".bright_black(),
                    t.bright_cyan().bold(),
                    "─".repeat(max_width - t.len()).bright_black()
                );
            } else {
                eprintln!(
                    "{}",
                    format!("┌{}┐", "─".repeat(max_width + 2)).bright_black()
                );
            }
            for line in &lines {
                eprintln!(
                    "{} {:<width$} {}",
                    "│".bright_black(),
                    line,
                    "│".bright_black(),
                    width = max_width
                );
            }
            eprintln!(
                "{}",
                format!("└{}┘", "─".repeat(max_width + 2)).bright_black()
            );
        } else {
            if let Some(t) = title {
                eprintln!("┌─ {} {}", t, "─".repeat(max_width - t.len()));
            } else {
                eprintln!("┌{}┐", "─".repeat(max_width + 2));
            }
            for line in &lines {
                eprintln!("│ {:<width$} │", line, width = max_width);
            }
            eprintln!("└{}┘", "─".repeat(max_width + 2));
        }
    }

    /// Print refined prompt in a beautiful format
    pub fn refined_prompt(&self, prompt: &str) {
        eprintln!();
        if self.color_enabled {
            eprintln!(
                "{}",
                "╭─ Refined Prompt ─────────────────────────────────────────╮".bright_green()
            );
            for line in prompt.lines() {
                eprintln!("{} {}", "│".bright_green(), line);
            }
            eprintln!(
                "{}",
                "╰──────────────────────────────────────────────────────────╯".bright_green()
            );
        } else {
            self.boxed(prompt, Some("Refined Prompt"));
        }
        eprintln!();
    }

    /// Ask for clarification answers
    pub fn ask_questions(&self, questions: &[String]) -> anyhow::Result<Vec<String>> {
        self.header("Clarification Needed");
        eprintln!();

        let mut answers = Vec::new();
        let theme = ColorfulTheme::default();

        for (i, question) in questions.iter().enumerate() {
            if self.color_enabled {
                eprintln!(
                    "{} {}",
                    format!("Q{}:", i + 1).bright_yellow().bold(),
                    question.bright_white()
                );
            } else {
                eprintln!("Q{}: {}", i + 1, question);
            }

            let answer: String = Input::with_theme(&theme)
                .with_prompt(format!("A{}", i + 1))
                .interact_text()?;

            answers.push(answer);
            eprintln!();
        }

        Ok(answers)
    }

    /// Print a key-value pair
    pub fn kv(&self, key: &str, value: &str) {
        if self.color_enabled {
            eprintln!("  {} {}", format!("{}:", key).bright_black(), value.white());
        } else {
            eprintln!("  {}: {}", key, value);
        }
    }

    /// Print a list item
    pub fn list_item(&self, bullet: &str, content: &str) {
        if self.color_enabled {
            eprintln!("  {} {}", bullet.bright_cyan(), content);
        } else {
            eprintln!("  {} {}", bullet, content);
        }
    }

    /// Print the prep banner
    pub fn banner(&self) {
        if self.color_enabled {
            eprintln!(
                "{}",
                r#"
  ┌─────────────────────────────────────┐
  │         ✨ prep v1.0.0 ✨           │
  │   Refine prompts for AI assistants  │
  └─────────────────────────────────────┘
"#
                .bright_cyan()
            );
        }
    }

    /// Print verbose debug info
    pub fn debug(&self, label: &str, content: &str) {
        if self.color_enabled {
            eprintln!(
                "{} {}",
                format!("[{}]", label).bright_black(),
                content.bright_black()
            );
        } else {
            eprintln!("[{}] {}", label, content);
        }
    }
}
