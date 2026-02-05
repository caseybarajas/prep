# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Prep, please report it responsibly:

1. **Do NOT** open a public GitHub issue
2. Email the maintainer directly with details
3. Include steps to reproduce if possible
4. Allow reasonable time for a fix before public disclosure

## Security Considerations

### API Keys

- **Never commit API keys** to version control
- API keys are loaded from environment variables only, never stored in config files
- Use `--api-key` flag for one-time use (key will be visible in shell history)
- Prefer environment variables: `OLLAMA_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`

### Local Data Storage

- Configuration: `~/.config/prep/config.toml` - does not contain secrets
- History database: `~/.local/share/prep/history.db` - contains your prompts
- Clear history with: `prep history clear`

### Network Security

- All cloud API calls use HTTPS
- No data is sent to any service other than the configured AI provider
- Local Ollama mode keeps all data on your machine

### Best Practices

```bash
# Good: Use environment variables
export OPENAI_API_KEY="sk-..."
prep "my prompt"

# Okay: Use --api-key (visible in shell history)
prep --api-key "sk-..." "my prompt"

# Bad: Never put keys in config files or scripts
```

## Dependencies

We regularly update dependencies to patch security vulnerabilities. Run `cargo audit` to check for known vulnerabilities in dependencies.
