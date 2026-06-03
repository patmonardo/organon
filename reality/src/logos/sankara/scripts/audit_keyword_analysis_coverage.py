#!/usr/bin/env python3
"""Audit keyword technical-analysis coverage for Sankara translation records.

This script performs a complete sweep over passage records and produces:
- JSON audit report (machine-readable)
- Markdown remediation queue (human workflow)

Default scope targets BS_C01 records under translation/passage_records.
"""

from __future__ import annotations

import argparse
import json
import re
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
SANKARA_DIR = SCRIPT_DIR.parent
DEFAULT_RECORD_GLOB = "translation/passage_records/BS_C01_S*/*.json"
DEFAULT_OUTPUT_DIR = SANKARA_DIR / "translation" / "translation-records" / "BS_C01"

REQUIRED_LEXICAL_FIELDS = (
    "headword_iast",
    "gloss",
    "contextual_sense",
    "grammatical_role",
)


@dataclass(frozen=True)
class RecordAudit:
    record_id: str
    section_code: str
    path: str
    keyword_count: int
    complete_keyword_count: int
    missing_field_count: int
    status: str
    reasons: list[str]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Audit keyword technical-analysis depth across translation records."
    )
    parser.add_argument(
        "--record-glob",
        default=DEFAULT_RECORD_GLOB,
        help="Glob (relative to sankara root) for record JSON files.",
    )
    parser.add_argument(
        "--min-keywords",
        type=int,
        default=2,
        help="Minimum number of lexical keyword entries required per record.",
    )
    parser.add_argument(
        "--min-complete-keywords",
        type=int,
        default=2,
        help="Minimum number of fully populated lexical entries required.",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=DEFAULT_OUTPUT_DIR,
        help="Directory for audit artifacts.",
    )
    parser.add_argument(
        "--json-out",
        default="keyword_analysis_sweep_report.json",
        help="Output JSON filename.",
    )
    parser.add_argument(
        "--md-out",
        default="keyword_analysis_sweep_queue.md",
        help="Output Markdown filename.",
    )
    return parser.parse_args()


def record_sort_key(record_id: str) -> tuple[Any, ...]:
    parts = record_id.split(".")
    if len(parts) < 5:
        return (record_id,)
    work = parts[1]
    chapter = int(parts[2]) if parts[2].isdigit() else 0
    section = int(parts[3]) if parts[3].isdigit() else 0
    tail = parts[4]
    if tail == "PRE":
        unit = -1
    else:
        m = re.search(r"(\d+)", tail)
        unit = int(m.group(1)) if m else 0
    return (work, chapter, section, unit, tail)


def section_sort_key(section_code: str) -> tuple[Any, ...]:
    m = re.match(r"^(\d+)\.(\d+)$", section_code)
    if not m:
        return (section_code,)
    return (int(m.group(1)), int(m.group(2)))


def lexical_entry_missing_fields(entry: dict[str, Any]) -> int:
    missing = 0
    for field in REQUIRED_LEXICAL_FIELDS:
        raw = entry.get(field, "")
        if not isinstance(raw, str) or not raw.strip():
            missing += 1
    return missing


def audit_record(
    data: dict[str, Any],
    path: Path,
    min_keywords: int,
    min_complete_keywords: int,
) -> RecordAudit:
    record_id = str(data.get("record_id", "")).strip() or path.stem
    section_code = str(data.get("section_code", "")).strip() or "unknown"

    analysis = data.get("analysis") or {}
    lexical = analysis.get("lexical_notes")

    entries: list[dict[str, Any]] = []
    if isinstance(lexical, list):
        for item in lexical:
            if isinstance(item, dict):
                entries.append(item)

    keyword_count = len(entries)
    missing_field_count = 0
    complete_keyword_count = 0

    for entry in entries:
        missing = lexical_entry_missing_fields(entry)
        missing_field_count += missing
        if missing == 0:
            complete_keyword_count += 1

    reasons: list[str] = []
    if keyword_count < min_keywords:
        reasons.append("keyword_count_below_min")
    if complete_keyword_count < min_complete_keywords:
        reasons.append("complete_keyword_count_below_min")
    if missing_field_count > 0:
        reasons.append("lexical_entries_missing_required_fields")

    status = "needs_sweep" if reasons else "ok"

    return RecordAudit(
        record_id=record_id,
        section_code=section_code,
        path=path.as_posix(),
        keyword_count=keyword_count,
        complete_keyword_count=complete_keyword_count,
        missing_field_count=missing_field_count,
        status=status,
        reasons=reasons,
    )


def load_records(record_glob: str) -> list[Path]:
    return sorted((SANKARA_DIR / ".").glob(record_glob))


def build_markdown(
    audits: list[RecordAudit],
    min_keywords: int,
    min_complete_keywords: int,
) -> str:
    total = len(audits)
    needs = [x for x in audits if x.status == "needs_sweep"]
    ok = total - len(needs)

    by_section: dict[str, list[RecordAudit]] = defaultdict(list)
    for row in needs:
        by_section[row.section_code].append(row)

    lines: list[str] = [
        "# BS_C01 Keyword Analysis Sweep Queue",
        "",
        "## Audit Basis",
        f"- Minimum keyword entries: {min_keywords}",
        f"- Minimum complete keyword entries: {min_complete_keywords}",
        "- Required lexical fields: headword_iast, gloss, contextual_sense, grammatical_role",
        "",
        "## Summary",
        f"- Records scanned: {total}",
        f"- Records needing sweep: {len(needs)}",
        f"- Records passing audit: {ok}",
        "",
    ]

    if not needs:
        lines.extend([
            "## Queue",
            "- All records pass current keyword analysis thresholds.",
            "",
        ])
        return "\n".join(lines)

    lines.append("## Queue By Section")

    for section in sorted(by_section.keys(), key=section_sort_key):
        rows = sorted(by_section[section], key=lambda x: record_sort_key(x.record_id))
        lines.append("")
        lines.append(f"### Section {section}")
        lines.append(f"- Pending records: {len(rows)}")
        lines.append("")
        for row in rows:
            reasons = ", ".join(row.reasons)
            lines.append(
                f"- {row.record_id} | keywords={row.keyword_count} | complete={row.complete_keyword_count} | missing_fields={row.missing_field_count} | {reasons}"
            )

    lines.append("")
    return "\n".join(lines)


def main() -> int:
    args = parse_args()

    records = load_records(args.record_glob)
    if not records:
        raise SystemExit(f"No records matched glob: {args.record_glob}")

    audits: list[RecordAudit] = []
    for record_path in records:
        data = json.loads(record_path.read_text(encoding="utf-8"))
        audits.append(
            audit_record(
                data=data,
                path=record_path,
                min_keywords=args.min_keywords,
                min_complete_keywords=args.min_complete_keywords,
            )
        )

    audits_sorted = sorted(audits, key=lambda x: record_sort_key(x.record_id))

    payload = {
        "meta": {
            "record_glob": args.record_glob,
            "records_scanned": len(audits_sorted),
            "min_keywords": args.min_keywords,
            "min_complete_keywords": args.min_complete_keywords,
            "required_lexical_fields": list(REQUIRED_LEXICAL_FIELDS),
        },
        "records": [
            {
                "record_id": row.record_id,
                "section_code": row.section_code,
                "path": row.path,
                "keyword_count": row.keyword_count,
                "complete_keyword_count": row.complete_keyword_count,
                "missing_field_count": row.missing_field_count,
                "status": row.status,
                "reasons": row.reasons,
            }
            for row in audits_sorted
        ],
    }

    out_dir = args.output_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    json_path = out_dir / args.json_out
    md_path = out_dir / args.md_out

    json_path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    md_path.write_text(
        build_markdown(
            audits=audits_sorted,
            min_keywords=args.min_keywords,
            min_complete_keywords=args.min_complete_keywords,
        )
        + "\n",
        encoding="utf-8",
    )

    needs = sum(1 for x in audits_sorted if x.status == "needs_sweep")
    print(
        f"Audit complete: scanned={len(audits_sorted)} needs_sweep={needs} json={json_path.as_posix()} md={md_path.as_posix()}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
