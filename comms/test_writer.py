"""Tests for comms.writer — stdlib-only, run with: python3 -m unittest comms.test_writer"""

from __future__ import annotations

import json
import re
import tempfile
import unittest
from pathlib import Path

from comms import writer
from comms.writer import GENESIS_HASH, log_event, verify_chain


def _read_lines(path: Path) -> list[str]:
    return [ln.rstrip("\n") for ln in path.read_text().splitlines() if ln.strip()]


class LogEventTests(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.log = Path(self._tmp.name) / "comms.log.jsonl"

    def test_first_entry_uses_genesis_prev_hash(self) -> None:
        entry = log_event("skillrt", "wheel.consulted", path=self.log)
        self.assertEqual(entry["prev_hash"], GENESIS_HASH)
        self.assertEqual(entry["silo"], "skillrt")
        self.assertEqual(entry["event"], "wheel.consulted")
        # RFC 3339 with offset, seconds precision
        self.assertRegex(entry["ts"], r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[+-]\d{2}:\d{2}$")

    def test_optional_fields_are_passed_through(self) -> None:
        entry = log_event(
            "mayor",
            "fling",
            path=self.log,
            task_id="abc123",
            task_class="digest_gen",
            tier="autonomous",
            direction="out",
            target="pasha/inbox",
            border_result="ok",
        )
        for k, v in {
            "task_id": "abc123",
            "task_class": "digest_gen",
            "tier": "autonomous",
            "direction": "out",
            "target": "pasha/inbox",
            "border_result": "ok",
        }.items():
            self.assertEqual(entry[k], v)

    def test_unknown_field_rejected(self) -> None:
        with self.assertRaises(ValueError):
            log_event("skillrt", "sling", path=self.log, nonsense=1)

    def test_invalid_direction_rejected(self) -> None:
        with self.assertRaises(ValueError):
            log_event("skillrt", "sling", path=self.log, direction="sideways")

    def test_chain_on_ten_event_sample(self) -> None:
        events = [
            ("skillrt", "wheel.consulted", {}),
            ("skillrt", "sling", {"task_id": "t1", "direction": "local"}),
            ("skillrt", "sling", {"task_id": "t2", "direction": "local"}),
            ("mayor", "fling", {"task_id": "t3", "direction": "out", "target": "pasha/inbox",
                                "tier": "autonomous", "task_class": "digest_gen"}),
            ("mayor", "fling.border", {"task_id": "t3", "border_result": "ok"}),
            ("pasha", "fling.received", {"task_id": "t3", "direction": "in"}),
            ("pasha", "wheel.add", {"task_id": "t3"}),
            ("pasha", "wheel.done", {"task_id": "t3"}),
            ("mayor", "fling.result", {"task_id": "t3", "direction": "in", "target": "mayor/outbox"}),
            ("skillrt", "wheel.consulted", {}),
        ]
        for silo, event, extra in events:
            log_event(silo, event, path=self.log, **extra)

        lines = _read_lines(self.log)
        self.assertEqual(len(lines), 10)

        # Each prev_hash must match sha256 of the prior line.
        self.assertTrue(verify_chain(self.log))

        # Spot-check: first line's prev_hash is GENESIS; second line's
        # prev_hash hashes to the first line's bytes exactly.
        first = json.loads(lines[0])
        second = json.loads(lines[1])
        self.assertEqual(first["prev_hash"], GENESIS_HASH)
        self.assertEqual(second["prev_hash"], writer._hash_line(lines[0]))

    def test_tampering_is_detected(self) -> None:
        for i in range(3):
            log_event("skillrt", f"e{i}", path=self.log)
        lines = _read_lines(self.log)
        self.assertTrue(verify_chain(self.log))

        # Flip one character inside the 2nd entry's event value.
        tampered = re.sub(r'"event":"e1"', '"event":"e9"', lines[1], count=1)
        self.log.write_text("\n".join([lines[0], tampered, lines[2]]) + "\n")
        self.assertFalse(verify_chain(self.log))

    def test_append_resumes_chain_across_invocations(self) -> None:
        # Simulates two separate process invocations against the same file.
        a = log_event("skillrt", "first", path=self.log)
        b = log_event("skillrt", "second", path=self.log)
        lines = _read_lines(self.log)
        self.assertEqual(a["prev_hash"], GENESIS_HASH)
        self.assertEqual(b["prev_hash"], writer._hash_line(lines[0]))
        self.assertTrue(verify_chain(self.log))


if __name__ == "__main__":
    unittest.main()
