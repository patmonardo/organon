# Form Server Operational Management Plan

## Status and Purpose

This document defines the next development stage for the Organon Form Server.
It begins from the present repository rather than from an idealized replacement
architecture.

The FormVM is the top-level management authority for linked executable Forms.
It does not replace GraphFrame, TaskFrame, Shell, Dataset, or their established
execution machinery. It links their capabilities into one executable Form,
governs the lifecycle of that whole, and reports its determinate results.

This is a Kernel and Agent bridge plan.

- Primary target: rationale coherence.
- Operational target: empirical adequacy through observable execution.
- Control target: conceptual validity across Form definitions, linked artifacts,
  active runs, and returned evidence.

## Governing Thesis

A Form Server is not primarily an application dispatcher. It is an operational
management system for Forms.

Its canonical movement is:

```text
GDSL / GivenForm
    -> PureForm principle
    -> ApplicationForm service and feature configuration
    -> LinkedExecutableForm
    -> managed FormVM run
    -> GraphFrame determination
    -> TaskFrame constitution
    -> Shell execution
    -> Dataset evidence
    -> internal Eval/Form control
    -> returned Form
```

PureForm supplies the invariant principle and constraints. ApplicationForm
supplies available services, features, and bindings. Linking reconciles those
moments into a determinate executable artifact. Runtime execution tests that
artifact against empirical conditions. The returned evidence provides the basis
for Agent judgment, revision, or continuation.

## Present Reality

The repository already contains substantial execution infrastructure.

### Form facilities

- `ProgramSpec` carries the current GivenForm/compiler boundary.
- `ProgramFeatures` articulates program meaning before runtime compilation.
- `ApplicationForm` declares application capabilities and operator patterns.
- `FormLinker` lowers Program patterns into typed `FormVmOperation` values.
- `LinkedExecutableForm` preserves the linked principle, features, operations,
  and link report.
- `FormVm` records lifecycle events, outcomes, faults, and evidence references.
- Eval/Form currently coordinates linking, preparation, application, and return.

### Existing operational facilities

- GraphFrame owns graph selection, graph semantics, and
  `GraphExecutionIntent`.
- `GraphAgentProcessingContract` already combines compiled GraphForm semantics,
  TaskFrame, empirical outflows, synthetic moments, and GDSL transmission.
- TaskFrame owns workflow constitution, stages, resources, monitoring policy,
  return contracts, and TaskJob construction.
- `GraphTaskDaemonSubmission` already carries GraphFrame work toward the task
  daemon.
- Shell owns procedure execution and component invocation.
- Dataset and GraphStore facilities own empirical artifacts and durable graph
  state.
- The Form application layer already exposes capability snapshots, service
  manifests, evidence composition, and transport gateways.

The principal deficiency is therefore not an absence of execution machinery.
It is the absence of one durable management path that links these facilities,
controls a run, correlates their identities, and returns a coherent report.

## Architectural Decision

The FormVM shall manage the executable whole while delegating differentiated
work to the systems that already own it.

### Form Server owns

- Form definition identity and version.
- linked executable identity and version.
- run identity and lifecycle state.
- capability and service resolution.
- control commands and authorization.
- correlation of GraphFrame, TaskFrame, Shell, and evidence identities.
- contradiction and fault reporting.
- terminal Form outcome and return evidence.

### GraphFrame owns

- graph-view semantics.
- graph selection and orientation.
- graph procedure planning.
- `GraphExecutionIntent`.
- `GraphAgentProcessingContract`.
- empirical outflow declarations.

### TaskFrame owns

- workflow constitution.
- stage and resource definitions.
- monitoring policy.
- TaskSpec and TaskJob creation.
- task-local state transitions.

### Shell and task daemon own

- scheduling and invocation.
- cancellation, retries, and supervision.
- operator execution.
- task and component receipts.

### Dataset and GraphStore own

- graph and dataset state.
- persisted execution artifacts.
- empirical evidence bodies.
- stable references to those bodies.

### Eval/Form owns

- the internal Agent/GDSL control point.
- readiness judgment before execution.
- comparison of returned evidence with the Form return contract.
- the decision to accept, revise, retry, or continue a Plan.

Eval/Form is not a public peer service alongside GraphFrame or TaskFrame.

## Canonical Managed Objects

The Form Server shall distinguish three persistent identities and one runtime
projection.

### FormDefinition

The normalized semantic input, whether authored as GDSL, received as GivenForm,
or carried temporarily as JSON `ProgramSpec`.

### LinkedExecutableForm

An immutable and reproducible artifact containing:

- PureForm principle.
- selected ApplicationForms.
- ProgramFeatures.
- typed FormVM operations.
- service and capability bindings.
- evidence and return contracts.
- link report and deferred compatibility obligations.

### ManagedFormRun

The operational record of one execution attempt:

```rust
pub struct ManagedFormRun {
    pub run_id: FormRunId,
    pub linked_form_id: FormId,
    pub lifecycle: FormVmLifecycle,
    pub control_state: FormControlState,
    pub graph_contracts: Vec<GraphContractRef>,
    pub task_jobs: Vec<TaskJobRef>,
    pub evidence: Vec<FormVmEvidenceRef>,
    pub outcome: FormVmOutcome,
}
```

The exact Rust types may evolve. The distinctions between definition, linked
artifact, and run must not collapse.

### FormRunReport

A client-facing, serializable projection of the managed run. It reports facts
and reasons but does not expose live runtime objects.

## Required Management Operations

The Form Server should converge on the following control surface:

```text
define
link
inspect_definition
inspect_link
start
inspect_run
suspend
resume
cancel
collect_evidence
return
```

The first delivery does not need to implement every command. Types and state
transitions should reserve their meanings so later supervision is additive
rather than disruptive.

## Reporting Standard

Reports are contracts for rational agents. They must use complete, precise
English and typed data together.

Every material report should answer:

1. What Form was requested?
2. What principle and constraints governed it?
3. Which services and features were selected?
4. Which bindings were established, deferred, or rejected?
5. What executable artifact was produced?
6. Which graph intent was determined?
7. Which tasks were constituted and executed?
8. What empirical evidence was produced?
9. Which contradictions or faults occurred?
10. Why does the final outcome count as succeeded, failed, canceled, or
    unresolved?

Reports must distinguish:

- declaration from availability.
- availability from successful linking.
- successful linking from execution.
- execution from empirical success.
- empirical success from satisfaction of the Form return contract.
- application failure from FormVM infrastructure fault.
- deferred compatibility from silently ignored intent.

Machine fields should use stable vocabulary. Human explanations should be
grammatical statements rather than fragments assembled from enum names.

## Consolidation Requirement

The current `applications/form/runtime.rs` is a useful vertical bridge. It proves
that typed `DetermineGraph` and `ConstituteTask` operations can create a real
`GraphExecutionIntent` and `TaskFrame`.

It must now be consolidated with the existing GraphFrame processing path.

The permanent route should be:

```text
FormVmOperation::DetermineGraph
    -> GraphFrame adapter
    -> GraphAgentProcessingContract

FormVmOperation::ConstituteTask
    -> TaskFrame already carried by that contract

FormVmOperation::ExecuteTask
    -> GraphTaskDaemonSubmission
    -> TaskJob / Shell runtime
```

The Form layer must not maintain a second implementation of GraphFrame planning
or TaskFrame construction. It may translate inputs, call the owning API, retain
references, and interpret returned receipts.

Likewise:

- `FormLinkReport` should incorporate service-registry resolution rather than
  contradict `serviceManifest`.
- FormVM lifecycle evidence should correlate TaskFrame and Shell receipts rather
  than duplicate their internal monitoring models.
- `LinkedExecutableForm` may aggregate existing artifacts but should not create
  competing definitions of ProgramFeatures or execution plans.

## Delivery Stages

### Stage 1: Stabilize the FormVM language

Goal: make the linked instruction stream the single runtime language.

Work:

1. Define the complete initial `FormVmOperationKind` taxonomy.
2. Distinguish linking operations, runtime operations, control operations, and
   return operations.
3. Give every operation ProgramFeature and source-Form provenance.
4. Replace raw pattern interpretation in runtime paths with typed operations.
5. Retain raw patterns only as source evidence and compatibility transport.
6. Define explicit contradiction types for missing or incompatible bindings.

Acceptance:

- no recognized Form operation is silently skipped.
- operation order respects declared dependencies.
- every linked operation identifies its source feature and binding rationale.
- repeated linking of the same normalized definition produces the same identity.

### Stage 2: Consolidate GraphFrame and TaskFrame mediation

Goal: delegate to the existing graph-agent processing contract.

Work:

1. Replace direct GraphFrame/TaskFrame construction in the provisional runtime
   adapter with a `GraphFormRuntimeProvider` boundary.
2. Implement the provider using GraphFrame's existing compilation APIs.
3. Return `GraphAgentProcessingContract` or a stable reference projection.
4. Obtain TaskFrame from that contract rather than constituting it twice.
5. Compile `GraphTaskDaemonSubmission` without executing it during linking.
6. Record graph and task identities in the FormVM lifecycle.

Acceptance:

- one GraphFrame compilation path exists.
- one TaskFrame constitution path exists.
- FormVM reports graph and task references without owning their internal state.
- graph determination can fail independently from task constitution.

### Stage 3: Introduce managed runs

Goal: establish the operational-management center of the Form Server.

Work:

1. Add a `FormRunRegistry` trait.
2. Provide an in-memory implementation for initial development.
3. Replace process-local incidental identity with injectable `FormRunId`
   generation.
4. Store lifecycle snapshots, linked-form identity, task references, evidence
   references, and terminal outcome.
5. Implement `start` and `inspect_run` first.
6. Reserve valid state transitions for suspend, resume, and cancel.

Acceptance:

- every run can be inspected independently of the initiating request.
- linked artifacts remain immutable across runs.
- invalid control transitions are rejected and reported.
- run reports survive transport serialization without live-object leakage.

### Stage 4: Submit constituted work

Goal: execute through existing TaskFrame and Shell authority.

Work:

1. Add a `FormTaskRuntime` provider interface.
2. Submit `GraphTaskDaemonSubmission` through the existing daemon path.
3. Correlate TaskJob identity with Form run identity.
4. Translate task state changes into FormVM lifecycle observations.
5. Route cancel and later suspend/resume commands to the task owner.
6. Preserve the current Apply backend as an explicit compatibility provider
   until its operations are migrated.

Acceptance:

- FormVM does not call algorithm implementations directly.
- TaskFrame and Shell remain execution authorities.
- every execution result includes task and component provenance.
- compatibility execution is clearly labeled in reports.

### Stage 5: Close the evidence and return cycle

Goal: make evidence constitutive of Form completion.

Work:

1. Define `FormEvidenceContract` and `FormReturnContract` projections.
2. Store evidence bodies in their owning Dataset or GraphStore facilities.
3. Store only stable evidence references in managed runs.
4. Distinguish operator failure, task failure, canceled work, contract failure,
   and FormVM fault.
5. Pass the completed evidence projection to internal Eval/Form.
6. Produce a final English and typed `FormRunReport`.

Acceptance:

- a run cannot report success solely because an operator returned without error.
- success requires satisfaction of the declared return contract.
- contradictions remain visible in evidence and explanation.
- Agent planning can determine why it should accept, revise, or retry.

### Stage 6: Establish the Form Server facade

Goal: expose management operations through thin transport adapters.

Work:

1. Add the Form Server service over the definition, linked-form, and run
   registries.
2. Keep TS-JSON/N-API handlers transport-focused.
3. Add GDSL-first ingress when its parser boundary is ready.
4. Preserve JSON `ProgramSpec` as compatibility IR.
5. Publish stable report schemas for Agent clients.

Acceptance:

- transport code contains no graph, task, or algorithm semantics.
- GDSL and compatibility JSON use the same linker and run manager.
- clients can define, link, start, inspect, and receive a returned Form through
  one coherent authority.

## Immediate Work Package

The next implementation package should be limited to consolidation and managed
identity.

1. Add explicit `ExecuteTask` and management operation distinctions to the
   FormVM language.
2. Introduce `GraphFormRuntimeProvider`.
3. Adapt it to `GraphAgentProcessingContract` and
   `GraphTaskDaemonSubmission`.
4. Remove duplicate TaskFrame constitution from the provisional adapter.
5. Introduce `ManagedFormRun` and an in-memory `FormRunRegistry`.
6. Return a stable `FormRunReport` containing linked, graph, task, lifecycle,
   and evidence references.
7. Keep actual daemon submission behind an interface until run-state ownership
   and cancellation semantics are explicit.

This package establishes the management plane without prematurely absorbing the
task daemon or persistence layer.

## Proof Obligations

Every stage must preserve the following invariants.

### Empirical adequacy

- linked operations have executable or explicitly deferred bindings.
- runtime claims are supported by receipts or stable evidence references.
- tests cover successful, failed, missing-resource, and invalid-transition paths.

### Reflexive consistency

- the GDSL runtime is itself represented as a linked executable Form.
- reports can explain every runtime operation from its principle, ProgramFeature,
  and selected ApplicationForm.
- FormVM management does not conceal execution delegated to another authority.

### Dialectical mediation

- PureForm constraints and ApplicationForm capabilities are reconciled through
  explicit linking.
- runtime contradiction produces refinement evidence rather than silent
  suppression.
- empirical results return to Agent judgment without being mistaken for their
  own rationale.

## Completion Criterion

The Form Server reaches its first coherent operational form when a rational
Agent can:

1. submit a GivenForm or GDSL definition.
2. inspect the resulting linked executable and its reasons.
3. start a separately identified managed run.
4. observe the GraphFrame, TaskFrame, and Shell work associated with that run.
5. issue a valid control command.
6. inspect stable empirical evidence references.
7. receive a report explaining whether the Form satisfied its return contract
   and why.

At that point the FormVM will be neither a decorative top-level abstraction nor
a duplicated execution engine. It will be the operational unity through which
the differentiated platform becomes manageable as one Form.
