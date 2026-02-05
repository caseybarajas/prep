use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "prep")]
#[command(
    author,
    version,
    about = "✨ Refine messy prompts into precise, well-structured prompts for AI assistants"
)]
#[command(long_about = r#"
Prep takes your casual, messy prompts and transforms them into precise, 
well-structured prompts optimized for AI assistants like ChatGPT, Claude, 
or coding assistants.

Examples:
  prep "make a website"
  echo "fix my code" | prep
  prep --provider openai --model gpt-4o "build an api"
  prep --copy "write tests for my app"
"#)]
#[command(styles = get_styles())]
pub struct Cli {
    /// Raw prompt to refine. If not provided, reads from stdin
    #[arg(trailing_var_arg = true)]
    pub prompt: Vec<String>,

    /// AI provider to use
    #[arg(short, long, value_enum, env = "PREP_PROVIDER")]
    pub provider: Option<ProviderChoice>,

    /// Model name to use (overrides provider default)
    #[arg(short, long, env = "PREP_MODEL")]
    pub model: Option<String>,

    /// API key (overrides config and environment)
    #[arg(long, hide = true)]
    pub api_key: Option<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    pub output: OutputFormat,

    /// Copy result to clipboard
    #[arg(short = 'C', long)]
    pub copy: bool,

    /// Include file as additional context
    #[arg(long, value_name = "FILE")]
    pub context: Option<PathBuf>,

    /// Use a prompt template
    #[arg(short, long)]
    pub template: Option<String>,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Show what would be sent without calling API
    #[arg(long)]
    pub dry_run: bool,

    /// Print verbose diagnostic information
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable saving to history
    #[arg(long)]
    pub no_history: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// View and manage refinement history
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
    /// Work with prompt templates
    Templates {
        #[command(subcommand)]
        action: TemplateAction,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Initialize configuration file with defaults
    Init {
        /// Overwrite existing config
        #[arg(long)]
        force: bool,
    },
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., default.provider)
        key: String,
        /// Value to set
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Show path to config file
    Path,
}

#[derive(Subcommand, Debug)]
pub enum HistoryAction {
    /// List recent refinements
    List {
        /// Number of entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Show details of a specific entry
    Show {
        /// Entry ID
        id: i64,
    },
    /// Search history
    Search {
        /// Search query
        query: String,
    },
    /// Clear all history
    Clear {
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum TemplateAction {
    /// List available templates
    List,
    /// Show template details
    Show {
        /// Template name
        name: String,
    },
    /// Use a template interactively
    Use {
        /// Template name
        name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProviderChoice {
    /// Local Ollama instance
    #[value(name = "ollama", alias = "local")]
    OllamaLocal,
    /// Ollama Cloud API
    #[value(name = "ollama-cloud", alias = "cloud")]
    OllamaCloud,
    /// OpenAI API
    #[value(name = "openai", alias = "gpt")]
    OpenAI,
    /// Anthropic API
    #[value(name = "anthropic", alias = "claude")]
    Anthropic,
}

impl std::fmt::Display for ProviderChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OllamaLocal => write!(f, "ollama"),
            Self::OllamaCloud => write!(f, "ollama-cloud"),
            Self::OpenAI => write!(f, "openai"),
            Self::Anthropic => write!(f, "anthropic"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Plain text (default)
    Text,
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            clap::builder::styling::AnsiColor::BrightCyan
                .on_default()
                .bold(),
        )
        .header(
            clap::builder::styling::AnsiColor::BrightCyan
                .on_default()
                .bold(),
        )
        .literal(clap::builder::styling::AnsiColor::BrightGreen.on_default())
        .placeholder(clap::builder::styling::AnsiColor::BrightMagenta.on_default())
}
