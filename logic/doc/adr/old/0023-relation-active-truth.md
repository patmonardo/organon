# ADR 0023 — Relation as ActiveTruth (of Morph, and of Entity:Property)

Status: Accepted
Date: 2025-08-17

Context
- Relation already exists at schema level. In the engine, Relation plays a double role:
  1) As the realized truth of Morph (rules actualized by Ground), and
  2) As the truth that binds Entity together with its Properties and other Entities under Context.
- We want a precise, engine-oriented account without changing schema.

Decision
- Establish `absolute/essence/relation.ts` as the ActiveRelation module that:
  - Defines invariants (e.g., every essential must point to an absolute container via `particularityOf`).
  - Provides helpers to inspect and maintain the Absolute ↔ Particular linkage.
  - Encodes the alias policy: in engine checks, `kind === 'relation'` is treated as `kind === 'essential'` for ergonomics.
  - Optionally carries advisory annotations (e.g., spectrum from Reflection) for explanation.
- Keep the schema `Relation` unchanged; engine adds runtime-only metadata and checks.

Rationale
- Clarifies the metaphysical mapping in operational terms: Absolute (universal container) and Essential (particular edge) form a pair, making truth legible and testable.
- Unifies “Relation as truth of Morph” with “Relation as truth of Entity:Property” without duplicating types.

Details (contract)
- Essential relation: `{ id, sourceId, targetId, type, kind: 'essential' | 'relation', particularityOf?: absoluteId, provenance?, meta? }`.
- Absolute container: `{ id, sourceId, targetId, type: `${essentialType}:absolute`, kind: 'absolute', provenance?, meta? }`.
- Invariants: `assertRelationHasAbsolute(relations)` verifies linkage and kind of container.
- Utilities: `findAbsoluteFor(relationId, relations)`, `findParticularsFor(absoluteId, relations)`, `assertRelationHasAbsolute(relations)`.

Consequences
- Ground and other processors can compose with Relation invariants consistently.
- Diagnostics (spectrum/appearance) can be attached uniformly to both particular and absolute forms.

References
- Implementation: `logic/src/absolute/essence/relation.ts`
- Concept: `docs/concepts/essential-relation.md`
- Related ADRs: 0020 Reflect (ActiveContext), 0021 World (ActiveWorld), 0022 Ground (ActiveMorph)
