# ADR 0001 — Reorganize `absolute/` and introduce Module Registry

Status: Proposed
Date: 2025-08-16
Authors: Organon team

## Context
The `logic/src/absolute` area contains processors and heuristics that implement the "Relative Absolute" portion of the system. Current layout is inconsistent and ad-hoc, making discovery, testing, and runtime composition difficult. Processors need a lightweight, uniform way to self-describe and register with runtime systems (processors, pipelines, test harnesses).

## Decision
Introduce:
- A canonical folder layout for Absolute processors under `logic/src/processors/absolute/`.
- A small module metadata convention (`moduleInfo`) and optional `register(registry)` helper exported by processor modules.
- A runtime Registry API with minimal primitives to register and lookup processors.
- An ADR and migration plan before doing bulk file moves.

This ADR records the Registry shape, metadata conventions, and migration steps.

## Goals
- Make processors discoverable and composable.
- Standardize metadata so build tools and docs can enumerate processors.
- Allow runtime to register/unregister processors without modifying module internals.
- Preserve backward compatibility by supporting default exports (existing call sites continue to work).

## Non-Goals
- Enforce a specific DI framework or change runtime architecture beyond a minimal Registry API.
- Replace existing tests; instead provide migration guidance and small compatibility shims.

## Module metadata convention
Each processor module SHOULD export:
- `moduleInfo: { id: string; suggestedPath?: string; description?: string; tags?: string[] }`
- `register(registry?: any): boolean` — optional helper that calls `registry.registerProcessor(moduleInfo.id, processorFn)` and returns boolean.
- default export remains the main processor function (for backward compatibility).

Example shape (informational):
{
  id: "absolute.action",
  suggestedPath: "logic/src/processors/absolute/action.ts",
  description: "Influence action emitter for Absolute processor (heuristic).",
  tags: ["absolute","influence","heuristic"]
}

## Registry API (minimal)
The runtime Registry SHOULD implement:
- registerProcessor(id: string, fn: Function, meta?: Record<string, any>): boolean
- unregisterProcessor(id: string): boolean
- getProcessor(id: string): Function | undefined
- listProcessors(): Array<{ id: string; meta?: Record<string, any> }>

Registry is intentionally minimal to allow different runtime implementations (in-memory, service-backed, file-based).

## Folder layout (recommended)
- logic/src/processors/
  - absolute/
    - action.ts
    - being.ts
    - essence.ts
    - index.ts (barrel export + auto-discovery helpers)
  - relative/
  - common/

Keep `logic/src/absolute` in place until migration completes; provide compatibility barrel exports during transition.

## Migration plan
1. Create `logic/src/processors/absolute/` and move modules one-by-one.
2. Add `moduleInfo` and optional `register()` helper to modules during move.
3. Update barrels (e.g., `logic/src/processors/index.ts`) to export both old and new paths for a deprecation window.
4. Run and fix tests incrementally. Add small shims in old locations that re-export from new paths.
5. After a deprecation period, remove shims and old folders.

## Tests and verification
- Unit tests for each processor must still import via barrel path; add tests for `moduleInfo` presence.
- Add a small integration test that constructs an in-memory Registry, loads processors (require/import), calls their `register()` helper, and asserts listProcessors includes expected ids.
- Add CI job that runs the migration using the shims to ensure no regressions.

## Implementation hints
- Use NodeNext import semantics; keep default export for processor function.
- Keep metadata light and stable (avoid including environment-specific data).
- Provide `barrel` files that call `register()` on CI-only registry for static verification.

## Consequences
- Better discoverability and runtime composition.
- Slight churn during migration that requires careful CI and shims.
- A small API dependency introduced (Registry primitive) that core runtimes must implement.

## Next steps
- Review ADR with team.
- Prototype a minimal in-memory Registry under `logic/src/registry/in-memory.ts`.
- Move `action.ts` into `logic/src/processors/absolute/action.ts` and add `moduleInfo` + `register()` (follow the convention).
- Add integration test that loads and registers all processors.

