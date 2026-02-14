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
- Acts as a generic compute surface for REST/JSON, MCP, and agent-to-agent runtime calls.

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
- Cypher Knowledge Graph semantics are mapped into TS-JSON via Zod contracts; Rust consumes the projected contract for execution.

## GDSL/SDSL Embedding Doctrine

- GDSL/SDSL specifications define the epistemic pathway shared by Rust and TS toolchains.
- FormDB embedding is represented as specification metadata, not ad-hoc execution flags.
- Canonical business meaning may originate from Cypher FactStore/KnowledgeStore, while Rust holds an execution-ready projection.
- Existing `logic/model/task` schema structure is treated as non-arbitrary and must be referenced, not re-invented.

### Rust-first mock strategy (current phase)

- Keep runtime structs stable; avoid speculative deep type expansion.
- Encode FormDB/GDSL/SDSL embedding using namespaced metadata attributes.
- Preserve compatibility with partial schema definitions while allowing progressive refinement.
- Use provenance fields to mark:
  - specification kind (`gdsl`/`sdsl`),
  - schema reference,
  - Cypher graph bindings (fact/knowledge),
  - Cypher ontology bindings (labels/relationship types),
  - engine role and human-in-loop boundary.

## DataFrame/Dataset Orientation

- GDSL/SDSL is the epistemic path for DataFrame/Dataset work in both Rust and TS.
- RustScript is treated as a Polars client SDK (client-of-clients), not a replacement for TS/Zod contracts.
- Ontology structure (labels + relationship types) is first-class metadata for meta-analysis and editing by agents/humans.

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
