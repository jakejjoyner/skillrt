"""Per-silo comms.log.jsonl writer.

Append-only, hash-chained JSONL log of every wheel / sling / fling / border
event a silo initiates or receives. Mirrors the Plaza audit-log locally so a
silo can replay its own story offline.

Schema (per docs/ARCHITECTURE-CROSS-SILO-COMM.md §5):

    ts             RFC 3339 timestamp with local offset
    silo           originating silo name (e.g. "mayor", "skillrt")
    event          event name (e.g. "wheel.consulted", "sling", "fling")
    task_id        optional — correlates entries across silos
    task_class     optional — §4 autonomy classification key
    tier           optional — "autonomous" | "approval_gated"
    direction      optional — "in" | "out" | "local"
    target         optional — "<silo>/<box>" when direction is in/out
    border_result  optional — "ok" | "refused:<reason>"
    prev_hash      SHA-256 of the previous entry's canonical JSON line
                   (empty string on the genesis entry)

Only fields listed above are accepted. Unknown keys raise ValueError so the
schema stays disciplined as new event classes are added by policy rather than
by typo.

stdlib only (hashlib, json, datetime, pathlib) — no venv needed.
"""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Optional

DEFAULT_LOG_PATH = Path.home() / "gt-lab" / "comms.log.jsonl"

GENESIS_HASH = ""

_DIRECTIONS = frozenset({"in", "out", "local"})

_OPTIONAL_FIELDS: tuple[str, ...] = (
    "task_id",
    "task_class",
    "tier",
    "direction",
    "target",
    "border_result",
)


def _now_rfc3339() -> str:
    return datetime.now(timezone.utc).astimezone().isoformat(timespec="seconds")


def _canonical(entry: dict[str, Any]) -> str:
    return json.dumps(entry, sort_keys=True, separators=(",", ":"))


def _hash_line(line: str) -> str:
    return hashlib.sha256(line.encode("utf-8")).hexdigest()


def _tail_hash(path: Path) -> str:
    """SHA-256 of the last non-empty line, or GENESIS_HASH if the file is
    missing or empty."""
    if not path.exists() or path.stat().st_size == 0:
        return GENESIS_HASH
    last: Optional[str] = None
    with path.open("r", encoding="utf-8") as f:
        for raw in f:
            stripped = raw.rstrip("\n")
            if stripped:
                last = stripped
    if last is None:
        return GENESIS_HASH
    return _hash_line(last)


def log_event(
    silo: str,
    event: str,
    *,
    path: Optional[Path] = None,
    **extra: Any,
) -> dict[str, Any]:
    """Append a hash-chained event to the silo's comms log.

    Returns the entry dict that was written (canonical field order).

    Use ``path`` to override the default ``~/gt-lab/comms.log.jsonl`` (tests
    and ad-hoc scripting).
    """
    if not silo:
        raise ValueError("silo must be a non-empty string")
    if not event:
        raise ValueError("event must be a non-empty string")

    unknown = set(extra) - set(_OPTIONAL_FIELDS)
    if unknown:
        raise ValueError(f"unknown fields: {sorted(unknown)}")

    direction = extra.get("direction")
    if direction is not None and direction not in _DIRECTIONS:
        raise ValueError(
            f"direction must be one of {sorted(_DIRECTIONS)}, got {direction!r}"
        )

    log_path = path if path is not None else DEFAULT_LOG_PATH
    log_path.parent.mkdir(parents=True, exist_ok=True)

    entry: dict[str, Any] = {"ts": _now_rfc3339(), "silo": silo, "event": event}
    for field in _OPTIONAL_FIELDS:
        value = extra.get(field)
        if value is not None:
            entry[field] = value
    entry["prev_hash"] = _tail_hash(log_path)

    line = _canonical(entry)
    with log_path.open("a", encoding="utf-8") as f:
        f.write(line + "\n")
    return entry


def verify_chain(path: Path) -> bool:
    """Return True iff every entry's prev_hash matches the SHA-256 of the
    previous canonical line (GENESIS_HASH for the first entry)."""
    if not path.exists() or path.stat().st_size == 0:
        return True
    expected = GENESIS_HASH
    with path.open("r", encoding="utf-8") as f:
        for raw in f:
            line = raw.rstrip("\n")
            if not line:
                continue
            entry = json.loads(line)
            if entry.get("prev_hash") != expected:
                return False
            expected = _hash_line(line)
    return True
