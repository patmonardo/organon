# SUBSTANCE-COMPILER-V2-WORKBOOK

Status: Scaffold workbook (translation in progress)
Doctrine scope: Substantiality / Causality / Reciprocity
Primary source texts: `relation-substantiality.txt`, `relation-causality.txt`, `reciprocity-action.txt`

## Purpose

Define the chapter-level compiler contract for Substance so generator and compiler behavior can be derived from one workbook authority.

This workbook is orchestration-level: detailed first-order claims remain in `SUBSTANCE-PART-A-WORKBOOK.md`, `SUBSTANCE-PART-B-WORKBOOK.md`, and `SUBSTANCE-PART-C-WORKBOOK.md`.

## Non-negotiable boundary

- Form Processor computes reflection-validity only.
- Outputs are admissibility/ground/trace artifacts.
- No scientific-causal determination in this stage.

## Source authority map

- Idea anchor: `SUBSTANCE-IDEA-WORKBOOK.md`
- Part A: `SUBSTANCE-PART-A-WORKBOOK.md`
- Part B: `SUBSTANCE-PART-B-WORKBOOK.md`
- Part C: `SUBSTANCE-PART-C-WORKBOOK.md`

## V2 contract model

### Input envelope

- Source text chunks from Substantiality / Causality / Reciprocity topic maps.
- Existing IR states for substance chapter sections.
- Context envelope (`phase`, provenance, boundary profile).

### Normalized algebra terms

- Nodes: `Substance`, `Accident`, `Cause`, `Effect`, `Reciprocity`, `Freedom`, `Concept`, `Fact`.
- Edges: `REFLECTS`, `NEGATES`, `SUBLATES`, `MEDIATES`, `NEXT`, `VALID_IN`, `SUPPORTED_BY`, `CONDITIONED_BY`.

### Core survival predicate

$$
exists\_fact(J) := generated(J) \land admissible\_morph(J) \land valid\_path(J) \land non\_contradicted(J)
$$

### Substantial-power conversion rule

$$
causal\_emergence(X) := substantial\_power(X) \land posits\_effect(X) \land returns\_to\_self(X)
$$

Reading: causal determination survives only where substantial power posits effect and re-joins itself.

### Reciprocity-concept closure rule

$$
concept\_closure(X) := reciprocal\_action(X) \land unity(universal, singular, particular, X)
$$

Reading: chapter closure is reached when reciprocal causality resolves in the concept-form triad.

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

## Generator logic contract (Substance)

Generator must:

1. Read and normalize chunk graphs for A/B/C sources.
2. Preserve line-evidence from Part workbooks as authority.
3. Build chapter trace from substantiality -> causality -> reciprocity.
4. Materialize candidate judgments with deterministic IDs.
5. Emit surviving judgments, blessed facts, and rejection records.

Required conceptual outputs:

- `substanceChunkGraph`
- `substanceTraceGraph`
- `candidateJudgments`
- `survivingJudgments`
- `blessedFacts`
- `rejections`

## Relations regeneration gate (must be true)

- [x] `SUBSTANCE_PART_A_POPULATED`
- [x] `SUBSTANCE_PART_B_POPULATED`
- [x] `SUBSTANCE_PART_C_POPULATED`
- [x] `SUBSTANCE_IDEA_ANCHOR_LINKED`
- [ ] `SUBSTANCE_RELATIONS_REVIEWED`
- [ ] `SUBSTANCE_RELATION_IDS_FROZEN`

## Compiler derivation contract

Compilers derived from this workbook must:

1. Preserve state progression (`substantiality|formal_causality|determinate_causality|reciprocity|concept_handoff`).
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

- Decision: initialize minimal Substance compiler scaffold after A/B/C + Idea completion.
- Decision: keep this file orchestration-level; no duplication of part-level claim decomposition.
- Decision: use `sub-idea-001-c1` as current chapter handoff anchor to concept trajectory.
