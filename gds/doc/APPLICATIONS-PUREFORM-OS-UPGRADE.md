# Applications -> PureForm OS Upgrade Blueprint

## Purpose

Upgrade the entire `applications` surface from facade-first routing into a PureForm-governed Component OS while preserving existing TS-JSON compatibility.

This is an architecture migration guide, not a breaking rewrite.

## Canonical Framing

- Layer: Agent + Kernel bridge
- Target: rationale coherence with executable mediation
- Invariants:
  - empirical adequacy: every component plan can be executed or explicitly validated
  - reflexive consistency: one semantic path from GDSL Program to runtime behavior
  - dialectical mediation: PureForm intent becomes Shell/Procedure movement plus evidence artifacts

## Current Reality (What Already Exists)

1. Form entry already exists through `form_eval` in TS-JSON.
2. Program/GivenForm semantics already exist in Form core.
3. `ApplicationForm` already acts like a component descriptor (name/domain/features/patterns/specifications).
4. Algorithm operations already bind through shell component identities.

This means the migration is mostly boundary and module reorganization, not greenfield semantics.

## Target Model

### 1) Entry Language

- IDE speaks OG Script (GDSL) as authoritative input.
- JSON remains transport IR only.

### 2) PureForm Program Authority

- Program/GivenForm is the only semantic container.
- `ApplicationForm` is treated as `PureFormComponent` conceptually.

### 3) Component OS Under Form

- Components are selected/resolved under Form control.
- Component resolution emits:
  - selected components
  - ProgramFeatures
  - shell-plan-ready operator sequence

### 4) Shell as Task Daemon

- Shell remains procedure-facing.
- Shell executes plans compiled by PureForm component resolution.
- Shell returns persisted mediation traces.

## Proposed Folder Evolution

###[A] New module layer in applications

Create a focused PureForm OS layer under `applications`:

- `applications/form_os/mod.rs`
- `applications/form_os/program_gateway.rs` (OG Script/JSON Program ingress)
- `applications/form_os/component_registry.rs` (PureFormComponent normalization)
- `applications/form_os/plan_compiler.rs` (Program -> shell-plan-ready op sequence)
- `applications/form_os/evidence.rs` (ProgramFeatures + traces + artifact links)

This layer orchestrates existing modules, it does not duplicate algorithm kernels.

###[B] Services become thin adapters

Keep `services` as transport adapters only:

- `tsjson_napi.rs` routes to `form_os` for form_eval and related program operations.
- graph_store, collections, and algorithms remain callable, but are invoked through Form-OS contracts when request type is Program/GivenForm.

###[C] Application module semantics

- `applications/algorithms/**` remain implementation handlers.
- `applications/collections/**` remain dataset operations.
- `applications/graph_store_catalog/**` remain storage/graph lifecycle operations.

Difference after migration: they are no longer conceptual entry authorities. PureForm Program is.

## Phase Plan

### Phase 1 (No Breakage)

1. Extract form_eval parsing and evaluation into `applications/form_os/program_gateway.rs`.
2. Introduce `PureFormComponent` aliases at boundary level (already partially done via request aliases).
3. Keep current response shape and add Form-OS proof fields in parallel.

Deliverable: same API behavior, cleaner architecture seam.

### Phase 2 (Semantic Consolidation)

1. Introduce internal `PureFormComponentSpec` type alias/wrapper around existing `ApplicationForm`.
2. Move component normalization and selected-component handling into `component_registry.rs`.
3. Standardize proof payload:
   - programForm
   - programFeatures
   - selectedComponents
   - shellPlanDraft

Deliverable: one internal semantic pipeline for all Program requests.

### Phase 3 (Daemonized Execution)

1. Introduce `plan_compiler` mapping ProgramFeatures/op patterns to shell component plans.
2. Hand plans to Shell/Procedure execution runtime.
3. Persist execution evidence references in `evidence.rs`.

Deliverable: PureForm compiles task daemon work.

### Phase 4 (OG Script-first)

1. Add OG Script parser gateway (or temporary script IR parser) in `program_gateway.rs`.
2. OG Script -> ProgramSpec normalization -> existing Form pipeline.
3. Keep JSON Program path as compatibility fallback.

Deliverable: IDE can speak OG Script directly.

## Minimal Interface Contracts

### Program Ingress

- Accept one of: `program`, `givenForm`, `givenForms`, `ogScript`.
- Normalize all paths into one `ProgramSpec`.

### Component Semantics

- Accept one of: `pureFormComponents`, `components`, `applicationForms`.
- Normalize into a single selected component list.

### Evidence Egress

- Always return ProgramFeatures.
- Return selectedForms + selectedComponents aliases until migration is complete.
- Attach shell-plan draft when available.

## Guardrails

1. Do not break existing facade operation names during migration.
2. Do not duplicate algorithm execution logic in Form-OS modules.
3. Keep services files transport-focused; move semantics into form_os.
4. Maintain library-scoped validation commands during each phase.

## First Mechanical Refactor (Recommended Next PR)

1. Add `applications/form_os/mod.rs` and `program_gateway.rs`.
2. Move from `services/tsjson_napi.rs`:
   - Program value extraction
   - ProgramSpec parsing
   - ProgramFeatures payload generation
3. Keep `handle_form_eval` signature unchanged; call new gateway functions.
4. Run focused tests plus `cargo +stable check -p gds --lib`.

This gives immediate structural clarity while preserving all current behavior.
