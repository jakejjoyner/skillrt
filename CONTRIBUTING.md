# Contributing to skillrt

Thanks for your interest. This project is open-source under MIT and we welcome contributions from humans and models alike.

## Quick channels

| If you want to... | Go to... |
|---|---|
| Report a bug | [GitHub Issues](https://github.com/jakejjoyner/skillrt/issues) |
| Propose a small improvement / bugfix | Pull request |
| Propose a change to the spec | Open an issue first; likely an RFC will follow |
| Write and publish a skill | See README's quick start; v0.2 will add `skill publish` |

## Principles

1. **The spec is the product.** Reference runtimes implement the spec. Spec changes go through RFCs; runtime changes follow the spec.
2. **Prose and structured skills must both be first-class.** No PR should degrade support for one in favour of the other.
3. **Forward compatibility matters.** Parsers preserve unknown fields. Runtimes ignore what they don't understand.
4. **Documentation is load-bearing.** If you add a feature, update `README.md`, `spec/`, `llms.txt`, and `CHANGELOG.md`. If you can't explain it to a model, you haven't finished it.

## Development setup

```sh
git clone https://github.com/jakejjoyner/skillrt
cd skillrt
cargo build
cargo test
./target/debug/skill --help
```

Rust 1.75+ required. The `2024` edition is used.

## Code style

- `rustfmt` is authoritative. CI fails on unformatted code. Run `cargo fmt` before committing.
- `clippy` with `-D warnings` must pass. Run `cargo clippy --all-targets --all-features`.
- Public API additions must have doc comments.
- Unit tests for parsing / validation changes are required.

## Commit style

- One logical change per commit.
- Present tense, imperative mood: `add X`, not `added X`.
- Body explains **why**, not what (the diff shows what).
- Link issues by number when relevant.

## RFCs

Non-trivial changes to the spec go through an RFC:

1. Copy `rfc/0000-template.md` (added in v0.1 follow-up) to `rfc/XXXX-your-proposal.md`.
2. Open a PR titled `RFC: your proposal`.
3. Iterate with reviewers until consensus.
4. Once merged, reference the RFC number in spec changes.

## Writing skills (for contributors who want to publish)

- Use the structured format unless the content is purely prose knowledge.
- Name skills in kebab-case: `pr-review`, not `prReview` or `PR_Review`.
- Declare all dependencies explicitly; runtimes should never auto-discover.
- Tag skills for discoverability.
- Write at least one example usage in the body.

## Reporting security issues

Email `security@<domain-TBD>` rather than opening a public issue. We'll reply within 72 hours.

## Code of conduct

Be kind. Attack ideas, not people. Assume good faith. If you wouldn't say it to a colleague's face, don't say it in a PR review.

## License

By contributing you agree your contributions are licensed under the MIT license, same as the project.
