# ADR 0004: Container–Contained and Particularity

Status: Accepted
Date: 2025-08-17

## Context
- Absolutes act as containers; particulars reference their Absolute via `particularityOf`.
- We need invariants and backprop hints to keep graphs sound.

## Decision
- Every essential relation must reference an Absolute container (`particularityOf`).
- Backprop hints: when absolutes change, particulars can re-evaluate via processor triggers.
- Absolute scoping: partitions indexes and projections by Absolute.

## Consequences
- Safer graph evolution and clearer world assembly.

## References
- `logic/src/absolute/essence/relation.ts` (helpers and invariants)
- ADR 0001 (Absolute Form)
