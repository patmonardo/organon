# ADR 0005: ActiveTruth and ActiveGround

Status: Accepted
Date: 2025-08-17

## Context
- EssentialRelation should carry epistemic state. We codify this as ActiveTruth.
- We also need a world-level grounding measure per Absolute (ActiveGround).

## Decision
- ActiveTruth on relations: `active`, `revoked`, `confidence`, optional `provenance`.
- Helpers:
  - `truthScore`, `isActiveTruth`, `assertActiveTruthInvariants`.
  - `groundScore`, `isActiveGround`, `chooseCanonicalTruth`, `assertActiveGroundInvariants`.
- Use thresholds (default 0.5) for activation; explicit `active=true` dominates; `revoked=true` negates.

## Consequences
- Unified semantics for truth evaluation and grounding across engines.
- Better explainability via provenance and invariant checks.

## References
- `logic/src/absolute/essence/relation.ts`
- ADR 0002 (Bridge)
