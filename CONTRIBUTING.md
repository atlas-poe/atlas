# Contributing to Atlas

Thank you for your interest in contributing to Atlas!

Please note that this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Getting Started

### Prerequisites

- Rust toolchain (stable) - [Install Rust](https://www.rust-lang.org/tools/install)
- Git
- GitHub account

### Setup

1. Fork and clone the repository
2. Run the setup script to install git hooks:
   ```bash
   ./scripts/setup-hooks.sh
   ```
3. Create a branch for your changes:
   ```bash
   git checkout -b feat/my-new-feature
   ```

### Project Structure

Atlas uses a Rust workspace with modular crate architecture. See [Architecture](docs/architecture.md) for details on the project structure and design decisions.

## Development Workflow

### Commit Messages

Atlas uses [Conventional Commits](https://www.conventionalcommits.org/) for commit messages. This enables automatic versioning and changelog generation.

**Format:** `<type>[scope][!]: <description>`

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `build`: Build system changes
- `ci`: CI/CD changes
- `chore`: Other changes
- `revert`: Reverting a previous commit

**Examples:**
```
feat: add clipboard parsing
fix(api): correct response handling
feat!: redesign public API
docs: update README with setup instructions
```

Adding `!` after the type indicates a breaking change.

### Pre-Push Checks

Before pushing, the following checks run automatically:
- Code formatting (`cargo fmt`)
- Linting (`cargo clippy`)
- Tests (`cargo test`)

All checks must pass before you can push.

### Pull Requests

1. Ensure your branch is up to date with `main`
2. Push your changes and create a pull request
3. Select the appropriate PR template (see Branch Naming below)
4. Wait for CI checks to pass
5. Request a review

#### Branch Naming

Branch names determine PR labels automatically:

| Branch Prefix | Label | Template |
|---------------|-------|----------|
| `feat/` | `feature` | Feature implementation |
| `fix/` | `bug` | Bug fix |
| `docs/` | `documentation` | Documentation changes |
| `chore/` | `chore` | Maintenance tasks |
| `refactor/` | `refactor` | Code refactoring |
| `test/` | `test` | Test additions/updates |
| `ci/` | `ci` | CI/CD changes |

Example: `feat/clipboard-parsing` → auto-labeled `feature`

## Release Process

Atlas uses automatic semantic versioning based on conventional commits:

- `fix:` commits → patch version bump (0.1.0 → 0.1.1)
- `feat:` commits → minor version bump (0.1.0 → 0.2.0)
- `feat!:` or `BREAKING CHANGE:` → major version bump (0.x.0 → 1.0.0)

When you merge a PR to `main` with conventional commits:
1. The CI workflow determines the version bump
2. Updates `Cargo.toml` with the new version
3. Generates a `CHANGELOG.md`
4. Builds release binaries for Linux, macOS, and Windows
5. Creates a GitHub release with the binaries

## Code Style

- Follow Rust idioms and conventions
- Use `cargo fmt` to format code
- Run `cargo clippy` to catch common mistakes
- Write tests for new functionality

## Security

If you discover a security vulnerability, please follow our [Security Policy](SECURITY.md) for responsible disclosure.

## Questions?

If you have questions about contributing, feel free to open an issue or reach out to the maintainers.
