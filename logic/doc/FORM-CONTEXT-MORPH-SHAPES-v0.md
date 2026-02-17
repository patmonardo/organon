# Form / Context / Morph — Shapes v0

Status: working draft (runtime root-shape vocabulary for Organon).

Purpose

- Define `Form`, `Context`, and `Morph` as canonical runtime "shapes" (engine-facing schemas) distinct from compiler `Definitions`.
- Provide a minimal, implementable bag-of-fields that is intentionally engine-oriented (persistence/processing friendly) while remaining projection-friendly for export profiles.

Design principles

- Shapes are runtime artifacts: small, stable, and focused on execution metadata and evaluation facets.
- Definitions (GDSL) are the compiler-facing artefacts describing language/semantics; mapping from Definitions -> Shapes is a lowering step implemented by the compiler.
- Keep shape fields minimal; attach rich metadata at persistence/export layers.

1. FormShape (runtime)

- Purpose: represent a callable form or template that realizes an Entity/Record at runtime.
- Suggested fields:
  - `id`: stable runtime id (string)
  - `type`: logical type (e.g., `appearance.form`)
  - `formId`: canonical definitional id (links back to GDSL `Entity`/`Form`)
  - `name` / `label`
  - `description`
  - `signature`: { `concept`, `phase`, `irId`?, `nextStates?` }
  - `facets`: engine-facing execution facets (e.g., `moments`, `invariants`, `provenance`, `context`)
  - `status`: `seed|compiled|resolved|grounded` (engine lifecycle)
  - `meta`: compiler/runtime hints

2. ContextShape (runtime)

- Purpose: capture the environmental, dataset, and provenance context required to evaluate forms and morphs.
- Suggested fields:
  - `id`
  - `scope`: dataset or provenance scope id
  - `source`: origin reference (IR id, package)
  - `parameters`: evaluation knobs, policies, trust thresholds
  - `provenance`: minimal provenance envelope (recordedAt, agent?)
  - `targetStores`: hints (FactStore/RelationStore/KnowledgeStore) for evaluation

3. MorphShape (runtime)

- Purpose: represent compiled, runtime-state-bearing transformations and evaluative machinery (pre-existing shapes that carry lifecycle flags).
- Rationale: Morph is a shape (not a compiler definition) because it is persisted and consulted by runtime controllers (evaluators, handoffs, seeds).
- Suggested fields:
  - `id` (runtime id)
  - `morphId` or `morphSpecId` (link to processor spec)
  - `concept`, `title` — human-friendly identifiers
  - `phase` (semantic phase: `reflection|appearance|judgment|...`)
  - `signature`: { `irId`?, `nextStates`?, `previousStates`? }
  - `moments`, `invariants`, `forces`, `transitions` (dialectic facets)
  - `morphState`: small state bag capturing lifecycle booleans and transitionTarget
  - `handoff`: runtime handoff flags (nextState, factStoreCandidate, reflectiveUnityCandidate, note)
  - `provenance`
  - `meta`: compiler/runtime producer tag

Validation & invariants

- Shapes should be independently validatable by the runtime: minimal required fields must parse.
- Lowering contract: every `Entity/Property/Aspect` Definition that is lowered must produce a corresponding `FormShape` or `EntityShape` instance whose `formId` references the original definitional id.
- MorphShapes are allowed to carry evaluation-specific fields (e.g., `morphState`) but should not embed compiler internals (AST, unresolved references).

Projection and export

- Shapes map to external profiles on export (SHACL/OWL/RDFS) via projection layers — do not conflate runtime enrichment with ontology-level assertions.

Non-goals

- Do not encode full proof derivations inside shapes.
- Do not replace GDSL `Definition` as the canonical language spec.

Next steps

- Consolidate this draft into `logic/doc/ORGANON-DEFINITIONAL-LAWS.md` as a "runtime shape" appendix.
- Produce a short mapping table: `Definition` -> `Shape` for `Entity`, `Property`, `Aspect`, `Form`, `Context`, `Morph`.

Questions for review

- Which lifecycle states must be persisted for `MorphShape` (seed vs compiled vs active)?
- Are `ContextShape` parameters and `targetStores` implementation-specific or should we standardize a minimal envelope?
