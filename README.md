<div align="center">

# ✨ prep

**Transform messy prompts into precise, powerful instructions for AI assistants.**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

[Features](#features) •
[Installation](#installation) •
[Quick Start](#quick-start) •
[Providers](#providers) •
[Configuration](#configuration) •
[Documentation](#documentation)

</div>

---

## Why Prep?

Ever type a vague prompt like *"make a website"* and get mediocre results? **Prep** transforms your casual thoughts into precise, actionable prompts that get better results from any AI assistant.

```bash
$ prep "make a website for my cat photos"

# Output:
Build a responsive single-page website to showcase cat photos with the following 
requirements: a masonry-style photo grid layout, lightbox functionality for 
enlarged viewing, lazy loading for performance, mobile-first responsive design, 
and a simple navigation header. Use HTML5, CSS3 with Flexbox/Grid, and vanilla 
JavaScript. Include alt text placeholders for accessibility.
```

## Features

🔄 **Multi-Provider Support** — Local Ollama, Ollama Cloud, OpenAI, Anthropic  
🎨 **Beautiful Terminal UI** — Colored output, progress spinners, interactive prompts  
⚙️ **Persistent Configuration** — Set your defaults once, use everywhere  
📋 **Clipboard Integration** — Copy refined prompts with `--copy`  
📜 **History Tracking** — SQLite-backed history with search  
📝 **Prompt Templates** — 10 built-in templates for common tasks  
🐚 **Shell Completions** — Bash, Zsh, Fish, PowerShell  

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/caseybarajas/prep.git
cd prep

# Build and install
cargo install --path .
```

### Prerequisites

- **Rust 1.70+** — [Install Rust](https://rustup.rs/)
- **AI Provider** — One of:
  - [Ollama](https://ollama.ai) running locally, OR
  - API key for Ollama Cloud, OpenAI, or Anthropic

## Quick Start

### Basic Usage

```bash
# Simple prompt refinement
prep "write a python script to parse json"

# Pipe from file or other commands
cat rough_idea.txt | prep

# Use with a specific provider
prep --provider openai "build a REST API"
```

### With Ollama (Local)

```bash
# Start Ollama
ollama serve

# Pull a model
ollama pull llama3.2

# Use prep
prep "create a dockerfile"
```

### With Cloud Providers

```bash
# Set your API key
export OPENAI_API_KEY="sk-..."
# OR
export ANTHROPIC_API_KEY="sk-ant-..."
# OR
export OLLAMA_API_KEY="..."

# Use the cloud provider
prep --provider openai "design a database schema"
prep --provider anthropic "write unit tests"
prep --provider ollama-cloud "explain microservices"
```

## Providers

| Provider | Flag | Model Default | API Key Variable |
|----------|------|---------------|------------------|
| Ollama (Local) | `--provider ollama` | `llama3.2` | Not required |
| Ollama Cloud | `--provider ollama-cloud` | `llama3.2` | `OLLAMA_API_KEY` |
| OpenAI | `--provider openai` | `gpt-4o` | `OPENAI_API_KEY` |
| Anthropic | `--provider anthropic` | `claude-3-5-sonnet` | `ANTHROPIC_API_KEY` |

## Configuration

### Initialize Config File

```bash
prep config init
```

Configuration is stored at `~/.config/prep/config.toml`:

```toml
[default]
provider = "ollama"           # Default provider
model = "llama3.2"            # Default model
output_format = "text"        # text, json, or markdown
copy_to_clipboard = false     # Auto-copy results

[providers.ollama-local]
endpoint = "http://localhost:11434"

[providers.openai]
endpoint = "https://api.openai.com/v1"
model = "gpt-4o"

[providers.anthropic]
endpoint = "https://api.anthropic.com/v1"
model = "claude-3-5-sonnet"

[ui]
color = true                  # Colored output
spinner = true                # Show progress spinners

[history]
enabled = true                # Track refinement history
max_entries = 1000            # Max history entries
```

### Config Commands

```bash
prep config show              # Display current config
prep config set KEY VALUE     # Set a value
prep config get KEY           # Get a value
prep config path              # Show config file location
```

## Documentation

### Command Reference

```
prep [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  Raw prompt to refine (reads from stdin if not provided)

Options:
  -p, --provider <NAME>    AI provider (ollama, ollama-cloud, openai, anthropic)
  -m, --model <NAME>       Model to use (overrides config)
  -o, --output <FORMAT>    Output format: text, json, markdown
  -C, --copy               Copy result to clipboard
  -t, --template <NAME>    Use a prompt template
      --context <FILE>     Include file as additional context
      --dry-run            Preview without calling API
  -v, --verbose            Show diagnostic output
      --no-color           Disable colored output
      --no-history         Don't save to history
  -h, --help               Print help
  -V, --version            Print version

Subcommands:
  config       Manage configuration
  history      View and manage refinement history
  templates    Work with prompt templates
  completions  Generate shell completions
```

### Templates

Built-in templates optimize prompts for specific tasks:

| Template | Description |
|----------|-------------|
| `code` | Code generation requests |
| `debug` | Debugging assistance |
| `docs` | Documentation writing |
| `explain` | Concept explanations |
| `review` | Code review requests |
| `refactor` | Refactoring tasks |
| `test` | Test writing |
| `api` | API design |
| `security` | Security analysis |
| `architecture` | System architecture |

```bash
# List all templates
prep templates list

# Use a template
prep --template code "parse CSV files in rust"
prep --template debug "my function returns null"
```

### History

```bash
# List recent refinements
prep history list

# Show specific entry
prep history show 42

# Search history
prep history search "python"

# Clear all history
prep history clear
```

### Shell Completions

```bash
# Bash
prep completions bash > ~/.local/share/bash-completion/completions/prep

# Zsh (add ~/.zfunc to fpath in .zshrc first)
prep completions zsh > ~/.zfunc/_prep

# Fish
prep completions fish > ~/.config/fish/completions/prep.fish

# PowerShell
prep completions powershell >> $PROFILE
```

## Examples

### Basic Refinement

```bash
$ prep "make api"

# Output:
Design and implement a RESTful API with the following specifications: 
define the resource endpoints (GET, POST, PUT, DELETE), implement proper 
HTTP status codes, add request validation, include error handling with 
meaningful messages, and document the endpoints. Specify the programming 
language and framework to use.
```

### With Context File

```bash
$ prep --context src/main.rs "add error handling"

# The context file content is sent along with your prompt
# for more relevant refinements
```

### Different Output Formats

```bash
# JSON output (for scripting)
prep --output json "write tests" | jq .refined_prompt

# Markdown (for documentation)
prep --output markdown "explain oauth"
```

### Pipeline Integration

```bash
# Refine and pass to another tool
prep "write a greeting function" | pbcopy

# Use with AI coding assistants
prep "add caching to this function" --context utils.py | claude
```

## Security

- API keys are **never** stored in config files
- Keys are loaded from environment variables only
- History is stored locally in SQLite
- See [SECURITY.md](SECURITY.md) for details

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License — see [LICENSE](LICENSE) for details.

---

<div align="center">

**[⬆ Back to Top](#-prep)**

Made with ❤️ and Rust

</div>
