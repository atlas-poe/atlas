# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

As Atlas is in early development, only the latest version receives security updates.

## Reporting a Vulnerability

If you discover a security vulnerability in Atlas, please report it responsibly.

**Do not open a public GitHub issue for security vulnerabilities.**

Instead, please email security concerns to: [security@atlas-poe.dev]

### What to include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix timeline**: Depends on severity

## Security Considerations

### API Keys and Tokens

Atlas will interact with Path of Exile APIs that may require authentication. We are committed to:

- Never hardcoding API keys or tokens in source code
- Using environment variables or secure credential storage
- Not logging sensitive authentication data
- Implementing proper token refresh mechanisms

### Network Security

- All external API calls use HTTPS
- Certificate validation is never disabled
- Rate limiting is respected to prevent abuse

### Data Handling

- Game data parsing occurs locally
- No user data is transmitted without explicit consent
- Clipboard data is processed locally and not stored persistently

### Dependencies

We regularly audit dependencies for known vulnerabilities using:

- `cargo audit` in CI pipelines
- GitHub Dependabot alerts
- Manual review of critical dependencies

## Best Practices for Contributors

When contributing to Atlas, please:

1. Never commit API keys, tokens, or credentials
2. Use environment variables for sensitive configuration
3. Validate and sanitize all external input
4. Follow Rust security best practices
5. Run `cargo audit` before submitting PRs

## Disclosure Policy

We follow coordinated disclosure:

1. Reporter privately notifies us
2. We acknowledge and investigate
3. We develop and test a fix
4. We release the fix
5. We publicly disclose the vulnerability

Thank you for helping keep Atlas and its users safe.
