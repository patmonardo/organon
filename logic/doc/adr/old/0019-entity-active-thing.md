---
id: 0019
title: Entity — engine-facing Active Thing
date: 2025-08-17
status: proposed
authors: [patmonardo]
---

## Context

This repository models a layered distinction between canonical schema (Zod-backed forms and persisted documents) and engine-facing runtime artifacts used by the Absolute/Reflect/Ground processor pipeline. Recent ADRs have established that `Shape` is an "engine-facing (active) Essence" (see ADR 0018). We now need an explicit decision for `Entity` so implementation work is aligned across the codebase.

Currently, the codebase uses a variety of terms interchangeably: entity, thing, form, essence. That ambiguity makes it harder to decide what belongs in schema (the canonical representation) and what belongs only to the engine runtime (transient computed metadata, signatures, spectrums, active facets).

## Decision

We will adopt the following principle:

- Entity is an engine-facing, active Thing. In engine code we will refer to runtime entities as Thing or EngineThing where helpful. These runtime constructs are not schema changes; they are convenience wrappers and transient handles the processor uses while building relations, applying morphs, and computing models.
- Shape remains the engine-facing, active Essence (per ADR 0018). We do not introduce an additional `ShapeEssence` abstraction as a separate canonical artifact — the engine will treat `Shape` as the active essence representation.
- The canonical schema files (Zod schemas and persisted forms) remain unchanged. Entities and Shapes in the schema are source-of-truth documents. Engine-facing Things and Essences are runtime-only adaptors; their job is to expose derived helpers, deterministic signatures, computed facets (for example `spectrum`), and ergonomics for rule evaluation.

## Consequences

- Positive:
  - Clarifies terminology: schema = canonical, engine = active/runtime. Developers know where to add invariants (schema) vs. where to add heuristics/derived signals (engine).
  - Simplifies the runtime model: we can implement `Thing` wrappers and keep Shape as the active essence without introducing additional layers.
  - Preserves backward compatibility: no changes to persisted schemas are required; any engine metadata is attached in `meta` objects or kept in transient runtime structures.

- Negative / Trade-offs:
  - The distinction requires discipline. Contributors must avoid leaking engine-only fields into persisted schema without explicit design review.
  - Consumers of the engine who expect pure schema objects may need adapter code when moving between persistence and runtime.

## Implementation notes

- Add a small runtime wrapper module at `logic/src/absolute/essence/thing.ts` (or similar) that exposes the minimal `Thing` API used by reflect/ground:
  - helpers to access id, type, properties, and computed facets
  - a deterministic signature function (reused by `reflect`'s `computeSpectrum`)
  - lightweight constructors/adaptors: `fromSchema(entitySchema: any): Thing`
- Keep the wrapper minimal: it should not re-define schema invariants — it only exposes convenience methods and computed metadata.
- Where engine code currently treats raw schema objects as `any` and computes ad-hoc signals, migrate the most-used helpers to `Thing` to reduce duplication and make tests clearer.

## Alternatives considered

- Introduce `ShapeEssence` as a separate, named abstraction for engine shapes. This was rejected because ADR 0018 already establishes `Shape` as the active essence. Introducing another name would add conceptual friction without clear gains.
- Collapse engine vs. schema and accept a single structure for both runtime and persisted forms. Rejected: persistence models should remain compact and invariant; engine helpers are allowed to be more permissive and transient.

## Backwards compatibility and migration

- No schema migrations required. Any new runtime module should be additive. When adding `meta` fields to derived relations (e.g., `meta.spectrum`), prefer non-breaking, optional keys and document them in the ADR and relevant code comments.

## Related

- ADR 0018 — Shape as Active Essence (engine-facing)
- ADR 0016 — Spectrum / Appearance (engine advisory signals)

## Next steps

1. Create `logic/src/absolute/essence/thing.ts` with a minimal `Thing` wrapper and unit tests (happy path + 1-2 edge cases).
2. Migrate small, high-value helpers from `reflect.ts` and `ground.ts` into the wrapper (signature, facet accessors, deterministic hashing).
3. Keep Shape unchanged; if additional engine helpers are useful, add them as methods on a lightweight `Essence` wrapper that mirrors the `Thing` approach.
