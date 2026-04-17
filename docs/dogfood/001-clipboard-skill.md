# Dogfood 001: clipboard-for-paste-commands

- **Date:** 2026-04-17
- **Bead:** sk-80b
- **Phase:** A (proof-of-concept)
- **Source memory:** `/home/jjoyner/gt-lab/memory/feedback_clipboard_for_paste_commands.md`
- **Skill:** `skills/bob-clipboard-for-paste-commands/SKILL.md`

This is the first port from the skillrt dogfooding exercise (see
`memory/project_skillrt_dogfood_seed.md`). Goal: convert one of Bob's pinned
memory entries to a skillrt skill, validate it with the reference parser, and
record the spec gaps that surface.

## Outcome

- Skill parses and validates cleanly (`skill validate` → `ok (structured) …`).
- Unknown frontmatter fields (`principal`, `priority`) round-trip via the
  `extra` map per spec §4.2 — no parser changes needed for the proof.
- Body translates the memory content 1:1, preserving the "do / don't
  auto-clipboard" structure and the reminder-cancel clarification from
  2026-04-17.

## Spec gaps observed

### 1. No formal "prose skill with extended metadata" mode

The bead asked for prose-mode-with-frontmatter. The spec today is binary:
prose = no frontmatter, structured = full schema with `description` required.
RFC 0001 (Draft) proposes a minimal-frontmatter subtype for prose — but it
closes the shape at exactly `{name, version}`. Any extra field (here:
`description`, `principal`, `priority`) reclassifies the file as structured
and subjects it to full validation.

Result: this skill is structurally classified as **structured**, even though
its body is pure prose with no inputs, outputs, or executable procedure. The
two-mode mental model breaks down for memory-derived skills — they carry
identity + rules + principal metadata but do not describe an executable.

**Possible resolutions** (defer to an RFC, not this bead):

- Introduce a third mode, e.g. `reference` or `note`, where the frontmatter
  schema is "identity + discovery metadata, no execution contract."
- Relax the structured/prose split: allow `description` in the RFC-0001
  minimal-frontmatter shape, accepting the argument-against-#3 risk
  (frontmatter creep) and codifying a max shape for the non-executable case.
- Keep the binary split and direct memory ports to structured mode, accepting
  that `inputs: []` / `outputs: null` is the agreed "not executable" signal.
  (This is what the PoC does today.)

### 2. `principal` has no spec home

Bob's user-scoped memories are bound to a specific human (`user_*` memories
in particular). The seed doc anticipated this: "skillrt may need a
`principal:` field." I used `principal: jake` as an unknown field; it
preserves through the parser but consuming agents have no guidance on:

- What type of value is expected (handle? email? account URI? `user:<id>`?).
- Whether multiple principals can share a skill.
- Whether `principal` absence means "anyone" or "undefined".
- How `principal` interacts with a hosted-registry namespace (is it a scoping
  axis, or an orthogonal binding?).

**Recommendation:** RFC for `principal` after we port 2–3 more user-scoped
memories; we need the real authoring variance before we pick a shape.

### 3. `priority` has no defined values or ordering

Memory entries are stratified into `[PINNED]`, `[FOUNDATIONAL]`, and normal
(no tag). The seed doc flagged this: "Skill 'level' or 'priority' field —
skillrt skills may need analog of `[PINNED]` vs. normal (enforcement
strength signal to consuming agents)."

I used `priority: pinned` (free-form string). Open questions:

- Is this an enum (`pinned | foundational | normal | deprecated`) or a
  numeric rank (1..N, lower = stronger)?
- Does priority encode **enforcement strength** ("you MUST follow this") or
  **discoverability rank** ("surface this first in search") or both?
- What does a consuming runtime do with it? The spec today has no hook for
  "print a banner for pinned skills" or "warn if deprecated skill is used."

**Recommendation:** define `priority` jointly with `principal` in the same
RFC — they both encode "how strongly does this bind on the agent." Start
narrow: `enum { pinned, normal, deprecated }`, numeric ordering deferred.

### 4. No provenance / derived-from field

This skill is a **derived view** of a memory file. The memory is canonical;
the skill is a projection. There is no frontmatter field to express that:

```yaml
# hypothetical
source: memory:feedback_clipboard_for_paste_commands
derived-at: 2026-04-17
```

I put the provenance in the body ("## Provenance" section at the end).
Works, but it is unindexed and not machine-readable — a registry cannot
query "all skills derived from memory X" or "all skills by provenance
`memory/`". This matters when skillrt hosts the canonical skill library and
memory is deprecated (phase D) — we will want to trace every skill back to
its source for audit.

**Recommendation:** defer until phase B surfaces more derived skills. If
every ported memory adds a similar section, generalize.

### 5. `skill info` does not surface unknown fields

The `info` CLI prints `name`, `version`, `description`, `authors`,
`license`, `inputs`, `tags` — but not the `extra` map. The `principal` and
`priority` values are preserved in memory but invisible to an operator
running `skill info`. For dogfooding we want **maximum observability into
experimental fields** so authors can see whether their extensions landed.

**Recommendation (fixable now, separate bead):** add a generic "extra"
dump to `skill info`, e.g. `extras: principal=jake, priority=pinned`. Tiny
change; high leverage for the dogfood feedback loop.

### 6. No structured lane for anti-patterns

Memory entries often pair "do X" with "don't do Y" (the clipboard memory
has both: always-copy rules + five explicit don't-auto-clipboard cases).
The skill body handles this fine in prose, but there is no structured
frontmatter or body-section convention. A registry cannot distill "what is
this skill telling agents NOT to do" from the markdown alone.

This matches the seed doc's spec-additions note: "Anti-patterns — some
feedback entries encode 'don't do X' alongside 'do Y'; skill spec should
support both lanes."

**Recommendation:** defer until phase B. If the anti-pattern section
becomes a consistent shape ("## Do NOT" heading), we can either:

- Standardize the heading and let tools parse it positionally, or
- Introduce structured fields (`rules:` + `anti-rules:` arrays).

## What we didn't hit

- The `dependencies`, `permissions`, `inputs`, `outputs` machinery is all
  unused — expected for a reference/rule skill. No friction there.
- Soapstones compatibility is preserved for the **body**; only the
  frontmatter departs from pure prose.
- The parser's unknown-field preservation (§4.2) made the experiment
  cheap: I extended the schema informally without touching `frontmatter.rs`.

## Next step in the dogfood track

Per the seed doc's Phase A/B split, this bead ends Phase A. Phase B ports
3–5 more pinned/foundational memories. I recommend prioritizing ones that
would stress the same gaps (principal, priority, anti-patterns) to
corroborate the RFC direction before Phase C proposes spec changes.
