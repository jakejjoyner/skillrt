# skillrt Roadmap

The full plan, ordered by shippable-MVP speed. Each milestone compounds into the next.

## Milestone 0 - Foundation (v0.1) [CURRENT]

**Scope:** Spec draft, local runtime CLI, parser, validator. No execution, no remote registry.

**Deliverables:**
- [x] Rust crate scaffold with `parser`, `frontmatter`, `skill`, `registry`, `runtime`, `error` modules
- [x] Clap-based CLI: `validate`, `info`, `list`, `install`, `run`, `where`
- [x] Unit tests for parser (prose mode, structured mode, unterminated frontmatter)
- [x] `spec/SKILL-SPEC.md` v0.1-draft
- [x] `llms.txt` for model-readable project summary
- [x] README, LICENSE, ROADMAP, CONTRIBUTING, CHANGELOG
- [ ] `examples/` with at least one prose + one structured skill
- [ ] GitHub Actions CI: build + test + clippy + fmt on push
- [ ] Published to crates.io

**Validation target:** Spec publishable; reference runtime installable via `cargo install --path .`; community feedback on spec can begin.

---

## Milestone 1 - Hosted registry (v0.2)

**Scope:** The `skills.dev` (or whatever domain we land on) public registry.

**Deliverables:**
- Hosted registry: upload / publish / search / download
- `skill publish` and `skill install <name>@<version>` in the CLI
- Semver resolution
- Signed releases (minisign or cosign)
- Web UI: browse, search, view metadata
- Rate-limiting, abuse handling

**Stack:** Cloudflare Workers + R2 for storage, Supabase for metadata + auth (familiar territory).

**Validation target:** One high-signal external skill published and installed by an unrelated user.

---

## Milestone 2 - Structured shared memory (v0.3)

**Scope:** A better-than-Soapstones memory substrate: typed, queryable, dedupe-aware, decayed.

**Deliverables:**
- `skill-memory` runtime integration: skills can read/write typed memories
- Query API: `memory.find(tag: "rtt-api")`, `memory.upsert(...)`, `memory.decay_older_than(30d)`
- Shared backend (hosted service) with per-namespace access control
- Soapstones migration tool: import Cal Paterson's registry into skillrt memory as prose skills

**Validation target:** A Claude Code session and a Cursor session share memory through skillrt with no friction.

---

## Milestone 3 - Execution engine (v0.4)

**Scope:** Actually run structured skills end-to-end.

**Deliverables:**
- Input validation against frontmatter schema
- Permission enforcement (network/fs/env) via OS sandboxing where available
- Dependency resolution (required tools, MCP servers, sibling skills)
- Output schema validation
- Structured telemetry (OTLP export)
- Streaming output for long-running skills

**Validation target:** `skill run pr-review --inputs '{"pr_url":"..."}'` runs end-to-end and produces a validated JSON report.

---

## Milestone 4 - Agent-readable documentation standard (v0.5)

**Scope:** Formalize a `agent-doc.yaml` or extend `llms.txt` so libraries and APIs publish machine-first docs consumable by skills.

**Deliverables:**
- Spec: `spec/AGENT-DOC-SPEC.md`
- OpenAPI → agent-doc.yaml generator
- MCP server that serves agent-doc queries
- Reference library docs (we publish skillrt's own agent-doc first)

**Validation target:** One library (Resend, Stripe, or similar) publishes an agent-doc.yaml; a skill consumes it autonomously.

---

## Milestone 5 - Model-to-model handoff protocol (v0.6)

**Scope:** Structured protocol for delegating tasks between agents / models, carried as a skill.

**Deliverables:**
- Spec: `spec/HANDOFF-SPEC.md`
- Reference implementation of a "delegate" skill: spawn a subagent with typed context + budget + success criteria
- Cross-model compatibility shims: Claude, OpenAI, Gemini, local open-weights

**Validation target:** A multi-step workflow where Claude Code delegates a sub-task to a cheaper model, receives typed output, and continues.

---

## Milestone 6 - Observability (v0.7 → v1.0)

**Scope:** The Datadog for skill execution.

**Deliverables:**
- Hosted telemetry endpoint
- Per-skill metrics: invocations, p50/p95/p99 latency, error rate, cost
- Trace viewer for multi-skill workflows
- Alerting on regression (new skill version slower / less accurate)
- Eval harness: golden-path tests that run on every `skill publish`

**Validation target:** Someone ships a skill update, regression catches it before users see it.

---

## v1.0: Stability

- Spec frozen (breaking changes require major bump and 12-month deprecation cycle)
- Reference runtime covered by integration tests against ≥3 agent frameworks
- At least 100 public skills published
- Ecosystem: third-party runtime implementations in Python and TypeScript

---

## Out of scope (for now)

- Sandboxed execution environments (Firecracker-speed ephemeral VMs). Depends on E2B-class infrastructure; if we need it, we build on top of existing providers rather than rolling our own. Revisit in v1.x.
- Payment/billing for premium skills. Too early. Will be addressed if and when the registry has commercial demand.
- Mobile SDKs. Out of scope until browser/mobile agents are a real thing.

---

## Non-goals

- **Competing with MCP.** MCP is a tool-call protocol; skillrt is a skill distribution format. They compose: skills can declare MCP server dependencies.
- **Replacing Claude Code / Cursor / Aider.** These are agent UIs. skillrt is the substrate any of them can adopt.
- **A single-vendor platform.** The spec is the product; we explicitly want multiple runtime implementations and competing registries.
