# Cognition Compiler Workbook

Part: `COMPILER. RELATION-TO-OPERATION TARGET MAP`
Status: active
Authority: derivative from Cognition workbooks (non-authoritative)

## Purpose

- This workbook captures compiler-target mappings from validated claim/relation structures.
- It is downstream of Part A/B + IDEA workbooks.

## Inputs

- `COGNITION-PART-A-WORKBOOK.md`
- `COGNITION-PART-B-WORKBOOK.md`
- `COGNITION-IDEA-WORKBOOK.md`

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

### OperationMap cog-op-001 — True analytic-to-synthetic transition map

- sourceClaims:
  - `con-idea-cog-a-001-c2`
  - `con-idea-cog-a-002-c2`
  - `con-idea-cog-a-003-c1`
- sourceRelations:
  - `con-idea-cog-a-001-r1`
  - `con-idea-cog-a-002-r2`
- protocolMode: mixed
- targetOperation: cognition_transition_projection
- confidence: 0.69
- proofPath:
  - `con-idea-cog-a-001-c2`
  - `con-idea-cog-a-001-r1`
  - `con-idea-cog-a-002-c2`
  - `con-idea-cog-a-002-r2`
  - `con-idea-cog-a-003-c1`
- status: review_pending

### OperationMap cog-op-002 — Good-to-absolute consummation map

- sourceClaims:
  - `con-idea-cog-b-002-c2`
  - `con-idea-cog-b-003-c2`
  - `con-idea-cog-004-c1`
- sourceRelations:
  - `con-idea-cog-b-002-r2`
  - `con-idea-cog-003-r2`
- protocolMode: mixed
- targetOperation: practical_consummation_projection
- confidence: 0.72
- proofPath:
  - `con-idea-cog-b-002-c2`
  - `con-idea-cog-b-002-r2`
  - `con-idea-cog-b-003-c2`
  - `con-idea-cog-003-r2`
  - `con-idea-cog-004-c1`
- status: review_pending

Notes:

- No Part C operation input is defined for Cognition by design in this pass.
- Operation targets remain placeholders until review outcomes are accepted.
