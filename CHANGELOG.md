# Changelog

All notable changes to Atlas will be documented in this file.

## [Unreleased]

### Added

- Initial project scaffolding with Rust workspace architecture
- `atlas-core` crate placeholder for core domain logic
- CI/CD pipeline with check, test, clippy, and format jobs
- Git hooks for commit message validation and pre-push checks
- Contributing guide with development workflow documentation
- MIT License
- PR templates for documentation, features, fixes, and chores
- GitHub Actions for auto-labeling PRs and populating changed files
- Architecture documentation

### Changed

- Removed release workflow until binary target exists

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
