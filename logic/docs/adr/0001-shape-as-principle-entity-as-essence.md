# 0001 — Shape as Principle; Entity as Essence; Concurrency on Entities

- Status: Accepted
- Date: 2025-08-14
- Owners: @organon/logic

## Context
We reframed “Shape” and “Entity” to clarify roles in Form Theory:
- Shape is loose/principled (not bound), the pure Principle of a thing.
- Entity is the instantiated Essence (Dharma) of a Shape.
- Concurrency reflects Container/Contained split; we reason about concurrency on Entities (Contained), not on Shapes (Principle).

Form layer services wrap canonical Zod schemas. Repositories vary in API shape across environments.

## Decision
- Shapes remain principle-level (no direct concurrency semantics).
- Entities, Contexts, Properties, Morphs, Relations are the mutable/concurrent actors.
- Service event taxonomy is standardized:
  - created, core.set, state.set, state.patched, described, deleted
  - Relation: endpoints.set, direction.set
  - Property: core includes key; created includes { key, contextId }
- Persistence: prefer repo when provided; otherwise in-memory. To avoid repo API mismatch, services may use delete+create as a safe default until a common update(id, doc) is standardized.
- Schema invariants are enforced at all writes (Zod parse on create/update).

## Rationale
- Separates Principle (Shape) from Essence (Entity), simplifying reasoning about mutability and concurrency.
- Standardized events enable consistent testability and downstream processing.
- Schema-first validation preserves invariants across engines and repos.
- Simplified persistence path reduces coupling to repo specifics while the API stabilizes.

## Consequences
- Tests assert event presence and partial state (BaseState defaults), not strict sequences.
- Relation endpoints require typed refs { id, type }.
- Property requires key and contextId at creation.
- Repo update may temporarily be implemented as delete+create to normalize behavior.

## Alternatives Considered
- Single “Entity” concept (no separate Shape): rejected; conflates principle and instance.
- Concurrency on Shapes: rejected; violates Principle/Contained separation.
- Hard dependency on a specific repo API: rejected; harms portability.

## Migration Notes
- Older engine code may be replaced by schema-backed services.
- Normalize repository interface toward: create(doc), update(id, doc), get(id), delete(id).

## Testing Strategy
- Happy-path service suites for all pillars; schema tests for Shape with BaseState defaults awareness.
- Add a few negative-path tests (missing keys, bad refs) over time.

## References
- logic/src/schema/* — canonical Zod schemas
- logic/src/form/* — services over schemas
- logic/test/* — happy-path suites
