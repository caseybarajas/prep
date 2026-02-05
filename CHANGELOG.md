# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-04

### Added
- Initial public release
- Multi-provider support: Local Ollama, Ollama Cloud, OpenAI, Anthropic
- Beautiful terminal UI with colored output and progress spinners
- Configuration file support at `~/.config/prep/config.toml`
- Environment variable support for API keys
- SQLite-backed history tracking
- 10 built-in prompt templates (code, debug, docs, explain, review, refactor, test, api, security, architecture)
- Shell completions for Bash, Zsh, Fish, and PowerShell
- Clipboard integration with `--copy` flag
- Context file inclusion with `--context` flag
- Dry run mode with `--dry-run` flag
- Multiple output formats: text, JSON, markdown
- Interactive clarification flow when prompts need more detail
- Subcommands: `config`, `history`, `templates`, `completions`

### Security
- API keys loaded from environment variables, never stored in config files
- `.gitignore` includes common secret file patterns
