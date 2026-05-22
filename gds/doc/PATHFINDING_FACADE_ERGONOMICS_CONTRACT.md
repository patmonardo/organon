# Pathfinding Facade Ergonomics Contract

Status: Active
Scope: `gds/src/procedures/pathfinding/*.rs`
Purpose: Keep facade APIs readable, predictable, and integration-safe across model upgrades.

## 1) Public Facade Shape

Each facade should follow this order:

1. Type definition + alias (`*Facade` and `*Builder` alias when applicable)
2. Construction and config parsing:
   - `new(...)`
   - `from_spec_config(...)` and/or `from_spec_json(...)`
   - `with_spec_config(...)` where supported
3. Fluent setters/accessors
4. Progress/task registry setters (when facade supports progress customization)
5. Local `validate()` only when facade-owned invariants exist
6. Private `compute(...)`
7. Execution wrappers in this order:
   - `stream(self)`
   - `stats(self)`
   - `mutate(self, ...)`
   - `write(self, ...)`
8. `estimate_memory(...)`
9. Tests

## 2) Validation Rule

Preferred inline validation when facade does not own extra invariants:

```rust
self.config
    .validate()
    .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
```

Local facade `fn validate(&self)` is allowed only if it checks non-config facade state, extra invariants, or conversion boundaries.

## 3) Ownership Rule for Execution Wrappers

Public execution wrappers should consume the builder (`self`) for consistency across Pathfinding:

- `stream(self)`
- `stats(self)`
- `mutate(self, property_name: &str)`
- `write(self, property_name: &str)`

Rationale:

- One-shot execution semantics are explicit.
- Wrapper signatures stay uniform for TS-side orchestration and codegen.
- Reduces per-algorithm adapter branching.

## 4) Progress Registry Rule

When progress customization is exposed, support both fluent setters:

- `task_registry_factory(...)`
- `user_log_registry_factory(...)`

If not yet wired to concrete runtime behavior, placeholders are acceptable, but the facade API should remain consistent.

## 5) Memory Estimate Rule

Canonical Pathfinding shape:

- `estimate_memory(&self) -> MemoryRange`

Rationale:

- Pathfinding memory estimates are deterministic calculations over facade and graph metadata.
- The estimate path does not perform I/O or other fallible operations.
- A direct return removes unnecessary error branching in applications and TS adapters.

## 6) Architecture Guardrail

Facade layer remains procedure-first:

- Application/examples call `procedures` facades.
- Facades may call storage/computation runtimes.
- Application/examples do not call `algo` internals directly.

## 7) Review Checklist

Before merging a facade change:

1. Method order matches this contract.
2. Validation logic follows the rule above.
3. Wrapper ownership is `self`.
4. Progress/task setter consistency is preserved.
5. Focused tests for touched algorithm pass.
6. `cargo check -p gds` passes.

## 8) Execute Routing Rule

For application-layer execute handlers under `gds/src/applications/algorithms/pathfinding/*.rs`:

- Parse base request fields through `CommonRequest::parse(...)`.
- Route operation flow through `Mode` enum variants (`Mode::Stream`, `Mode::Stats`, `Mode::Estimate`, `Mode::Mutate`, `Mode::Write`).
- Do not hand-parse `mode`, `concurrency`, `submode`, or `graphName` ad hoc in individual handlers.

Rationale:

- Shell/Pipeline/Eval entrypoints depend on stable request semantics.
- Shared parsing keeps error messages and boundary checks consistent.
- Centralized routing reduces drift across per-algorithm handlers.
