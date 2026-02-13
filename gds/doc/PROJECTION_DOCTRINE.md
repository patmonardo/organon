# Projection Doctrine

Status: Canonical runtime doctrine for FormDB execution in GDS.

## Purpose

Define the invariant architecture for Absolute/Relative knowing in the kernel runtime:

- Absolute operation: Projection.
- Relative operation: ProgramForm execution inside a projected world.
- Public facade operation: direct compute for external/non-FormDB usage.

## Core Principle

Projection is the single constitutive operation and unfolds as:

1. Codegen
2. Factory
3. Eval-context

ProgramForm execution is valid only as in-world interpretation under a pre-established projection.

## Runtime Ontology

### Absolute layer (RootAgent)

- Establishes world: `RootProjectionContext`.
- Declares projection identity and graph world.
- Defines projection pipeline path (`codegen`, `factory`, `eval`).

### Relative layer (SingleAgent)

- Enters established world through `ProgramFormSession`.
- Executes `Eval(Form) -> Apply(Form) -> Print` using spec-driven backend by default.
- Preserves provenance (`spec_binding`) for each executed pattern.

### Public facade layer

- Uses `DirectCompute` path.
- Valid for external utility APIs.
- Does not define FormDB epistemic semantics.

## Binding Doctrine

- Pattern resolution must bind to dedicated `AlgorithmSpec` where available.
- Unknown patterns may use generic fallback, but must still report explicit binding identity.
- Binding provenance is mandatory in execution artifacts.

## Execution Doctrine

- Agent/FormDB path MUST use `ExecuteSpec` (AlgorithmSpec `execute`).
- Public path MAY use `DirectCompute` (dispatcher `compute`).
- World identity (`projectionId`, `graphName`, `rootAgent`) must be carried into execution input.

## Contract Doctrine (Rust ↔ TS)

- Rust runtime is authoritative for execution semantics.
- TS/Zod is authoritative for human-facing schema contract design.
- JSON-safe Form contract (`FormContract`) is the bridge surface.
- Any contract evolution must preserve compatibility rules or explicitly version-break.

## Architectural Invariants (PR Checklist)

- [ ] Does this change preserve `Codegen -> Factory -> Eval` ordering?
- [ ] Is projection world established before ProgramForm execution?
- [ ] Is Agent path still spec-driven (`ExecuteSpec`)?
- [ ] Is public path still isolated as facade (`DirectCompute`)?
- [ ] Are pattern executions annotated with `spec_binding`?
- [ ] Are Root/Single agent responsibilities still separated?
- [ ] If contract fields changed, are Rust and TS/Zod updated together?

## Current Reference Types

- `RootProjectionContext`
- `ProgramFormSession`
- `ProgramFormApi`
- `ProgramFormApplyBackend`
- `FormContract`

## Short Formula

One projection, many interpretations, one provenance discipline.
