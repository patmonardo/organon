# ADR 0011: Top-Level Components of @gdsl Process (Updated)

- Status: Proposed
- Date: 2025-09-06
- Owners: @pat

## Context
@GDSL treats everything as a Dataset (Graph). HLOs are recast as High Level Operators (HLOps) for general clausal operations.

## Decision
Define @gdsl as a staged pipeline with the following top-level components:

1. **Parsing Stage**: Raw text → Chunks + Clauses (ADT).
   - Input: Manifest assets (TS/JSON).
   - Output: Parsed AST (chunks, clauses).
   - Components: clause-parser.ts (generalized to Clause ADT, not HLO-specific).

2. **Validation Stage**: AST → Validated Dataset.
   - Input: Parsed AST.
   - Output: Validation report (issues, ok/not ok).
   - Components: validate.ts (extend for Clause ADT).

3. **Canonicalization Stage**: Validated AST → Graph Dataset.
   - Input: Validated AST.
   - Output: Canonical graph (nodes/edges).
   - Components: canonicalize-dataset.ts (from ingest rework).

4. **Evaluation Stage**: Graph Dataset → Evaluated Results.
   - Input: Canonical graph + logic programs.
   - Output: Evaluated assertions/judgments.
   - Components: eval/evaluator.ts (core EVAL).

5. **Projection Stage**: Evaluated Graph → Specialized View via HLOps.
   - HLOps: Generalized ADT for high-level operations (assert, tag, project).
   - Components: project-dataset.ts with HLOps interface.

Stages are composable via CLI nouns (dataset, eval, project).

## Consequences
- + Clear, staged architecture for automation.
- + Projection as absolute: Enables subproject-specific transformations.
- + Generalized Clauses: Clause ADT supports any relational assertions, not just HLOs.
- - Requires refactoring HLO-specific code to Clause ADT.
- + HLOps enable general clausal language across subprojects.
- + Datasets remain the core unit.

## References
- gdsl/src/analysis/ (parsing/validation).
- gdsl/src/eval/ (evaluation).
- gdsl/src/adapters/canonicalize-dataset.ts (canonicalization).
- New: gdsl/src/adapters/project-dataset.ts (projection ADT).
- gdsl/src/schema/types.ts (add HLOp interface).
