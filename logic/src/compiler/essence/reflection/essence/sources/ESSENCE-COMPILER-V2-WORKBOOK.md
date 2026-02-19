# ESSENCE-COMPILER-V2-WORKBOOK

Status: Active rewrite workbook (source-local)
Doctrine scope: Essence / Reflection / Shine
Primary source texts: `essence.txt`, `reflection.txt`, `shine.txt`

## Purpose

Define the contract logic that replaces the current `generate-*` + `compile-*` split for Essence.

The workbook is the source of truth. Generator behavior is specified here first; compiler modules are derived from this workbook contract.

## Non-negotiable boundary

- Form Processor computes **reflection-validity** only.
- Outputs are admissibility/ground/trace artifacts.
- No scientific-causal determination in this stage.

## V2 contract model

### Input envelope

- Source text chunks from Essence/Reflection/Shine topic maps.
- Existing IR states for doctrine sections.
- Context envelope (`phase`, provenance, boundary profile).

### Normalized algebra terms

- Nodes: `Form`, `Context`, `Morph`, `Judgment`, `Fact`.
- Edges: `REFLECTS`, `NEGATES`, `SUBLATES`, `MEDIATES`, `NEXT`, `VALID_IN`, `SUPPORTED_BY`, `CONDITIONED_BY`.

### Core survival predicate

$$
exists\_fact(J) := generated(J) \land admissible\_morph(J) \land valid\_path(J) \land non\_contradicted(J)
$$

### Negative philosophy form (morphological dissolution)

Use conditioned-negation as a first-class contract rule:

$$
\neg generated(J) \lor \neg admissible\_morph(J) \lor \neg valid\_path(J) \lor \neg non\_contradicted(J) \Rightarrow \neg exists\_fact(J)
$$

Reading: if any sustaining condition is absent, the fact-form does not survive.

Operational consequence:

- Treat rejection as structural (condition-loss), not incidental failure.
- Keep rejection evidence tied to the first missing condition encountered.
- Preserve reflection-only boundary: this is form-validity negation, not scientific-causal explanation.

### Entry into existence (temporal morphology; Yoga 4.12)

Use a temporal-path contract for dharma variation across phases:

$$
exists\_mode(J, t, s) := exists\_fact(J) \land dharma\_path\_valid(J, t) \land svarupa\_coherent(J, s)
$$

Where:

- `t` is temporal mode (`past`, `present`, `future`).
- `s` is standpoint-indexed form (`sva` as own-form index in this morphology contract).

Adhvabheda rule (difference of temporal path):

$$
t_1 \neq t_2 \Rightarrow dharma\_profile(J, t_1) \neq dharma\_profile(J, t_2)
$$

Identity-continuity constraint:

$$
same\_thread(J, t_1, t_2) \land t_1 \neq t_2 \Rightarrow transform\_consistent(J, t_1, t_2)
$$

Negative corollary:

$$
\neg dharma\_path\_valid(J, t) \lor \neg svarupa\_coherent(J, s) \Rightarrow \neg exists\_mode(J, t, s)
$$

Reading: past/future are not null; they are determinate mode-forms along differentiated dharma paths.

### Dharma-path ontology (Yoga 4.13; guṇa law)

FactStore is a path ontology: each surviving fact must be recorded as a dharma-path mark with both manifest and subtle quality state.

Core guṇa composition contract:

$$
guna\_structured(F) := has\_guna\_signature(F) \land vyakta\_state(F) \land sukshma\_state(F)
$$

FactStore admission rule:

$$
admit\_fact\_store(F) := exists\_fact(F) \land path\_of\_dharma\_marked(F) \land guna\_structured(F)
$$

Dharma-path mark requirements:

- `path_of_dharma_marked(F)` must include ordered phase traces and provenance anchors.
- `vyakta_state(F)` captures manifest, line-evidenced determinations.
- `sukshma_state(F)` captures subtle determinations that are inferentially constrained and reviewable.

Quality-transform rule:

$$
same\_thread(F, t_1, t_2) \Rightarrow guna\_transform\_traceable(F, t_1, t_2)
$$

Negative corollary (law of qualities):

$$
\neg has\_guna\_signature(F) \lor \neg vyakta\_state(F) \lor \neg sukshma\_state(F) \Rightarrow \neg admit\_fact\_store(F)
$$

Reading: what is not quality-structured across manifest/subtle registers is not admissible as a fact.

### Thing/Entity transition (Yoga 4.14; pariṇāma-identity)

Transition rule into Thing/Entity is grounded in transformation-unity, not static abstraction.

Core vastu-tattva contract:

$$
vastu\_tattva(E) := transformation\_unity(E) \land dharma\_thread\_continuous(E)
$$

Where transformation-unity is expressed as:

$$
transformation\_unity(E) := \forall t_i,t_j\,(same\_thread(E,t_i,t_j) \Rightarrow law\_consistent\_parinama(E,t_i,t_j))
$$

Thing/Entity admission rule:

$$
admit\_entity(E) := admit\_fact\_store(E) \land vastu\_tattva(E)
$$

Negative corollary:

$$
\neg transformation\_unity(E) \lor \neg dharma\_thread\_continuous(E) \Rightarrow \neg admit\_entity(E)
$$

Reading: object-truth is the identity of lawful transformation across phases.

Boundary declaration:

- This workbook follows Greater Logic trajectory (Essence -> Thing/Entity via determinate transformation).
- Do not reduce entity identity to content-empty or merely formal invariance.

## Best-of inherited patterns (from legacy compiler/generator)

These are the parts worth keeping from old `compile-*` and `generate-*` work.

1. **Shape/Definition boundary stays explicit**
   - Runtime layer speaks in Shapes.
   - Compiler layer speaks in Definitions and lowering.
2. **Deterministic ID + provenance discipline**
   - every emitted artifact keeps stable IDs and source provenance.
3. **State-guarded compilation**
   - compile only accepted state sets; skip or reject unknown states explicitly.
4. **Schema-validated envelopes**
   - parse/validate output envelopes before emission.
5. **Directional repository flow**
   - Fact -> Relation -> Knowledge direction remains explicit.

## Language policy (readability-first)

- Prolog is inspiration only (predicate and constraint ideas), not surface syntax.
- Compiler workbook language must be **Scheme-Python style**:
  - small composable transforms,
  - explicit data records,
  - readable functional steps,
  - minimal symbolic punctuation.
- Contract preference: clear function/data pipelines over clause-style Prolog notation.

## Generator logic contract (Essence)

The V2 generator must:

1. Read source chunks by line range.
2. Build deterministic chunk graph (sequential + inferred reflective traces).
3. Derive tags and dialectical role labels from chunk semantics.
4. Validate doctrine IR envelope against schema.
5. Emit canonical integrated doctrine artifact (single source of generation for compile stage).

Required outputs (conceptual):

- `integratedTopicMap`
- `reflectiveTraceGraph`
- `candidateJudgments`
- `survivingJudgments`
- `blessedFacts`
- `rejections`

## Relations regeneration (must happen first)

This workbook is not considered generator-ready until Relations are regenerated from source.

### Required relation set (Essence)

- `REFLECTS`
- `NEGATES`
- `SUBLATES`
- `MEDIATES`
- `NEXT`

### Pass discipline

1. Regenerate relations directly from source chunks (`essence.txt`, `reflection.txt`, `shine.txt`).
2. Keep only line-evidenced first-order relations in this pass.
3. Mark uncertain relations as `review_pending` instead of forcing acceptance.
4. Freeze relation IDs before compiler derivation.

### Gate

- [ ] `ESSENCE_RELATIONS_REGENERATED_V2`
- [ ] `ESSENCE_RELATIONS_REVIEWED`
- [ ] `ESSENCE_RELATION_IDS_FROZEN`

## Compiler derivation contract (from workbook)

Compiler modules are generated from workbook-defined contracts, not hand-grown ad hoc.

Derived compiler responsibilities:

1. Materialize shape repos from surviving judgments.
2. Preserve deterministic IDs and provenance.
3. Emit record views keyed by typed state IDs.
4. Never materialize a fact without a validity path.

## Rejection taxonomy (mandatory)

- `RJ_GENESIS_UNSAT`
- `RJ_MORPH_ADMISSIBILITY_FAIL`
- `RJ_CONTEXT_INVALID`
- `RJ_PATH_BROKEN`
- `RJ_CONTRADICTION_UNRESOLVED`
- `RJ_GROUND_MISSING`

Each rejection carries:

- `candidateId`
- `code`
- `phase`
- `evidence[]`
- `explanation`

## Migration intent for old modules

- Do not preserve all existing `generate-*` scripts.
- Keep only patterns that directly satisfy this contract.
- Rebuild compiler modules from workbook contracts with readable KG semantics.

## Working checklist

- [ ] Essence relations regenerated and frozen.
- [ ] Chunk graph and trace graph schema stabilized.
- [ ] Judgment survival gates finalized.
- [ ] Rejection evidence envelope stabilized.
- [ ] Compiler derivation templates finalized.
- [ ] Reflection-only boundary assertions added.
