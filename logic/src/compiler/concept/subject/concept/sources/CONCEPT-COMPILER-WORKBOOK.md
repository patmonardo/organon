# Concept Compiler Workbook

Part: `COMPILER. RELATION-TO-OPERATION TARGET MAP`
Status: active
Authority: derivative from Concept workbooks (non-authoritative)

## Purpose

- This workbook captures compiler-target mappings from validated claim/relation structures.
- It is downstream of Part A/B/C + IDEA workbooks.

## Inputs

- `CONCEPT-PART-A-WORKBOOK.md`
- `CONCEPT-PART-B-WORKBOOK.md`
- `CONCEPT-PART-C-WORKBOOK.md`
- `CONCEPT-IDEA-WORKBOOK.md`

## Mapping contract

- Only accepted, line-anchored claims can feed operation generation.
- Relations become operation directives only after review acceptance.
- Keep speculative protocol notes in `CONCEPT_NOTEBOOK.ipynb` until promoted.

## Working template

### OperationMap <id> — <title>

- sourceClaims: list of claim ids
- sourceRelations: list of relation ids
- protocolMode: `membership|consequentia|inherence|mixed`
- targetOperation: compiler logical operation name
- confidence: 0-1
- proofPath: ordered claim/relation chain
- status: `draft|review_pending|accepted`

## Session: 2026-02-20 (initial scaffold)

### OperationMap con-op-001 — Placeholder map

- sourceClaims:
  - `con-sub-a-001-c1`
- sourceRelations:
  - pending
- protocolMode: mixed
- targetOperation: pending
- confidence: 0.5
- proofPath:
  - `con-sub-a-001-c1`
- status: review_pending

Notes:

- Replace placeholder as soon as Part A/B/C first pass claims are anchored.
