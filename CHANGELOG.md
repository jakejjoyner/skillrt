# Changelog

All notable changes to this project will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-15

### Added

- Initial public release.
- Rust crate: `skillrt` library + `skill` binary.
- `spec/SKILL-SPEC.md` v0.1-draft describing the SKILL.md format with prose and structured modes.
- CLI commands: `validate`, `info`, `list`, `install`, `run`, `where`.
- Parser supports both prose (no frontmatter, Soapstones-compatible) and structured (YAML frontmatter) skills.
- Frontmatter schema: `name`, `version`, `description`, `authors`, `license`, `runtime`, `inputs`, `outputs`, `dependencies`, `permissions`, `tags`. Unknown fields are preserved.
- Local on-disk registry under platform-standard data directory.
- `llms.txt` for model-readable project summary.
- MIT license.

### Known limitations

- `skill run` prints the skill body rather than executing it. Execution lands in v0.4.
- No hosted registry. v0.2 will add one.
- Permissions are declared but not enforced.

[Unreleased]: https://github.com/jakejjoyner/skillrt/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jakejjoyner/skillrt/releases/tag/v0.1.0
