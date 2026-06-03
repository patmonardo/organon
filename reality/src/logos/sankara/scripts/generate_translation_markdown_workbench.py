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
DEFAULT_WORKBENCH_DIR = SANKARA_DIR / "translation" / "workbench"


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
        help="Workbench output root (default: translation/workbench).",
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


def read_record_doc(path: Path, md_root: Path) -> RecordDoc | None:
    data = json.loads(path.read_text(encoding="utf-8"))

    record_id = str(data.get("record_id", "")).strip()
    if not record_id:
        return None

    work_code = str(data.get("work_code", "")).strip()
    section_code = str(data.get("section_code", "")).strip()

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

    lexical_notes_raw = analysis.get("lexical_notes")
    lexical_notes: list[dict[str, Any]] = []
    if isinstance(lexical_notes_raw, list):
        for note in lexical_notes_raw:
            if isinstance(note, dict):
                lexical_notes.append(note)

    return RecordDoc(
        record_id=record_id,
        work_code=work_code,
        section_code=section_code,
        source_text=str(text.get("source_text", "")).strip(),
        transliteration_iast=str(text.get("transliteration_iast", "")).strip(),
        literal_translation=str(translation.get("literal_translation", "")).strip(),
        technical_translation=str(translation.get("technical_translation", "")).strip(),
        interpretive_note=str(translation.get("interpretive_note", "")).strip(),
        argument_tags=as_list_of_str(analysis.get("argument_tags")),
        lexical_notes=lexical_notes,
        source_url=str(source.get("source_url", "")).strip(),
        language=str(source.get("language", "")).strip(),
        script=str(source.get("script", "")).strip(),
        edition_note=str(source.get("edition_note", "")).strip(),
        qa_status=str(qa.get("status", "")).strip(),
        qa_confidence=qa_confidence,
        qa_uncertainty_notes=as_list_of_str(qa.get("uncertainty_notes")),
        translator_version=str(provenance.get("translator_version", "")).strip(),
        reviewer_version=str(provenance.get("reviewer_version", "")).strip(),
        extraction_timestamp_utc=str(provenance.get("extraction_timestamp_utc", "")).strip(),
        source_manifest_path=str(provenance.get("source_manifest_path", "")).strip(),
        input_path=path,
        output_path=out_path,
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
                "literal_translation": doc.literal_translation,
                "technical_translation": doc.technical_translation,
                "md_path": doc.output_path.relative_to(workbench_root).as_posix(),
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
        <select id=\"sectionFilter\">
          <option value=\"\">All sections</option>
        </select>
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
    };

    const dom = {
      searchInput: document.getElementById('searchInput'),
      sectionFilter: document.getElementById('sectionFilter'),
      recordList: document.getElementById('recordList'),
      count: document.getElementById('count'),
      doc: document.getElementById('doc'),
      currentMeta: document.getElementById('currentMeta'),
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

    function filteredRecords() {
      const query = dom.searchInput.value.trim().toLowerCase();
      const section = dom.sectionFilter.value;

      return state.records.filter((record) => {
        if (section && record.section_code !== section) {
          return false;
        }

        if (!query) {
          return true;
        }

        const haystack = [
          record.record_id,
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
        button.innerHTML = `<div class=\"id\">${escapeHtml(record.record_id)}</div><div class=\"meta\">Section ${escapeHtml(record.section_code)}</div>`;
        button.addEventListener('click', () => {
          state.selectedRecordId = record.record_id;
          renderRecordList();
          openRecord(record);
        });
        dom.recordList.appendChild(button);
      }
    }

    async function openRecord(record) {
      dom.currentMeta.textContent = `${record.record_id} | Section ${record.section_code}`;
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

      buildSectionOptions(state.manifest.sections || []);
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
      dom.sectionFilter.addEventListener('change', renderRecordList);
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
