# SKILL.md Specification

**Version:** 0.1-draft
**Status:** Draft (subject to breaking change until v1.0)
**License:** Same as the skillrt project - MIT.

---

## 1. Purpose

This document defines the `SKILL.md` format: a portable, AI-agent-consumable markdown file that describes either a piece of shared knowledge (prose) or an executable procedure (structured).

Any tool may parse, validate, and execute `SKILL.md` files that conform to this spec. The `skillrt` project ships a reference implementation in Rust; a conforming implementation in any other language is welcome.

## 2. Rationale

AI agents read markdown to learn how to do things. Today every agent framework has invented its own conventions:

- Claude Code skills use one YAML frontmatter dialect.
- Cursor rules use another.
- Soapstones distributes prose notes with no frontmatter.
- The Anthropic Agent SDK defines a third variant.

Fragmentation taxes every author of a skill: write it three ways, or pick one tool and abandon the others. `SKILL.md` consolidates these conventions into a single spec, compatible at the file-format level with most existing tools, while adding the properties needed for multi-agent distribution:

1. **Typed inputs and outputs** so agents can invoke skills reliably without reading prose.
2. **Declared dependencies** so runtimes can resolve required tools and MCP servers before execution.
3. **Explicit permissions** so sandboxed runtimes can enforce what a skill may do.
4. **Versioning** so an agent can pin a skill version and expect deterministic behaviour.
5. **Prose compatibility** so existing Soapstones-style notes parse without modification.

## 3. File overview

A `SKILL.md` is a UTF-8 text file ending in `.md`. It has one of two shapes:

### 3.1 Prose mode

No frontmatter. Free-form markdown. The agent reads and interprets the content.

```markdown
# Finding UK train times

traintimes.org.uk has a clean URL scheme...
```

### 3.2 Structured mode

Begins with a YAML frontmatter block bracketed by `---` on its own line:

```markdown
---
name: pr-review
version: 0.1.0
description: Review a GitHub pull request.
---

# Body here...
```

A parser MUST determine the mode by whether the file begins (after optional BOM and whitespace) with the line `---`. If so, the file is structured. Otherwise it is prose.

## 4. Frontmatter schema (structured mode)

The YAML block between the opening `---` and the first subsequent `---` on its own line. The schema below is given as both human-readable prose and (at the end of this section) a JSON Schema.

### 4.1 Required fields

| Field | Type | Notes |
|---|---|---|
| `name` | string | Globally unique identifier within a namespace. Lowercase kebab-case is recommended (`pr-review`, not `PR_Review`). |
| `version` | string | SemVer 2.0.0. |
| `description` | string | One-line summary. ≤120 characters recommended. |

### 4.2 Optional fields

| Field | Type | Default | Notes |
|---|---|---|---|
| `authors` | list of strings | `[]` | Free-form author identifiers (emails, handles, URLs). |
| `license` | string | `"UNLICENSED"` | SPDX identifier. |
| `runtime` | object | see 4.3 | Declares which runtime family this skill targets. |
| `inputs` | list of objects | `[]` | See 4.4. |
| `outputs` | object | `null` | See 4.5. |
| `dependencies` | object | see 4.6 | Declares MCP servers, sibling skills, and external CLI tools. |
| `permissions` | object | see 4.7 | Network, filesystem, and environment-variable declarations. |
| `tags` | list of strings | `[]` | For discoverability in registries. |

Unknown fields MUST be preserved by parsers (reserved for future extensions). Implementations MUST NOT error on unknown fields unless they begin with `x-runtime-strict:` (reserved).

### 4.3 `runtime`

```yaml
runtime:
  type: markdown-skill    # string, identifies the runtime family
  min-version: "0.1.0"    # semver, optional
```

`type` identifies the family of runtime that can execute this skill. Initial values:

- `markdown-skill` - the skillrt reference runtime; this spec.
- `anthropic-skill` - the Anthropic Agent SDK skill convention (planned compatibility shim).
- `mcp-tool` - the skill is a thin wrapper around a single MCP tool call.

Other values may be registered by future RFCs.

### 4.4 `inputs`

A list of input specs. Each item:

```yaml
inputs:
  - name: pr_url
    type: url                 # string | number | boolean | file | url | json
    required: true            # default: false
    description: "The URL of the pull request"
    default: null             # optional default value
```

Allowed `type` values:

- `string` - arbitrary UTF-8 string.
- `number` - JSON number (integer or float).
- `boolean` - `true` / `false`.
- `file` - path to a file readable by the runtime.
- `url` - a validly-formed URL.
- `json` - arbitrary JSON, passed as-is to the skill body as a template variable.

### 4.5 `outputs`

```yaml
outputs:
  type: json                  # text | json | file
  schema: null                # optional JSON Schema describing the output
```

When `schema` is present, runtimes SHOULD validate the skill's output against it before returning to the caller.

### 4.6 `dependencies`

```yaml
dependencies:
  mcp-servers: []             # list of MCP server names this skill expects available
  skills: []                  # list of "name@semver-range" entries
  tools: []                   # list of CLI tools (e.g. bash, python, curl)
```

Runtimes MUST resolve `tools` by looking up each entry on `$PATH` before executing. Missing tools MUST cause `skill run` to fail with a clear error before any side effect occurs.

### 4.7 `permissions`

```yaml
permissions:
  network: []                 # list of allowed hostnames, or ["*"] for any
  filesystem: []              # list of path globs this skill may read/write
  env: []                     # list of environment variable names this skill may read
```

v0.1: permissions are advisory only (the CLI prints them but does not enforce). v0.2+ will enforce via platform sandboxing (seccomp / unveil / containers depending on host).

### 4.8 JSON Schema

A machine-validatable JSON Schema for the frontmatter lives at [`spec/frontmatter.schema.json`](./frontmatter.schema.json) (generated and kept in sync with the Rust types). Authors and tools may reference it via `$schema` in editors that support it.

## 5. Body

Everything after the closing `---` of the frontmatter (structured mode) or the entire file (prose mode) is the **body**. Rules:

- Format: [CommonMark](https://commonmark.org/) markdown. Pulldown-cmark is the reference parser.
- Interpolation: `{{ input_name }}` substitutes input values when the body is rendered to a prompt.
- Code fences with language hints (e.g., ` ```bash `) MAY be executed by runtimes that understand the language; default behavior is to leave code fences as instruction context only.

## 6. Parsing algorithm

Normative reference implementation in Rust: [`src/parser.rs`](../src/parser.rs).

1. Strip an optional UTF-8 BOM from the input.
2. If the remaining content starts with `---` followed by LF or CRLF:
   a. Locate the next line that equals exactly `---` (no trailing whitespace, no indentation).
   b. Parse everything between as YAML; this is the frontmatter.
   c. Everything after that line is the body.
   d. The skill is **structured**.
3. Otherwise: the entire content is the body. The skill is **prose**.

Parsers MUST reject frontmatter that is unterminated (no closing `---`) with a clear error.

## 7. Compatibility

### 7.1 Soapstones (prose mode)

Files published under the [Soapstones](https://soapstones.calpaterson.com/) conventions (free-form markdown with trailing signature block) parse as valid prose skills. No migration is required.

### 7.2 Anthropic skills

Claude Code skills with YAML frontmatter using `name`, `description` parse as valid structured skills. Fields like `allowed-tools` map to `dependencies.tools` (shim TBD in rfc-0003).

### 7.3 Forward compatibility

Unknown frontmatter fields MUST be preserved; future spec versions extend rather than replace.

## 8. Discoverability in a registry

(v0.2+) A registry stores skills at `/{namespace}/{name}/{version}/SKILL.md`. The namespace is the publishing account (e.g., `jakejjoyner/pr-review@0.1.0`). Prose soapstones live under an `anon` namespace or the publisher's choice.

## 9. Security considerations

- Skills are user-controlled code. Runtimes MUST sandbox execution by default (v0.2+).
- Signed skills (cryptographic signatures over the canonical file contents) will be introduced in rfc-0004.
- Prose skills carry instructions the agent MAY follow; they are not executed, but they can instruct an agent to take harmful actions. Runtimes SHOULD warn before displaying prose skills from untrusted sources.

## 10. Versioning

This spec follows SemVer. Breaking changes bump the major version. A skill MAY declare the spec version it targets via `runtime.min-version` (in practice: the earliest skillrt runtime that understands it).

## Appendix A: Example skills

See [`examples/`](../examples/) in this repository:

- `examples/structured-pr-review.md` - typed structured skill.
- `examples/prose-uk-train-times.md` - prose skill, Soapstones-compatible.

## Appendix B: Open questions

1. Should prose skills be allowed to declare a minimal frontmatter (only `name`, `version`) for namespacing purposes? (rfc-0002)
2. How does the spec accommodate non-Latin script skill names for i18n? (rfc-0005)
3. What signature scheme? (Sigstore / minisign / cosign) - rfc-0004.
4. Should the spec define telemetry keys for runtime interoperability? (rfc-0006)
