# Architecture

This document describes the internal structure of the `skillrt` reference runtime. The spec (`spec/SKILL-SPEC.md`) is the authoritative source for the file format; this document is about the Rust implementation.

## Layer diagram

```
   ┌───────────────────────────────────────────────────────┐
   │ CLI  (src/main.rs)                                     │
   │   clap-based; `skill validate | info | list | ...`     │
   └───────────────────────────────────────────────────────┘
                        │
                        ▼
   ┌───────────────────────────────────────────────────────┐
   │ Library (src/lib.rs re-exports)                        │
   │                                                        │
   │  ┌─────────────┐   ┌─────────────┐   ┌──────────────┐ │
   │  │ parser      │   │ frontmatter │   │ skill        │ │
   │  │ (md → AST)  │   │ (YAML type) │   │ (core types) │ │
   │  └─────────────┘   └─────────────┘   └──────────────┘ │
   │                                                        │
   │  ┌─────────────┐   ┌─────────────┐   ┌──────────────┐ │
   │  │ registry    │   │ runtime     │   │ error        │ │
   │  │ (disk I/O)  │   │ (execution) │   │ (SkillError) │ │
   │  └─────────────┘   └─────────────┘   └──────────────┘ │
   └───────────────────────────────────────────────────────┘
                        │
                        ▼
                  Platform: filesystem,
                  network (v0.2+), sandbox (v0.3+)
```

## Module responsibilities

### `error`

Defines `SkillError` (an enum of failure modes) and the crate-wide `Result<T>` alias. All public fallible APIs return this. External crate errors (`std::io::Error`, `serde_yaml::Error`) are wrapped via `#[from]`.

### `skill`

The core data types: `Skill`, `SkillKind { Prose | Structured }`, `SkillBody`. These are the canonical in-memory representation of a parsed SKILL.md. Deliberately free of I/O; construction and serialization happen elsewhere.

### `frontmatter`

Typed `serde`-derived structs for the YAML frontmatter schema. Mirrors `spec/SKILL-SPEC.md` section 4. Adding fields to the spec means changing this module. Unknown fields land in `Frontmatter::extra` (a `BTreeMap<String, serde_yaml::Value>`) so they round-trip.

### `parser`

Reads raw markdown into a `Skill`. Handles:

- UTF-8 BOM stripping.
- Detection of structured vs prose mode (presence of opening `---` line).
- YAML frontmatter extraction and deserialization.
- Retention of the body as a raw string (preserving whitespace and code fences).

Unit-tested in `#[cfg(test)] mod tests`. Tests cover prose-only, structured-with-frontmatter, and unterminated-frontmatter cases.

### `registry`

Local on-disk skill storage. v0.1 uses `directories::ProjectDirs` to locate the platform-standard data directory (e.g., `~/.local/share/skillrt/` on Linux).

- `root()` returns the base directory, creating it if absent.
- `list()` walks the directory tree looking for `SKILL.md` files.
- `load(path)` reads one file into a `Skill`.
- `install_from_path(path)` copies a SKILL.md into the store under `{name}/{version}/SKILL.md`.

v0.2 will add HTTP-backed install, signature verification, and a hosted registry client.

### `runtime`

v0.1 is a stub returning a static "not yet implemented" response. The real execution engine (v0.4) will:

1. Validate inputs against frontmatter `inputs` schema.
2. Check permissions (network, filesystem, environment).
3. Resolve dependencies (`tools` present on PATH, `mcp-servers` reachable, sibling `skills` installed).
4. Render the body with input substitution (`{{ name }}` templating).
5. Dispatch to the appropriate sub-runtime:
   - `markdown-skill`: pass rendered body to an invoking LLM as instructions (or shell out to a provided "interpreter" process).
   - `mcp-tool`: call the named MCP tool with inputs and return the result.
6. Validate output against `outputs.schema`.
7. Emit OTLP telemetry spans for observability.

### `main`

Thin CLI layer. Translates argv into library calls and prints results. All business logic lives in the library; `main.rs` only handles argument parsing and output formatting.

## Compatibility promises

- **Parser:** preserves round-trip of unknown frontmatter keys so future spec versions do not break old files.
- **CLI:** subcommand names and output formats are stable within a minor version; breaking changes require a major bump.
- **Library:** public API stability begins at v0.2; v0.1 is explicitly unstable.

## Testing strategy

- **Unit tests** live alongside each module in `#[cfg(test)] mod tests`.
- **Integration tests** (to be added pre-v0.2) will live in `tests/` and use `assert_cmd` + `tempfile` to drive the CLI end-to-end.
- **Golden-file tests** for the parser (a directory of known-good SKILL.md fixtures with expected `Skill` JSON). Planned for v0.2.

## Performance targets (aspirational)

- Parse and validate a 10KB SKILL.md in ≤5ms cold.
- `skill list` across 1,000 installed skills in ≤50ms.
- Binary size ≤5MB stripped release build on Linux x86_64.

These are not enforced in v0.1 but should be benchmarked before v1.0.
