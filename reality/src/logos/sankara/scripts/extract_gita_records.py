#!/usr/bin/env python3
"""Extract verse-level Gita records from downloaded Sankara HTML pages.

Outputs aggregate and chapter-split records in configurable formats
(JSON, JSONL, XML).
"""

from __future__ import annotations

import argparse
import glob
import hashlib
import html
import json
import re
from collections import defaultdict
from pathlib import Path
import xml.etree.ElementTree as ET
from typing import Any


VERSE_OPEN_RE = re.compile(
    r'<div\s+class="verse"[^>]*\sid="(BG_C\d+_V\d+)"[^>]*>',
    re.IGNORECASE,
)
DIV_TOKEN_RE = re.compile(r"<div\b[^>]*>|</div>", re.IGNORECASE)
ID_ATTR_RE = re.compile(r'\sid="([^"]+)"', re.IGNORECASE)
TAG_RE = re.compile(r"<[^>]+>", re.DOTALL)
QUOTE_RE = re.compile(
    r'<span[^>]*class="[^"]*\bqt(?:_o)?\b[^"]*"[^>]*>(.*?)</span>',
    re.IGNORECASE | re.DOTALL,
)
HREF_RE = re.compile(r'href="([^"]+)"', re.IGNORECASE)


SCRIPT_DIR = Path(__file__).resolve().parent
SANKARA_DIR = SCRIPT_DIR.parent


def resolve_default_path(value: str | Path, *base_dirs: Path) -> Path:
    path = Path(value)
    if path.is_absolute():
        return path
    for base_dir in base_dirs:
        candidate = (base_dir / path).resolve()
        if candidate.exists() or candidate.parent.exists():
            return candidate
    return (Path.cwd() / path).resolve()


def resolve_glob_pattern(pattern: str, *base_dirs: Path) -> str:
    path = Path(pattern)
    if path.is_absolute():
        return str(path)

    wildcard_chars = "*?["
    anchor_parts: list[str] = []
    for part in path.parts:
        if any(ch in part for ch in wildcard_chars):
            break
        anchor_parts.append(part)
    anchor = Path(*anchor_parts) if anchor_parts else Path(".")

    for base_dir in base_dirs:
        if (base_dir / anchor).exists():
            return str((base_dir / path).resolve())
    return str((Path.cwd() / path).resolve())


def normalize_text(raw: str) -> str:
    raw = raw.replace("<br/>", "\n").replace("<br />", "\n").replace("<br>", "\n")
    raw = TAG_RE.sub(" ", raw)
    raw = html.unescape(raw)
    raw = raw.replace("\xa0", " ")
    raw = re.sub(r"[ \t\r\f\v]+", " ", raw)
    raw = re.sub(r"\n\s+", "\n", raw)
    raw = re.sub(r"\n{3,}", "\n\n", raw)
    return raw.strip()


def extract_balanced_div(html_text: str, start_idx: int) -> str:
    depth = 0
    for token in DIV_TOKEN_RE.finditer(html_text, pos=start_idx):
        frag = token.group(0).lower()
        if frag.startswith("<div"):
            depth += 1
        else:
            depth -= 1
            if depth == 0:
                return html_text[start_idx:token.end()]
    return ""


def parse_blocks(verse_html: str, class_name: str) -> list[dict[str, str]]:
    pat = re.compile(
        rf'<div\s+class="{class_name}"([^>]*)>(.*?)</div>',
        re.IGNORECASE | re.DOTALL,
    )
    out: list[dict[str, str]] = []
    for m in pat.finditer(verse_html):
        attrs = m.group(1) or ""
        body = m.group(2) or ""
        text = normalize_text(body)
        if not text:
            continue
        id_match = ID_ATTR_RE.search(attrs)
        out.append(
            {
                "id": id_match.group(1) if id_match else "",
                "text": text,
            }
        )
    return out


def parse_citations(verse_html: str) -> list[dict[str, str]]:
    citations: list[dict[str, str]] = []
    for m in QUOTE_RE.finditer(verse_html):
        body = m.group(1) or ""
        href_match = HREF_RE.search(body)
        text = normalize_text(body)
        if not text:
            continue
        citations.append(
            {
                "href": href_match.group(1) if href_match else "",
                "text": text,
            }
        )
    return citations


def kinds_from_components(
    versetext_blocks: list[dict[str, str]],
    leading_blocks: list[dict[str, str]],
    bhashya_blocks: list[dict[str, str]],
) -> list[str]:
    kinds: list[str] = []
    if versetext_blocks:
        kinds.append("sutra")
    if leading_blocks:
        kinds.append("preamble")
    if bhashya_blocks:
        kinds.append("bhasya")
    return kinds


def extract_records(html_text: str) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []

    for vm in VERSE_OPEN_RE.finditer(html_text):
        verse_id = vm.group(1)
        chapter_id = verse_id.rsplit("_", 1)[0]

        verse_html = extract_balanced_div(html_text, vm.start())
        if not verse_html:
            continue

        versetext_blocks = parse_blocks(verse_html, "versetext")
        leading_blocks = parse_blocks(verse_html, "leading_bhashya")
        bhashya_blocks = parse_blocks(verse_html, "bhashya")
        citations = parse_citations(verse_html)

        if not versetext_blocks and not leading_blocks and not bhashya_blocks:
            continue

        records.append(
            {
                "chapter_id": chapter_id,
                "verse_id": verse_id,
                "versetext": "\n".join(b["text"] for b in versetext_blocks).strip(),
                "leading_bhashya": "\n\n".join(b["text"] for b in leading_blocks).strip(),
                "bhashya_blocks": bhashya_blocks,
                "source_block_types_present": [
                    block_type
                    for block_type, blocks in (
                        ("versetext", versetext_blocks),
                        ("leading_bhashya", leading_blocks),
                        ("bhashya", bhashya_blocks),
                    )
                    if blocks
                ],
                "record_kinds_present": kinds_from_components(
                    versetext_blocks,
                    leading_blocks,
                    bhashya_blocks,
                ),
                "citations": citations,
            }
        )

    return records


def record_score(row: dict[str, Any]) -> int:
    """Prefer richer duplicate records when merging multiple HTML witnesses."""
    text_len = len(str(row.get("versetext", "")))
    lead_len = len(str(row.get("leading_bhashya", "")))
    blocks = row.get("bhashya_blocks", [])
    citations = row.get("citations", [])
    block_text_len = sum(len(str(b.get("text", ""))) for b in blocks if isinstance(b, dict))
    return text_len + lead_len + block_text_len + 50 * len(blocks) + 10 * len(citations)


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        for row in rows:
            f.write(json.dumps(row, ensure_ascii=False) + "\n")


def write_json(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        json.dump(rows, f, ensure_ascii=False, indent=2)
        f.write("\n")


def append_xml_value(parent: ET.Element, key: str, value: Any) -> None:
    child = ET.SubElement(parent, key)
    if isinstance(value, dict):
        for sub_key, sub_val in value.items():
            append_xml_value(child, str(sub_key), sub_val)
        return
    if isinstance(value, list):
        for item in value:
            append_xml_value(child, "item", item)
        return
    child.text = "" if value is None else str(value)


def write_xml(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    root = ET.Element("records")
    for row in rows:
        rec = ET.SubElement(root, "record")
        for key, value in row.items():
            append_xml_value(rec, str(key), value)

    tree = ET.ElementTree(root)
    tree.write(path, encoding="utf-8", xml_declaration=True)


def group_rows_by_chapter(rows: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    chapters: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        chapter_id = str(row.get("chapter_id", "")).strip()
        if not chapter_id:
            continue
        chapters[chapter_id].append(row)
    return chapters


def write_chapter_files_json(out_dir: Path, rows: list[dict[str, Any]]) -> int:
    chapters = group_rows_by_chapter(rows)
    out_dir.mkdir(parents=True, exist_ok=True)
    written = 0
    for chapter_id in sorted(chapters.keys()):
        chapter_path = out_dir / f"{chapter_id}.json"
        write_json(chapter_path, chapters[chapter_id])
        written += 1
    return written


def write_chapter_files_jsonl(out_dir: Path, rows: list[dict[str, Any]]) -> int:
    chapters = group_rows_by_chapter(rows)
    out_dir.mkdir(parents=True, exist_ok=True)
    written = 0
    for chapter_id in sorted(chapters.keys()):
        chapter_path = out_dir / f"{chapter_id}.jsonl"
        write_jsonl(chapter_path, chapters[chapter_id])
        written += 1
    return written


def write_chapter_files_xml(out_dir: Path, rows: list[dict[str, Any]]) -> int:
    chapters = group_rows_by_chapter(rows)
    out_dir.mkdir(parents=True, exist_ok=True)
    written = 0
    for chapter_id in sorted(chapters.keys()):
        chapter_path = out_dir / f"{chapter_id}.xml"
        write_xml(chapter_path, chapters[chapter_id])
        written += 1
    return written


def emission_formats(emit: str) -> list[str]:
    if emit == "all":
        return ["json", "jsonl", "xml"]
    return [emit]


def main() -> int:
    parser = argparse.ArgumentParser(description="Extract verse-level Gita records from Sankara HTML")
    parser.add_argument(
        "--html-file",
        type=Path,
        default=None,
        help="Path to one downloaded Gita HTML witness (optional)",
    )
    parser.add_argument(
        "--html-glob",
        default=resolve_glob_pattern(
            "raw/pages/*display_bhashya_Gita_devanagari_*/index.html",
            SANKARA_DIR,
            SCRIPT_DIR,
        ),
        help="Glob for Gita HTML witnesses used when --html-file is not provided",
    )
    parser.add_argument(
        "--out-file",
        type=Path,
        default=resolve_default_path("derived/Gita/BG.json", SANKARA_DIR, SCRIPT_DIR),
        help="Output aggregate JSON path",
    )
    parser.add_argument(
        "--emit",
        choices=["json", "jsonl", "xml", "all"],
        default="json",
        help="Output format(s) for aggregate and chapter files",
    )
    parser.add_argument(
        "--source-url",
        default="https://advaitasharada.sringeri.net/display/bhashya/Gita/devanagari",
        help="Source URL for provenance field",
    )
    args = parser.parse_args()

    html_files: list[Path] = []
    if args.html_file is not None:
        if not args.html_file.exists():
            raise SystemExit(f"HTML file not found: {args.html_file}")
        html_files = [args.html_file]
    else:
        html_files = sorted(Path(p) for p in glob.glob(args.html_glob))
        if not html_files:
            raise SystemExit(f"No HTML files matched glob: {args.html_glob}")

    merged_by_verse: dict[str, dict[str, Any]] = {}
    for html_file in html_files:
        raw = html_file.read_text(encoding="utf-8", errors="replace")
        sha256 = hashlib.sha256(raw.encode("utf-8")).hexdigest()
        records = extract_records(raw)
        for r in records:
            row = {"source_url": args.source_url, "source_sha256": sha256, **r}
            verse_id = str(row.get("verse_id", "")).strip()
            if not verse_id:
                continue
            prev = merged_by_verse.get(verse_id)
            if prev is None or record_score(row) > record_score(prev):
                merged_by_verse[verse_id] = row

    enriched = [merged_by_verse[k] for k in sorted(merged_by_verse.keys())]
    if not enriched:
        raise SystemExit("No verse records extracted")

    format_handlers = {
        "json": (".json", write_json, write_chapter_files_json),
        "jsonl": (".jsonl", write_jsonl, write_chapter_files_jsonl),
        "xml": (".xml", write_xml, write_chapter_files_xml),
    }
    outputs: list[str] = []
    for fmt in emission_formats(args.emit):
        suffix, agg_writer, chapter_writer = format_handlers[fmt]
        aggregate_path = args.out_file.with_suffix(suffix)
        agg_writer(aggregate_path, enriched)
        chapter_count = chapter_writer(aggregate_path.parent, enriched)
        outputs.append(
            f"{fmt}: aggregate={aggregate_path} chapters={chapter_count}"
        )

    print(f"processed {len(html_files)} HTML file(s)")
    print(f"extracted {len(enriched)} merged records")
    for line in outputs:
        print(line)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
