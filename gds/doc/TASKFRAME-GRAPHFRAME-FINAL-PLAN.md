# TaskFrame and GraphFrame Final Plan

## Expressed Principle

AbsoluteForm encloses itself as PureForm by mediating GraphFrame and TaskFrame through one shared Shell Component system.

- GraphFrame determines the objective computational form and preserves its evidence identity.
- TaskFrame constitutes that objective intent as an agential workflow and preserves it through terminal receipt.
- Shell Components are the shared program language of the mediation; Shell evaluates them without owning GraphFrame selection or TaskFrame policy.
- PureForm remains the special expressive and reflexive interface rather than another execution substrate.

## Implementation Status

The first processor-substrate slice is implemented:

- `TaskStage` is the Kernel execution-stage record formerly named `TaskFrame`.
- `TaskWorkflow` is the validated ordered stage contract formerly named `TaskFramePlan`.
- `TaskFrame<Program>` is the Agent-facing controller that preserves objective evidence, workflow, program, and expected-return policy when producing `TaskSpec` and `TaskJob`.
- `GraphExecutionIntent` is the objective GraphFrame artifact carrying the selected store/view, Shell program, compute intent, and return declaration without owning a Task workflow.
- `TaskFrame::from_graph_intent` now constitutes seed, compute, and optional persistence stages under Agent policy.
- GraphFrame jobs preserve the complete `GraphExecutionIntent` rather than reducing their program to a bare Shell plan.
- GraphFrame `Stream` and `Stats` jobs remain ephemeral; `Mutate` and `Write` explicitly request persistence.
- `ShellProcedureEvaluator` is the primary Shell execution name; `ShellProcedureRuntime` remains a compatibility alias.
- Whole-store GraphExecutionIntent jobs execute through TaskDaemon and Shell. Selected views fail explicitly until a GraphStore view adapter is implemented, preventing whole-store fallback.
- Terminal TaskJob receipts preserve the validated objective, workflow, and return contract on success, failure, and cancellation.
- GraphFrame objective identity is a deterministic signature of relationship types, property selectors, and orientation.

Daemon lifecycle, cancellation lineage, resource admission, and stage receipts remain subsequent implementation work.

## Layering

### Kernel

- Treat TaskStage as the Kernel runtime-stage contract for shell- and subsystem-generated execution.
- Treat GraphFrame as an executable, immutable graph-view surface over a GraphStore.
- Keep both structures grounded in concrete compilation paths rather than abstract placeholders.

### Agent

- Use GraphFrame plans as the bridge from graph-view semantics to shell-level procedure execution.
- Let TaskFrame constitute objective intent as a validated workflow and preserve its resource, return, and provenance commitments.

### Logic

- Preserve the distinction between semantic determination and operational realization.
- Dataset/DataFrame remain the semantic substrate; Shell/TaskFrame provide execution and runtime mediation.

## Core Invariants

- Empirical adequacy: the plan must compile into executable, inspectable artifacts that can be tested.
- Reflexive consistency: GraphFrame view state, shell compilation, and TaskFrame plan generation must all derive from the same source specification.
- Dialectical mediation: the design should avoid collapsing into either pure logical idealism or pure procedural optimization.

## Final Architecture

1. GraphFrame is built from a GraphStore plus a GraphViewSpec.
2. GraphFramePlan collects view expressions and procedure expressions without committing to execution prematurely.
3. The plan compiles into three interlocking outputs:
   - a GraphViewSpec for view selection,
   - a ShellComponentPlan for pure-shell procedure expression,
   - a TaskFramePlan and TaskSpec for runtime realization.
4. TaskFrame carries the execution body for the graph workflow, while GraphFrame remains the graph-native planning and projection surface.

## Implementation Milestones

1. Stabilize the API surface.
   - Finalize GraphFrame, GraphFramePlan, TaskFrame, and TaskFramePlan semantics.
   - Keep the abstractions minimal and explicit.

2. Complete the compilation path.
   - Compile view expressions into GraphViewSpec.
   - Compile procedure expressions into ShellComponentPlan.
   - Compile the same plan into TaskFramePlan and TaskSpec.

3. Add runtime semantics.
   - Preserve scheduling, storage backend, progress namespace, and I/O lineage.
   - Make execution state legible to both shell and task layers.

4. Harden examples and tests.
   - Cover graph view selection, procedure compilation, task-frame generation, and catalog/persistence mediation.

## Implementation Checklist

1. Confirm the public API for GraphFrame and GraphFramePlan.
   - Keep constructors, view-selection methods, and procedure-entry methods minimal and explicit.
   - Preserve the current distinction between immutable plan construction and runtime execution.

2. Wire the core compilation path.
   - Ensure GraphFramePlan can compile view expressions into a GraphViewSpec.
   - Ensure GraphFramePlan can compile procedure expressions into a ShellComponentPlan.
   - Ensure the same plan can compile into a TaskFramePlan and TaskSpec.

3. Preserve runtime semantics in TaskFrame.
   - Maintain namespace, pipeline, and step identity.
   - Preserve resource, storage-backend, progress, and I/O metadata.
   - Keep the task frame legible to both shell and task layers.

4. Add examples and fixtures.
   - Provide at least one example showing a graph view being selected, a procedure being compiled, and a task frame being generated.
   - Keep the example aligned with the existing workbench and doctrine patterns.

5. Add regression tests.
   - Cover view compilation, pure-shell compilation, and task-frame compilation.
   - Ensure the tests assert real behavior rather than only internal plumbing.

## Acceptance Criteria

- A GraphFrame can be created from a GraphStore and expose a valid view.
- A procedure expression can be compiled into a pure-shell plan.
- The same GraphFrame plan can compile into a TaskFramePlan with explicit runtime steps.
- The mediation path is visible in examples and tests, not only in internal types.

## Non-Goals

- This is not a general-purpose distributed scheduler.
- This is not a replacement for Dataset/DataFrame semantics.
- This is not a separate graph-only stack disconnected from shell and task execution.
