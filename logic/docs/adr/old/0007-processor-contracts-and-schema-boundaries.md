# ADR 0007: Processor Contracts and Schema Boundaries

Status: Accepted
Date: 2025-08-17

## Context
- We need durable boundaries for inputs and outputs that prioritize schema primacy.

## Decision
- Canonical boundary objects:
  - `ProcessorInputs` (schemas for shapes, entities, properties, contexts, morphs, relations, content, concepts, judgments, syllogisms).
  - `ProcessorSnapshot` (world + indexes + projections; details in code).
- Form wrappers can add ergonomics but must serialize back to schema for persistence and transport.

## Consequences
- Stability for integrations and tests; freedom to evolve internals.

## References
- `logic/src/absolute/core/contracts.ts`
- ADR 0001 (Absolute Form)
