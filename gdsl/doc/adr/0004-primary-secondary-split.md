# ADR 0004: Primary vs Secondary split

- Status: Accepted
- Date: 2025-09-05
- Owners: @pat

## Context
Primary units (YS/SK/LOGIC) vs secondary bundles/views (crosswalks, bridges).

## Decision
- Primary: DatasetUnit in REALITY/LOGIC (via GDSL types).
- Secondary: Dataset (bundles) and View (terms/edges) live as GDSL-managed artifacts.

## Consequences
- Positive: preserves source purity; keeps bridges modular.
- Negative: more artifacts to track.
- Follow-ups: dataset/view validators and GraphStore adapters.