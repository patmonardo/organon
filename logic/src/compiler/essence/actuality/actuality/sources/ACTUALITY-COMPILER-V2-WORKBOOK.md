# ACTUALITY-COMPILER-V2-WORKBOOK

Status: Scaffold workbook (translation in progress)
Doctrine scope: Formal Actuality / Real Actuality / Absolute Necessity
Primary source texts: `contingency.txt`, `relative-necessity.txt`, `absolute-necessity.txt`

## Purpose

Define the chapter-level compiler contract for Actuality so generator and compiler behavior can be derived from one workbook authority.

This workbook is orchestration-level: detailed first-order claims remain in `ACTUALITY-PART-A-WORKBOOK.md`, `ACTUALITY-PART-B-WORKBOOK.md`, and `ACTUALITY-PART-C-WORKBOOK.md`.

## Non-negotiable boundary

- Form Processor computes reflection-validity only.
- Outputs are admissibility/ground/trace artifacts.
- No scientific-causal determination in this stage.

## Source authority map

- Idea anchor: `ACTUALITY-IDEA-WORKBOOK.md`
- Part A: `ACTUALITY-PART-A-WORKBOOK.md`
- Part B: `ACTUALITY-PART-B-WORKBOOK.md`
- Part C: `ACTUALITY-PART-C-WORKBOOK.md`

## V2 contract model

### Input envelope

- Source text chunks from Contingency / Relative Necessity / Absolute Necessity topic maps.
- Existing IR states for actuality chapter sections.
- Context envelope (`phase`, provenance, boundary profile).

### Normalized algebra terms

- Nodes: `Form`, `Content`, `Actuality`, `Possibility`, `Necessity`, `Contingency`, `Substance`, `Fact`.
- Edges: `REFLECTS`, `NEGATES`, `SUBLATES`, `MEDIATES`, `NEXT`, `VALID_IN`, `SUPPORTED_BY`, `CONDITIONED_BY`.

### Core survival predicate

$$
exists\_fact(J) := generated(J) \land admissible\_morph(J) \land valid\_path(J) \land non\_contradicted(J)
$$

### Contingency-necessity conversion rule

$$
necessary\_form(X) := contingency\_cycle(X) \land self\_rejoining\_identity(X)
$$

Reading: what survives as necessary must demonstrate contingency conversion and rejoining identity.

### Absolute necessity closure rule

$$
absolute\_necessity\_closure(X) := unity(actuality, possibility, X) \land unity(being, essence, X)
$$

Reading: chapter closure is reached when actuality/possibility and being/essence are jointly unified.

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

## Generator logic contract (Actuality)

Generator must:

1. Read and normalize chunk graphs for A/B/C sources.
2. Preserve line-evidence from Part workbooks as authority.
3. Build chapter trace from formal actuality -> real actuality -> absolute necessity.
4. Materialize candidate judgments with deterministic IDs.
5. Emit surviving judgments, blessed facts, and rejection records.

Required conceptual outputs:

- `actualityChunkGraph`
- `actualityTraceGraph`
- `candidateJudgments`
- `survivingJudgments`
- `blessedFacts`
- `rejections`

## Relations regeneration gate (must be true)

- [x] `ACTUALITY_PART_A_POPULATED`
- [x] `ACTUALITY_PART_B_POPULATED`
- [x] `ACTUALITY_PART_C_POPULATED`
- [x] `ACTUALITY_IDEA_ANCHOR_LINKED`
- [ ] `ACTUALITY_RELATIONS_REVIEWED`
- [ ] `ACTUALITY_RELATION_IDS_FROZEN`

## Compiler derivation contract

Compilers derived from this workbook must:

1. Preserve state progression (`formal_actuality|real_actuality|absolute_necessity|substance_handoff`).
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

- Decision: initialize minimal Actuality compiler scaffold after A/B/C + Idea completion.
- Decision: keep this file orchestration-level; no duplication of part-level claim decomposition.
- Decision: use `act-idea-001-c1` as current chapter handoff anchor to Substance trajectory.
