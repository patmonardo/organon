# ABSOLUTE-COMPILER-V2-WORKBOOK

Status: Scaffold workbook (translation in progress)
Doctrine scope: Exposition / Attribute / Mode
Primary source texts: `exposition.txt`, `attribute.txt`, `mode.txt`

## Purpose

Define the chapter-level compiler contract for Absolute so generator and compiler behavior can be derived from one workbook authority.

This workbook is orchestration-level: detailed first-order claims remain in `ABSOLUTE-PART-A-WORKBOOK.md`, `ABSOLUTE-PART-B-WORKBOOK.md`, and `ABSOLUTE-PART-C-WORKBOOK.md`.

## Non-negotiable boundary

- Form Processor computes reflection-validity only.
- Outputs are admissibility/ground/trace artifacts.
- No scientific-causal determination in this stage.

## Source authority map

- Idea anchor: `ABSOLUTE-IDEA-WORKBOOK.md`
- Part A: `ABSOLUTE-PART-A-WORKBOOK.md`
- Part B: `ABSOLUTE-PART-B-WORKBOOK.md`
- Part C: `ABSOLUTE-PART-C-WORKBOOK.md`

## V2 contract model

### Input envelope

- Source text chunks from Exposition / Attribute / Mode topic maps.
- Existing IR states for absolute chapter sections.
- Context envelope (`phase`, provenance, boundary profile).

### Normalized algebra terms

- Nodes: `Absolute`, `Identity`, `Attribute`, `Mode`, `Reflection`, `Actuality`, `Fact`.
- Edges: `REFLECTS`, `NEGATES`, `SUBLATES`, `MEDIATES`, `NEXT`, `VALID_IN`, `SUPPORTED_BY`, `CONDITIONED_BY`.

### Core survival predicate

$$
exists\_fact(J) := generated(J) \land admissible\_morph(J) \land valid\_path(J) \land non\_contradicted(J)
$$

### Attribute-mode conversion rule

$$
mode\_emergence(X) := attribute\_determination(X) \land reflective\_self\_sublation(X)
$$

Reading: mode is valid where attribute-determination is self-sublated through reflection.

### Actuality handoff closure rule

$$
absolute\_handoff\_closure(X) := self\_manifestation(X) \land transitions\_to\_actuality(X)
$$

Reading: chapter closure is reached when absolute manifestation yields actuality handoff.

## Best-of inherited patterns (from legacy compiler/generator)

1. Keep Shape/Definition boundary explicit.
2. Preserve deterministic IDs and provenance.
3. Compile only accepted state sets with explicit guards.
4. Validate envelopes before emission.
5. Preserve directional Fact -> Relation -> Knowledge flow.

## Language policy (readability-first)

- Prolog contributes predicate ideas only; it is not the surface syntax.
- Workbook contracts are authored in Scheme-Python style:
  - composable transforms,
  - explicit records,
  - readable functional flow,
  - minimal symbolic punctuation.

## Generator logic contract (Absolute)

Generator must:

1. Read and normalize chunk graphs for A/B/C sources.
2. Preserve line-evidence from Part workbooks as authority.
3. Build chapter trace from exposition -> attribute -> mode.
4. Materialize candidate judgments with deterministic IDs.
5. Emit surviving judgments, blessed facts, and rejection records.

Required conceptual outputs:

- `absoluteChunkGraph`
- `absoluteTraceGraph`
- `candidateJudgments`
- `survivingJudgments`
- `blessedFacts`
- `rejections`

## Relations regeneration gate (must be true)

- [x] `ABSOLUTE_PART_A_POPULATED`
- [x] `ABSOLUTE_PART_B_POPULATED`
- [x] `ABSOLUTE_PART_C_POPULATED`
- [x] `ABSOLUTE_IDEA_ANCHOR_LINKED`
- [ ] `ABSOLUTE_RELATIONS_REVIEWED`
- [ ] `ABSOLUTE_RELATION_IDS_FROZEN`

## Compiler derivation contract

Compilers derived from this workbook must:

1. Preserve state progression (`exposition|attribute|mode|actuality_handoff`).
2. Preserve deterministic IDs and provenance through all projections.
3. Never materialize a fact without valid path + chapter trace support.
4. Emit chapter-level record views keyed by typed state IDs.

## Rejection taxonomy (mandatory)

- `RJ_GENESIS_UNSAT`
- `RJ_MORPH_ADMISSIBILITY_FAIL`
- `RJ_CONTEXT_INVALID`
- `RJ_PATH_BROKEN`
- `RJ_CONTRADICTION_UNRESOLVED`
- `RJ_GROUND_MISSING`

Each rejection includes:

- `candidateId`
- `code`
- `phase`
- `evidence[]`
- `explanation`

## Session log

### Session: 2026-02-20

- Decision: initialize minimal Absolute compiler scaffold after A/B/C + Idea completion.
- Decision: keep this file orchestration-level; no duplication of part-level claim decomposition.
- Decision: use `abs-idea-001-c1` as current chapter handoff anchor to Actuality trajectory.
