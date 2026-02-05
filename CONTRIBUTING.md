# Contributing to Prep

First off, thank you for considering contributing to Prep! 🎉

## Code of Conduct

This project and everyone participating in it is governed by our commitment to making this a welcoming and inclusive space. Please be respectful and constructive in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

**When reporting a bug, include:**
- Your operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected vs actual behavior
- Any error messages (with `--verbose` flag output if applicable)

### Suggesting Enhancements

Enhancement suggestions are welcome! Please include:
- Clear description of the feature
- Use case / why this would be useful
- Any implementation ideas you have

### Pull Requests

1. Fork the repo and create your branch from `main`
2. Make your changes
3. Ensure the code compiles without warnings: `cargo build --release`
4. Run clippy: `cargo clippy -- -D warnings`
5. Format your code: `cargo fmt`
6. Update documentation if needed
7. Write a clear PR description

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/prep.git
cd prep

# Build
cargo build

# Run tests
cargo test

# Run with verbose output
cargo run -- --verbose "test prompt"
```

## Project Structure

```
src/
├── main.rs          # Entry point and CLI orchestration
├── lib.rs           # Library exports
├── cli.rs           # Clap CLI definitions
├── config.rs        # Configuration management
├── ui.rs            # Terminal UI components
├── refiner.rs       # Core refinement types
├── history.rs       # SQLite history storage
├── templates.rs     # Built-in templates
└── providers/       # AI provider implementations
    ├── mod.rs
    ├── ollama_local.rs
    ├── ollama_cloud.rs
    ├── openai.rs
    └── anthropic.rs
```

## Adding a New Provider

1. Create a new file in `src/providers/`
2. Implement the `Provider` trait
3. Add the provider to `src/providers/mod.rs`
4. Add CLI option in `src/cli.rs`
5. Add configuration in `src/config.rs`
6. Update the README

## Style Guide

- Follow Rust conventions and idioms
- Use `rustfmt` for formatting
- Document public APIs with doc comments
- Prefer explicit error handling over `.unwrap()`
- Keep functions focused and small

## Questions?

Feel free to open an issue for any questions about contributing!
