#!/usr/bin/env python3
"""Readable console view for Sankara passage records.

This avoids manually reading raw JSON/JSONL by printing compact, human-oriented
record blocks with key fields.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

DEFAULT_RECORDS_DIR = Path("reality/src/logos/sankara/translation/passage_records")


def pick_first(data: dict[str, Any], keys: list[str]) -> Any:
    for key in keys:
        current: Any = data
        found = True
        for part in key.split("."):
            if isinstance(current, dict) and part in current:
                current = current.get(part)
            else:
                found = False
                break
        if found and current is not None:
            return current
    return None


def pick_first_str(data: dict[str, Any], keys: list[str]) -> str:
    value = pick_first(data, keys)
    if value is None:
        return ""
    return str(value).strip()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Print readable passage records from Sankara translation JSON files."
    )
    parser.add_argument(
        "--records-dir",
        type=Path,
        default=DEFAULT_RECORDS_DIR,
        help="Directory containing passage record JSON files.",
    )
    parser.add_argument(
        "--work-code",
        default="BS",
        help="Optional work code filter (BS, GITA, UPAN, ...).",
    )
    parser.add_argument(
        "--section",
        default="",
        help="Optional section filter (e.g. 1.1).",
    )
    parser.add_argument(
        "--kind",
        default="",
        help="Optional record kind filter (sutra, bhasya, preamble, transition).",
    )
    parser.add_argument(
        "--record-id",
        default="",
        help="Optional exact record id filter.",
    )
    parser.add_argument(
        "--view",
        choices=("paired", "records"),
        default="paired",
        help="Display mode: paired (sutra + following bhasya) or flat records.",
    )
    parser.add_argument(
        "--max-chars",
        type=int,
        default=0,
        help="Per-field character cap. 0 means full text.",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=20,
        help="Maximum number of records to print (default: 20).",
    )
    return parser.parse_args()


def short(text: str, max_len: int = 280) -> str:
    clean = " ".join((text or "").split())
    if max_len <= 0:
        return clean
    if len(clean) <= max_len:
        return clean
    return clean[: max_len - 1] + "…"


def parse_record_sort_key(record_id: str) -> tuple[Any, ...]:
    parts = record_id.split(".")
    if len(parts) < 5:
        return (record_id,)

    work = parts[1]
    try:
        chapter = int(parts[2])
    except ValueError:
        chapter = 0

    try:
        section = int(parts[3])
    except ValueError:
        section = 0

    unit = parts[4]
    if unit == "PRE":
        unit_order = -1
    else:
        match = re.search(r"(\d+)", unit)
        unit_order = int(match.group(1)) if match else 0

    return (work, chapter, section, unit_order, unit)


def infer_kind(data: dict[str, Any]) -> str:
    source = data.get("source") or {}
    text = data.get("text") or {}
    record_id = pick_first_str(data, ["record_id", "id"])

    if pick_first_str(data, ["bhasya_text", "text.bhasya_text", "bhasya", "text.bhasya"]):
        return "bhasya"
    if pick_first_str(data, ["sutra_text", "text.sutra_text", "sutra", "text.sutra"]):
        return "sutra"

    signals = " ".join(
        [
            str(source.get("edition_note", "")).lower(),
            str(text.get("segmentation_note", "")).lower(),
            pick_first_str(
                data,
                [
                    "segmentation_note",
                    "text.segmentationNote",
                    "source.editionNote",
                    "notes.segmentation",
                ],
            ).lower(),
        ]
    )

    # Strong text-level markers are authoritative and should override stale tags.
    if (
        record_id.endswith(".PRE")
        or "preamble" in signals
        or "_pre/" in signals
        or "_pre_" in signals
    ):
        return "preamble"
    if "versetext" in signals or "core sutra line" in signals or "sutra" in signals or "mantra" in signals:
        return "sutra"
    if "transition" in signals:
        return "transition"

    for key in ("record_kind", "kind"):
        val = str(data.get(key, "")).strip().lower()
        if val:
            return val
    if "bhashya" in signals:
        return "bhasya"
    return "unknown"


def iter_records(records_dir: Path):
    for path in sorted(records_dir.glob("**/*.json")):
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except Exception:
            continue
        if isinstance(data, dict):
            yield path, data


def render_record(path: Path, data: dict[str, Any], records_dir: Path, max_chars: int) -> None:
    work_code = pick_first_str(data, ["work_code", "workCode", "work.code"]).upper()
    section_code = pick_first_str(data, ["section_code", "sectionCode", "section.code"])
    kind = infer_kind(data)

    text = data.get("text") or {}
    translation = data.get("translation") or {}

    source_text = pick_first_str(
        data,
        [
            "text.source_text",
            "text.sourceText",
            "source_text",
            "sourceText",
            "sutra_text",
            "bhasya_text",
            "sutra",
            "bhasya",
        ],
    )
    translit = pick_first_str(
        data,
        [
            "text.transliteration_iast",
            "text.transliterationIAST",
            "transliteration_iast",
            "transliterationIAST",
        ],
    )
    literal = pick_first_str(
        data,
        [
            "translation.literal_translation",
            "translation.literalTranslation",
            "literal_translation",
            "literalTranslation",
        ],
    )
    technical = pick_first_str(
        data,
        [
            "translation.technical_translation",
            "translation.technicalTranslation",
            "technical_translation",
            "technicalTranslation",
        ],
    )

    print("=" * 96)
    print(f"Record: {pick_first_str(data, ['record_id', 'id'])}")
    print(f"Work/Section: {work_code} / {section_code}    Kind: {kind}")
    print(f"File: {path.relative_to(records_dir.parent).as_posix()}")
    print("- Source Text:")
    print(f"  {short(source_text, max_chars)}" if source_text else "  (empty)")
    print("- Transliteration:")
    print(f"  {short(translit, max_chars)}" if translit else "  (empty)")
    print("- Translation (Literal):")
    print(f"  {short(literal, max_chars)}" if literal else "  (empty)")
    print("- Translation (Technical):")
    print(f"  {short(technical, max_chars)}" if technical else "  (empty)")


def print_paired_block(
    title: str,
    items: list[tuple[Path, dict[str, Any]]],
    records_dir: Path,
    max_chars: int,
) -> None:
    print("\n" + "#" * 96)
    print(title)
    for path, data in items:
        render_record(path, data, records_dir, max_chars)


def main() -> None:
    args = parse_args()
    records_dir = args.records_dir.resolve()
    if not records_dir.exists():
        raise SystemExit(f"records dir not found: {records_dir}")

    rows: list[tuple[Path, dict[str, Any]]] = []
    for path, data in iter_records(records_dir):
        work_code = pick_first_str(data, ["work_code", "workCode", "work.code"]).upper()
        section_code = pick_first_str(data, ["section_code", "sectionCode", "section.code"])
        kind = infer_kind(data)

        if args.work_code and work_code != args.work_code.upper():
            continue
        if args.section and section_code != args.section:
            continue
        if args.kind and kind != args.kind.lower():
            continue
        if args.record_id and pick_first_str(data, ["record_id", "id"]) != args.record_id:
            continue
        rows.append((path, data))

    rows.sort(key=lambda row: parse_record_sort_key(pick_first_str(row[1], ["record_id", "id"])))

    if args.view == "records":
        shown = 0
        for path, data in rows:
            render_record(path, data, records_dir, args.max_chars)
            shown += 1
            if shown >= max(1, args.limit):
                break
        if shown == 0:
            print("No records matched filters.")
        return

    preamble_rows: list[tuple[Path, dict[str, Any]]] = []
    paired_blocks: list[tuple[tuple[Path, dict[str, Any]], list[tuple[Path, dict[str, Any]]]]] = []
    orphan_bhasya: list[tuple[Path, dict[str, Any]]] = []

    current_sutra: tuple[Path, dict[str, Any]] | None = None
    current_bhasya: list[tuple[Path, dict[str, Any]]] = []

    for row in rows:
        _, data = row
        kind = infer_kind(data)
        if kind == "preamble":
            preamble_rows.append(row)
            continue
        if kind == "sutra":
            if current_sutra is not None:
                paired_blocks.append((current_sutra, current_bhasya))
            current_sutra = row
            current_bhasya = []
            continue
        if current_sutra is None:
            orphan_bhasya.append(row)
            continue
        current_bhasya.append(row)

    if current_sutra is not None:
        paired_blocks.append((current_sutra, current_bhasya))

    shown_blocks = 0
    if preamble_rows:
        print_paired_block("OPENING PREAMBLE", preamble_rows, records_dir, args.max_chars)
        shown_blocks += 1

    if orphan_bhasya:
        print_paired_block("OPENING BHASYA (BEFORE FIRST SUTRA)", orphan_bhasya, records_dir, args.max_chars)
        shown_blocks += 1

    for sutra_row, bhasya_rows in paired_blocks:
        sutra_id = str(sutra_row[1].get("record_id", ""))
        title = f"SUTRA UNIT: {sutra_id}"
        unit_rows = [sutra_row] + bhasya_rows
        print_paired_block(title, unit_rows, records_dir, args.max_chars)
        shown_blocks += 1
        if shown_blocks >= max(1, args.limit):
            break

    if shown_blocks == 0:
        print("No records matched filters.")


if __name__ == "__main__":
    main()
