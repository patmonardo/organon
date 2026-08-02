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

###[A] Form service layer in applications

The focused PureForm OS layer now lives under `applications/form`:

- `applications/form/program_gateway.rs` normalizes Program/GivenForm ingress.
- `applications/form/capability_source.rs` projects ApplicationForms and ProgramFeatures into a replaceable capability snapshot.
- `applications/form/service_manifest.rs` defines the typed advertised-machine contract.
- `applications/form/service_registry.rs` activates machines from the compiled Program.
- `applications/form/evidence.rs` composes Program, feature, service, and artifact evidence.

This layer orchestrates existing modules, it does not duplicate algorithm kernels.

## Form Capability Source

Form is the root capability authority. The current `InMemoryDatasetCapabilitySource` projects the
Program's complete ApplicationForm set into `proof.formCapabilities`, including declared features,
operator patterns, specifications, selection state, and associated canonical ProgramFeatures.

This source is an explicit Dataset mock:

- `source.kind` is `dataset_mock`.
- `source.persistent` is `false`.
- `source.semantics` is `in_memory_program_snapshot`.

The trait boundary is intended to admit a real Dataset catalog source later without changing the
Form evidence contract. This slice does not claim persisted capability learning.

## Advertised Form Service Machines

The Form Server currently advertises five virtual operating machines:

1. `form.program`

- Always active for normalized Program/GivenForm requests.
- Executes through `ProgramForm/ExecuteSpec`.
- Reports `actual` because this path is invoked today.

2. `form.algorithms`

- Activates from algorithm operator patterns or an explicit `serviceId`.
- Resolves canonical Shell component descriptors and supported modes.
- Reports `bindable` when descriptors resolve and `planned` when they do not.

3. `form.datasets`

- Activates from dataset, dataframe, or collections operator patterns, or `serviceId`.
- Reports `planned` until the Dataset compilation runtime is invoked through this path.

4. `form.shell`

- Activates from shell/task/procedure patterns, algorithm or dataset activation, or `serviceId`.
- Declares long-lived daemon runtime policy (`daemonRuntime`) for supervision and heartbeat.
- Reports `planned` until Shell task-daemon execution is invoked through this path.

5. `form.recursion`

- Activates from recursion/recursive/triadic operator patterns, or `serviceId`.
- Declares triadic recursive runtime policy for iterative checkpointed mediation.
- Reports `planned` until recursive Form-cycle execution is invoked through this path.

`serviceManifest.unresolvedPatterns` preserves algorithm intent that has no registered Shell
component instead of suppressing it or falsely reporting execution.

###[B] Services become thin adapters

Keep `services` as transport adapters only:

- `tsjson_napi.rs` routes to `applications/form` for form_eval and related program operations.
- graph_store, collections, and algorithms remain callable, but are invoked through Form-OS contracts when request type is Program/GivenForm.

###[C] Application module semantics

- `applications/algorithms/**` remain implementation handlers.
- `applications/collections/**` remain dataset operations.
- `applications/graph_store_catalog/**` remain storage/graph lifecycle operations.

Difference after migration: they are no longer conceptual entry authorities. PureForm Program is.

## Phase Plan

### Phase 1 (No Breakage)

1. Extract form_eval parsing and evaluation into `applications/form/program_gateway.rs`.
2. Introduce `PureFormComponent` aliases at boundary level (already partially done via request aliases).
3. Keep current response shape and add Form-OS proof fields in parallel.

Status: implemented, including `serviceManifest` as an additive proof field.

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
- Return `formCapabilities` as the Dataset-shaped account of what all ApplicationForms declare and
  which forms are selected for the current Program.
- Return selectedForms + selectedComponents aliases until migration is complete.
- Return a typed serviceManifest with activation, runtime binding, execution state, resolved
  Shell components, unresolved patterns, and optional daemon runtime profile.
- Attach shell-plan draft when available.

## Guardrails

1. Do not break existing facade operation names during migration.
2. Do not duplicate algorithm execution logic in Form-OS modules.
3. Keep services files transport-focused; move semantics into `applications/form`.
4. Maintain library-scoped validation commands during each phase.

## Current Implementation Boundary

The gateway and service registry determine what a Program means and which machine is active.
`ProgramFormApi` remains the actual evaluator. Shell descriptors establish bindability but do not
claim Shell execution. Dataset activation remains planned until Phase 3 supplies the daemonized
runtime handoff and persisted mediation evidence.

Real Dataset catalog reads, capability persistence, execution-feedback updates, and capability
gating are deliberately deferred. The current mock establishes their replaceable semantic boundary
without representing in-memory declarations as learned knowledge.
