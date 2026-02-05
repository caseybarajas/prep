use anyhow::{Context, Result};
use arboard::Clipboard;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use colored::control::set_override;
use dialoguer::Confirm;
use std::io::{self, Read, Write};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use prep::cli::{
    Cli, Commands, ConfigAction, HistoryAction, OutputFormat, ProviderChoice, TemplateAction,
};
use prep::config::Config;
use prep::history::History;
use prep::providers::{
    AnthropicProvider, OllamaCloudProvider, OllamaLocalProvider, OpenAIProvider, Provider,
};
use prep::refiner::{build_clarification_summary, RefinerResponse};
use prep::templates;
use prep::ui::UI;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\x1b[31m✗ Error: {:#}\x1b[0m", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Handle color override
    if cli.no_color {
        set_override(false);
    }

    // Setup logging if verbose
    if cli.verbose {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .without_time()
            .with_writer(io::stderr)
            .finish();
        tracing::subscriber::set_global_default(subscriber).ok();
    }

    // Load config
    let config = Config::load().context("Failed to load configuration")?;

    // Create UI helper
    let ui = UI::new(!cli.no_color && config.ui.color, config.ui.spinner);

    // Handle subcommands
    match cli.command {
        Some(Commands::Config { action }) => handle_config(action, &ui)?,
        Some(Commands::History { action }) => handle_history(action, &ui)?,
        Some(Commands::Templates { action }) => handle_templates(action, &ui)?,
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "prep", &mut io::stdout());
            return Ok(());
        }
        None => {
            // Main refinement flow
            return handle_refine(cli, config, ui).await;
        }
    }

    Ok(())
}

fn handle_config(action: ConfigAction, ui: &UI) -> Result<()> {
    match action {
        ConfigAction::Init { force } => {
            let path = Config::init(force)?;
            ui.success(&format!(
                "Configuration file created at: {}",
                path.display()
            ));
        }
        ConfigAction::Show => {
            let config = Config::load()?;
            let content = toml::to_string_pretty(&config)?;
            println!("{}", content);
        }
        ConfigAction::Path => {
            let path = Config::path()?;
            println!("{}", path.display());
        }
        ConfigAction::Get { key } => {
            let config = Config::load()?;
            match config.get(&key) {
                Some(value) => println!("{}", value),
                None => {
                    ui.error(&format!("Unknown configuration key: {}", key));
                    std::process::exit(1);
                }
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = Config::load()?;
            config.set(&key, &value)?;
            config.save()?;
            ui.success(&format!("Set {} = {}", key, value));
        }
    }
    Ok(())
}

fn handle_history(action: HistoryAction, ui: &UI) -> Result<()> {
    let history = History::open()?;

    match action {
        HistoryAction::List { limit } => {
            let entries = history.list(limit)?;
            if entries.is_empty() {
                ui.info("No history entries found.");
                return Ok(());
            }

            ui.header("Recent Refinements");
            for entry in entries {
                println!();
                ui.kv("ID", &entry.id.to_string());
                ui.kv(
                    "Date",
                    &entry.created_at.format("%Y-%m-%d %H:%M").to_string(),
                );
                ui.kv("Provider", &entry.provider);
                ui.kv("Model", &entry.model);

                let preview = entry.original_prompt.chars().take(60).collect::<String>();
                ui.kv("Prompt", &format!("{}...", preview));
            }
        }
        HistoryAction::Show { id } => match history.get(id)? {
            Some(entry) => {
                ui.header(&format!("History Entry #{}", id));
                println!();
                ui.kv(
                    "Date",
                    &entry.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                );
                ui.kv("Provider", &entry.provider);
                ui.kv("Model", &entry.model);
                println!();
                ui.boxed(&entry.original_prompt, Some("Original Prompt"));
                println!();
                ui.boxed(&entry.refined_prompt, Some("Refined Prompt"));
            }
            None => {
                ui.error(&format!("No history entry with ID {}", id));
                std::process::exit(1);
            }
        },
        HistoryAction::Search { query } => {
            let entries = history.search(&query)?;
            if entries.is_empty() {
                ui.info(&format!("No results for '{}'", query));
                return Ok(());
            }

            ui.header(&format!("Search Results for '{}'", query));
            for entry in entries {
                println!();
                ui.kv("ID", &entry.id.to_string());
                ui.kv(
                    "Date",
                    &entry.created_at.format("%Y-%m-%d %H:%M").to_string(),
                );
                let preview = entry.original_prompt.chars().take(60).collect::<String>();
                ui.kv("Prompt", &format!("{}...", preview));
            }
        }
        HistoryAction::Clear { force } => {
            if !force {
                let confirm = Confirm::new()
                    .with_prompt("Are you sure you want to clear all history?")
                    .default(false)
                    .interact()?;

                if !confirm {
                    ui.info("Cancelled.");
                    return Ok(());
                }
            }

            let count = history.clear()?;
            ui.success(&format!("Cleared {} history entries.", count));
        }
    }
    Ok(())
}

fn handle_templates(action: TemplateAction, ui: &UI) -> Result<()> {
    match action {
        TemplateAction::List => {
            ui.header("Available Templates");
            println!();
            for (name, desc) in templates::list_templates() {
                ui.list_item("•", &format!("{:<15} {}", name, desc));
            }
        }
        TemplateAction::Show { name } => match templates::get_template(&name) {
            Some(template) => {
                ui.header(&format!("Template: {}", name));
                println!();
                ui.kv("Description", template.description);
                println!();
                ui.boxed(template.prefix.trim(), Some("Prefix"));
                println!();
                ui.boxed(template.suffix.trim(), Some("Suffix"));
            }
            None => {
                ui.error(&format!("Unknown template: {}", name));
                ui.info("Run 'prep templates list' to see available templates.");
                std::process::exit(1);
            }
        },
        TemplateAction::Use { name } => match templates::get_template(&name) {
            Some(template) => {
                ui.info(&format!("Using template '{}'. Enter your prompt:", name));
                print!("> ");
                io::stdout().flush()?;

                let mut prompt = String::new();
                io::stdin().read_line(&mut prompt)?;

                let result = template.apply(prompt.trim());
                println!("\n{}", result);
            }
            None => {
                ui.error(&format!("Unknown template: {}", name));
                std::process::exit(1);
            }
        },
    }
    Ok(())
}

async fn handle_refine(cli: Cli, config: Config, ui: UI) -> Result<()> {
    // Get the raw prompt
    let raw_prompt = get_prompt(&cli)?;

    if raw_prompt.is_empty() {
        anyhow::bail!("No prompt provided. Pass a prompt as arguments or pipe it via stdin.\n\nUsage: prep \"your prompt here\"\n       echo \"your prompt\" | prep");
    }

    // Apply template if specified
    let raw_prompt = if let Some(template_name) = &cli.template {
        let template = templates::get_template(template_name)
            .with_context(|| format!("Unknown template: {}", template_name))?;
        template.apply(&raw_prompt)
    } else {
        raw_prompt
    };

    // Load context file if specified
    let context = if let Some(path) = &cli.context {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read context file: {}", path.display()))?;
        Some(content)
    } else {
        None
    };

    // Determine provider
    let provider_choice = cli.provider.unwrap_or_else(|| {
        config
            .get_default_provider()
            .unwrap_or(ProviderChoice::OllamaLocal)
    });

    // Get model
    let model = config.get_model(provider_choice, cli.model.as_deref());

    // Get API key
    let api_key = config.get_api_key(provider_choice, cli.api_key.as_deref());

    // Get endpoint
    let endpoint = config.get_endpoint(provider_choice);

    if cli.verbose {
        ui.debug("Provider", &format!("{}", provider_choice));
        ui.debug("Model", &model);
        ui.debug("Endpoint", &endpoint);
        if cli.dry_run {
            ui.debug("Mode", "Dry run");
        }
    }

    // Handle dry run
    if cli.dry_run {
        ui.header("Dry Run");
        ui.kv("Provider", &format!("{}", provider_choice));
        ui.kv("Model", &model);
        ui.kv("Endpoint", &endpoint);
        println!();
        ui.boxed(&raw_prompt, Some("Prompt to be sent"));
        if let Some(ctx) = &context {
            println!();
            ui.boxed(ctx, Some("Context"));
        }
        return Ok(());
    }

    // Create provider
    let provider: Box<dyn Provider> = match provider_choice {
        ProviderChoice::OllamaLocal => Box::new(OllamaLocalProvider::new(endpoint, model.clone())),
        ProviderChoice::OllamaCloud => {
            let key = api_key.context(
                "Ollama Cloud requires an API key. Set OLLAMA_API_KEY environment variable or use --api-key."
            )?;
            Box::new(OllamaCloudProvider::new(endpoint, model.clone(), key))
        }
        ProviderChoice::OpenAI => {
            let key = api_key.context(
                "OpenAI requires an API key. Set OPENAI_API_KEY environment variable or use --api-key."
            )?;
            Box::new(OpenAIProvider::new(endpoint, model.clone(), key))
        }
        ProviderChoice::Anthropic => {
            let key = api_key.context(
                "Anthropic requires an API key. Set ANTHROPIC_API_KEY environment variable or use --api-key."
            )?;
            Box::new(AnthropicProvider::new(endpoint, model.clone(), key))
        }
    };

    // First refinement call
    let spinner = ui.spinner(&format!(
        "Refining prompt with {} ({})...",
        provider.name(),
        provider.model()
    ));

    let response = provider.refine(&raw_prompt, context.as_deref(), None).await;

    if let Some(pb) = spinner {
        pb.finish_and_clear();
    }

    let response = response?;

    // Handle clarification if needed
    let final_response = if response.needs_clarification && !response.questions.is_empty() {
        // Check if we're in an interactive terminal
        if !atty::is(atty::Stream::Stdin) {
            // Non-interactive mode: output questions and use initial refined prompt
            ui.warning("Clarification needed but running non-interactively.");
            ui.info("Questions the AI wanted to ask:");
            for (i, q) in response.questions.iter().enumerate() {
                eprintln!("  Q{}: {}", i + 1, q);
            }
            ui.info("Using initial refined prompt. Re-run interactively for better results.");
            response
        } else {
            let answers = ui.ask_questions(&response.questions)?;
            let summary = build_clarification_summary(&response.questions, &answers);

            let spinner = ui.spinner("Refining with clarifications...");

            let final_resp = provider
                .refine(&raw_prompt, context.as_deref(), Some(&summary))
                .await;

            if let Some(pb) = spinner {
                pb.finish_and_clear();
            }

            final_resp?
        }
    } else {
        response
    };

    // Output the result
    output_result(&cli, &final_response, &ui)?;

    // Copy to clipboard if requested
    if cli.copy || config.default.copy_to_clipboard {
        match Clipboard::new() {
            Ok(mut clipboard) => {
                clipboard.set_text(&final_response.refined_prompt)?;
                ui.success("Copied to clipboard!");
            }
            Err(e) => {
                ui.warning(&format!("Could not copy to clipboard: {}", e));
            }
        }
    }

    // Save to history
    if config.history.enabled && !cli.no_history {
        if let Ok(history) = History::open() {
            let _ = history.add(
                &raw_prompt,
                &final_response.refined_prompt,
                &format!("{}", provider_choice),
                &model,
            );
            // Prune old entries
            let _ = history.prune(config.history.max_entries);
        }
    }

    Ok(())
}

fn get_prompt(cli: &Cli) -> Result<String> {
    if !cli.prompt.is_empty() {
        return Ok(cli.prompt.join(" "));
    }

    // Check if stdin has data
    if atty::is(atty::Stream::Stdin) {
        return Ok(String::new());
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn output_result(cli: &Cli, response: &RefinerResponse, _ui: &UI) -> Result<()> {
    match cli.output {
        OutputFormat::Text => {
            // Print to stdout for piping
            println!("{}", response.refined_prompt);
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(response)?;
            println!("{}", json);
        }
        OutputFormat::Markdown => {
            println!("## Refined Prompt\n");
            println!("{}\n", response.refined_prompt);
            if response.needs_clarification && !response.questions.is_empty() {
                println!("### Clarification Questions\n");
                for (i, q) in response.questions.iter().enumerate() {
                    println!("{}. {}", i + 1, q);
                }
            }
        }
    }
    Ok(())
}
