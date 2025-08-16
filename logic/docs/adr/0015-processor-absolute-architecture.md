---
adr: 0015
title: Processor (Absolute) — Responsibilities, Contract, and Pipeline
status: proposed
date: 2025-08-16
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

The `Absolute` folder implements a processor layer that orchestrates the Logic
system. It wraps schema validation, runtime derivation (ground), reflective
faceting, projection/indexing, control/plan generation, and optional
persistence. This ADR records a compact, actionable specification of what the
Processor must do, how the contract looks, where responsibilities currently live
in the codebase, and a set of minimal next steps for incremental improvements.

This document is intentionally practical: it maps architectural responsibilities
to existing modules in `logic/src/absolute` so engineers can read, test and
refactor the Processor without cross-package work.

## Decision (summary)

Treat `Absolute` as the canonical processor. It must:

- Accept validated `ProcessorInputs` and `KriyaOptions`.
- Produce a deterministic `World` snapshot and a `KriyaResult` that includes
  derived ground (relations + properties), projections/indexes, control plans,
  optional reflect facets, and an attachable `knowledge` delta.
- Be defensive: derivation is best-effort (fail fast in test/dev; degrade
  gracefully in production). Provenance must be attached to derived items.
- Keep schema-level types at API boundaries only; use runtime-friendly shapes
  inside stages (e.g., `ThingLike`, `PropertyLike`, or `Loose*`).

## Inputs / Outputs (contract)

Canonical inputs
- ProcessorInputs (schema-level): { entities: Entity[], properties: Property[], morphs: Morph[], contexts?, content?, triad? }
- KriyaOptions (canonical): see `logic/src/absolute/core/kriya.ts` (fixpoint, projectContent, contentIndexSource, deriveSyllogistic, triad, commitGround, reflectOpts, actionThreshold, ...)

Canonical outputs
- KriyaResult (public): {
  world: World,
  indexes: { content: IndexResult },
  projectedContent: Content[],
  derivedEdges: Relation[],
  ground?: { relations: Relation[], properties: Property[] },
  knowledge?: KnowledgeDelta,
  reflect?: ReflectResult,
  controls?: Controls,
  work?: Work,
}

Error modes
- Validation errors: raise at API boundary (use schema validators).
- Derivation errors: logged and degraded (ground/reflect should return
  partial results; in dev/tests they may throw to surface problems).

## Pipeline stages and current code mapping

- Assemble World — deterministic snapshot
  - function: `assembleWorld`
  - file: `logic/src/absolute/essence/world.ts`

- Project Content from Contexts
  - function: `projectContentFromContexts`
  - file: `logic/src/absolute/form/project.ts`

- Reflect Stage (optional runtime faceting)
  - function: `reflectStage`
  - file: `logic/src/absolute/essence/reflect.ts`
  - runtime shapes: `ThingLike`, `PropertyLike`

- Ground Stage (derivation, fixpoint)
  - function: `groundStage`
  - file: `logic/src/absolute/essence/ground.ts`
  - input: morphs + entities/properties; output: `GroundResult`

- Concept-level inference (syllogism / chemism / teleology)
  - files: `logic/src/absolute/concept/*` (e.g., `syllogism.ts`)
  - these are higher-level drivers that may seed morphs or post-process ground

- Model / Control / Plan (projections & work)
  - StageFns: `model`, `control`, `plan` as used by `runCycle`
  - file: `logic/src/absolute/core/kriya.ts` and `core/orchestrator.ts`

- Action stage (reciprocal effects)
  - file: `logic/src/absolute/core/action.ts` (optional)

- Commit / Persistence (triad)
  - function: `commitGroundResults`
  - file: `logic/src/absolute/essence/ground.ts`

## Important invariants and runtime guidance

- Schema vs runtime boundary
  - Only validation/persistence code should import schema types directly.
  - Processor stages should use runtime aliases (ThingLike/PropertyLike) or
    local `Loose*` types.

- Provenance
  - Every derived relation/property must carry provenance: ruleId, source,
    timestamp, metaphysical annotations, modality/confidence.

- Fixpoint termination
  - Derivation runs to a bounded fixpoint controlled by `fixpointMaxIters`.

- Production resiliency
  - Prefer logging and partial results in production. Throwing should be
    reserved for tests/dev to surface contract violations.

## Edge cases to watch

- Missing or partial `essence` in entities — reflect/ground must behave
  sensibly when fields are absent.
- Duplicate morph rules — ensure idempotence and deterministic ids for
  derived relations to avoid non-determinism.
- Persistence commit failures — triad commit should be best-effort and not
  break the Processor.

## Tests & verification

- Unit tests per stage (fast):
  - world assembly, reflect facets, ground derivations (happy path + idempotence),
    orchestrator end-to-end with triad stub.
- Smoke harness: a small script that runs `new FormProcessor().run(sampleInput)`
  to assert shapes and stability.

## Minimal next actions (short-term)

1) Document import policy for `absolute` (schema vs runtime). (Suggested: add `IMPORTS.md`.)
2) Add a small smoke test under `logic/test/` that calls `runKriya` with a tiny
   sample input and asserts shape of result.
3) Medium-term: tighten `RuleSpec`/Morph typing and remove remaining `as any`
   uses in `essence/ground.ts` (safer refactor).

## Rationale

This ADR keeps the processor pragmatic and testable. It preserves the single
responsibility of `Absolute`: orchestrate Logic stages and provide a stable
engine contract while avoiding schema leakage into runtime internals. The
approach allows incremental hardening (typing and tests) while keeping the
repository buildable and usable today.

## Requirements coverage

- Consolidate `KriyaOptions` — addressed (canonical in `core/kriya.ts`) — Done
- Soften invariant checks in ground — addressed (throws in dev/test, warns in prod) — Done
- Avoid exporting schema types from processor surface — addressed (reflect uses runtime types) — Done

## Next steps

If approved, create `logic/src/absolute/IMPORTS.md` documenting imports and add
the smoke harness test in `logic/test/` as the next PR.
