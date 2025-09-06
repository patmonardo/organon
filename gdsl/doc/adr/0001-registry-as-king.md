# ADR 0001: GDSL Registry as the single source of truth

- Status: Accepted
- Date: 2025-09-05
- Owners: @pat

## Context
Authoring lives in LOGIC/REALITY; schema drift and multi-agent curation require one governing contract.

## Decision
- GDSL provides canonical types, IDs, validators, policies (speculative/curative), and adapters.
- REALITY/LOGIC consume Registry types; no direct schema in those packages.

## Consequences
- Positive: single contract; safer codegen; easier refactors.
- Negative: central dependency; needs versioning.
- Follow-ups: add REGISTRY_VERSION; changelog; CI validation.

## References
- Concept: GDSL as “Registry provider to Repositories”.
