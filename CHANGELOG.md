# Changelog

All notable changes to Atlas will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project scaffolding with Rust workspace architecture
- `atlas-core` crate placeholder for core domain logic
- CI/CD pipeline with check, test, clippy, and format jobs
- Git hooks for commit message validation and pre-push checks
- Contributing guide with development workflow documentation
- MIT License

### Changed

- Removed release workflow until binary target exists

## [0.1.0] - 2026-07-15

### Added

- Initial commit with project structure
- README with project description
- Rust toolchain configuration
- AI agent instructions (AGENTS.md)

---

## Release Plan

Atlas follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality
- **PATCH** version for backwards-compatible bug fixes

Releases are automated using Conventional Commits:
- `feat:` commits trigger MINOR version bumps
- `fix:` commits trigger PATCH version bumps
- `feat!:` or `BREAKING CHANGE:` commits trigger MAJOR version bumps

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on the release process.
