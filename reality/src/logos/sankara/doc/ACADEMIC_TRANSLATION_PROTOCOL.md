# Sankara Corpus Academic Translation Protocol

## Mission
Produce a fully auditable, source-faithful, technically rigorous English translation layer for the Sankara corpus, aligned to passage-level textual evidence and reusable for semantic graph construction.

## Core Principles
- Source-first: raw source text is primary witness, translation is derivative.
- Passage granularity: every claim must be attached to a concrete passage unit.
- Philological transparency: lexical, syntactic, and interpretive choices are explicit.
- Separation of layers: literal rendering, technical rendering, and interpretive commentary must remain distinct fields.
- Reproducibility: each artifact must include provenance and version metadata.
- Compatibility-preserving ingestion: schema shape may vary across curation passes, but validated Sutra/Bhasya witness text must be preserved.

## Output Unit
Each passage is represented by one JSON record validated against:
- schemas/passage_record.schema.json

The canonical output folder is:
- translation/passage_records/

## Required Workflow
1. Ingest page from raw corpus using URL + checksum from manifest.
2. Segment source into stable passage unit with deterministic ID.
3. Record source text (Devanagari and transliteration when available).
4. Produce literal translation.
5. Produce technical translation (terminology-preserving).
6. Add lexical notes for doctrinally dense terms.
7. Add argument structure tags (thesis, objection, response, resolution as applicable).
8. Add confidence and uncertainty notes.
9. Run QA validator script.
10. Commit only validated records.

## Passage ID Standard
Use this format:
- SKR.<work_code>.<section_code>.<passage_index>

Examples:
- SKR.BS.1.1.001
- SKR.BG.2.020

Constraints:
- passage_index is zero-padded to 3 digits minimum.
- IDs are immutable once published.

## Translation Layers
For each passage provide:
- literal_translation: close syntactic mapping with minimal smoothing.
- technical_translation: readable English preserving doctrinal precision.
- interpretive_note: optional and clearly marked as interpretation.

Never blend interpretation into literal or technical fields.

## Lexical Policy
For key terms (for example: adhyasa, avidya, atman, brahman), include:
- headword (IAST)
- grammatical role where identifiable
- short gloss
- contextual sense in this passage
- alternate plausible glosses

Incremental lexical updates are allowed between full translation passes when all of the following hold:
- Source witness fields remain unchanged for the passage.
- Sutra/Bhasya classification remains stable.
- The update only appends or refines lexical notes and provenance/reviewer metadata.

## Citation and Provenance
Every passage record must include:
- source_url
- source_sha256
- extraction_timestamp_utc
- extractor_version
- translator_version
- reviewer_version

If source page changes and checksum differs, create a new record version and preserve prior one.

## Quality Gates
A record is valid only if:
- Required fields are present.
- Literal and technical translations are non-empty and distinct.
- At least one lexical note exists when technical Sanskrit terms appear.
- Argument structure tags are present when dialectical markers are detected.
- Record validates against JSON schema.

## Review Tiers
- Tier 1: Translation self-check.
- Tier 2: Internal doctrinal consistency review.
- Tier 3: Comparative review against classical editions where available.

## Publication Artifact
A publishable package for any work section must include:
- passage_records/*.json
- section_index.json
- unresolved_questions.md
- translation_decisions.md

## Non-Goals
- No paraphrase-only outputs.
- No ungrounded synthetic commentary.
- No deletion of provenance metadata for convenience.

## Long-Term Alignment
This protocol is designed so Sringeri or other traditional institutions can audit each translation claim back to source witness and evaluate the entire pipeline for textual fidelity.
