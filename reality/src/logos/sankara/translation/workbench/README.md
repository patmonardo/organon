# Sankara Translation Workbench

This folder contains deterministic Markdown projections generated from translation record JSON files.

## Pipeline

1. Source JSON records under `translation/passage_records/`
2. Generated Markdown under `translation/workbench/md/`
3. Browser manifest and viewer under `translation/workbench/`

## Generate Artifacts

From `reality/src/logos/sankara/`:

```bash
python scripts/generate_translation_markdown_workbench.py
```

Optional work filter:

```bash
python scripts/generate_translation_markdown_workbench.py --work-code BS
```

## Open Basic MD Browser View

Use a local static server from `translation/workbench/`:

```bash
python -m http.server 8765
```

Then open:

- `http://localhost:8765/viewer.html`

The viewer supports:

- Record navigation
- Search (record id + text fields)
- Section filter
- Basic Markdown rendering

## Determinism Rule

Markdown files are projections. Do not hand-edit generated files under `md/`; regenerate from source JSON records.
