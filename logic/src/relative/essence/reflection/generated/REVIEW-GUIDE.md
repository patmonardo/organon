# Reflection IR Review Guide (15-minute critical pass)

Purpose: run a fast but rigorous validation of the integrated graph with a critical lens on provenance, sufficiency of grounding, and doctrine coherence.

## Preconditions

- Generate latest artifacts:
  - `pnpm --filter @organon/logic codegen:cit:cypher-ir`
- Use merged files:
  - `integrated-cit-topicmap-ir.cypher`
  - `integrated-cit-topicmap-query-pack.cypher`
  - `integrated-cit-topicmap-ir.debug.json`
- Load `integrated-cit-topicmap-ir.cypher` into Neo4j first.

## 0–2 min: Structural sanity

Run **Q1**, **Q2**, **Q10** from `integrated-cit-topicmap-query-pack.cypher`.

Pass criteria:

- Node labels include at least: `SourceText`, `ChunkSegment`, `Topic`, `IntegratedChunk`, `ConsciousnessLayer`, `CITCategory`.
- Relationship types include at least: `HAS_CHUNK_SEGMENT`, `YIELDS_TOPIC`, `LIFTED_TO_IR`, `IN_LAYER`, `SUBLATES`, `NEGATES`, `NEXT`.
- Counts are non-zero and coherent (`segments == topics == chunks` should generally hold for this pipeline).

## 2–6 min: Provenance (TXT -> CHUNK -> TOPIC -> IR)

Run **Q3** first (single example), then **Q4** for one full source (e.g. `source-condition`).

Pass criteria:

- Every sampled `IntegratedChunk` has a complete chain from `SourceText`.
- `lineStart`/`lineEnd` correspond to expected chunk ranges from source plans.
- No orphan `Topic` or `ChunkSegment` nodes.

## 6–10 min: Doctrine and layered movement

Run **Q5**, **Q6**, **Q9**.

Pass criteria:

- Layer membership is populated for `principle-citta`, `law-consciousness`, `science-reflection`.
- Traversal from Principle to Science exists through lawful sublation/negation paths.
- Layer chunk ordering is intelligible and trace links are present.

## 10–13 min: Cross-section transition checks

Run **Q7**.

Focus transitions:

- `ref-15 -> det-1`
- `det-14 -> abs-1`
- `abs-16 -> con-1`

Pass criteria:

- Transition edges exist with explanatory `reason`.
- Source boundaries are crossed intentionally (not random leakage).

## 13–15 min: Membership protocol validation

Run **Q8**.

Pass criteria:

- `CIT`/`CITI`-classified chunks are connected by `SUBLATED_MEMBER_OF -> CITTA`.
- Classification and sublation are consistent with intended protocol semantics.

## Review notes template

Record:

- **What is philosophically exact** (keep)
- **What is graph-theoretically weak** (revise)
- **What is text-provenance ambiguous** (tighten)
- **What should be split into next pass** (multi-pass refinement queue)

## Suggested next refinement passes

1. **Pass A (Provenance strictness):** enforce 1:1 constraints and add orphan detection queries.
2. **Pass B (Trace precision):** replace heuristic traces with explicit operator rules per source family.
3. **Pass C (Doctrine calibration):** tune layer membership by explicit criteria document.
4. **Pass D (Neo4j ergonomics):** add indexes and dashboard-style saved queries.
