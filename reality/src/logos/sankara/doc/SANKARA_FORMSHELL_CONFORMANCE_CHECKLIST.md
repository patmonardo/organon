# Sankara Form/Shell Conformance Checklist

## Purpose

Use this checklist during implementation and QA to verify Sankara route/state behavior matches the formal contract.

Reference contract:

- SANKARA_FORMSHELL_ROUTE_STATE_CONTRACT.md

## Route Parsing

- [ ] /sankara resolves with defaults (work=BS, mode=markdown, queue=normal, threshold=0.90).
- [ ] /sankara/BS resolves work correctly.
- [ ] /sankara/BS/1.1 resolves section correctly.
- [ ] /sankara/BS/1.1/SKR.BS.1.1.001 resolves record correctly.
- [ ] Invalid section format is dropped during normalization.
- [ ] Invalid record id falls back to first valid record in effective list.

## Query Normalization

- [ ] Unknown mode normalizes to markdown.
- [ ] Unknown queue normalizes to normal.
- [ ] threshold parse failure normalizes to 0.90.
- [ ] threshold below 0 normalizes to 0.00.
- [ ] threshold above 1 normalizes to 1.00.
- [ ] threshold is rendered to two decimals in canonical URL.
- [ ] Empty q is removed from canonical URL.
- [ ] Empty section filter is removed from canonical URL.

## Queue Semantics

- [ ] queue=normal applies no queue constraints.
- [ ] queue=review keeps only records with uncertainty notes.
- [ ] queue=strict keeps only records with uncertainty notes and confidence <= threshold.
- [ ] Strict queue count changes as threshold changes.

## Preset Behavior

- [ ] Clicking 0.95 sets threshold to 0.95 and queue to strict.
- [ ] Clicking 0.90 sets threshold to 0.90 and queue to strict.
- [ ] Clicking 0.85 sets threshold to 0.85 and queue to strict.
- [ ] Clicking 0.80 sets threshold to 0.80 and queue to strict.
- [ ] Active preset styling reflects current threshold exactly.
- [ ] Manual threshold edits clear active preset when value is non-preset.

## State Restoration

- [ ] URL values override persisted preferences.
- [ ] Missing URL values restore from preferences.
- [ ] Missing preferences fall back to defaults.
- [ ] Page reload restores same visible state for identical URL + manifest.

## Selection Resolution

- [ ] URL record remains selected when valid under current filters.
- [ ] If URL record is invalid under filters, fallback selects first valid record.
- [ ] Empty filtered list shows non-blocking empty-state panel.

## Command Hooks

- [ ] sankara.regen command status transitions: idle -> running -> success/error.
- [ ] sankara.qa command status transitions: idle -> running -> success/error.
- [ ] sankara.report command status transitions: idle -> running -> success/error.

## Determinism and Safety

- [ ] App state after route normalization is deterministic.
- [ ] Canonical URL emitted after normalization is stable.
- [ ] No generated markdown artifacts are hand-edited in workflow.
- [ ] Queue logic remains inspectable in code and test logs.
