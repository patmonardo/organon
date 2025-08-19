---
adr: 0020
title: Reflect — Determinations of Reflection as ActiveContext Driver
status: proposed
date: 2025-08-17
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

We want the Absolute layer to mirror Hegelian terminology precisely. In this alignment, Reflect should model the "determinations of reflection" and operate over Context, treating Context as the artifact of reflective consciousness. Current `reflect.ts` handles multiple concerns (thing/property signatures, spectrum hints), which dilutes its principle role and overlaps with World/Relation responsibilities.

## Decision

- Reflect is the engine-facing ActiveContext driver.
- Its principle output is a small set of determinations of reflection over Context (positing, external, determining, evidence) — minimal, deterministic, and context-focused.
- Property- and Relation-oriented derivations (including world-scale synthesizing signals) move to World/Relation drivers. Reflect will not be the locus for Relation synthesis.
- Spectrum/appearance remains allowable as advisory metadata, but any property-scoped analytics should be shifted to World as needed.

## Consequences

- Clear separation of concerns:
  - Reflect: Context-focused determinations of reflection (ActiveContext).
  - World: Aggregation/synthesis across properties/relations (ActiveWorld).
  - Relation: Relational determinations and absolute/particular synthesis.
- Reduced coupling and simpler tests for each driver.

## Implementation notes

- Stage 1 (non-breaking):
  - Add an ActiveContext shape and a `determineContext(...)` helper exported from `reflect.ts` alongside the existing API.
  - Annotate `reflect.ts` with ADR references and mark property-facet work as slated for relocation.
- Stage 2 (progressive narrowing):
  - Move property facet/signature work into World/Relation drivers.
  - Keep `reflectStage` for compatibility while internal focus is on Context.
- Stage 3 (cleanup):
  - Trim deprecated exports and consolidate tests around ActiveContext.

## Migration

- No schema changes.
- Engine tests to be split: context-determination tests remain under reflect; property/relation synthesis tests move under world/relation.

## Related

- ADR 0018 — Shape as Active Essence (ActiveShape)
- ADR 0019 — Entity as Active Thing (ActiveEntity)
- ADR 0021 — World as ActiveWorld aggregator
