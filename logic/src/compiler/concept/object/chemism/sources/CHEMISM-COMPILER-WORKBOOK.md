# Chemism Compiler Workbook

Part: `COMPILER. RELATION-TO-OPERATION TARGET MAP`
Status: active
Authority: derivative from Chemism workbooks (non-authoritative)

## Purpose

- This workbook captures compiler-target mappings from validated claim/relation structures.
- It is downstream of Part A/B/C + IDEA workbooks.

## Inputs

- `CHEMISM-PART-A-WORKBOOK.md`
- `CHEMISM-PART-B-WORKBOOK.md`
- `CHEMISM-PART-C-WORKBOOK.md`
- `CHEMISM-IDEA-WORKBOOK.md`

## Mapping contract

- Only accepted, line-anchored claims can feed operation generation.
- Relations become operation directives only after review acceptance.
- Keep speculative protocol notes in `CHEMISM-NOTEBOOK.ipynb` until promoted.

## Block ID convention

- Entry IDs: `obj-chem-<part>-<section>-<nnn>`
- Claim IDs: `<entry-id>-c1|c2|c3`
- Compiler proof paths must preserve these IDs unchanged.

## Working template

### OperationMap <id> — <title>

- sourceClaims: list of claim ids
- sourceRelations: list of relation ids
- protocolMode: `membership|consequentia|inherence|mixed`
- targetOperation: compiler logical operation name
- confidence: 0-1
- proofPath: ordered claim/relation chain
- status: `draft|review_pending|accepted`

## Session: 2026-02-22 (initial scaffold)

### OperationMap obj-chem-op-001 — Placeholder map

- sourceClaims:
  - `obj-chem-a-001-c1`
- sourceRelations:
  - pending
- protocolMode: mixed
- targetOperation: pending
- confidence: 0.5
- proofPath:
  - `obj-chem-a-001-c1`
- status: review_pending

Notes:

- Replace placeholder after Part A/B/C claims are anchored and reviewed.

## Non-negotiable boundary

- No operation mapping may be accepted without line-anchored claim evidence in upstream part workbooks.
- No notebook-only hypothesis may be promoted directly to accepted operation maps.

## Rejection taxonomy

- reject_unanchored_claim
- reject_missing_relation_path
- reject_protocol_mode_mismatch
- reject_cross_part_reference_unresolved
