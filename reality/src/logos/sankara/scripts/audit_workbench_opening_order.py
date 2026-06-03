#!/usr/bin/env python3
"""Audit opening-order consistency in Sankara workbench manifest.

Purpose:
- Detect sections where records tagged as preamble appear after the first sutra
  in numeric record-id order.
- Preserve existing analysis while making anomalies explicit and reviewable.
"""

from __future__ import annotations

import argparse
import json
import re
from collections import defaultdict
from pathlib import Path
from typing import Any

DEFAULT_MANIFEST = Path("reality/src/logos/sankara/workbench/manifest.json")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Audit section-level opening order anomalies in workbench manifest."
    )
    parser.add_argument(
        "--manifest",
        type=Path,
        default=DEFAULT_MANIFEST,
        help="Path to workbench manifest JSON.",
    )
    parser.add_argument(
        "--work-code",
        default="BS",
        help="Optional work-code filter (default: BS).",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Print JSON report instead of plain text.",
    )
    return parser.parse_args()


def section_sort_key(section: str) -> tuple[int, ...]:
    out: list[int] = []
    for part in str(section).split("."):
        try:
            out.append(int(part))
        except ValueError:
            out.append(0)
    return tuple(out)


def record_order_value(record_id: str) -> int:
    parts = str(record_id).split(".")
    if len(parts) < 5:
        return 10**9
    unit = parts[4]
    if unit == "PRE":
        return -1
    match = re.search(r"(\d+)", unit)
    return int(match.group(1)) if match else 10**9


def normalize_kind(value: Any) -> str:
    return str(value or "").strip().lower()


def build_report(records: list[dict[str, Any]]) -> dict[str, Any]:
    by_section: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in records:
        by_section[str(row.get("section_code", ""))].append(row)

    section_reports: list[dict[str, Any]] = []
    anomaly_count = 0

    for section in sorted(by_section.keys(), key=section_sort_key):
        rows = sorted(
            by_section[section],
            key=lambda r: record_order_value(str(r.get("record_id", ""))),
        )

        first_sutra_index = -1
        for idx, row in enumerate(rows):
            if normalize_kind(row.get("record_kind")) == "sutra":
                first_sutra_index = idx
                break

        preamble_after_sutra: list[str] = []
        if first_sutra_index >= 0:
            for row in rows[first_sutra_index + 1 :]:
                if normalize_kind(row.get("record_kind")) == "preamble":
                    preamble_after_sutra.append(str(row.get("record_id", "")))

        if preamble_after_sutra:
            anomaly_count += 1

        section_reports.append(
            {
                "section_code": section,
                "record_count": len(rows),
                "first_sutra_record_id": (
                    str(rows[first_sutra_index].get("record_id", ""))
                    if first_sutra_index >= 0
                    else None
                ),
                "preamble_after_sutra": preamble_after_sutra,
            }
        )

    return {
        "section_count": len(section_reports),
        "sections_with_anomaly": anomaly_count,
        "sections": section_reports,
    }


def main() -> int:
    args = parse_args()
    manifest_path = args.manifest.resolve()
    if not manifest_path.exists():
        raise SystemExit(f"manifest not found: {manifest_path}")

    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    records_raw = manifest.get("records") or []
    records: list[dict[str, Any]] = []
    for row in records_raw:
        if not isinstance(row, dict):
            continue
        if args.work_code and str(row.get("work_code", "")).upper() != args.work_code.upper():
            continue
        records.append(row)

    report = build_report(records)

    if args.json:
        print(json.dumps(report, ensure_ascii=False, indent=2))
        return 0

    print(f"work_code={args.work_code or 'ALL'}")
    print(f"sections={report['section_count']}")
    print(f"sections_with_anomaly={report['sections_with_anomaly']}")
    for section in report["sections"]:
        anomalies = section["preamble_after_sutra"]
        if not anomalies:
            continue
        print(
            f"- section {section['section_code']}: first_sutra={section['first_sutra_record_id']} preamble_after_sutra={', '.join(anomalies)}"
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
