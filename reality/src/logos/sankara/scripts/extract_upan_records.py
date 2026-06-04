#!/usr/bin/env python3
"""Extract block-level Upanishad records from downloaded Sankara HTML pages.

Covers all Upanishad witnesses under raw/pages/*display_bhashya_*_devanagari_*.
Outputs aggregate and split records in configurable formats (JSON, JSONL, XML):
- Whole corpus aggregate:       derived/Upan/Upan.*
- Per-text aggregate:           derived/{TextName}/{text_code}.*
- Per-chapter files:            derived/{TextName}/{text_code}_C##.*
- Per-section files:            derived/{TextName}/{text_code}_C##_S##.*  (where sections exist)
"""

from __future__ import annotations

import argparse
import fnmatch
import glob
import hashlib
import html
import json
import re
from collections import defaultdict
from pathlib import Path
import xml.etree.ElementTree as ET
from typing import Any


SOURCE_URL_BASE = "https://advaitasharada.sringeri.net/display/bhashya"

# Matches verse-level div anchor IDs for all Upanishad texts.
# Texts use various prefixes (AI, BR, Ch, IS, Ka, KP, KV, MK, MD, PR, SV, Ta)
# Chapter+section+verse: XX_C##_S##_V## or XX_C##_S##_I##
# Chapter+verse: XX_C##_V## or XX_C##_I##
VERSE_OPEN_RE = re.compile(
    r'<div\s+class="verse"[^>]*\sid="([A-Z][A-Za-z0-9]*_C\d+[^"]*)"[^>]*>',
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

# Extract text code, chapter, and optional section from a verse ID
TEXT_CODE_RE = re.compile(r"^([A-Za-z]+)_C(\d+)(?:_S(\d+))?", re.IGNORECASE)

# Extract text name from an HTML witness directory name
TEXT_NAME_DIR_RE = re.compile(r"display_bhashya_([A-Za-z_]+)_devanagari_")


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


def text_name_from_path(html_file: Path) -> str:
    m = TEXT_NAME_DIR_RE.search(html_file.parent.name)
    if m:
        return m.group(1)
    return ""


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


def parse_citations(block_html: str) -> list[dict[str, str]]:
    citations: list[dict[str, str]] = []
    for m in QUOTE_RE.finditer(block_html):
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


def parse_blocks(verse_html: str, class_name: str) -> list[dict[str, Any]]:
    pat = re.compile(
        rf'<div\s+class="{class_name}"([^>]*)>(.*?)</div>',
        re.IGNORECASE | re.DOTALL,
    )
    out: list[dict[str, Any]] = []
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
                "citations": parse_citations(body),
            }
        )
    return out


def ids_from_verse_id(verse_id: str) -> tuple[str, str, str]:
    """Return (text_id, chapter_id, section_id) derived from a verse ID."""
    m = TEXT_CODE_RE.match(verse_id)
    if not m:
        return ("", "", "")
    code = m.group(1)
    chap = m.group(2).zfill(2)
    sec = m.group(3)
    text_id = code
    chapter_id = f"{code}_C{chap}"
    section_id = f"{code}_C{chap}_S{sec.zfill(2)}" if sec else ""
    return text_id, chapter_id, section_id


def record_kind_from_block_type(block_type: str) -> str:
    normalized = str(block_type or "").strip().lower()
    if normalized == "versetext":
        return "sutra"
    if normalized == "intro_bhashya":
        return "preamble"
    if normalized in {"leading_bhashya", "bhashya"}:
        return "bhasya"
    return "unknown"


def extract_records(html_text: str) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    block_classes = ("versetext", "intro_bhashya", "leading_bhashya", "bhashya")

    for vm in VERSE_OPEN_RE.finditer(html_text):
        verse_id = vm.group(1)
        text_id, chapter_id, section_id = ids_from_verse_id(verse_id)
        if not text_id:
            continue

        verse_html = extract_balanced_div(html_text, vm.start())
        if not verse_html:
            continue

        for block_type in block_classes:
            blocks = parse_blocks(verse_html, block_type)
            for i, block in enumerate(blocks, start=1):
                block_id = str(block.get("id", "")).strip()
                if not block_id:
                    block_id = f"{verse_id}_{block_type.upper()}_{i:02d}"
                record_kind = record_kind_from_block_type(block_type)
                records.append(
                    {
                        "text_id": text_id,
                        "chapter_id": chapter_id,
                        "section_id": section_id,
                        "verse_id": verse_id,
                        "block_id": block_id,
                        "block_type": block_type,
                        "record_kind": record_kind,
                        "source_block_class": block_type,
                        "text": block["text"],
                        "citations": block["citations"],
                    }
                )

    return records


def record_score(row: dict[str, Any]) -> int:
    text_len = len(str(row.get("text", "")))
    citations = row.get("citations", [])
    return text_len + 10 * len(citations)


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
    ET.ElementTree(root).write(path, encoding="utf-8", xml_declaration=True)


def emission_formats(emit: str) -> list[str]:
    if emit == "all":
        return ["json", "jsonl", "xml"]
    return [emit]


def group_rows(rows: list[dict[str, Any]], key: str) -> dict[str, list[dict[str, Any]]]:
    groups: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        k = str(row.get(key, "")).strip()
        if not k:
            continue
        groups[k].append(row)
    return groups


def write_group_files(
    out_dir: Path,
    rows: list[dict[str, Any]],
    key: str,
    suffix: str,
    writer: Any,
) -> int:
    groups = group_rows(rows, key)
    out_dir.mkdir(parents=True, exist_ok=True)
    written = 0
    for group_id in sorted(groups.keys()):
        path = out_dir / f"{group_id}{suffix}"
        writer(path, groups[group_id])
        written += 1
    return written


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Extract block-level Upanishad records from Sankara HTML witnesses"
    )
    parser.add_argument(
        "--html-glob",
        default=resolve_glob_pattern(
            "raw/pages/*display_bhashya_*_devanagari_*/index.html",
            SANKARA_DIR,
            SCRIPT_DIR,
        ),
        help="Glob for Upanishad HTML witnesses (excludes Gita/BS by default if --exclude is set)",
    )
    parser.add_argument(
        "--exclude-glob",
        default="*display_bhashya_Gita_devanagari_*,*display_bhashya_BS_devanagari_*",
        help="Comma-separated glob patterns to exclude from the witness set",
    )
    parser.add_argument(
        "--out-file",
        type=Path,
        default=resolve_default_path("derived/Upan/Upan.json", SANKARA_DIR, SCRIPT_DIR),
        help="Output aggregate path base (extension overridden by --emit)",
    )
    parser.add_argument(
        "--source-url",
        default=SOURCE_URL_BASE,
        help="Source URL base for provenance field",
    )
    parser.add_argument(
        "--emit",
        choices=["json", "jsonl", "xml", "all"],
        default="json",
        help="Output format(s) for aggregate/text/chapter/section files",
    )
    args = parser.parse_args()

    exclude_patterns = [p.strip() for p in args.exclude_glob.split(",") if p.strip()]
    all_files = sorted(Path(p) for p in glob.glob(args.html_glob))
    html_files = [
        f for f in all_files
        if not any(fnmatch.fnmatch(str(f), f"*{pat}*") or fnmatch.fnmatch(f.parent.name, pat) for pat in exclude_patterns)
    ]
    if not html_files:
        raise SystemExit(f"No HTML files matched glob: {args.html_glob}")

    merged_by_block: dict[str, dict[str, Any]] = {}
    for html_file in html_files:
        raw = html_file.read_text(encoding="utf-8", errors="replace")
        sha256 = hashlib.sha256(raw.encode("utf-8")).hexdigest()
        tname = text_name_from_path(html_file)
        records = extract_records(raw)
        for r in records:
            row = {"text_name": tname, "source_url": args.source_url, "source_sha256": sha256, **r}
            block_id = str(row.get("block_id", "")).strip()
            if not block_id:
                continue
            prev = merged_by_block.get(block_id)
            if prev is None or record_score(row) > record_score(prev):
                merged_by_block[block_id] = row

    enriched = [merged_by_block[k] for k in sorted(merged_by_block.keys())]
    if not enriched:
        raise SystemExit("No Upanishad records extracted")

    format_handlers = {
        "json": (".json", write_json),
        "jsonl": (".jsonl", write_jsonl),
        "xml": (".xml", write_xml),
    }

    # Group by text_name for per-text subfolder output
    by_text: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in enriched:
        tname = str(row.get("text_name", "")).strip()
        if tname:
            by_text[tname].append(row)

    if args.out_file.parent.name.lower() == "upan":
        derived_root = args.out_file.parent.parent
    else:
        derived_root = args.out_file.parent

    outputs: list[str] = []
    for fmt in emission_formats(args.emit):
        suffix, writer = format_handlers[fmt]
        # Aggregate: derived/Upan/Upan.*
        aggregate_path = args.out_file.with_suffix(suffix)
        writer(aggregate_path, enriched)

        # Per-text subfolders: derived/{TextName}/{text_code}.*, chapters, sections
        total_texts = 0
        total_chapters = 0
        total_sections = 0
        for tname in sorted(by_text.keys()):
            text_rows = by_text[tname]
            text_dir = derived_root / tname
            total_texts += write_group_files(text_dir, text_rows, "text_id", suffix, writer)
            total_chapters += write_group_files(text_dir, text_rows, "chapter_id", suffix, writer)
            total_sections += write_group_files(text_dir, text_rows, "section_id", suffix, writer)

        outputs.append(
            f"{fmt}: aggregate={aggregate_path} texts={total_texts} chapters={total_chapters} sections={total_sections}"
        )

    print(f"processed {len(html_files)} HTML file(s)")
    print(f"extracted {len(enriched)} merged block records")
    for line in outputs:
        print(line)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
