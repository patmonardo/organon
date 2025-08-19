---
adr: 0018
title: Shape as Active Essence — ADR for the Absolute Processor
status: proposed
date: 2025-08-17
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

The Absolute processor uses Shape as a first-class engine concept. Philosophically, Shape = Active Essence: the selective, structuring moment that turns raw input (Dead Being) into recognitions, affordances and predictable behaviors. Practically, Shape seeds signatures, drives reflection (spectrum/aspects), and is the core input to Morph-driven derivations.

This ADR records the principle "Shape as Active Essence" and maps it to concrete runtime types, file locations, and next steps so the processor can begin treating Shapes as engine primitives (before we extend to Entities, Contexts, Properties and Relations).

## Decision

1. Treat Shape as an engine-level Essence object (ShapeEssence) derived from the canonical schema Shape (FormShape).
2. Provide a small runtime API for ShapeEssence:
   - computeSignature(shape) — deterministic signature based on shape facets/state.
   - extractFacets(shape) — engine facets used by reflectStage.
   - toThing(shape, values?) — instantiate a Thing/Entity from a Shape (extension point).
3. Keep schema-level Shape (schema/shape.ts) as the canonical source of truth; the engine uses the FormShape wrapper to validate inputs and produce ShapeEssence.
4. Place engine code under: logic/src/absolute/essence/*
   - essence.ts — ShapeEssence helpers and API
   - thing.ts — Thing / Entity instantiation helpers and runtime wrapper
5. Keep additions non-breaking: new runtime fields optional; no schema mutations required in this initial pass.

## Rationale

- Philosophical: Shape captures the "selection" that makes an object appear as an object (Active Essence).
- Engineering: Centralizing shape-derived logic simplifies reflectStage, Morph evaluation, and creates a clear pathway from Shapes → Things → Properties/Relations.
- Pragmatic: The processor can iterate on ShapeEssence heuristics (signatures, spectrum cues) without schema churn.

## Runtime model (informal)

- ShapeEssence
  - id, type, name
  - signature: deterministic fingerprint used by reflect
  - facets: engine facets (counts, salient keys, inferred aspects)
  - behaviors: optional metadata describing expected transitions

- Thing (engine-facing)
  - id, type, shapeId
  - values: appearance values (property bag)
  - derivedFacets: spectrum/qualities produced by reflect

## Implementation plan (minimal, low-risk)

1. Add runtime scaffolds: `essence.ts` and `thing.ts` under `logic/src/absolute/essence/`.
2. Implement computeSignature & extractFacets using deterministic, testable heuristics (hash of selected facets, counts).
3. Wire FormShape → ShapeEssence in reflectStage (use computeSignature).
4. Provide a `toThing` helper to construct engine Thing objects from a Shape + supplied values.
5. Add unit tests for computeSignature, extractFacets, and toThing.

## Next steps

- Scaffold files: `logic/src/absolute/essence/essence.ts` and `logic/src/absolute/essence/thing.ts` (small helpers).
- Add tests under `logic/test/absolute/`:
  - shape-essence.spec.ts (signature/facets deterministic)
  - thing-instantiation.spec.ts (toThing)
- On success, reflectStage should consume computeSignature for spectrum seeding.

## Consequences

- Short term: small, non-breaking runtime additions that make Shape central to reflection and ground.
- Long term: ShapeEssence can carry richer behavioral primitives and be the main input to Morphs (pure qualitative grounds).
