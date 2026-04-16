# RFC 0001: Namespaced prose skills (minimal frontmatter)

- **Status:** Draft
- **Date:** 2026-04-16
- **Tracking issue:** sk-29p
- **Targets:** `spec/SKILL-SPEC.md` v0.1-draft, ROADMAP Milestone 1 (hosted registry, v0.2)

---

## Background

The v0.1 spec defines two shapes for a `SKILL.md` (spec §3):

- **Prose mode** — no frontmatter, free-form markdown. Zero-migration for Soapstones.
- **Structured mode** — YAML frontmatter with `name`, `version`, `description` required.

Mode is decided purely by whether the file opens with `---` (spec §6). Prose skills therefore have no self-declared identity: no `name`, no `version`, no `authors`.

That was cheap in v0.1, where install is a file copy and `skill list` walks a local directory. It gets expensive in v0.2. Milestone 1 (ROADMAP) introduces a hosted registry with `skill publish`, `skill install <name>@<version>`, semver resolution, and storage at `/{namespace}/{name}/{version}/SKILL.md` (spec §8). Every one of those operations is keyed on `name` and `version` — fields a prose skill does not have.

The spec already flags this as an open question (SKILL-SPEC.md Appendix B, #1). This RFC resolves it before v0.2 lands.

## Proposal

Make `description` **optional** in structured mode, so the minimum valid frontmatter is:

```yaml
---
name: finding-uk-train-times
version: 0.1.0
---

# Finding UK train times
...body...
```

Concretely:

1. Amend spec §4.1: move `description` from required to optional. Required fields become `name` and `version` only.
2. Keep the §6 parsing rule intact (`---` at byte 0 ⇒ structured). No new mode, no new marker.
3. A skill with only `name` + `version` is structured but behaviourally prose — no `inputs`, `outputs`, `runtime`, or `dependencies` means the runtime renders the body as instruction context, identical to pure prose today.
4. Pure prose (no frontmatter) remains fully supported. The registry assigns identity at publish time from CLI flags (`skill publish --name foo --version 0.1.0 path/`) and stores that metadata externally.

Soapstones-compatible files continue to parse with zero migration. Authors who want a stable registry handle opt in by adding two YAML lines.

## Arguments for

- **Registry addressability.** `/{namespace}/{name}/{version}/` needs `name` and `version`. Carrying them in the file (rather than only in upload metadata) means a skill keeps its identity when moved between machines, forks, or registries.
- **Semver resolution needs a version.** `skill install foo@^0.2` is undefined without an in-file `version`, or it forces the registry to be the single source of truth — which defeats the "spec is the product" principle (CONTRIBUTING §Principles 1).
- **Low-friction opt-in.** Two lines. No YAML schema to learn, no runtime to declare. Authors who don't care pay nothing.
- **Zero new parser work.** The existing `---` detection, frontmatter deserialiser, and `Frontmatter::extra` round-tripping already handle this case. We only relax one required field.
- **Forward-compat lane.** Prose authors who later want to add `tags` or `license` (§4.2 optional fields) can do so without a format migration.

## Arguments against

- **Erodes the prose purity guarantee.** Design principle 3 (README) is "prose and structured coexist in the same registry." A two-tier prose world — "pure" vs. "namespaced" — adds conceptual weight, even if mechanically simple.
- **Frontmatter creep.** Today's minimal `name` + `version` becomes tomorrow's minimal `name` + `version` + `tags` + `license`. Each addition is individually defensible; the aggregate re-invents the structured schema we said prose authors wouldn't have to learn.
- **Soapstones compatibility is principled, not incidental.** Cal Paterson's format deliberately has no identifiers. Encouraging authors to add them shifts the norm, even while leaving pure prose technically supported.
- **Alternative exists.** Registry-side metadata (namespace + name + version recorded at publish time, not in the file) solves identity without touching the spec. Name collisions are a publish-time CLI concern, not a file-format concern.
- **Dual representation of identity.** If a file says `name: foo` but gets published under `bar/foo`, which wins? This is answerable but adds a rule the current spec doesn't need.

## Recommendation

**Adopt the proposal.** Make `description` optional; accept `name` + `version` as the minimal structured frontmatter.

The registry-metadata alternative is clean in isolation but fails the portability test: a prose skill fetched from registry A, inspected locally, and re-published to registry B has no way to retain its identity without external context. The spec has to carry identity to be the product; CONTRIBUTING §Principles 1 and design principle 4 ("no lock-in") both point in this direction.

Soapstones compatibility is preserved — pure prose still parses. Authors are not forced into YAML; they are offered it when they want a stable handle. The two-tier concern is real but bounded: the boundary ("do I want a registry handle?") is meaningful to authors in a way that a purity distinction is not.

We accept the frontmatter-creep risk and mitigate it by keeping `description` the only field moved in this RFC. Any further relaxation of required fields — or expansion of what's considered "minimal" — requires its own RFC.

## Open questions

1. **Version scheme.** `version` is SemVer 2.0.0 in structured mode (§4.1). Should namespaced-prose skills be allowed CalVer or free-form strings, given that "release cadence" is fuzzy for prose notes? Recommendation leans toward keeping strict SemVer for uniform registry resolution, but prose authors may resist.
2. **Dispatch when `runtime` is absent.** Spec §4.3 defaults aren't explicit for a frontmatter that omits `runtime`. Must the runtime implicitly assume `type: markdown-skill`, or should the absence be a distinct signal ("treat as prose, never try to execute")? Probably fine to default to `markdown-skill` with no `inputs`/`outputs`, but worth stating.
3. **Registry handling of pure prose.** What namespace does a soapstone with no frontmatter land in after `skill publish`? Options: (a) `anon/<content-hash>`, (b) `<publisher>/<slug-from-first-heading>`, (c) require `--name` on the CLI. Needs resolution before Milestone 1 ships.
4. **`description` discoverability.** If `description` is absent, registry search has nothing to index. Do we fall back to the first paragraph of the body, the H1, or leave the field empty? Search quality issue, not a correctness issue.
5. **Relationship to rfc-0002 and later.** SKILL-SPEC.md Appendix B currently points the prose-frontmatter question at "rfc-0002". This RFC files it as 0001 per the tracking bead. Appendix B should be updated to match once this RFC is accepted.
6. **Migration of existing examples.** `examples/prose-uk-train-times.md` is pure prose today. Should the repo ship a second "namespaced prose" example to demonstrate the minimal-frontmatter shape, or is one canonical prose example enough?
