# Comms-log integration hooks

> **Status:** Phase 1 seed (sk-79h). The module lives here (skillrt) for
> dogfood; it will graduate to `~/gt-lab/comms/` when a second silo adopts it.
> See `docs/ARCHITECTURE-CROSS-SILO-COMM.md` §5 and ADR-001 (upstream, in the
> gt-lab tree) for the why.

Every cross-silo event must be audit-logged twice — once in the origin silo's
`comms.log.jsonl`, once in the Plaza `.audit-log` (hash-chained). This writer
implements the silo-local half. The three initial call sites are listed below.
They are **not wired here** — each system lives outside this worktree — but the
CLI shape is pinned so integrators do not have to re-derive it.

## Schema recap

| field | required | values |
|---|---|---|
| `silo` | yes | `$GT_SILO` or basename of `$GT_ROLE` |
| `event` | yes | dotted name, e.g. `wheel.consulted`, `sling`, `fling.border` |
| `task_id` | optional | bead id or fling UUID — correlates across silos |
| `task_class` | optional | §4 autonomy-classification key |
| `tier` | optional | `autonomous` \| `approval_gated` |
| `direction` | optional | `in` \| `out` \| `local` |
| `target` | optional | `<silo>/<box>` for cross-silo flows |
| `border_result` | optional | `ok` \| `refused:<reason>` |
| `prev_hash` | auto | SHA-256 of previous canonical line (empty on genesis) |

Unknown fields are refused — schema changes go through ADR review.

## 1. `wheel/bin/spin` — `wheel.consulted`

Fires once per spin invocation, before priority resolution. Lets the digest
and churn-check (ADR-004) see how often each silo's wheel is actually
consulted vs. slung from.

```sh
# In wheel/bin/spin, just after arg parse:
comms-log wheel.consulted
# When a wheel item is added from an external input:
comms-log wheel.add --kv task_id="$TASK_ID" --kv direction=in
# When an item reaches done:
comms-log wheel.done --kv task_id="$TASK_ID"
```

## 2. `gt sling` — `sling` (intra-silo delegation)

Fires on every polecat dispatch. `direction=local` because sling is the
intra-silo primitive; cross-silo flow is `fling`, below.

```sh
comms-log sling \
    --kv task_id="$BEAD_ID" \
    --kv direction=local \
    --kv target="$RIG/polecats/$POLECAT"
```

Emit this from the dispatching side only — the polecat doesn't re-log
receipt, since sling is in-process within a rig.

## 3. `gt fling` (future) — `fling`, `fling.border`, `fling.received`

When Phase 1's cross-silo primitive lands, emit three entries per flow —
one on dispatch, one after Border policy runs, one when the receiving
silo's poller picks up the task:

```sh
# Origin silo:
comms-log fling \
    --kv task_id="$TASK_ID" --kv task_class="$CLASS" --kv tier="$TIER" \
    --kv direction=out --kv target="$TARGET_SILO/$BOX"
comms-log fling.border \
    --kv task_id="$TASK_ID" --kv border_result="$BORDER_RESULT"

# Receiving silo (poller):
comms-log fling.received \
    --kv task_id="$TASK_ID" --kv direction=in --kv target="$BOX"
```

On completion, the result flows back with `event=fling.result`,
`direction=in`, and the origin silo's outbox as `target`.

## 4. `bin/comms-log --verify`

Runs the hash-chain check. This is the seed hook the future
`audit-integrity-crawler` (§6) will call; it can also be dropped into a
pre-spin self-test so a silo refuses to add new entries on top of a
corrupted tail.

```sh
bin/comms-log --verify
# prints "ok: <path>" (exit 0) or "corrupt: <path>" (exit 2)
```

## Notes for integrators

- The writer is stdlib-only; no venv setup required on call sites.
- `log_event()` creates parent directories of the log path on first write.
- The CLI reads silo from `$GT_SILO` first, then the tail of `$GT_ROLE`,
  so shell hooks generally do not need to pass `--silo` explicitly.
- Do **not** write to Plaza `.audit-log` from this writer — that path is
  Border's sole responsibility (Principle 2 in the architecture doc).
