#!/usr/bin/env python3
"""Report curation progress for Sankara BS passage-record translations.

Default scope is BS_C01 passage records.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path

PLACEHOLDER_PREFIX = "DRAFT: Literal translation pending hand curation"


def iter_json_files(path: Path):
    if path.is_file() and path.suffix == ".json":
        yield path
        return
    if path.is_dir():
        for p in sorted(path.rglob("*.json")):
            if p.is_file():
                yield p


def main() -> int:
    parser = argparse.ArgumentParser(description="Report BS translation curation progress")
    parser.add_argument(
        "--path",
        type=Path,
        default=Path("reality/src/logos/sankara/translation/passage_records"),
        help="Directory (or single file) of passage record JSON files",
    )
    args = parser.parse_args()

    files = list(iter_json_files(args.path))
    if not files:
        print("No JSON files found.")
        return 1

    total = 0
    curated = 0
    scaffold = 0
    by_section = defaultdict(lambda: Counter(total=0, curated=0, scaffold=0))
    by_block_type = defaultdict(lambda: Counter(total=0, curated=0, scaffold=0))

    for f in files:
        data = json.loads(f.read_text(encoding="utf-8"))
        total += 1

        section = str(data.get("section_code", "unknown"))
        seg = str(data.get("text", {}).get("segmentation_note", ""))
        edition_note = str(data.get("source", {}).get("edition_note", ""))
        block_type = "unknown"

        # segmentation_note format may include trailing "(...block_type...)"
        if "(" in seg and ")" in seg:
            parsed = seg.rsplit("(", 1)[-1].split(")", 1)[0].strip()
            if parsed:
                block_type = parsed

        # Fallback for scaffold records: edition_note ends with "(...block_type...)."
        if block_type == "unknown" and "(" in edition_note and ")" in edition_note:
            parsed = edition_note.rsplit("(", 1)[-1].split(")", 1)[0].strip()
            if parsed:
                block_type = parsed

        literal = str(data.get("translation", {}).get("literal_translation", ""))
        is_scaffold = literal.startswith(PLACEHOLDER_PREFIX)

        if is_scaffold:
            scaffold += 1
            by_section[section]["scaffold"] += 1
            by_block_type[block_type]["scaffold"] += 1
        else:
            curated += 1
            by_section[section]["curated"] += 1
            by_block_type[block_type]["curated"] += 1

        by_section[section]["total"] += 1
        by_block_type[block_type]["total"] += 1

    print(f"Records: total={total} curated={curated} scaffold={scaffold}")
    pct = (curated / total) * 100 if total else 0.0
    print(f"Curation completion: {pct:.2f}%")

    print("\nBy section:")
    for section in sorted(by_section.keys(), key=lambda s: [int(x) if x.isdigit() else x for x in s.split(".")]):
        c = by_section[section]
        spct = (c["curated"] / c["total"]) * 100 if c["total"] else 0.0
        print(
            f"  {section}: total={c['total']} curated={c['curated']} scaffold={c['scaffold']} ({spct:.2f}% curated)"
        )

    print("\nBy block type:")
    for bt in sorted(by_block_type.keys()):
        c = by_block_type[bt]
        bpct = (c["curated"] / c["total"]) * 100 if c["total"] else 0.0
        print(
            f"  {bt}: total={c['total']} curated={c['curated']} scaffold={c['scaffold']} ({bpct:.2f}% curated)"
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
