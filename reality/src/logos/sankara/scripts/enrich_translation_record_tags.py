#!/usr/bin/env python3
"""Enrich translation passage records with explicit structural tags.

Why this exists:
- Raw Sankara extractors already carry strong structure tags (for example `block_type`).
- Translation passage records often only preserve those cues indirectly in notes.
- We cannot afford full re-analysis loops; this script upgrades tags in place (or dry-run)
  without touching translation text fields.

Behavior:
- Infers and writes `record_kind` when missing/unknown and optionally repairs stale values.
- Adds `analysis.tags` with stable values:
  - `strata:<kind>`
  - `layout:chapter-section-record`
  - `policy:append-and-supersede-never-delete`
- Leaves existing translations untouched.
"""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any

DEFAULT_RECORDS_DIR = Path("reality/src/logos/sankara/translation/passage_records")
KIND_SET = {"sutra", "bhasya", "preamble", "transition", "unknown"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Enrich Sankara translation passage records with explicit structural tags "
            "without re-running full extraction/translation pipelines."
        )
    )
    parser.add_argument(
        "--records-dir",
        type=Path,
        default=DEFAULT_RECORDS_DIR,
        help="Directory containing translation passage record JSON files.",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="Write updates to files. By default runs in dry-run mode.",
    )
    parser.add_argument(
        "--section",
        default=None,
        help="Optional section filter (e.g. 1.1).",
    )
    parser.add_argument(
        "--work-code",
        default=None,
        help="Optional work-code filter (e.g. BS).",
    )
    parser.add_argument(
        "--repair-kind",
        action="store_true",
        help="Rewrite stale record_kind values when inferred kind is stronger.",
    )
    return parser.parse_args()


def normalize_kind(value: str) -> str:
    candidate = (value or "").strip().lower()
    if candidate in KIND_SET:
        return candidate

    if candidate in {"versetext", "mantra", "verse", "sutra-text"}:
        return "sutra"
    if "bhashya" in candidate:
        return "bhasya"
    if "preamble" in candidate or candidate.endswith("_pre"):
        return "preamble"
    if "transition" in candidate:
        return "transition"
    return "unknown"


def parse_trailing_paren_value(text: str) -> str:
    if not text or "(" not in text or ")" not in text:
        return ""
    return text.rsplit("(", 1)[-1].split(")", 1)[0].strip()


def infer_kind(data: dict[str, Any]) -> str:
    source = data.get("source") or {}
    text = data.get("text") or {}
    record_id = str(data.get("record_id", "")).lower()

    haystack = " ".join(
        [
            str(text.get("segmentation_note", "")).lower(),
            str(source.get("edition_note", "")).lower(),
            str(source.get("label", "")).lower(),
            record_id,
        ]
    )

    # Strong textual markers override stale legacy tags.
    if any(token in haystack for token in ("versetext", "core sutra line", "sutra text unit", "mantra", "sutra")):
        return "sutra"
    if any(
        token in haystack
        for token in (
            "preamble",
            "_pre/",
            "_pre_",
            "translator preamble",
            "editorial preamble",
        )
    ):
        return "preamble"
    if "transition" in haystack:
        return "transition"

    for candidate in (
        data.get("record_kind"),
        source.get("record_kind"),
        text.get("record_kind"),
        parse_trailing_paren_value(str(text.get("segmentation_note", ""))),
        parse_trailing_paren_value(str(source.get("edition_note", ""))),
    ):
        normalized = normalize_kind(str(candidate or ""))
        if normalized != "unknown":
            return normalized

    if any(token in haystack for token in ("intro_bhashya", "leading_bhashya", "bhashya")):
        return "bhasya"

    return "unknown"


def enrich_tags(data: dict[str, Any], kind: str, repair_kind: bool = False) -> bool:
    changed = False

    current_kind = normalize_kind(str(data.get("record_kind", "")))
    should_write_kind = (
        kind != "unknown" and (current_kind == "unknown" or (repair_kind and current_kind != kind))
    )
    if should_write_kind:
        data["record_kind"] = kind
        changed = True

    analysis = data.get("analysis")
    if not isinstance(analysis, dict):
        analysis = {}
        data["analysis"] = analysis
        changed = True

    tags = analysis.get("tags")
    if not isinstance(tags, list):
        tags = []

    existing = {str(t) for t in tags}
    existing = {tag for tag in existing if not tag.startswith("strata:")}
    required = {
        f"strata:{kind}",
        "layout:chapter-section-record",
        "policy:append-and-supersede-never-delete",
    }
    merged = sorted(existing.union(required))

    if merged != tags:
        analysis["tags"] = merged
        changed = True

    return changed


def iter_records(records_dir: Path):
    for path in sorted(records_dir.glob("**/*.json")):
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except Exception:
            continue
        if isinstance(data, dict):
            yield path, data


def matches_filters(data: dict[str, Any], work_code: str | None, section: str | None) -> bool:
    if work_code and str(data.get("work_code", "")).upper() != work_code.upper():
        return False
    if section and str(data.get("section_code", "")) != section:
        return False
    return True


def main() -> None:
    args = parse_args()
    records_dir = args.records_dir.resolve()
    if not records_dir.exists():
        raise SystemExit(f"records dir not found: {records_dir}")

    inspected = 0
    changed_count = 0
    write_count = 0
    kind_counts = Counter()

    for path, data in iter_records(records_dir):
        if not matches_filters(data, args.work_code, args.section):
            continue

        inspected += 1
        kind = infer_kind(data)
        kind_counts[kind] += 1
        changed = enrich_tags(data, kind, repair_kind=args.repair_kind)

        if changed:
            changed_count += 1
            if args.write:
                path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
                write_count += 1

    mode = "write" if args.write else "dry-run"
    print(f"mode={mode}")
    print(f"records_inspected={inspected}")
    print(f"records_changed={changed_count}")
    print(f"records_written={write_count}")
    print("kind_counts=" + json.dumps(dict(sorted(kind_counts.items())), ensure_ascii=False))


if __name__ == "__main__":
    main()
