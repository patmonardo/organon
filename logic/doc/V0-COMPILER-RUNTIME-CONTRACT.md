# V0 Compiler-Runtime Contract

Status: normative implementation scope for the basic Organon compiler-runtime system.

## Purpose

Define the minimal, buildable contract between:

- Language Definitions (spec-oriented forms)
- Compiler normalization
- Runtime Definitions (Form Engine execution shapes)
- Persistence (Fact/Relation repository strategies)

This document is intentionally goal-oriented and excludes full Beginning/End closure.

## In Scope (V0)

1. Essence-layer compiler-runtime loop.
2. Form Engine orchestration and persistence control.
3. Normalization from definitional forms to runtime shapes.
4. Fact/Relation persistence semantics with repository strategy boundaries.
5. Validation and traceability required for runtime correctness.

## Out of Scope (V0)

1. Full Being token/tag/parser architecture as mandatory runtime dependency.
2. Full Pure Concept / UPS closure as executable requirement.
3. Absolute end-state synthesis as a blocking compiler condition.
4. Broad interop completeness across all external standards.

## Canonical Artifacts

- Language Definitions: `logic/src/schema/gdsl-definitional-forms.ts`
- Runtime Definitions: `logic/src/schema/root-shapes.ts`
- Definitional Spec: `logic/doc/ORGANON-DEFINITIONAL-LAWS.md`

## Layer Contract (V0)

- Form Engines are invoke/orchestrate/persist controllers.
- Repositories are internal dependencies of Form Engines.
- Compiler enforces semantic normalization and consistency.
- Runtime schemas remain Essence-oriented and execution-ready.

## Required Flow

1. Parse/author definitional forms package.
2. Normalize forms into runtime-compatible Root Principle/Law shapes.
3. Execute through Form Engine procedures.
4. Persist through Fact/Relation repository strategies.
5. Emit validation/provenance traces.

## Minimal Admission Rules

- At least one Entity Definition is required.
- Property/Aspect references must resolve to declared Entity Definitions.
- Rule/Constraint targets must resolve to known definition ids when specified.
- Runtime persistence state must declare target repository when required.

## Runtime State Semantics (V0)

- `resolved` -> supports Property-level determination.
- `grounded` -> supports Aspect-level relational coherence.
- `persisted` -> requires explicit persistence target strategy.

## Principle-Law Invariants (Normative)

### PrincipleGateInvariant

The compiler MUST reject candidates that violate Root Principle admissibility
before Law realization.

- Scope: Principle chain (`form -> context -> morph`).
- Condition: unresolved contradiction at Principle stage.
- Result: no transition into Law generation.

### LawRealizationInvariant

The runtime/compiler MUST generate and validate Law-level determinations only
from Principle-admitted inputs.

- Scope: Law chain (`entity -> property -> aspect`).
- Condition: Principle admission passed.
- Result: Property and Aspect realization is permitted.

### PrincipleLawConformanceInvariant

Persistence MUST enforce coherence between Principle and Law stages.

- Scope: Form Engine persistence contract.
- Condition: Law output references and stage correspondence are valid relative
  to Principle lineage.
- Result: candidate is eligible for persistence (`fact-repo` / `relation-repo`
  / `knowledge-repo` strategy as applicable).

## Boundary Rules

- Language layer may evolve and expand forms.
- Runtime layer accepts only normalized, execution-safe shapes.
- Being-oriented parser details can feed normalization but do not define runtime contracts.
- Concept closure remains a planned extension, not a V0 gate.

## V0 Done Criteria

V0 is complete when all are true:

1. Compiler accepts definitional package and emits valid runtime shapes.
2. Form Engine executes those shapes through procedure-first orchestration.
3. Fact/Relation persistence paths are exercised and validated.
4. Build/test pipeline verifies schema integrity and basic runtime flow.

## Next After V0

- Add optional Being ingestion profile (tokens/tags/parsers) as upstream adapter.
- Add optional Concept-layer uplift profile (UPS-oriented synthesis).
- Expand projection adapters for OWL/SHACL/SPARQL/SPIN/SKOS interoperability.
