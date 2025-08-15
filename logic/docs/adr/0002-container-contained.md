# 0002 — Container vs. Contained: Context/Morph/Shape as Principles; Entity/Property/Relation as Essence

- Status: Accepted
- Date: 2025-08-14
- Owners: @organon/logic

## Context
We refine the Form Theory split:
- Principles (Pure Being / Container Logic): Shape, Context, Morph.
- Essence (Contained): Entity, Property, Relation.

Concurrency and mutation belong to the Contained. Principles shape and constrain but do not themselves participate in concurrent mutation.

## Decision
- Treat Shape, Context, Morph as principle-level constructs (no direct concurrency semantics).
- Treat Entity, Property, Relation as essence-level constructs (mutable, versioned, evented).
- Continue to enforce invariants at schema level; services wrap schemas with evented verbs.
- Concurrency policies apply only to E/P/R.

## Rationale
This clarifies responsibilities and aligns mutation with experienced content (Essence), while keeping Principles simple and compositional.

## Consequences
- Tests and events focus on E/P/R for stateful behaviors.
- Context and Morph services remain thin; future work may add principled composition without introducing concurrency.
- Processor work will respect the Container/Contained split.

## Open Questions
- Elaborate “Essence / Reflection / Ground” as qualitative logic framing for E/P/R.
- Map how Morph (Principle) induces Relations (Essence) without becoming concurrent itself.

## References
- ADR-0001 (Shape as Principle; Entity as Essence)
- logic/src/schema/*, logic/src/form/*
