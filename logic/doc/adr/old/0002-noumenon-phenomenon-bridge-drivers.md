# ADR 0002: Noumenon–Phenomenon Bridge (Drivers)

Status: Accepted
Date: 2025-08-17

## Context
- Drivers are the noumenal artifacts that encode semantics for the transactional engines.
- In the repo layout:
  - `logic/src/absolute/essence` contains Drivers (noumenal definitions following Hegel's terminology: Essence, Reflection, Ground → Thing, World, Relation). It supplies drivers for two complementary triads used across the system:
    - Form/Shape triad: Shape, Context, Morph (how forms are structured semantically)
    - Carrier triad: Entity, Property, Morph/Relation (how content is carried and grounded)
  - `logic/src/essence` contains the transactional Engines (the phenomena): engines that operate on the phenomenal carriers (Entity, Property, Relation) and perform commits.

## Decision
- Both Driver names (Shape, Context, Morph) and Carrier names (Entity, Property, Relation) are noumenal descriptors within the Absolute/Essence vocabulary: they name aspects of structure and meaning rather than implying a strict left/right ontological split.

- `logic/src/absolute/essence` holds the Drivers: 
    canonical noumenal definitions and semantics for both sides of the bridge:
  - Container side: Shape, Context, Morph — corresponding to Essence, Reflection, Ground.
  - Contained side: Entity, Property, Relation — corresponding to Thing, World, Relation.
- `logic/src/essence` contains the transactional Engines. These engines execute processes and perform commits; they realize the semantics declared by Drivers

- Active objects are the explicit bridges between the Container side (non-transactional drivers) and the Contained side (transactional carriers):

  - Container (non-transactional, unity/drivers): ActiveShape, ActiveContext, ActiveMorph — these are noumenal unities that name the trinitarian moments (begin/middle/end) of form and semantics.
  - Contained (transactional carriers/committers): ActiveEntity, ActiveProperty, ActiveRelation — these are the objects engines under `logic/src/essence` operate on and commit.

  Mapping (driver → carrier):

  - Absolute(Essence) -> ActiveShape  → ShapeEngine
  - Absolute(Reflect) -> ActiveContext  → ContextEngine
  - Absolute(Ground) -> ActiveMorph  → MorphEngine
  - Absolute(Thing) -> ActiveEntity  → EntityEngine
  - Absolute(World) -> ActiveProperty  → PropertyEngine
  - Absolute(Relation) -> ActiveRelation → RelationEngine

Additional note:
-- `logic/src/absolute` exposes six Active Objects used as the bridge vocabulary and drivers: ActiveShape, ActiveContext, ActiveMorph, ActiveEntity, ActiveProperty, ActiveRelation. These are first-class noumenal artifacts that engines under `logic/src/essence` consume to perform transactional work.

## Consequences
- Processors and UIs can reason about activation explicitly at the bridge points.
- Clear handoff: noumenal semantics push, phenomenal forms enact.

## References
- ADR 0001 (Absolute Form)
- ADR 0005 (ActiveTruth & ActiveGround)
