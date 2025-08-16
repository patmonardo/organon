# 0011 — Official Stages & Pipeline (Ground / Synth / Reflect / Actualize / Commit)

- Status: Proposed
- Deciders: Logic/Form maintainers
- Date: 2025-08-15
- Related: 0008-outer-inner-actuality-modality, 0009-thing-as-vastu-reflective-consciousness, 0010-property-as-ground-reflection-and-causality

## Summary

This ADR declares a canonical set of pipeline stages used by the Form/Absolute processors in ORGANON. It documents which stages currently exist, which are design-level proposals, where to find (or place) their implementation, and the recommended API contract for each stage. The goal is to make the pipeline explicit and consistent across code, docs, tests and orchestrator wiring.

## Motivation

- Developers must have a single, discoverable spec for the FormProcessor pipeline: what runs, in what order, and what each stage's responsibilities and contracts are.
- There is drift between design docs (which mention `synthStage`, `reflectStage`) and code (which implements `groundStage` and orchestrator wiring). This ADR unifies the concepts and gives a minimal path to finish missing stages.

## Canonical stages (ordered)

1) reflectStage (optional)
   - Purpose: compute reflective facets (positing/external/determining) for Things and Properties; compute non-persistent signatures (vastuSignature/propertySignature); emit `knowledge.*.reflected` events for inspection.
   - Inputs: world/shape (entities, properties, relations), optional rules, context.
   - Outputs: transient ReflectResult { thingFacets?, propertyFacets?, signatures?, evidence[] }
   - Where to implement: `logic/src/absolute/reflect.ts` or `logic/src/absolute/thing/reflect.ts` and `logic/src/absolute/property/reflect.ts`.
   - Notes: ADR 0009 and 0010 propose the design; reflectStage is descriptive and should not mutate persisted artifacts by default.

2) groundStage (required)
   - Purpose: apply Morph rule specs (forces) deterministically to derive Essential Relations/Properties (Expressions) and produce Absolute containers / Particulars with provenance.
   - Inputs: morphs/rules, world graph (entities/properties), options (fixpointMaxIters, provenance source)
   - Outputs: GroundResult { relations: Relation[], properties: Property[], absolutes: Absolute[], provenance }
   - Where implemented: `logic/src/absolute/ground.ts` (existing). See tests in `test/absolute/*.test.ts`.

3) synthStage (optional / synthesis-level)
   - Purpose: cluster Expressions/Conditions produced by groundStage and synthesize Grounds (Absolute universals); compute modality (possible→actual) per policy; link Conditions.groundedBy and create `ground` artifacts if justified.
   - Inputs: GroundResult (from groundStage), cluster policies, thresholds, provenance
   - Outputs: SynthResult { grounds: Ground[], updates: {conditionId -> groundId}, modalityAssignments }
   - Where to implement: `logic/src/absolute/synth.ts` (new) or extend `logic/src/absolute/ground.ts` with a synth export.
   - Notes: Proposed in ADR 0008. Must be idempotent (signature hashing) and versioned (previousVersionId).

4) actualizeStage / truth-actualization (optional adoption step)
   - Purpose: convert synthesis and evidence into Actuality for Essential relations/properties; set modality.actual = { value: true, confidence } and publish `knowledge.relation.actualized` events.
   - Inputs: SynthResult / GroundResult, classifiers (qualquant.classifyTruthOfRelation), thresholds
   - Outputs: ActualizationResult { actualizedIds[], provenanceUpdates }
   - Where implemented: partial logic exists in `logic/src/absolute/kriya/orchestrator.ts` (truth-actualization loop). Propose moving into `logic/src/absolute/actualize.ts` for testability.

5) commitStage (optional but common)
   - Purpose: persist derivations (relations, absolutes, grounds) produced by earlier stages via triad/triad.commit helpers; publish bus events for create/update.
   - Inputs: GroundResult / SynthResult / ActualizationResult, triad proxy
   - Outputs: CommitResult { persistedIds[], busEvents[] }
   - Where implemented: `commitGroundResults` exists in `logic/src/absolute/ground.ts`. `commitSynthResults` proposed in ADR 0008 should be added at `logic/src/absolute/synth.ts`.

6) knowledgeStage / delta computation (bookkeeping)
   - Purpose: compute knowledge delta between before/after runs (new relations/properties, modality upgrades), publish `knowledge.delta` and return knowledge summary on KriyaResult.
   - Inputs: beforeWorldSnapshot, afterWorldSnapshot
   - Outputs: KnowledgeDelta
   - Where implemented: `logic/src/absolute/knowledge.ts` (existing). The orchestrator currently calls computeKnowledgeDelta and emits events.

7) project/assemble (projection stage)
   - Purpose: project Absolute artifacts into higher-level forms (World/Shape projection, assemble things, syllogisms derivation). Called by Kriya before/after ground/synth as needed.
   - Where implemented: `logic/src/absolute/form/project.ts`, `logic/src/absolute/world/project.ts`, `logic/src/absolute/world/assemble.ts`.

## Stage contracts (recommended TypeScript shapes)

- StageOptions (global partial): { synth?: boolean, commit?: boolean, reflect?: boolean, fixpointMaxIters?: number, triad?: TriadProxy, thresholds?: Record<string, any> }

- StageFn<TIn, TOut> = (input: TIn, opts?: StageOptions) => Promise<TOut>

- Each stage MUST:
  - accept well-documented inputs and options,
  - return a serializable result object (no side-effectful persistence unless explicitly asked via commit option and triad provided),
  - attach provenance metadata to produced artifacts (ruleId, source, timestamp, contextId, metaphysics if available),
  - be idempotent when given identical inputs (where applicable) and report stable signatures to support idempotence.

## Discoverability & Exports

- Export canonical stages from `logic/src/absolute/index.ts` so other packages can import them easily:
  - export { groundStage, commitGroundResults } from './ground';
  - export { synthStage, commitSynthResults } from './synth';
  - export { reflectStage } from './reflect';
  - export { actualizeStage } from './actualize';
  - export { computeKnowledgeDelta } from './knowledge';

- Add brief JSDoc comments to each exported stage in its module to help IDE discoverability.

## Tests and QA

- Unit tests per stage with happy-path + 2 edge cases (empty input, repeated input for idempotence).
- Integration test that runs full pipeline with `opts = { reflect: true, synth: true, commit: false }` and verifies stable result shapes.
- Smoke test with commit paths enabled against the triad memory repo.

## Implementation plan (minimal, non-breaking)

1. Create new ADR (this file) — done.
2. Add `synthStage` module stub at `logic/src/absolute/synth.ts` implementing the StageFn contract and returning a no-op SynthResult (for compilation/testing). Add corresponding export to `logic/src/absolute/index.ts`.
3. Add `reflectStage` and `reflect` modules under `logic/src/absolute/` as stubs that compute simple facets (e.g., signature hashes) and return ReflectResult.
4. Move Kriya’s truth-actualization loop out of the orchestrator into `actualize.ts` and have orchestrator call the exported function (no public API change).
5. Wire stage options in `kriya/orchestrator.ts` to call reflect → ground → synth → actualize → commit → knowledge in order when enabled.
6. Add tests and run the package test suite.

Small, incremental approach avoids breaking current behavior while making the pipeline explicit.

## Compatibility and migration

- Default behavior: if no options are passed, Kriya keeps current behavior (calls `groundStage` as today, commits only if triad provided). Synth/reflect/actualize are opt-in via StageOptions.
- When synthStage exists and `opts.synth === true`, orchestrator will invoke synthStage after groundStage.

## Rationale

- Stages model the dialectical steps (thesis → antithesis → synthesis) that the codebase already references. Making them explicit simplifies reasoning, testing and policy application (e.g., modality thresholds) and prevents ad hoc pipeline wiring across the repo.

## Acceptance criteria

- This ADR is accepted by maintainers.
- Stubs for `synthStage` and `reflectStage` exist and are exported.
- Kriya options accept `reflect` and `synth` booleans; orchestrator wiring follows the documented order when enabled.
- Tests demonstrate idempotence and stable signatures for stage outputs.

## Next steps (short)

- I can implement the minimal stubs and exports (synth/reflect modules plus index exports) and add simple unit tests next. Would you like me to create those stubs now? 
