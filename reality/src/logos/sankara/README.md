# Sankara Corpus Development Workspace

This folder is the staging area for rebuilding Sankara corpus ingestion from clean source pages.

Goal:
- Re-download source HTML deterministically.
- Preserve provenance for every fetch.
- Extract corpus records for downstream NLP and research workflows.
- Keep active script surface minimal and stable.

## Layout

- `sources/seed_urls.txt`: initial URL seeds.
- `scripts/download_corpus.py`: fetches HTML into `raw/pages/` and writes fetch manifest rows.
- `scripts/extract_gita_records.py`: extracts and merges Gita records from HTML witnesses.
- `scripts/extract_bs_records.py`: extracts and merges BS records from HTML witnesses.
- `scripts/extract_upan_records.py`: extracts and merges Upanishad records from HTML witnesses.
- `scripts/legacy/`: optional scripts kept for non-core workflows.
- `doc/ACADEMIC_TRANSLATION_PROTOCOL.md`: passage-level translation and review standard.
- `schemas/passage_record.schema.json`: canonical record schema for translation units.
- `translation/passage_record.template.json`: template for authoring new passage records.
- `raw/manifest.jsonl`: per-fetch provenance ledger.
- `derived/Gita/`: Gita aggregate and chapter outputs.
- `derived/BS/`: BS aggregate, chapter, and section outputs.
- `derived/Upan/`: Upanishad full-corpus aggregate.
- `derived/{TextName}/`: per-text aggregate, chapter, and section outputs (Aitareya, Brha, Chandogya, Isha, Kathaka, Kena_pada, Kena_vakya, Mandukya, Mundaka, Prashna, Taitiriya).

## Usage

1. Add URLs to `sources/seed_urls.txt`.

2. Download source pages:

```bash
python3 scripts/download_corpus.py \
  --url-file sources/seed_urls.txt
```

`raw/` is cumulative: repeated runs append newly fetched URLs and keep
previously fetched pages unless you explicitly delete the output folder.

3. For recursive corpus crawl from seeds:

```bash
python3 scripts/download_corpus.py \
  --url-file sources/seed_urls.txt \
  --recursive \
  --max-pages 6000 \
  --max-depth 8 \
  --ignore-manifest \
  --allow-host advaitasharada.sringeri.net \
  --allow-query-key id \
  --allow-query-key page \
  --allow-query-key vyakhya \
  --max-links-per-page 150 \
  --max-queue-size 25000 \
  --sleep-ms 400
```

For full Bhagavad Gita coverage, prefer:
`https://advaitasharada.sringeri.net/listing/moola/Gita/devanagari`

For Brahma Sutra, use:
`https://advaitasharada.sringeri.net/display/bhashya/BS/devanagari`

4. Extract Gita records:

```bash
python3 scripts/extract_gita_records.py \
  --out-file derived/Gita/BG.json \
  --emit all
```

This emits:
- `derived/Gita/BG.{json,jsonl,xml}`
- `derived/Gita/BG_C##.{json,jsonl,xml}`

5. Extract Brahma Sutra records:

```bash
python3 scripts/extract_bs_records.py \
  --out-file derived/BS/BS.json \
  --emit all
```

This emits:
- `derived/BS/BS.{json,jsonl,xml}`
- `derived/BS/BS_C##.{json,jsonl,xml}`
- `derived/BS/BS_C##_S##.{json,jsonl,xml}`

6. Extract Upanishad records (all texts in one pass):

```bash
python3 scripts/extract_upan_records.py \
  --out-file derived/Upan/Upan.json \
  --emit all
```

This emits:
- `derived/Upan/Upan.{json,jsonl,xml}`                               — full corpus aggregate
- `derived/{TextName}/{text_code}.{json,jsonl,xml}`                  — per-text aggregate
- `derived/{TextName}/{text_code}_C##.{json,jsonl,xml}`              — per-chapter
- `derived/{TextName}/{text_code}_C##_S##.{json,jsonl,xml}`          — per-section (where sections exist)

## Notes

- This workspace is for development and research extraction.
- HTML is treated as source witness; semantic artifacts become the durable substrate.
- Use polite crawl settings (`--sleep-ms`) and host/path scoping for respectful source access.
- Optional scripts are kept under `scripts/legacy/` to keep the active toolchain sparse.
