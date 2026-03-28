# Life Compiler Workbook

Part: `COMPILER. RELATION-TO-OPERATION TARGET MAP`
Status: active
Authority: derivative from Life workbooks (non-authoritative)

## Purpose

- This workbook captures compiler-target mappings from validated claim/relation structures.
- It is downstream of Part A/B/C + IDEA workbooks.

## Inputs

- `LIFE-PART-A-WORKBOOK.md`
- `LIFE-PART-B-WORKBOOK.md`
- `LIFE-PART-C-WORKBOOK.md`
- `LIFE-IDEA-WORKBOOK.md`

## Mapping contract

- Only accepted, line-anchored claims can feed operation generation.
- Relations become operation directives only after review acceptance.
- Keep speculative protocol notes in notebooks until promoted.

## Working template

### OperationMap <id> — <title>

- sourceClaims: list of claim ids
- sourceRelations: list of relation ids
- protocolMode: `membership|consequentia|inherence|mixed`
- targetOperation: compiler logical operation name
- confidence: 0-1
- proofPath: ordered claim/relation chain
- status: `draft|review_pending|accepted`

## Session: 2026-02-23 (initial scaffold)

### OperationMap life-op-001 — Immediate-to-mediated life transition map

- sourceClaims:
  - `ilife-liv-001-c1`
  - `ilife-lif-001-c2`
- sourceRelations:
  - `ilife-liv-001-r1`
  - `ilife-lif-001-r1`
- protocolMode: mixed
- targetOperation: transition_chain_projection
- confidence: 0.66
- proofPath:
  - `ilife-liv-001-c1`
  - `ilife-liv-001-r1`
  - `ilife-lif-001-c2`
  - `ilife-lif-001-r1`
- status: review_pending

### OperationMap life-op-002 — Reproduction-to-genus elevation map

- sourceClaims:
  - `ilife-lif-003-c2`
  - `ilife-gen-001-c2`
  - `ilife-gen-004-c1`
- sourceRelations:
  - `ilife-lif-003-r1`
  - `ilife-gen-001-r1`
- protocolMode: mixed
- targetOperation: universality_elevation_projection
- confidence: 0.7
- proofPath:
  - `ilife-lif-003-c2`
  - `ilife-gen-001-c2`
  - `ilife-gen-001-r1`
  - `ilife-gen-004-c1`
- status: review_pending

Notes:

- Operation targets are placeholders until claim/relation review outcomes are accepted.
