"""Per-silo communication log (hash-chained JSONL).

See docs/ARCHITECTURE-CROSS-SILO-COMM.md §5 and ADR-001 in the gt-lab tree.
"""

from .writer import GENESIS_HASH, DEFAULT_LOG_PATH, log_event, verify_chain

__all__ = ["GENESIS_HASH", "DEFAULT_LOG_PATH", "log_event", "verify_chain"]
