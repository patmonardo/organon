---
adr: 0016
title: Spectrum / Aspect / Appearance — mapping phenomenology to the Processor model
status: proposed
date: 2025-08-16
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

The project treats Relations and Properties in a philosophical (Fichte–Hegel)
frame as phenomena of appearance and reflection. Practically, the Processor
needs a lightweight, testable model for "seeing" aspects of Things and
Relations — a spectrum of visible/signaling features — that engine stages can
consume to prefer, rank, or actualize relations.

This ADR defines a pragmatic runtime model for Spectrum and Aspect, maps where
it's computed and used in the codebase, and lists small, safe invariants and
tests to validate the approach.

## Decision (summary)

- Introduce a runtime `spectrum` facet attached to reflective outputs and
  derived relations/properties. `spectrum` is a small object that describes
  visible/signaling aspects and an intensity score.
- Compute `spectrum` in `reflectStage` from `ThingLike` and `PropertyLike`
  inputs and expose it via `ReflectResult`.
- Allow `groundStage` and concept drivers (`concept/*`) to consult `spectrum`
  when converting aggregates into `absolute` containers or when actualizing
  relations.
- Keep the model engine-only (runtime): schema documents remain unchanged; if
  persistent storage of spectrum is later required, map it into derived
  property documents via `commitGroundResults`.

## Runtime model

Add the following lightweight type to `essence/reflect.ts` (runtime only):

- Spectrum (example shape)

  {
    signature: string;      // stable short hash of the observed facets
    intensity: number;      // 0..1 confidence/intensity of the spectrum
    aspects: string[];      // human-readable aspect tags (e.g. ['teleology'])
    evidence?: string[];    // optional evidence references (sig ids)
  }

Attach `spectrum?: Spectrum` to:
- `ReflectFacet` (thingFacets / propertyFacets) — produced by `reflectStage`.
- Derived relation/properties' `meta` field (engine-level meta, not schema-level
  change) when produced in `groundStage`.

## Where to compute and use (mapping to files)

- Compute: `logic/src/absolute/essence/reflect.ts`
  - Use existing heuristics (property counts, essence richness, context signals)
  - Produce `ReflectResult` with `thingFacets[thingId].spectrum` and
    `propertyFacets[propId].spectrum`

- Surface: `logic/src/absolute/core/kriya.ts` / `core/orchestrator.ts`
  - `runCycle`/`runKriya` may attach `reflect` to the returned result so callers
    (or concept drivers) can inspect spectrum signals.

- Consume: `logic/src/absolute/essence/ground.ts` and `logic/src/absolute/concept/*`
  - `groundStage` may use `spectrum` intensity and aspects to decide whether to
    create an `absolute` container or to mark an essential relation as actual.
  - Concept drivers may prefer relations with certain aspect tags.

- Persist (optional): `commitGroundResults` can map `meta.spectrum` into a
  derived property or an archival field if persistence is desired.

## Invariants and guidance

- `spectrum` is an engine/view artifact only. Do not add it to canonical schema
  documents unless we commit to storing it persistently.
- `spectrum.intensity` ∈ [0,1] and `signature` should be stable (deterministic
  hash of facets) to support idempotent checks.
- Concept modules must treat `spectrum` as advisory: it influences but does not
  replace formal derivation rules — derivation remains rule-driven.

## Example heuristics (for reflectStage)

- intensity = normalize( alpha * propertyCount + beta * essenceScore + gamma * contextWeight )
- aspects = include tags from contexts, shape.state.tags, and heuristic labels
  derived from property keys (
  e.g., presence of `goal` or `purpose` → 'teleology')
- signature = stableHash([thingId, sorted(aspects), floor(intensity*100)])

## Tests

- Unit test for `reflectStage`: small ThingLike + PropertyLike with known
  signatures → assert `thingFacets[tid].spectrum` exists and intensity > 0.
- Integration test: run `runKriya` with reflect enabled and assert that
  `res.reflect.thingFacets[tid].spectrum` is present and that `groundStage`
  can observe `meta.spectrum` on derived relations when appropriate.

## Minimal implementation plan (safe steps)

1. Add Spectrum type and spectrum computation to `essence/reflect.ts` (small,
   deterministic function). Add tests.
2. Propagate `ReflectResult` through `runCycle`/`runKriya` (already supported).
3. Update `essence/ground.ts` to optionally consult `spectrum` when
   `groundToEssentialBridge` makes absolute containers (non-breaking).
4. Add an integration smoke test demonstrating a high-intensity spectrum causes
   creation of an absolute container for a derived relation.

## Rationale

This design keeps phenomenological concerns local to the engine layer, makes
`spectrum` deterministic and testable, and provides clear hooks for concept
modules to advance dialectical movement using perceptual-like signals (sight,
reflection) without forcing schema-level changes.

*** End
