# ADR 0006: Kriya Orchestration and Root Boundaries

Status: Accepted
Date: 2025-08-17

## Context
- We need a standard orchestration path and clear boundaries between non-transactional root and transactional engines.

## Decision
- Kriya pipeline (indicative): Seed → Contextualize → Reflect → Ground → Action → Model → Control → Plan.
- Root (`absolute/core/engine.ts`) remains non-transactional; transactional triads commit within Essence/World engines.
- Actions are emitted as decisions/side-effects, not direct mutations at the root.

## Consequences
- Predictable execution model; easy to test and extend.

## References
- `logic/src/absolute/core/kriya.ts`
- `logic/src/absolute/core/engine.ts`
