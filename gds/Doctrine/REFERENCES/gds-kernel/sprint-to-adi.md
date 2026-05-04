# Sprint to ADI

This note encodes the final practical sprint path to ADI (Artificial Dataset Intelligence)
for the GDS Kernel.

ADI here means: Dataset becomes SemDataset through an auditable, runnable, and
Shell-governed learning path.

In this sprint, `LM` has a double reading: it is the LanguageModel in code and the
Learning Module in doctrine. It is the Model in training underneath SemDataset,
not the final authority; Shell blesses the learned SemDataset path into PureForm return.

---

## Sprint Thesis

The Shell ended the split between competing abstractions.

Now the task is execution discipline:

1. Keep one golden path.
2. Keep semantic governance in Dataset and Shell.
3. Ship runnable evidence every iteration.

This is not a theory sprint. It is a get-something-working sprint.

---

## Golden Path

```text
Frame -> Model:Feature::Plan -> SemDataset -> PureForm return
```

A contribution is valid only if it strengthens or verifies this path.

---

## ADI Minimal Contract

For ADI readiness, the kernel must expose all of the following on a runnable path:

- Projection trace validity
- Semantic pipeline readiness checkpoints
- Learning report from Dataset to SemDataset
- Provenance-bearing semantic forms
- PureForm return contract

In implementation terms, this corresponds to:

- `GdsShell::validate_projection_trace()`
- `GdsShell::semantic_pipeline_knowledge()`
- `GdsShell::learning_report()`
- `GdsShell::materialize_semdataset_from_texts()`
- `GdsShell::corpus_report()`
- `GdsShell::capability_map()`
- `GdsShell::pipeline_progress_tracker()`
- `GdsShell::estimate_pipeline_memory(concurrency)`
- `ShellPipelineDescriptor::to_pure_form_return()`

---

## Sprint Backlog (Execution First)

1. Harden one canonical shell example as the golden path.
2. Require projection validity checks in integration tests.
3. Add one logic-rich fixture (principle, concept, and at least one logical kind).
4. Verify learning report deltas between minimal and enriched runs.
5. Keep doctrine synchronized with runtime APIs.

---

## Current Increments

### Logic-Ready Shell Path

- The canonical shell example includes `judgment` and `inference` ProgramFeature kinds.
- `shell_bridge` verifies a baseline Dataset path and a logic-rich ADI path.
- The enriched ADI path reaches `mathematical_logic_ready = true` while preserving
	projection trace validity and KG readiness.

This proves the sprint's first learning delta: Dataset can move from semantic readiness
to mathematical-logic readiness under Shell governance.

### Corpus Materialization Path

- `GdsShell::materialize_semdataset_from_texts()` explicitly materializes caller-provided
	texts into `Corpus -> MLE -> SemDataset`.
- The Shell ingests its `ProgramFeatures` into the materialized SemDataset and parses
	SemForms immediately.
- `GdsShell::corpus_report()` exposes document count, LM order, vocabulary size,
	SemForm count, and parsed form count.
- `shell_bridge` verifies that Corpus materialization preserves trace validity,
	KG readiness, and mathematical-logic readiness.

This proves the next ADI delta: the Shell path now connects to real Corpus production
without inferring corpus columns or weakening Dataset/Shell governance.

### Shell Capability Map

- `GdsShell::capability_map()` exposes a runtime map of platform capabilities visible
	from the current Shell address.
- The map is organized into three bands:
	- Immediate: Frame register, DataPipeline, PureForm return.
	- Mediation: Model:Feature::Plan, progress tracking, memory estimation, concurrency runtime.
	- Recursive: Pregel runtime, DefaultGraphStore, Corpus materialization,
		SemDataset learning, and mathematical logic readiness.
- Capability states separate availability from activity, so the Shell can name the
	whole platform without pretending every subsystem has already been invoked.

This proves the next ADI delta: the Shell can now read the kernel as a platform map.
It is not yet a dispatcher for Pregel, concurrency, or graph storage; it is the governing
awareness layer that makes those future dispatches auditable.

### Shell Pipeline Tooling

- `GdsShell::pipeline_progress_tracker()` creates a request-local tracker using the
	kernel progress infrastructure (`TaskProgressTracker`) with Shell-derived volume.
- `GdsShell::estimate_pipeline_memory(concurrency)` runs through kernel memory
	estimation machinery and returns a structured estimate (`ShellMemoryEstimate`).

This establishes Progress Tracking and Memory Estimation as common Shell-pipeline
tools rather than ad hoc algorithm-only concerns.

---

## Definition of Done

A sprint increment is complete when:

1. The golden path runs end-to-end.
2. Trace validity and learning report are both asserted in tests.
3. Doctrine and runtime surface describe the same behavior.
4. The standard verification command succeeds:

```text
cargo fmt -p gds && cargo test -p gds --test shell_bridge && cargo run -p gds --example collections_gds_shell && cargo check -p gds
```

---

## Product Positioning Constraint

GML/GNN and huge graph stores may scale as large as needed.
They are delivery magnitude, not semantic authority.

ADI authority stays with:

- containing Dataset identity
- ProgramFeature commitments
- Shell traceability

So scale is monetized without surrendering method.

---

## One-Line Rule

If it does not improve the Dataset -> SemDataset learning path under Shell governance,
it is not sprint-critical work.
