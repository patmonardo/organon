# ADR 0003: Speculative vs Curative policies

- Status: Accepted
- Date: 2025-09-05
- Owners: @pat

## Context
Multi-agent authoring needs creative codegen (Speculative) and strict curation (Curative).

## Decision
- Policy ∈ {speculative, curative} is attached to validators, compilers, and provenance.
- Speculative: relaxed rules; emit IR/docs for feedback.
- Curative: strict rules; emit Essence/Form for runtime.

## Consequences
- Positive: safe exploration; controlled publication.
- Negative: dual-path complexity.
- Follow-ups: add BuildProfiles and Manifests.