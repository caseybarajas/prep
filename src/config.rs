use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::cli::ProviderChoice;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultConfig,
    #[serde(default)]
    pub providers: ProvidersConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub history: HistoryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_output_format")]
    pub output_format: String,
    #[serde(default)]
    pub copy_to_clipboard: bool,
}

fn default_provider() -> String {
    "ollama".to_string()
}

fn default_model() -> String {
    "llama3.2".to_string()
}

fn default_output_format() -> String {
    "text".to_string()
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: default_model(),
            output_format: default_output_format(),
            copy_to_clipboard: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvidersConfig {
    #[serde(default, rename = "ollama-local")]
    pub ollama_local: OllamaLocalConfig,
    #[serde(default, rename = "ollama-cloud")]
    pub ollama_cloud: OllamaCloudConfig,
    #[serde(default)]
    pub openai: OpenAIConfig,
    #[serde(default)]
    pub anthropic: AnthropicConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaLocalConfig {
    #[serde(default = "default_ollama_local_endpoint")]
    pub endpoint: String,
    pub model: Option<String>,
}

fn default_ollama_local_endpoint() -> String {
    "http://localhost:11434".to_string()
}

impl Default for OllamaLocalConfig {
    fn default() -> Self {
        Self {
            endpoint: default_ollama_local_endpoint(),
            model: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaCloudConfig {
    #[serde(default = "default_ollama_cloud_endpoint")]
    pub endpoint: String,
    pub model: Option<String>,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
}

fn default_ollama_cloud_endpoint() -> String {
    "https://api.ollama.com".to_string()
}

impl Default for OllamaCloudConfig {
    fn default() -> Self {
        Self {
            endpoint: default_ollama_cloud_endpoint(),
            model: None,
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    #[serde(default = "default_openai_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_openai_model")]
    pub model: String,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
}

fn default_openai_endpoint() -> String {
    "https://api.openai.com/v1".to_string()
}

fn default_openai_model() -> String {
    "gpt-4o".to_string()
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            endpoint: default_openai_endpoint(),
            model: default_openai_model(),
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    #[serde(default = "default_anthropic_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_anthropic_model")]
    pub model: String,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
}

fn default_anthropic_endpoint() -> String {
    "https://api.anthropic.com/v1".to_string()
}

fn default_anthropic_model() -> String {
    "claude-3-5-sonnet".to_string()
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self {
            endpoint: default_anthropic_endpoint(),
            model: default_anthropic_model(),
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "bool_true")]
    pub color: bool,
    #[serde(default = "bool_true")]
    pub spinner: bool,
}

fn bool_true() -> bool {
    true
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            color: true,
            spinner: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    #[serde(default = "bool_true")]
    pub enabled: bool,
    #[serde(default = "default_max_entries")]
    pub max_entries: usize,
}

fn default_max_entries() -> usize {
    1000
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: default_max_entries(),
        }
    }
}



impl Config {
    /// Get the configuration file path
    pub fn path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("com", "prep", "prep")
            .context("Could not determine config directory")?;
        Ok(dirs.config_dir().join("config.toml"))
    }

    /// Load configuration from file, falling back to defaults
    pub fn load() -> Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let mut config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        // Load API keys from environment variables
        config.providers.ollama_cloud.api_key = std::env::var("OLLAMA_API_KEY").ok();
        config.providers.openai.api_key = std::env::var("OPENAI_API_KEY").ok();
        config.providers.anthropic.api_key = std::env::var("ANTHROPIC_API_KEY").ok();

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Initialize a new config file with defaults
    pub fn init(force: bool) -> Result<PathBuf> {
        let path = Self::path()?;

        if path.exists() && !force {
            anyhow::bail!(
                "Config file already exists at {}. Use --force to overwrite.",
                path.display()
            );
        }

        let config = Self::default();
        config.save()?;

        Ok(path)
    }

    /// Get the default model for a provider
    pub fn get_model(&self, provider: ProviderChoice, cli_model: Option<&str>) -> String {
        if let Some(model) = cli_model {
            return model.to_string();
        }

        match provider {
            ProviderChoice::OllamaLocal => self
                .providers
                .ollama_local
                .model
                .clone()
                .unwrap_or_else(|| self.default.model.clone()),
            ProviderChoice::OllamaCloud => self
                .providers
                .ollama_cloud
                .model
                .clone()
                .unwrap_or_else(|| self.default.model.clone()),
            ProviderChoice::OpenAI => self.providers.openai.model.clone(),
            ProviderChoice::Anthropic => self.providers.anthropic.model.clone(),
        }
    }

    /// Get API key for a provider
    pub fn get_api_key(&self, provider: ProviderChoice, cli_key: Option<&str>) -> Option<String> {
        if let Some(key) = cli_key {
            return Some(key.to_string());
        }

        match provider {
            ProviderChoice::OllamaLocal => None,
            ProviderChoice::OllamaCloud => self.providers.ollama_cloud.api_key.clone(),
            ProviderChoice::OpenAI => self.providers.openai.api_key.clone(),
            ProviderChoice::Anthropic => self.providers.anthropic.api_key.clone(),
        }
    }

    /// Get endpoint for a provider
    pub fn get_endpoint(&self, provider: ProviderChoice) -> String {
        match provider {
            ProviderChoice::OllamaLocal => self.providers.ollama_local.endpoint.clone(),
            ProviderChoice::OllamaCloud => self.providers.ollama_cloud.endpoint.clone(),
            ProviderChoice::OpenAI => self.providers.openai.endpoint.clone(),
            ProviderChoice::Anthropic => self.providers.anthropic.endpoint.clone(),
        }
    }

    /// Get a value by dot-notation key
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "default.provider" => Some(self.default.provider.clone()),
            "default.model" => Some(self.default.model.clone()),
            "default.output_format" => Some(self.default.output_format.clone()),
            "default.copy_to_clipboard" => Some(self.default.copy_to_clipboard.to_string()),
            "ui.color" => Some(self.ui.color.to_string()),
            "ui.spinner" => Some(self.ui.spinner.to_string()),
            "history.enabled" => Some(self.history.enabled.to_string()),
            "history.max_entries" => Some(self.history.max_entries.to_string()),
            _ => None,
        }
    }

    /// Set a value by dot-notation key
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "default.provider" => self.default.provider = value.to_string(),
            "default.model" => self.default.model = value.to_string(),
            "default.output_format" => self.default.output_format = value.to_string(),
            "default.copy_to_clipboard" => {
                self.default.copy_to_clipboard = value.parse().context("Invalid boolean value")?;
            }
            "ui.color" => {
                self.ui.color = value.parse().context("Invalid boolean value")?;
            }
            "ui.spinner" => {
                self.ui.spinner = value.parse().context("Invalid boolean value")?;
            }
            "history.enabled" => {
                self.history.enabled = value.parse().context("Invalid boolean value")?;
            }
            "history.max_entries" => {
                self.history.max_entries = value.parse().context("Invalid number")?;
            }
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }
        Ok(())
    }

    /// Get default provider from config
    pub fn get_default_provider(&self) -> Result<ProviderChoice> {
        match self.default.provider.as_str() {
            "ollama" | "ollama-local" | "local" => Ok(ProviderChoice::OllamaLocal),
            "ollama-cloud" | "cloud" => Ok(ProviderChoice::OllamaCloud),
            "openai" | "gpt" => Ok(ProviderChoice::OpenAI),
            "anthropic" | "claude" => Ok(ProviderChoice::Anthropic),
            other => anyhow::bail!("Unknown provider in config: {}", other),
        }
    }
}
