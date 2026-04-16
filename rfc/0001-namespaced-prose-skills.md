# RFC 0001: Namespaced prose skills (minimal frontmatter)

- **Status:** Draft
- **Date:** 2026-04-16
- **Tracking issue:** sk-29p
- **Tracks:** spec
- **Target release:** v0.2 (hosted registry)

---

## 1. Background

Prose-mode `SKILL.md` files carry no frontmatter: the entire file is free-form markdown the agent reads and interprets. This is deliberate. It keeps Soapstones-style notes (see [Cal Paterson's Soapstones](https://soapstones.calpaterson.com/)) zero-friction to publish, and it keeps the spec's core claim honest — "if you can write a markdown file, you can ship a skill."

Milestone 1 of the [roadmap](../ROADMAP.md) introduces a hosted registry: upload, search, install, semver-resolve, signed releases. A registry needs addressable artifacts. Structured skills already carry `name` and `version` in their frontmatter, so they address themselves. Prose skills do not — and today the only candidates for an identifier are the filename and a content hash, neither of which is stable across authors or edits.

The open question from [`spec/SKILL-SPEC.md`](../spec/SKILL-SPEC.md) Appendix B.1 is: do we permit prose skills to opt into a **minimal** frontmatter (just `name` and `version`) so the registry can address them, without forcing full structured mode?

This RFC resolves that question.

## 2. Proposal

Allow prose-mode `SKILL.md` files to declare an opt-in **minimal frontmatter** containing at most two fields:

```yaml
---
name: finding-uk-train-times
version: 0.1.0
---

# Finding UK train times

traintimes.org.uk has a clean URL scheme...
```

Rules:

1. A prose skill with minimal frontmatter MUST declare exactly `name` and `version` and nothing else. Any additional field promotes the file to structured mode and subjects it to full validation.
2. `name` and `version` carry the same semantics as in structured mode (kebab-case identifier, SemVer 2.0.0).
3. A prose skill with no frontmatter remains valid. The registry assigns it a content-addressed identifier under a reserved `anon/` namespace (see §6).
4. The parser's mode-detection rule does not change: if the file begins with `---`, it is parsed with a frontmatter block. A new classification — **prose with identity** — is introduced internally but is a subtype of prose mode from the author's perspective (no `inputs`, no `runtime`, no execution).

## 3. Arguments FOR

1. **The registry needs stable identifiers.** `skill install finding-uk-train-times@0.1.0` is unambiguous; `skill install <sha256>` is not usable by humans or models.
2. **Namespacing avoids collisions.** Two authors will independently write "pr-review" prose notes. Without `name`, either the registry rejects the second upload or silently picks a winner. Neither is acceptable.
3. **Low-friction opt-in.** Two lines of YAML is a negligible tax for authors who want their prose skill to be first-class in the registry. Authors who don't care keep publishing zero-frontmatter Soapstones exactly as before.
4. **Semver gives prose a migration story.** Prose skills are edited over time — URLs rot, APIs change. `version` lets downstream consumers pin a known-good snapshot without relying on the hosting provider's history.
5. **Aligns with existing conventions.** Claude Code skills, Anthropic Agent SDK skills, and most structured-skill dialects already carry `name` — a prose skill that wants to interoperate gains a seam for compatibility shims (see rfc-0003 placeholder).

## 4. Arguments AGAINST

1. **Erodes "pure prose" purity.** The design principle "prose and structured skills coexist" was clean; a third mode ("prose with frontmatter") weakens that clarity. The spec becomes harder to explain.
2. **Two-tier prose model.** Soapstones without frontmatter become second-class citizens in the registry (content-addressed, un-searchable by name). Authors will feel pressure to add frontmatter even when they shouldn't care.
3. **Frontmatter creep.** Once `name` and `version` are allowed, the next RFC adds `description` ("for search results"), then `tags` ("for discoverability"), then `authors` ("for attribution"), and the distinction collapses. Every one of those is defensible in isolation.
4. **Soapstones compatibility is principled, not incidental.** Cal Paterson's format deliberately has no identifiers — that's a design choice, not an oversight. Even though pure prose remains technically supported, encouraging authors to add `name`+`version` shifts the cultural norm: Soapstones with frontmatter become first-class in the registry while those without become content-addressed `anon/` second-class citizens. The two ecosystems diverge at the name level.
5. **Content hashes already work.** A registry can address prose skills by `sha256` with a human-readable alias layer on top — solving the addressing problem without touching the spec.

## 5. Recommendation

**Accept the proposal, with guardrails.**

Addressing is the load-bearing requirement for Milestone 1, and a content-hash-only design (argument-against #5) pushes the human-readable layer into the registry alone — meaning two registries will disagree on what a skill is called, and CLI commands like `skill install finding-uk-train-times@0.1.0` will work against `skills.dev` but not against a self-hosted mirror. That defeats "the spec is the product."

The purity concern (argument-against #1) is real but overstated: we already have two modes; a third opt-in subtype is a small delta relative to the clarity gained at the registry seam.

To contain frontmatter creep (argument-against #3), the spec MUST treat the two-field shape as **closed**: the presence of any field other than `name` or `version` reclassifies the file as structured mode, where it will fail validation without `description`. This makes creep a visible, breaking change — not a quiet slide. This structural defense is preferred over a procedural commitment ("future RFCs will be careful") because process promises decay; structural ones don't.

Soapstones compatibility (argument-against #4) is preserved at the technical level — pure prose still parses and ships unchanged. The cultural-norm shift is a real cost we're accepting in exchange for cross-registry portability, and one we mitigate by keeping frontmatter strictly opt-in and refusing to expand the minimal shape without a follow-up RFC.

## 6. Specification changes

If accepted, the spec changes as follows:

- **§3.1 Prose mode** gains a subsection permitting minimal frontmatter (`name`, `version` only).
- **§6 Parsing algorithm** adds a step: after YAML parsing, if the frontmatter contains only `name` and `version`, classify as prose-with-identity; otherwise require the full structured schema.
- **§8 Discoverability** specifies that prose-with-identity skills are stored at `/{namespace}/{name}/{version}/SKILL.md` like structured skills, and zero-frontmatter prose lives at `/anon/{sha256}/SKILL.md`.
- **JSON Schema** gains a `minimalFrontmatter` variant with `additionalProperties: false` and exactly `name` + `version` required.

The reference parser changes in a backwards-compatible way: existing zero-frontmatter prose and existing fully-structured skills continue to validate.

## 7. Open questions

1. **Version scheme for prose.** SemVer is defined for software. Does a prose note semantically have a "major / minor / patch"? Proposal: adopt SemVer strictly (major = substantive meaning change, minor = material addition, patch = typo/clarification). Document this in the spec so prose authors don't treat version as freeform.
2. **Required vs optional.** This RFC makes minimal frontmatter **optional**. Should the registry eventually require it for first-party publication? Deferred to the Milestone 1 registry RFC.
3. **Anonymous namespace policy.** Who owns `anon/`? What happens to a content-hashed skill when its author later claims it under a named namespace? Proposal: registry tracks `anon/{hash}` → `{namespace}/{name}@{version}` redirects; define in registry RFC, not here.
4. **Parser error surfaces.** Today, an unterminated frontmatter errors with "expected closing ---". A file with `---\nname: foo\n---` (no version) is ambiguous: is it malformed minimal frontmatter, or malformed structured? Proposal: if fewer than two fields are present, error as "minimal frontmatter requires both `name` and `version`"; if more than two fields are present but a required structured field (e.g. `description`) is missing, error as structured-mode validation failure.
5. **Interaction with signing (rfc-0004).** Do we sign the canonical body only, or body + frontmatter? If frontmatter, minimal and full skills share a signature algorithm; if body only, prose-with-identity skills have a weaker guarantee than structured. Deferred to rfc-0004.
6. **Spec numbering.** `spec/SKILL-SPEC.md` Appendix B.1 refers to this proposal as "rfc-0002." The bead directs this RFC to be filed as 0001. On acceptance, update Appendix B to point at 0001 and re-number downstream placeholders.
7. **Dispatch when `runtime` is absent in minimal frontmatter.** Spec §4.3 defaults aren't explicit. Should the runtime implicitly assume `type: markdown-skill`, or is the absence a distinct signal ("treat as prose, never try to execute")? Proposal: default to `markdown-skill` with no `inputs`/`outputs`, but state this explicitly in the spec change so prose-with-identity has a deterministic dispatch rule.
8. **`description` discoverability.** If `description` is absent — which is the whole point of minimal frontmatter — what does the registry index against for search? Options: first paragraph of the body, the H1 heading, or leave the search field empty. Search-quality issue, not a correctness issue, but worth resolving before the registry indexer ships.
9. **Migration of existing examples.** `examples/prose-uk-train-times.md` is pure prose today. Should the repo ship a second example demonstrating the minimal-frontmatter shape ("prose with identity"), or is one canonical pure-prose example enough? Adding a second example helps authors discover the opt-in path; keeping one keeps the examples directory focused.

## 8. Alternatives considered

- **Content-hash addressing only.** Rejected — see §5. Pushes human-readable naming into the registry layer and breaks cross-registry portability.
- **Full structured frontmatter required for registry inclusion.** Rejected — raises the bar for Soapstones authors and defeats the prose-first principle.
- **Sidecar file (`SKILL.meta.yaml`).** Rejected — two files are worse than two lines of frontmatter, and splits the artifact in a way that breaks the "markdown is the primary artifact" principle.
- **Relax `description` from required to optional in structured mode** (instead of introducing a new prose-with-identity subtype). Rejected — collapses the structured/prose distinction into a continuum, weakens the anti-creep defense (which becomes procedural rather than structural), and creates ambiguity about whether `name+version` skills are "structured without optional fields" or "prose with identity." Keeping the modes distinct preserves the spec's two-mode mental model.

---

## Changelog

- 2026-04-16 — Initial draft. Synthesized from two parallel polecat drafts (opal RFC base + quartz arguments-against #4 framing and open questions on runtime-dispatch, description discoverability, and example migration).
