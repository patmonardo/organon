# ADR 0002: Namespaces and IDs

- Status: Accepted
- Date: 2025-09-05
- Owners: @pat

## Context
We need stable, parseable IDs across units/datasets/views.

## Decision
- Namespace root: reality:logos
- Unit IDs: reality:logos:{domain}:{slug}
  - domain ∈ {ys, sk, logic, datasets, views}
  - slug mirrors folder path, dot-joined.
- Dataset IDs: reality:logos:{name}
- View IDs: reality:logos:views:{name}

## Consequences
- Positive: unambiguous import/graph keys; easy adapters.
- Negative: rename cost during moves.
- Follow-ups: add parseId/makeId helpers in GDSL.