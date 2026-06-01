# Corpus Integration Note: Sankara -> Top-Level Logos

Status: active draft
Date: 2026-06-01

## Why this matters

The Sankara ingestion pipeline now proves the corpus strategy across mixed source formats.

- Source witness formats: HTML and linked corpus pages
- Durable authoring format: JSON passage records
- Downstream philosophical synthesis: TypeScript DatasetUnit plus IDEA workbooks

This is the same architectural pattern needed for Yoga corpus scaling.

## Current working shape

1. Source acquisition
- Recursive crawler captures source witness pages with strict host/path/query controls.
- Provenance is written to manifest JSONL with URL and checksum.

2. Passage normalization
- Generator extracts stable source passage ids and emits sectioned JSON records.
- Each record carries source_text, provenance, QA status, and analysis placeholders.

3. QA gate
- Record validator checks structure and semantic sanity (id pattern, checksum shape, lexical tags, confidence bounds).

4. Publishing bundle
- Section package includes section_index.json plus editorial notes:
  - unresolved_questions.md
  - translation_decisions.md

## Namespace placement

Sankara belongs in top-level Logos as Advaita Vedanta, not in Yoga-specific namespaces.

- Canon/data target: reality/src/logos/sankara (typed units)
- Yoga-specific targets remain separate:
  - reality/src/logos/ys
  - reality/src/logos/yoga-sutras

## Field mapping draft (first pass)

Sankara passage record -> top-level Sankara Logos targets

- record_id -> unit id seed / source locator
- text.source_text -> SutraText.devanagari or source excerpt section
- text.transliteration_iast -> SutraText.iast
- analysis.lexical_notes -> tokens / philology notes section
- analysis.argument_tags -> Operator tags / conceptual alignment headers
- provenance.source_url + source_sha256 -> SourceRef entries
- translation.literal_translation -> literal layer in Sanskrit workbook section I
- translation.technical_translation -> technical layer in Sanskrit workbook section II/III handoff

## Recommended sequencing

1. Keep JSON as the system-of-record translation substrate.
2. Generate typed projections only after QA passes.
3. Keep IDEA workbook prose authoritative for final philosophical voice.
4. Treat TypeScript units as canonical machine-facing summaries, not replacements for the essays.

## Immediate next build steps

1. Keep Sankara projection scripts targeting reality/src/logos/sankara.
2. Add a markdown projection script from passage_records to Sankara-specific commentary templates.
3. Add round-trip source locator checks so every projected paragraph cites passage record ids.

## Design principle

Do not force one corpus format everywhere.

Use:
- HTML/XML for source witness and archival reality,
- JSON for normalized philological pipeline state,
- TypeScript and Markdown for philosophical and programmatic publication surfaces.
