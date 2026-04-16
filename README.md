# skillrt

> **A runtime and spec for executable markdown skills consumed by AI agents.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Status: alpha](https://img.shields.io/badge/status-alpha-orange.svg)](#status)

---

## What it is, in one paragraph

AI agents already read markdown files to learn how to do things. Claude Code reads `CLAUDE.md` + `.claude/skills/*.md`. Cursor reads `.cursor/rules`. Aider reads conventions. Cal Paterson's [Soapstones](https://soapstones.calpaterson.com/) distributes prose-style notes between agents. Every tool has its own conventions, its own schema, and its own runtime. **`skillrt` is the unified spec, CLI, and runtime for executable markdown skills - so a skill written once runs across every agent framework that adopts the spec.**

It supports two modes side-by-side:

| Mode | What it is | When it wins |
|---|---|---|
| **Prose** | Free-form markdown, no frontmatter. Compatible with Soapstones. | Sharing how-to knowledge an agent will read and interpret. |
| **Structured** | Typed `SKILL.md` with frontmatter: name, version, inputs, outputs, dependencies, permissions. | Procedures an agent invokes with arguments and gets typed output. |

One registry, one runtime, both content types.

## Status

**v0.1 shipped.** The spec draft, parser, validator, and local `install`/`list`/`info` commands are live. Install from source today; see the [CHANGELOG](./CHANGELOG.md) for what landed.

**Next up: v0.2 — hosted registry.** Publish and install skills by name from a public registry, with semver resolution and signed releases. See [ROADMAP.md](./ROADMAP.md) for the full six-milestone plan through execution, shared memory, and observability.

Everything is open-source (MIT). Breaking changes are likely until v0.2.

## Install

```sh
# From source (Rust 1.75+ required)
git clone https://github.com/jakejjoyner/skillrt
cd skillrt
cargo install --path .

# verify
skill --version
```

Pre-built binaries and a Homebrew tap land in v0.2.

## Quick start

### 1. Write a structured skill

```markdown
---
name: pr-review
version: 0.1.0
description: Review a GitHub pull request and report findings as JSON.
authors: [you@example.com]
license: MIT
runtime:
  type: markdown-skill
inputs:
  - name: pr_url
    type: url
    required: true
outputs:
  type: json
dependencies:
  tools: [gh]
tags: [github, code-review]
---

# Review a pull request

1. Run `gh pr view {{ pr_url }} --json title,body,files`.
2. Check for missing tests, TODO comments, console.logs, new `any` types.
3. Emit a JSON object: `{ "findings": [{ "file": str, "line": int, "issue": str }] }`.
```

Save as `pr-review/SKILL.md`.

### 2. Validate and install

```sh
skill validate pr-review/SKILL.md
# ok (structured) pr-review@0.1.0

skill install pr-review/SKILL.md
# installed to ~/.local/share/skillrt/skills/pr-review/0.1.0/SKILL.md

skill list
# pr-review@0.1.0  Review a GitHub pull request and report findings as JSON.
```

### 3. An AI agent consumes it

In v0.1 the runtime prints the skill body; your agent (Claude Code, Cursor, etc.) consumes it as instruction context. In v0.2 `skill run pr-review --inputs '{"pr_url":"..."}'` will invoke it end-to-end with permission enforcement and structured output.

### Prose-mode (Soapstones-compatible)

A `SKILL.md` with no frontmatter is treated as a prose soapstone - free-form markdown the agent reads and interprets:

```markdown
# Finding UK train times

traintimes.org.uk has a clean URL scheme:
  https://traintimes.org.uk/{origin}/{destination}/{time}/{date}
  https://traintimes.org.uk/KGX/MAN/09:00/today
...
```

Same `skill validate`, `skill install`, `skill list` commands apply.

## Why skillrt exists

Read the [spec rationale](./spec/SKILL-SPEC.md#rationale) and [llms.txt](./llms.txt) (machine-friendly project summary for AI models trying to understand the repo).

## Design principles

1. **Markdown is the primary artifact.** Anything that adds friction between "write a markdown file" and "an agent runs it" is wrong.
2. **Humans and models both must be able to read the spec fluently.** Every concept has a plain-English explanation and a typed schema.
3. **Prose and structured skills coexist in the same registry.** Nobody is forced to learn YAML to share knowledge.
4. **No lock-in.** The spec is the product. The runtime is a reference implementation. Anyone can ship a compatible runtime in any language.
5. **Soapstones-compatible.** Prose notes published under the Soapstones conventions parse as valid prose skills with zero migration.

## Project layout

```
skillrt/
├── src/                  # Rust crate: parser, runtime, registry, CLI
├── spec/                 # Authoritative SKILL.md specification
│   └── SKILL-SPEC.md
├── rfc/                  # Design RFCs (versioned proposals)
├── examples/             # Reference skills (prose + structured)
├── docs/                 # Architecture, internals
├── llms.txt              # Machine-friendly summary for AI models
├── ROADMAP.md            # Six-milestone development plan
├── CONTRIBUTING.md       # How to propose changes / file RFCs
├── CHANGELOG.md
├── README.md             # (this file)
└── LICENSE               # MIT
```

## Contributing

This is an open-source spec with an open-source reference runtime. See [CONTRIBUTING.md](./CONTRIBUTING.md).

Before proposing a change that modifies the spec, please open an issue or draft an RFC.

## Acknowledgments

- [Cal Paterson's Soapstones](https://soapstones.calpaterson.com/) proved the prose-first distribution pattern and inspired the name-free, anonymous-contribution ethos.
- Anthropic's [Claude Code](https://docs.claude.com/en/docs/claude-code/overview), [Model Context Protocol](https://modelcontextprotocol.io), and the broader [Claude Agent SDK](https://docs.claude.com/en/api/agent-sdk) skill conventions informed the frontmatter schema.
- The Rust async ecosystem (`tokio`, `serde`, `clap`) for making a single-binary distribution feasible.

## License

[MIT](./LICENSE).
