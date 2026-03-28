# Object Compiler Workbook

Part: `COMPILER. RELATION-TO-OPERATION TARGET MAP`
Status: active
Authority: derivative from Object workbooks (non-authoritative)

## Purpose

- This workbook captures compiler-target mappings from validated claim/relation structures.
- It is downstream of Part A/B/C + IDEA workbooks.

## Inputs

- `MECHANISM-PART-A-WORKBOOK.md`
- `MECHANISM-PART-B-WORKBOOK.md`
- `MECHANISM-PART-C-WORKBOOK.md`
- `MECHANISM-IDEA-WORKBOOK.md`

## Mapping contract

- Only accepted, line-anchored claims can feed operation generation.
- Relations become operation directives only after review acceptance.
- Keep speculative protocol notes in `MECHANISM-NOTEBOOK.ipynb` until promoted.

## Block ID convention

- Entry IDs: `obj-mech-<part>-<section>-<nnn>`
- Claim IDs: `<entry-id>-c1|c2|c3`
- Part/section examples:
  - Part A section blocks: `obj-mech-a-001`
  - Part B section blocks: `obj-mech-b-a-001`, `obj-mech-b-b-001`, `obj-mech-b-c-001`, `obj-mech-b-tr-001`
  - Part C section blocks: `obj-mech-c-a-001`, `obj-mech-c-b-001`, `obj-mech-c-c-001`
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

## Session: 2026-02-22 (block harmonization scaffold)

### OperationMap obj-op-001 — Part A baseline map

- sourceClaims:
  - `obj-mech-a-001-c1`
- sourceRelations:
  - `obj-mech-a-001:r1`
- protocolMode: mixed
- targetOperation: pending
- confidence: 0.55
- proofPath:
  - `obj-mech-a-001-c1`
- status: review_pending

### OperationMap obj-op-002 — Part B process-chain map

- sourceClaims:
  - `obj-mech-b-a-001-c1`
  - `obj-mech-b-b-001-c1`
- sourceRelations:
  - `obj-mech-b-a-003:r1`
  - `obj-mech-b-b-001:r1`
- protocolMode: mixed
- targetOperation: pending
- confidence: 0.5
- proofPath:
  - `obj-mech-b-a-001-c1`
  - `obj-mech-b-a-003:r1`
  - `obj-mech-b-b-001-c1`
- status: review_pending

### OperationMap obj-op-003 — Part C law-transition map

- sourceClaims:
  - `obj-mech-c-b-003-c2`
  - `obj-mech-c-c-003-c2`
- sourceRelations:
  - `obj-mech-c-b-003:r1`
  - `obj-mech-c-c-003:r1`
- protocolMode: consequentia
- targetOperation: pending
- confidence: 0.58
- proofPath:
  - `obj-mech-c-b-003-c2`
  - `obj-mech-c-b-003:r1`
  - `obj-mech-c-c-003-c2`
- status: review_pending

Notes:

- Replace placeholders after cross-part relation review is accepted.

## Non-negotiable boundary

- No operation mapping may be accepted without line-anchored claim evidence in upstream part workbooks.
- No notebook-only hypothesis may be promoted directly to accepted operation maps.

## Rejection taxonomy

- reject_unanchored_claim
- reject_missing_relation_path
- reject_protocol_mode_mismatch
- reject_cross_part_reference_unresolved
