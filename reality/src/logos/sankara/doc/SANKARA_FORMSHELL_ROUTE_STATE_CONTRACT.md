# Sankara Form/Shell Route + State Contract (v0.1)

## Purpose

This document is the implementation contract for route and state behavior in the Sankara prototype app.

It ensures deterministic, deep-linkable, and restorable user flow for curation sessions.

Reference implementation artifact:

- ../workbench/prototype_app/route_state_contract.ts

QA artifact:

- SANKARA_FORMSHELL_CONFORMANCE_CHECKLIST.md

## Scope

- App namespace: Sankara
- Phase target: BS-first prototype
- Route concerns:
  - work/section/record addressing
  - mode and queue controls
  - confidence threshold
  - search and section filters
- State concerns:
  - URL state
  - persistent user preferences
  - ephemeral session state
  - command hook state

## Canonical Route Model

### Path

Canonical path:

- /sankara/:work/:section/:record

Allowed reduced paths:

- /sankara/:work/:section
- /sankara/:work
- /sankara

### Path Segment Contracts

- :work
  - string enum in v0.1: BS
  - reserved for future: GITA, UPAN
- :section
  - format: N.N (example: 1.1)
- :record
  - format: SKR.<WORK>.<chapter>.<section>.<unit>
  - examples:
    - SKR.BS.1.1.001
    - SKR.BS.1.1.PRE

If a segment is invalid, Shell must normalize to nearest valid state and emit normalized URL.

## Query Model

### Canonical Query Keys

- mode
  - enum: markdown | source | literal | technical
  - default: markdown
- queue
  - enum: normal | review | strict
  - default: normal
- threshold
  - decimal in [0.00, 1.00]
  - precision: two decimals
  - default: 0.90
- q
  - free text search string
  - default: empty
- section
  - optional filter section code (N.N)
  - default: empty (all)

### Canonical Query Example

- /sankara/BS/1.1/SKR.BS.1.1.028?mode=literal&queue=strict&threshold=0.85&q=adhyasa&section=1.1

### Query Normalization Rules

- Unknown mode -> markdown
- Unknown queue -> normal
- threshold parse failure -> 0.90
- threshold < 0 -> 0.00
- threshold > 1 -> 1.00
- threshold render always with two decimals in URL
- empty q removed from URL
- empty section filter removed from URL

## State Partition

State is split into URL state, persisted preference state, and ephemeral session state.

### URL State (authoritative for link/share)

- work
- active section
- active record
- mode
- queue
- threshold
- q (search)
- section filter

### Persisted Preference State (Shell)

Persistence key:

- sankara.pref.v1

Persisted fields:

- lastMode
- lastQueue
- lastThreshold
- lastSectionFilter

Restore rules:

1. URL always wins when provided.
2. Missing URL fields are hydrated from preferences.
3. Missing preference fields fall back to defaults.

### Ephemeral Session State (non-persisted)

- loaded manifest metadata
- resolved filtered record list
- selected index in current list
- command running flags
- transient error banners

## Initialization Sequence

1. Load manifest from workbench/manifest.json.
2. Parse route path + query into RouteState.
3. Validate and normalize RouteState against manifest data.
4. Merge missing fields from persisted preferences.
5. Apply resolved state to Form controls.
6. Resolve selected record.
7. Emit canonical normalized URL if input URL differed.

## Selection Resolution Rules

Given current filters:

1. If record in URL exists and passes filters, select it.
2. Else if previous selected record exists and passes filters, keep it.
3. Else select first record in filtered list.
4. If filtered list is empty, clear selection and show empty-state panel.

## Queue Semantics

### normal

No queue constraints.

### review

Record must satisfy:

- qa_uncertainty_notes length > 0

### strict

Record must satisfy both:

- qa_uncertainty_notes length > 0
- qa_confidence <= threshold

## Threshold Preset Contract

Preset chips:

- 0.95
- 0.90
- 0.85
- 0.80

On preset click:

1. Set threshold input to selected value.
2. Set queue mode to strict.
3. Recompute filtered list.
4. Update selected record by selection resolution rules.
5. Update URL query with queue=strict and threshold=<value>.
6. Mark active preset chip.

If user manually edits threshold to non-preset value, no preset chip remains active.

## Command Hook Contract (Shell)

All commands are Shell-managed and Form-triggered.

### Command IDs

- sankara.regen
  - runs markdown projection generator
- sankara.qa
  - runs translation QA command
- sankara.report
  - runs progress report command

### Command Status State

- idle | running | success | error

### Command Result Envelope

- id
- status
- startedAt
- finishedAt
- summary
- logRef (optional)

## Type Contract (Reference)

```ts
export type SankaraMode = 'markdown' | 'source' | 'literal' | 'technical';
export type SankaraQueue = 'normal' | 'review' | 'strict';

export interface SankaraRouteState {
  work: string;             // BS in v0.1
  section?: string;         // 1.1
  record?: string;          // SKR.BS.1.1.001
  mode: SankaraMode;
  queue: SankaraQueue;
  threshold: number;        // 0.00 - 1.00
  q?: string;
  sectionFilter?: string;
}

export interface SankaraPreferencesV1 {
  lastMode: SankaraMode;
  lastQueue: SankaraQueue;
  lastThreshold: number;
  lastSectionFilter?: string;
}
```

## Error + Recovery Contract

- Manifest unavailable:
  - show blocking error panel with retry.
- Invalid route values:
  - auto-normalize and replace URL.
- Missing record id:
  - fallback to first valid record by current filters.
- No matching records under filters:
  - show non-blocking empty-state panel and keep controls editable.

## Telemetry (Optional, Recommended)

- route.normalized
- queue.mode.changed
- threshold.preset.clicked
- threshold.manual.changed
- command.started / command.finished

No source-text content should be logged in telemetry payloads.

## Compatibility Notes

- Contract is backward compatible with current viewer behavior.
- Future work codes (GITA, UPAN) should not require route schema changes.
- Query keys are stable and should be treated as API for deep links.

## Acceptance Criteria

1. Any valid deep link reconstructs the same visible view state.
2. Invalid links normalize deterministically.
3. Preset clicks always force strict mode and deterministic threshold state.
4. Refresh/reopen restores state from URL + preferences correctly.
5. Queue counts remain stable across reload for identical manifest data.
