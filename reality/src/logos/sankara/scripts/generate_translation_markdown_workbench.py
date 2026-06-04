#!/usr/bin/env python3
"""Generate deterministic Markdown workbench artifacts from translation records.

Outputs:
- Per-record Markdown files
- Per-section Markdown indexes
- Corpus index Markdown
- JSON manifest for browser viewer
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
SANKARA_DIR = SCRIPT_DIR.parent
DEFAULT_RECORDS_DIR = SANKARA_DIR / "translation" / "passage_records"
DEFAULT_WORKBENCH_DIR = SANKARA_DIR / "workbench"


@dataclass(frozen=True)
class RecordDoc:
    record_id: str
    work_code: str
    section_code: str
    source_text: str
    transliteration_iast: str
    literal_translation: str
    technical_translation: str
    interpretive_note: str
    argument_tags: list[str]
    lexical_notes: list[dict[str, Any]]
    source_url: str
    language: str
    script: str
    edition_note: str
    qa_status: str
    qa_confidence: float | None
    qa_uncertainty_notes: list[str]
    translator_version: str
    reviewer_version: str
    extraction_timestamp_utc: str
    source_manifest_path: str
    input_path: Path
    output_path: Path

    record_kind: str

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Project translation record JSON files into browserable markdown workbench artifacts."
    )
    parser.add_argument(
        "--records-dir",
        type=Path,
        default=DEFAULT_RECORDS_DIR,
        help="Directory containing translation record JSON files (default: translation/passage_records).",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=DEFAULT_WORKBENCH_DIR,
      help="Workbench output root (default: workbench).",
    )
    parser.add_argument(
        "--work-code",
        type=str,
        default="",
        help="Optional work-code filter (for example BS, GITA, UPAN).",
    )
    return parser.parse_args()


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
        m = re.search(r"(\d+)", unit)
        unit_order = int(m.group(1)) if m else 0

    return (work, chapter, section, unit_order, unit)


def discover_record_files(records_dir: Path) -> list[Path]:
    return sorted(records_dir.glob("**/*.json"))


def extract_section_slug(path: Path, section_code: str) -> str:
    parent = path.parent.name
    if parent:
        return parent

    safe_code = section_code.replace(".", "_")
    return f"SECTION_{safe_code}"


def as_list_of_str(value: Any) -> list[str]:
    if not isinstance(value, list):
        return []
    out: list[str] = []
    for item in value:
        if isinstance(item, str):
            out.append(item)
        else:
            out.append(str(item))
    return out


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


def normalize_record_kind(value: str) -> str:
    kind = value.strip().lower().replace("-", "_").replace(" ", "_")
    kind_map = {
        "verse": "sutra",
        "versetext": "sutra",
        "sutra": "sutra",
        "bhashya": "bhasya",
        "leading_bhashya": "bhasya",
        "intro_bhashya": "preamble",
        "preamble": "preamble",
        "preface": "preamble",
    }
    return kind_map.get(kind, kind)


def parse_trailing_paren_value(text: str) -> str:
    if "(" not in text or ")" not in text:
        return ""
    return text.rsplit("(", 1)[-1].split(")", 1)[0].strip()


def infer_record_kind(
    data: dict[str, Any],
    record_id: str,
    source: dict[str, Any],
    text: dict[str, Any],
) -> str:
    explicit_candidates = [
        data.get("kind"),
        data.get("record_kind"),
        source.get("kind"),
        source.get("record_kind"),
        text.get("kind"),
        text.get("record_kind"),
    ]
    for candidate in explicit_candidates:
        if isinstance(candidate, str) and candidate.strip():
            return normalize_record_kind(candidate)

    if pick_first_str(data, ["bhasya_text", "text.bhasya_text", "bhasya", "text.bhasya"]):
        return "bhasya"
    if pick_first_str(data, ["sutra_text", "text.sutra_text", "sutra", "text.sutra"]):
        return "sutra"

    text_signals = [
        str(text.get("segmentation_note", "")).lower(),
        str(source.get("edition_note", "")).lower(),
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

    if record_id.endswith(".PRE") or any(
        "preamble" in signal or "_pre/" in signal or "_pre_" in signal
        for signal in text_signals
    ):
        return "preamble"

    if any(
        "versetext" in signal
        or "core sutra line" in signal
        or "sutra text unit" in signal
        or "sutra" in signal
        or "mantra" in signal
        for signal in text_signals
    ):
        return "sutra"

    if any("transition" in signal for signal in text_signals):
        return "transition"

    if any("bhashya" in signal for signal in text_signals):
      return "bhasya"

    inferred_candidates = [
        parse_trailing_paren_value(str(text.get("segmentation_note", ""))),
        parse_trailing_paren_value(str(source.get("edition_note", ""))),
    ]
    for candidate in inferred_candidates:
        if candidate:
            normalized = normalize_record_kind(candidate)
            if normalized in {"preamble", "sutra", "bhasya", "transition"}:
                return normalized

    return "unknown"


def read_record_doc(path: Path, md_root: Path) -> RecordDoc | None:
    data = json.loads(path.read_text(encoding="utf-8"))

    record_id = pick_first_str(data, ["record_id", "id"])
    if not record_id:
        return None

    work_code = pick_first_str(data, ["work_code", "workCode", "work.code"])
    section_code = pick_first_str(data, ["section_code", "sectionCode", "section.code"])

    source = data.get("source") or {}
    text = data.get("text") or {}
    translation = data.get("translation") or {}
    analysis = data.get("analysis") or {}
    provenance = data.get("provenance") or {}
    qa = data.get("qa") or {}

    section_slug = extract_section_slug(path, section_code)
    out_path = md_root / "records" / section_slug / f"{record_id}.md"

    confidence_raw = qa.get("confidence")
    qa_confidence = None
    if isinstance(confidence_raw, (int, float)):
        qa_confidence = float(confidence_raw)

    lexical_notes_raw = pick_first(
        data,
        [
            "analysis.lexical_notes",
            "analysis.lexicalNotes",
            "lexical_notes",
            "lexicalNotes",
        ],
    )
    lexical_notes: list[dict[str, Any]] = []
    if isinstance(lexical_notes_raw, list):
        for note in lexical_notes_raw:
            if isinstance(note, dict):
                lexical_notes.append(note)

    argument_tags_raw = pick_first(
        data,
        [
            "analysis.argument_tags",
            "analysis.argumentTags",
            "argument_tags",
            "argumentTags",
        ],
    )

    record_kind = infer_record_kind(data, record_id, source, text)
    return RecordDoc(
        record_id=record_id,
        work_code=work_code,
        section_code=section_code,
        source_text=pick_first_str(
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
        ),
        transliteration_iast=pick_first_str(
            data,
            [
                "text.transliteration_iast",
                "text.transliterationIAST",
                "transliteration_iast",
                "transliterationIAST",
            ],
        ),
        literal_translation=pick_first_str(
            data,
            [
                "translation.literal_translation",
                "translation.literalTranslation",
                "literal_translation",
                "literalTranslation",
            ],
        ),
        technical_translation=pick_first_str(
            data,
            [
                "translation.technical_translation",
                "translation.technicalTranslation",
                "technical_translation",
                "technicalTranslation",
            ],
        ),
        interpretive_note=pick_first_str(
            data,
            [
                "translation.interpretive_note",
                "translation.interpretiveNote",
                "interpretive_note",
                "interpretiveNote",
            ],
        ),
        argument_tags=as_list_of_str(argument_tags_raw),
        lexical_notes=lexical_notes,
        source_url=pick_first_str(data, ["source.source_url", "source.sourceUrl", "source_url", "sourceUrl"]),
        language=pick_first_str(data, ["source.language", "language"]),
        script=pick_first_str(data, ["source.script", "script"]),
        edition_note=pick_first_str(data, ["source.edition_note", "source.editionNote", "edition_note", "editionNote"]),
        qa_status=pick_first_str(data, ["qa.status", "status"]),
        qa_confidence=qa_confidence,
        qa_uncertainty_notes=as_list_of_str(pick_first(data, ["qa.uncertainty_notes", "qa.uncertaintyNotes", "uncertainty_notes"])),
        translator_version=pick_first_str(data, ["provenance.translator_version", "provenance.translatorVersion", "translator_version", "translatorVersion"]),
        reviewer_version=pick_first_str(data, ["provenance.reviewer_version", "provenance.reviewerVersion", "reviewer_version", "reviewerVersion"]),
        extraction_timestamp_utc=pick_first_str(data, ["provenance.extraction_timestamp_utc", "provenance.extractionTimestampUtc", "extraction_timestamp_utc", "extractionTimestampUtc"]),
        source_manifest_path=pick_first_str(data, ["provenance.source_manifest_path", "provenance.sourceManifestPath", "source_manifest_path", "sourceManifestPath"]),
        input_path=path,
        output_path=out_path,
        record_kind=record_kind,
    )


def format_confidence(value: float | None) -> str:
    if value is None:
        return "n/a"
    return f"{value:.2f}"


def render_lexical_notes(notes: list[dict[str, Any]]) -> str:
    if not notes:
        return "- None\n"

    lines: list[str] = []
    for note in notes:
        headword = str(note.get("headword_iast", "")).strip() or "(unnamed)"
        gloss = str(note.get("gloss", "")).strip()
        contextual = str(note.get("contextual_sense", "")).strip()
        role = str(note.get("grammatical_role", "")).strip()

        lines.append(f"- **{headword}**")
        if gloss:
            lines.append(f"  - Gloss: {gloss}")
        if contextual:
            lines.append(f"  - Context: {contextual}")
        if role:
            lines.append(f"  - Grammar: {role}")

        alt = note.get("alternate_glosses")
        if isinstance(alt, list) and alt:
            alt_text = ", ".join(str(x) for x in alt)
            lines.append(f"  - Alternate glosses: {alt_text}")

    return "\n".join(lines) + "\n"


def render_record_markdown(doc: RecordDoc) -> str:
    tags_text = ", ".join(doc.argument_tags) if doc.argument_tags else "none"

    lines = [
        f"# {doc.record_id}",
        "",
        "## Metadata",
      f"- Kind: {doc.record_kind or 'unknown'}",
        f"- Work code: {doc.work_code or 'n/a'}",
        f"- Section code: {doc.section_code or 'n/a'}",
        f"- QA status: {doc.qa_status or 'n/a'}",
        f"- QA confidence: {format_confidence(doc.qa_confidence)}",
        f"- Argument tags: {tags_text}",
        "",
        "## Source",
        f"- Source URL: {doc.source_url or 'n/a'}",
        f"- Language: {doc.language or 'n/a'}",
        f"- Script: {doc.script or 'n/a'}",
        f"- Edition note: {doc.edition_note or 'n/a'}",
        "",
        "### Source Text",
        doc.source_text or "(empty)",
        "",
        "### Transliteration (IAST)",
        doc.transliteration_iast or "(empty)",
        "",
        "## Translation",
        "",
        "### Literal",
        doc.literal_translation or "(empty)",
        "",
        "### Technical",
        doc.technical_translation or "(empty)",
        "",
        "### Interpretive Note",
        doc.interpretive_note or "(empty)",
        "",
        "## Analysis",
        "",
        "### Lexical Notes",
        render_lexical_notes(doc.lexical_notes).rstrip("\n"),
        "",
        "## QA Uncertainty Notes",
    ]

    if doc.qa_uncertainty_notes:
        for note in doc.qa_uncertainty_notes:
            lines.append(f"- {note}")
    else:
        lines.append("- None")

    lines.extend(
        [
            "",
            "## Provenance",
            f"- Extraction timestamp (UTC): {doc.extraction_timestamp_utc or 'n/a'}",
            f"- Translator version: {doc.translator_version or 'n/a'}",
            f"- Reviewer version: {doc.reviewer_version or 'n/a'}",
            f"- Source manifest path: {doc.source_manifest_path or 'n/a'}",
            f"- Source JSON path: {doc.input_path.as_posix()}",
        ]
    )

    return "\n".join(lines) + "\n"


def write_record_docs(docs: list[RecordDoc]) -> None:
    for doc in docs:
        doc.output_path.parent.mkdir(parents=True, exist_ok=True)
        doc.output_path.write_text(render_record_markdown(doc), encoding="utf-8")


def section_sort_key(section: str) -> tuple[int, ...] | tuple[str]:
    m = re.match(r"^(\d+)\.(\d+)$", section)
    if not m:
        return (section,)
    return (int(m.group(1)), int(m.group(2)))


def write_section_indexes(md_root: Path, docs: list[RecordDoc]) -> list[dict[str, Any]]:
    section_dir = md_root / "sections"
    section_dir.mkdir(parents=True, exist_ok=True)

    by_section: dict[str, list[RecordDoc]] = {}
    for doc in docs:
        by_section.setdefault(doc.section_code, []).append(doc)

    section_manifest: list[dict[str, Any]] = []

    for section_code in sorted(by_section.keys(), key=section_sort_key):
        section_docs = sorted(by_section[section_code], key=lambda d: parse_record_sort_key(d.record_id))
        path = section_dir / f"{section_code}.md"

        lines = [
            f"# Section {section_code}",
            "",
            f"- Records: {len(section_docs)}",
            "",
            "## Records",
        ]

        for doc in section_docs:
            rel = doc.output_path.relative_to(md_root)
            lines.append(f"- [{doc.record_id}]({rel.as_posix()})")

        path.write_text("\n".join(lines) + "\n", encoding="utf-8")

        section_manifest.append(
            {
                "section_code": section_code,
                "record_count": len(section_docs),
                "md_path": path.relative_to(md_root.parent).as_posix(),
            }
        )

    return section_manifest


def write_corpus_index(md_root: Path, docs: list[RecordDoc], sections: list[dict[str, Any]]) -> None:
    corpus_path = md_root / "index.md"

    total = len(docs)
    by_work: dict[str, int] = {}
    for doc in docs:
        by_work[doc.work_code] = by_work.get(doc.work_code, 0) + 1

    lines = [
        "# Translation Workbench",
        "",
        "Deterministic Markdown projection generated from translation records.",
        "",
        "## Corpus Summary",
        f"- Total records: {total}",
    ]

    for work in sorted(by_work.keys()):
        lines.append(f"- {work}: {by_work[work]}")

    lines.extend(["", "## Section Index"])
    for section in sections:
        section_link = f"sections/{section['section_code']}.md"
        lines.append(
            f"- [Section {section['section_code']}]({section_link})"
            f" ({section['record_count']} records)"
        )

    corpus_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_manifest(workbench_root: Path, md_root: Path, docs: list[RecordDoc], sections: list[dict[str, Any]]) -> None:
    manifest_records: list[dict[str, Any]] = []

    for doc in sorted(docs, key=lambda d: parse_record_sort_key(d.record_id)):
        manifest_records.append(
            {
                "record_id": doc.record_id,
                "work_code": doc.work_code,
                "section_code": doc.section_code,
                "qa_status": doc.qa_status,
                "qa_confidence": doc.qa_confidence,
          "qa_uncertainty_notes": doc.qa_uncertainty_notes,
                "argument_tags": doc.argument_tags,
                "source_text": doc.source_text,
                "transliteration_iast": doc.transliteration_iast,
                "transliteration": doc.transliteration_iast,
                "literal_translation": doc.literal_translation,
                "technical_translation": doc.technical_translation,
                "md_path": doc.output_path.relative_to(workbench_root).as_posix(),
          "record_kind": doc.record_kind,
            }
        )

    manifest = {
        "meta": {
            "generator": "generate_translation_markdown_workbench.py@0.1",
            "record_count": len(manifest_records),
            "section_count": len(sections),
            "workbench_root": workbench_root.as_posix(),
        },
        "sections": sections,
        "records": manifest_records,
    }

    (workbench_root / "manifest.json").write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def write_viewer_html(workbench_root: Path) -> None:
    viewer_path = workbench_root / "viewer.html"
    if viewer_path.exists():
        return
    canonical_viewer = SANKARA_DIR / "workbench" / "viewer.html"
    if canonical_viewer.exists():
        viewer_path.write_text(
            canonical_viewer.read_text(encoding="utf-8"),
            encoding="utf-8",
        )
        return
    viewer_html = r"""<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\" />
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
  <title>Sankara Translation Workbench</title>
  <style>
    :root {
      --bg: #f5f2ea;
      --bg-panel: #fffdf8;
      --ink: #1f1b17;
      --muted: #6d6358;
      --line: #d8cfc1;
      --accent: #2b6f77;
      --accent-soft: #d8ecef;
    }

    * { box-sizing: border-box; }

    body {
      margin: 0;
      font-family: \"Source Serif 4\", \"Georgia\", serif;
      color: var(--ink);
      background: radial-gradient(circle at 10% 10%, #fefcf7 0%, #f1ece1 45%, #ebe3d6 100%);
      min-height: 100vh;
    }

    .layout {
      display: grid;
      grid-template-columns: 320px 1fr;
      min-height: 100vh;
    }

    .sidebar {
      border-right: 1px solid var(--line);
      background: linear-gradient(180deg, #f8f4ec 0%, #f3ebde 100%);
      padding: 16px;
      overflow: auto;
    }

    .brand {
      font-size: 20px;
      letter-spacing: 0.02em;
      margin: 0 0 10px;
    }

    .sub {
      margin: 0 0 12px;
      color: var(--muted);
      font-size: 14px;
    }

    .controls {
      display: grid;
      gap: 8px;
      margin-bottom: 14px;
    }

    .controls input,
    .controls select {
      width: 100%;
      border: 1px solid var(--line);
      border-radius: 8px;
      padding: 8px 10px;
      font-family: inherit;
      font-size: 14px;
      background: #fff;
      color: var(--ink);
    }

    .kind-presets {
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 6px;
    }

    .kind-presets button {
      border: 1px solid var(--line);
      border-radius: 8px;
      background: var(--bg-panel);
      color: var(--ink);
      font-family: inherit;
      font-size: 12px;
      padding: 6px 8px;
      cursor: pointer;
    }

    .kind-presets button.active {
      border-color: var(--accent);
      background: var(--accent-soft);
    }

    .count {
      font-size: 12px;
      color: var(--muted);
      margin-bottom: 10px;
    }

    .list {
      display: grid;
      gap: 6px;
    }

    .item {
      display: block;
      width: 100%;
      text-align: left;
      border: 1px solid var(--line);
      background: var(--bg-panel);
      border-radius: 8px;
      padding: 8px 10px;
      font-family: inherit;
      cursor: pointer;
    }

    .item:hover {
      border-color: var(--accent);
      background: var(--accent-soft);
    }

    .item.active {
      border-color: var(--accent);
      background: var(--accent-soft);
      box-shadow: inset 0 0 0 1px var(--accent);
    }

    .item .id {
      font-size: 14px;
      font-weight: 600;
      margin-bottom: 2px;
    }

    .item .meta {
      font-size: 12px;
      color: var(--muted);
    }

    .main {
      padding: 18px 24px;
      overflow: auto;
    }

    .toolbar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 10px;
      margin-bottom: 12px;
      border-bottom: 1px solid var(--line);
      padding-bottom: 10px;
    }

    .toolbar .label {
      font-size: 13px;
      color: var(--muted);
    }

    .doc {
      max-width: 900px;
      margin: 0 auto;
      background: var(--bg-panel);
      border: 1px solid var(--line);
      border-radius: 14px;
      padding: 20px 26px;
      box-shadow: 0 8px 24px rgba(66, 43, 12, 0.08);
      line-height: 1.55;
      white-space: normal;
    }

    .doc h1, .doc h2, .doc h3 {
      margin-top: 1.1em;
      margin-bottom: 0.4em;
      line-height: 1.25;
    }

    .doc h1 { font-size: 1.7rem; }
    .doc h2 { font-size: 1.3rem; border-bottom: 1px solid var(--line); padding-bottom: 4px; }
    .doc h3 { font-size: 1.05rem; }

    .doc p { margin: 0.5em 0; }
    .doc ul, .doc ol { margin: 0.4em 0 0.7em 1.3em; }
    .doc li { margin: 0.2em 0; }

    .doc code {
      font-family: \"JetBrains Mono\", \"Consolas\", monospace;
      background: #f2eee6;
      border: 1px solid #ded3c4;
      border-radius: 4px;
      padding: 0.05em 0.3em;
      font-size: 0.92em;
    }

    .doc pre {
      overflow: auto;
      padding: 10px;
      border-radius: 8px;
      border: 1px solid #ded3c4;
      background: #f2eee6;
    }

    .doc a {
      color: var(--accent);
      text-decoration: none;
    }

    .doc a:hover { text-decoration: underline; }

    @media (max-width: 980px) {
      .layout {
        grid-template-columns: 1fr;
      }

      .sidebar {
        border-right: 0;
        border-bottom: 1px solid var(--line);
        max-height: 40vh;
      }

      .main {
        padding: 12px;
      }

      .doc {
        padding: 16px;
      }
    }
  </style>
</head>
<body>
  <div class=\"layout\">
    <aside class=\"sidebar\">
      <h1 class=\"brand\">Sankara Translation Workbench</h1>
      <p class=\"sub\">Basic Markdown View (v0.1)</p>
      <div class=\"controls\">
        <input id=\"searchInput\" type=\"search\" placeholder=\"Search record id or content\" />
        <select id=\"viewProfile\">
          <option value=\"simple\" selected>View: Simple</option>
          <option value=\"core\">View: Core</option>
          <option value=\"source\">View: Source</option>
          <option value=\"translation\">View: Translation</option>
          <option value=\"curation\">View: Curation</option>
          <option value=\"markdown\">View: Full Markdown</option>
        </select>
        <select id=\"sectionFilter\">
          <option value=\"\">All sections</option>
        </select>
        <div class="kind-presets" id="kindPresets">
          <button type="button" data-preset="all">All</button>
          <button type="button" data-preset="verse">Verses</button>
          <button type="button" data-preset="verse-bhasya">Verse + Bhasya</button>
        </div>
      </div>
      <div id=\"count\" class=\"count\">Loading records...</div>
      <div id=\"recordList\" class=\"list\"></div>
    </aside>
    <main class=\"main\">
      <div class=\"toolbar\">
        <div class=\"label\" id=\"currentMeta\">No record selected</div>
      </div>
      <article id=\"doc\" class=\"doc\">
        <h2>Ready</h2>
        <p>Select a record from the left panel.</p>
      </article>
    </main>
  </div>

  <script>
    const state = {
      manifest: null,
      records: [],
      selectedRecordId: null,
      semantic: null,
    };

    const dom = {
      searchInput: document.getElementById('searchInput'),
      viewProfile: document.getElementById('viewProfile'),
      sectionFilter: document.getElementById('sectionFilter'),
      kindPresetButtons: Array.from(document.querySelectorAll('#kindPresets [data-preset]')),
      recordList: document.getElementById('recordList'),
      count: document.getElementById('count'),
      doc: document.getElementById('doc'),
      currentMeta: document.getElementById('currentMeta'),
    };

    const KIND_PRESETS = {
      all: [],
      verse: ['sutra'],
      'verse-bhasya': ['sutra', 'bhasya', 'preamble'],
    };

    const VIEW_PROFILES = {
      simple: [],
      core: ['kind', 'section'],
      source: ['kind', 'section', 'source'],
      translation: ['kind', 'literal', 'technical'],
      curation: ['kind', 'section', 'tags'],
      markdown: ['kind', 'section'],
    };

    function escapeHtml(text) {
      return text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\"/g, '&quot;')
        .replace(/'/g, '&#39;');
    }

    function markdownToHtml(markdown) {
      const lines = markdown.split(/\r?\n/);
      const out = [];
      let i = 0;
      let inCode = false;
      let listType = null;

      function closeList() {
        if (listType) {
          out.push(listType === 'ol' ? '</ol>' : '</ul>');
          listType = null;
        }
      }

      while (i < lines.length) {
        const raw = lines[i];
        const line = raw.trimEnd();

        if (line.startsWith('```')) {
          closeList();
          if (!inCode) {
            inCode = true;
            out.push('<pre><code>');
          } else {
            inCode = false;
            out.push('</code></pre>');
          }
          i += 1;
          continue;
        }

        if (inCode) {
          out.push(escapeHtml(raw) + '\n');
          i += 1;
          continue;
        }

        if (!line) {
          closeList();
          i += 1;
          continue;
        }

        const heading = line.match(/^(#{1,3})\s+(.*)$/);
        if (heading) {
          closeList();
          const level = heading[1].length;
          out.push(`<h${level}>${inlineMd(heading[2])}</h${level}>`);
          i += 1;
          continue;
        }

        const ordered = line.match(/^\d+\.\s+(.*)$/);
        if (ordered) {
          if (listType !== 'ol') {
            closeList();
            listType = 'ol';
            out.push('<ol>');
          }
          out.push(`<li>${inlineMd(ordered[1])}</li>`);
          i += 1;
          continue;
        }

        const unordered = line.match(/^[-*]\s+(.*)$/);
        if (unordered) {
          if (listType !== 'ul') {
            closeList();
            listType = 'ul';
            out.push('<ul>');
          }
          out.push(`<li>${inlineMd(unordered[1])}</li>`);
          i += 1;
          continue;
        }

        closeList();
        out.push(`<p>${inlineMd(line)}</p>`);
        i += 1;
      }

      closeList();
      if (inCode) {
        out.push('</code></pre>');
      }

      return out.join('');
    }

    function inlineMd(text) {
      let html = escapeHtml(text);
      html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
      html = html.replace(/\*([^*]+)\*/g, '<em>$1</em>');
      html = html.replace(/`([^`]+)`/g, '<code>$1</code>');
      html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>');
      return html;
    }

    function buildSectionOptions(sections) {
      for (const section of sections) {
        const option = document.createElement('option');
        option.value = section.section_code;
        option.textContent = `Section ${section.section_code} (${section.record_count})`;
        dom.sectionFilter.appendChild(option);
      }
    }

    function shortText(value, maxLen = 80) {
      const text = String(value || '').trim().replace(/\s+/g, ' ');
      if (!text) {
        return '(empty)';
      }
      return text.length > maxLen ? text.slice(0, maxLen - 1) + '…' : text;
    }

    function currentProfileKey() {
      const key = String(dom.viewProfile.value || 'simple').toLowerCase();
      return Object.prototype.hasOwnProperty.call(VIEW_PROFILES, key)
        ? key
        : 'simple';
    }

    function profileFieldText(record, field) {
      if (field === 'kind') {
        return String(record.record_kind || 'unknown');
      }
      if (field === 'section') {
        return `Section ${String(record.section_code || 'n/a')}`;
      }
      if (field === 'source') {
        return `Src: ${shortText(record.source_text, 70)}`;
      }
      if (field === 'literal') {
        return `Lit: ${shortText(record.literal_translation, 70)}`;
      }
      if (field === 'technical') {
        return `Tech: ${shortText(record.technical_translation, 70)}`;
      }
      if (field === 'tags') {
        const tags = Array.isArray(record.argument_tags)
          ? record.argument_tags.map((x) => String(x)).filter((x) => x.length > 0)
          : [];
        return tags.length > 0
          ? `Tags: ${tags.slice(0, 3).join(', ')}`
          : 'Tags: none';
      }
      return '';
    }

    function formatRecordMeta(record) {
      const fields = VIEW_PROFILES[currentProfileKey()] || VIEW_PROFILES.core;
      const values = fields
        .map((field) => profileFieldText(record, field))
        .filter((text) => text.length > 0);
      return values.join(' | ') || ' ';
    }

    function formatCurrentMeta(record) {
      const fields = VIEW_PROFILES[currentProfileKey()] || VIEW_PROFILES.core;
      const values = fields
        .map((field) => profileFieldText(record, field))
        .filter((text) => text.length > 0)
        .join(' | ');
      return `${record.record_id}${values ? ` | ${values}` : ''}`;
    }

    function renderSimpleStructured(record, profileKey) {
      const sourceText = shortText(record.source_text, 6000);
      const transliterationRaw = String(
        record.transliteration_iast || record.transliteration || '',
      ).trim();
      const transliteration = transliterationRaw || '(not available in record)';
      const literal = shortText(record.literal_translation, 6000);
      const technical = shortText(record.technical_translation, 6000);
      const tags = Array.isArray(record.argument_tags)
        ? record.argument_tags.map((x) => String(x)).filter((x) => x.length > 0)
        : [];

      const chunks = [];

      if (
        profileKey === 'simple' ||
        profileKey === 'source' ||
        profileKey === 'core'
      ) {
        chunks.push(
          '<h2>Source</h2>',
          `<h3>Source Text</h3><p>${escapeHtml(sourceText)}</p>`,
          `<h3>Transliteration</h3><p>${escapeHtml(transliteration)}</p>`,
        );
      }

      if (profileKey === 'simple' || profileKey === 'translation') {
        chunks.push(
          '<h2>Translation</h2>',
          `<h3>Literal</h3><p>${escapeHtml(literal)}</p>`,
          `<h3>Technical</h3><p>${escapeHtml(technical)}</p>`,
        );
      }

      if (profileKey === 'curation') {
        chunks.push(
          '<h2>Curation</h2>',
          `<p><strong>Kind:</strong> ${escapeHtml(String(record.record_kind || 'unknown'))}</p>`,
          `<p><strong>Tags:</strong> ${escapeHtml(tags.length ? tags.join(', ') : 'none')}</p>`,
        );
      }

      if (profileKey === 'core') {
        chunks.push(
          '<h2>Core</h2>',
          `<p><strong>Kind:</strong> ${escapeHtml(String(record.record_kind || 'unknown'))}</p>`,
          `<p><strong>Section:</strong> ${escapeHtml(String(record.section_code || 'n/a'))}</p>`,
        );
      }

      dom.doc.innerHTML = chunks.join('');
    }

    function parseListParam(params, key) {
      const raw = (params.get(key) || '').trim();
      if (!raw) {
        return [];
      }
      return raw
        .split(',')
        .map((x) => x.trim().toLowerCase())
        .filter((x) => x.length > 0);
    }

    function parseAnchorParam(raw) {
      const text = String(raw || '').trim();
      if (!text) {
        return null;
      }
      const parts = text
        .split('.')
        .map((x) => x.trim())
        .filter((x) => x.length > 0)
        .map((x) => Number.parseInt(x, 10));
      if (parts.some((x) => !Number.isFinite(x) || x <= 0)) {
        return null;
      }
      if (parts.length === 1) {
        return { sutra: parts[0] };
      }
      if (parts.length === 2) {
        return { chapter: parts[0], section: parts[1] };
      }
      return { chapter: parts[0], section: parts[1], sutra: parts[2] };
    }

    function parseRecordLocator(recordId) {
      const m = String(recordId || '').match(
        /^SKR\.([^.]+)\.(\d+)\.(\d+)\.([A-Z0-9]+)$/,
      );
      if (!m) {
        return null;
      }
      const chapter = Number.parseInt(m[2], 10);
      const section = Number.parseInt(m[3], 10);
      const unitRaw = m[4];
      const sutra = /^\d+$/.test(unitRaw)
        ? Number.parseInt(unitRaw, 10)
        : undefined;
      return {
        work: m[1],
        chapter,
        section,
        unitRaw,
        sutra,
      };
    }

    function parseSemanticFromLocation() {
      const params = new URLSearchParams(window.location.search);
      const kinds = parseListParam(params, 'kinds');
      const topics = parseListParam(params, 'topics');
      const transitions = parseListParam(params, 'tx');
      const anchor = parseAnchorParam(params.get('anchor'));
      const coLocatedRaw = (params.get('co_located') || '').toLowerCase();
      const coLocatedOnly = coLocatedRaw === '1' || coLocatedRaw === 'true';

      if (
        kinds.length === 0 &&
        topics.length === 0 &&
        transitions.length === 0 &&
        !anchor &&
        !coLocatedOnly
      ) {
        return null;
      }

      return {
        kinds,
        topics,
        transitions,
        anchor,
        coLocatedOnly,
      };
    }

    function recordTopicTags(record) {
      if (Array.isArray(record.topic_tags) && record.topic_tags.length > 0) {
        return record.topic_tags.map((x) => String(x).toLowerCase());
      }
      return Array.isArray(record.argument_tags)
        ? record.argument_tags.map((x) => String(x).toLowerCase())
        : [];
    }

    function ensureSemanticState() {
      if (!state.semantic) {
        state.semantic = {
          kinds: [],
          topics: [],
          transitions: [],
          anchor: null,
          coLocatedOnly: false,
        };
      }
      if (!Array.isArray(state.semantic.kinds)) {
        state.semantic.kinds = [];
      }
      return state.semantic;
    }

    function sameKinds(left, right) {
      if (left.length !== right.length) {
        return false;
      }
      const ls = [...left].sort().join('|');
      const rs = [...right].sort().join('|');
      return ls === rs;
    }

    function activeKindPreset() {
      const semantic = state.semantic;
      const activeKinds = semantic && Array.isArray(semantic.kinds)
        ? semantic.kinds.map((x) => String(x).toLowerCase())
        : [];

      if (activeKinds.length === 0) {
        return 'all';
      }
      if (sameKinds(activeKinds, KIND_PRESETS.verse)) {
        return 'verse';
      }
      if (sameKinds(activeKinds, KIND_PRESETS['verse-bhasya'])) {
        return 'verse-bhasya';
      }
      return 'custom';
    }

    function renderKindPresetButtons() {
      const active = activeKindPreset();
      for (const button of dom.kindPresetButtons) {
        button.classList.toggle('active', button.dataset.preset === active);
      }
    }

    function applyKindPreset(preset) {
      const semantic = ensureSemanticState();
      semantic.kinds = [...(KIND_PRESETS[preset] || [])];
      renderKindPresetButtons();
      renderRecordList();

      const records = filteredRecords();
      if (records.length === 0) {
        state.selectedRecordId = null;
        dom.currentMeta.textContent = 'No record selected';
        dom.doc.innerHTML = '<p>No records match the current filters.</p>';
        return;
      }

      if (!records.some((x) => x.record_id === state.selectedRecordId)) {
        state.selectedRecordId = records[0].record_id;
        renderRecordList();
        openRecord(records[0]).catch(() => {
          dom.doc.innerHTML = '<p>Failed to load markdown.</p>';
        });
      }
    }

    function matchesSemantic(record, semantic) {
      if (!semantic) {
        return true;
      }

      const locator = parseRecordLocator(record.record_id);

      if (semantic.kinds.length > 0) {
        const kind = String(record.record_kind || 'unknown').toLowerCase();
        if (!semantic.kinds.includes(kind)) {
          return false;
        }
      }

      if (semantic.topics.length > 0) {
        const tags = recordTopicTags(record);
        if (!semantic.topics.some((topic) => tags.includes(topic))) {
          return false;
        }
      }

      if (semantic.transitions.length > 0) {
        const hay = [
          record.source_text,
          record.literal_translation,
          record.technical_translation,
        ]
          .join(' ')
          .toLowerCase();
        if (!semantic.transitions.some((term) => hay.includes(term))) {
          return false;
        }
      }

      if (semantic.anchor && locator) {
        if (
          semantic.anchor.chapter != null &&
          locator.chapter !== semantic.anchor.chapter
        ) {
          return false;
        }
        if (
          semantic.anchor.section != null &&
          locator.section !== semantic.anchor.section
        ) {
          return false;
        }
        if (semantic.anchor.sutra != null) {
          if (!Number.isFinite(locator.sutra)) {
            return false;
          }
          if (semantic.coLocatedOnly) {
            if (locator.sutra !== semantic.anchor.sutra) {
              return false;
            }
          } else if (locator.sutra !== semantic.anchor.sutra) {
            return false;
          }
        }
      }

      return true;
    }

    function filteredRecords() {
      const query = dom.searchInput.value.trim().toLowerCase();
      const section = dom.sectionFilter.value;

      return state.records.filter((record) => {
        if (section && record.section_code !== section) {
          return false;
        }

        if (!matchesSemantic(record, state.semantic)) {
          return false;
        }

        if (!query) {
          return true;
        }

        const haystack = [
          record.record_id,
          record.record_kind,
          record.section_code,
          record.source_text,
          record.literal_translation,
          record.technical_translation,
          ...(record.argument_tags || []),
        ]
          .join(' ')
          .toLowerCase();

        return haystack.includes(query);
      });
    }

    function renderRecordList() {
      const records = filteredRecords();
      dom.recordList.innerHTML = '';
      dom.count.textContent = `${records.length} record(s)`;

      for (const record of records) {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'item' + (record.record_id === state.selectedRecordId ? ' active' : '');
        button.innerHTML = `<div class=\"id\">${escapeHtml(record.record_id)}</div><div class=\"meta\">${escapeHtml(formatRecordMeta(record))}</div>`;
        button.addEventListener('click', () => {
          state.selectedRecordId = record.record_id;
          renderRecordList();
          openRecord(record);
        });
        dom.recordList.appendChild(button);
      }
    }

    async function openRecord(record) {
      dom.currentMeta.textContent = formatCurrentMeta(record);
      const profileKey = currentProfileKey();
      if (profileKey !== 'markdown') {
        renderSimpleStructured(record, profileKey);
        return;
      }

      dom.doc.innerHTML = '<p>Loading markdown...</p>';
      const response = await fetch(record.md_path);
      if (!response.ok) {
        dom.doc.innerHTML = `<p>Failed to load ${escapeHtml(record.md_path)}</p>`;
        return;
      }
      const markdown = await response.text();
      dom.doc.innerHTML = markdownToHtml(markdown);
    }

    async function init() {
      const response = await fetch('./manifest.json');
      if (!response.ok) {
        dom.doc.innerHTML = '<h2>Manifest not found</h2><p>Run the generator script first.</p>';
        dom.count.textContent = '0 record(s)';
        return;
      }

      state.manifest = await response.json();
      state.records = state.manifest.records || [];
      state.semantic = parseSemanticFromLocation();

      buildSectionOptions(state.manifest.sections || []);
      renderKindPresetButtons();
      renderRecordList();

      const first = state.records[0];
      if (first) {
        state.selectedRecordId = first.record_id;
        renderRecordList();
        await openRecord(first);
      } else {
        dom.doc.innerHTML = '<h2>No records</h2><p>No projected markdown records found.</p>';
      }

      dom.searchInput.addEventListener('input', renderRecordList);
      dom.viewProfile.addEventListener('change', () => {
        renderRecordList();
        const selected = state.records.find((x) => x.record_id === state.selectedRecordId);
        if (selected) {
          openRecord(selected).catch(() => {
            dom.doc.innerHTML = '<p>Failed to load record.</p>';
          });
        }
      });
      dom.sectionFilter.addEventListener('change', renderRecordList);
      for (const button of dom.kindPresetButtons) {
        button.addEventListener('click', () => {
          applyKindPreset(button.dataset.preset || 'all');
        });
      }
    }

    init().catch((err) => {
      dom.doc.innerHTML = `<h2>Viewer error</h2><p>${escapeHtml(String(err))}</p>`;
      dom.count.textContent = '0 record(s)';
    });
  </script>
</body>
</html>
"""
    # The template is stored as a raw string to preserve JS regex literals.
    # Normalize escaped attribute quotes so ids/classes are valid in final HTML.
    viewer_path.write_text(viewer_html.replace('\\"', '"'), encoding="utf-8")


def main() -> int:
    args = parse_args()

    records_dir = args.records_dir.resolve()
    workbench_root = args.out_dir.resolve()
    md_root = workbench_root / "md"

    if not records_dir.exists():
        raise SystemExit(f"records directory not found: {records_dir}")

    candidates = discover_record_files(records_dir)
    docs: list[RecordDoc] = []

    for path in candidates:
        doc = read_record_doc(path, md_root)
        if doc is None:
            continue
        if args.work_code and doc.work_code != args.work_code:
            continue
        docs.append(doc)

    docs.sort(key=lambda d: parse_record_sort_key(d.record_id))

    if not docs:
        raise SystemExit("No translation records matched the current filters.")

    md_root.mkdir(parents=True, exist_ok=True)

    write_record_docs(docs)
    sections = write_section_indexes(md_root, docs)
    write_corpus_index(md_root, docs, sections)
    write_manifest(workbench_root, md_root, docs, sections)
    write_viewer_html(workbench_root)

    print(
        f"Generated workbench artifacts: records={len(docs)} sections={len(sections)} output={workbench_root.as_posix()}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
