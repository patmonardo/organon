# Sankara App Prototype Spec (Form/Shell Platform)

## Purpose

Sankara is no longer only a translation utility; it is a first-class prototype application in the Form/Shell platform.

This spec defines the implementation surface required to move from a local workbench to a platform-native app.

Companion implementation contract:

- `SANKARA_FORMSHELL_ROUTE_STATE_CONTRACT.md`

## Product Position

- App name: Sankara
- Role: Canonical textual-intelligence app for Advaita corpus curation and interpretation flow
- Status: Prototype app, implementation-ready
- Primary corpus (phase 1): Brahma Sutra (BS)
- Immediate expansion (phase 2): Bhagavad Gita, then Upanishads

## Core User Loop

1. Select section or queue
2. Read source and translations in focused views
3. Filter to strict review queue (uncertainty + confidence threshold)
4. Open record and make curation decisions
5. Run QA and regenerate deterministic projections

## Platform Fit (Form/Shell)

Sankara should be implemented as a Form/Shell app module with the following boundaries:

- Form layer responsibilities:
  - Record list and controls
  - View mode state (markdown/source/literal/technical)
  - Queue mode state (normal/review/strict)
  - Threshold and section filters
- Shell layer responsibilities:
  - Dataset loading from generated manifest
  - Route state and deep links (section + record + mode)
  - Command hooks (regen, QA, report)
  - Persistent user/session preferences

## Data Contracts

### Input Contract

Source of truth remains JSON records under:

- `translation/passage_records/**.json`

### Projection Contract

Deterministic generator emits:

- `translation/workbench/manifest.json`
- `translation/workbench/md/index.md`
- `translation/workbench/md/sections/*.md`
- `translation/workbench/md/records/**/*.md`

### Manifest Record Fields Required by App

- `record_id`
- `work_code`
- `section_code`
- `qa_status`
- `qa_confidence`
- `qa_uncertainty_notes`
- `argument_tags`
- `source_text`
- `literal_translation`
- `technical_translation`
- `md_path`

## Feature Set

### v0.2 (already proven in workbench)

- Search by id/text
- Section filter
- Review queue filter (uncertainty)
- Strict queue filter (uncertainty + max confidence)
- One-click threshold presets (0.95 / 0.90 / 0.85 / 0.80)
- View modes (markdown/source/literal/technical)

### v0.3 (Form/Shell integration target)

- Deep-linkable route model:
  - `/sankara/:work/:section/:record?mode=literal&queue=strict&threshold=0.85`
- Preference persistence:
  - selected mode
  - queue mode
  - confidence threshold
- Keyboard navigation:
  - next/previous record
  - jump to record id
- Bulk review operations:
  - open next strict item
  - pin record

### v0.4 (cross-text intelligence)

- BS to Gita continuity links
- Thematic bridges and argument-tag trajectories
- Session-level review packets

## Implementation Plan (Immediate)

1. Create Sankara app module in Form/Shell platform structure.
2. Move current viewer logic into platform components without changing behavior.
3. Keep generator as the deterministic compiler boundary.
4. Add route + preference state management in Shell.
5. Add command buttons for regen/QA/report entry points.

## Non-Negotiables

- Determinism: all UI projections must be reproducible from record artifacts.
- No hand-editing generated markdown artifacts.
- Queue logic must remain explicit and inspectable.
- BS remains prototype proving ground before Gita expansion.

## Acceptance Criteria (Prototype App)

- App loads manifest and records with no local manual tweaks.
- Strict queue and threshold presets behave identically to current workbench.
- Record deep-link opens with correct mode and filters.
- Regeneration updates app views without schema drift.
- Prototype can be used for daily morning curation sessions.

## Next Decision Gate

After 3-4 days of live use:

- Keep Sankara as standalone app in platform as-is, or
- Generalize as reusable shell for future Yoga app while preserving Sankara-specific semantics.
