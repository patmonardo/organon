# Form Theory in @organon/logic

Principles
- Shape = Principle of Entity (loose, not bound). Pure principle.
- Entity = Essence/Dharma of Shape (instantiated, participates in concurrency).
- Concurrency: applies to Entities (Contained), not Shapes (Container/Contained split).
- Six pillars: Entity, Context, Property, Morph, Relation, Shape (principle-level FormShape).

Mappings
- Schema layer (Being): canonical Zod models under `logic/src/schema/*`.
- Form layer (Essence): engine-friendly wrappers under `logic/src/form/*` that preserve schema invariants, emit events, and persist via repo/memory.

Lifecycle (happy-path)
1) create -> describe -> setCore -> setState/patchState -> delete
2) Property additionally requires: key, contextId.
3) Relation additionally requires: kind, endpoints { source, target, both with {id,type} }, direction.

Events (principle)
- `*.created`, `*.core.set`, `*.state.set`, `*.state.patched`, `*.described`, `*.deleted`
- Relation-only: `relation.endpoints.set`, `relation.direction.set`
- Property core includes key; Property created includes { key, contextId }.

Consistency rules
- All mutations go through schema helpers (e.g., `update*`) to keep invariants.
- Repos are source of truth when present; in-memory Map is fallback for tests.
- Persist strategy (simplified): delete+create on update to avoid API mismatch.

Concurrency notes
- Entities are concurrent (mutable, versioned); Shapes remain principled (no direct concurrency semantics).
- Relation endpoints require stable typed refs; mutations validate via schema before persistence.

Testing
- Vitest happy-path suites cover all services and schema/shape with BaseState defaults awareness.
- Prefer presence/partial assertions over strict ordering for event streams.
