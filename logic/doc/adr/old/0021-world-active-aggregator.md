---
adr: 0021
title: World — ActiveWorld Aggregator for Properties and Relations
status: proposed
date: 2025-08-17
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

To keep Reflect focused on determinations of reflection (ActiveContext), we need a locus for aggregating property signals and normalizing relations into a coherent world-view. The existing `world.ts` already assembles a world graph from entities and relations, but lacks an explicit ADR establishing its role as the ActiveWorld driver.

## Decision

- World is the engine-facing ActiveWorld aggregator.
- It normalizes Things (entities) into world nodes and Relations into edges, applying deduplication and stable ordering.
- Property-scoped analytics and any spectrum-like advisory metadata that is world- or relation-relevant are handled here, not in Reflect.

## Consequences

- Cleaner separation: Reflect (Context), World (aggregation), Relation (relational determinations).
- Tests can center on world assembly invariants (unique edges, stable ordering, horizon scoping) and optional global analytics (content indexing).

## Implementation notes

- Keep `assembleWorld()` and `indexContent()` as the primary API in `world.ts`.
- If we need property analytics at world scope, add small, deterministic helpers under `world.ts` or a sibling module.
- Maintain non-breaking signatures; attach any advisory metadata via optional `meta` fields.

## Migration

- No schema changes.
- Move property-derived appearance/spectrum logic out of Reflect as we stabilize world analytics.

## Related

- ADR 0020 — Reflect as ActiveContext driver
- ADR 0018 — ActiveShape
- ADR 0019 — ActiveEntity
