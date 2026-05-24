# Grand Pipeline Assembly Document

## Why this document exists

This document is the single high-level map for pipeline assembly in Rust GDS, with a Java-parity lens.

It explains:
- what is assembled,
- where assembly lives,
- how assembled pipelines execute,
- how procedures/applications feed projection/eval,
- where embeddings currently fit,
- and what is still split or deferred.

The goal is to support the transition toward concrete dataset and SemDataset wiring without losing orientation.

## Core thesis

Pipeline assembly is not just a convenience API. It is the composition boundary between:
- conceptual orchestration (procedures and applications), and
- executable projection/eval machinery (pipeline runtime, step executors, predict/train executors).

In code terms, procedures/applications are upstream control logic, while projection/eval is the execution engine that consumes assembled pipeline structure.

## Layer map

### Procedures and Applications (orchestration/control)

Primary orchestration lives in:
- `gds/src/procedures/pipelines/facade.rs`
- `gds/src/procedures/pipelines/pipeline_applications.rs`

Responsibilities:
- parse and normalize user configuration,
- manage named pipelines via repository/catalog,
- invoke train/predict entrypoints,
- bridge request-scoped dependencies,
- keep runtime model handles needed for predict flows.

### Projection/Eval Pipeline (assembly model + runtime)

Primary execution model lives in:
- `gds/src/projection/eval/pipeline/*`

Responsibilities:
- represent pipelines and steps,
- validate pipeline graph/property compatibility,
- execute node property steps,
- orchestrate training and prediction lifecycles.

### Catalog/Graph Store boundary

Graph catalog and store APIs are the state substrate:
- `gds/src/types/catalog/service.rs`
- `gds/src/types/catalog/in_memory.rs`
- `gds/src/types/graph_store/*`

Recent direction:
- no graph-store clone fallback for mutate pipeline procedures,
- explicit in-place mutation path with fast failure if graph is shared.

## Assembly surfaces

## 1) Pipeline creation and configuration

In `PipelineApplications`:
- create link prediction pipeline,
- create node classification pipeline,
- create node regression pipeline,
- configure split and auto-tuning,
- add feature steps and node property steps.

Representative file:
- `gds/src/procedures/pipelines/pipeline_applications.rs`

## 2) Node property step assembly

Step creation and validation is centralized in:
- `gds/src/projection/eval/pipeline/node_property_step_factory.rs`

Key behavior:
- normalize algorithm names to canonical mutate procedure form,
- reject reserved keys that runtime owns,
- validate mutate-property presence,
- validate algorithm presence via procedure registry.

Step representation:
- `gds/src/projection/eval/pipeline/node_property_step.rs`

A step is an executable mutate procedure descriptor with config and context labels/types.

## 3) Pipeline structure

Training and predict structures are represented under:
- `gds/src/projection/eval/pipeline/node_property_training_pipeline.rs`
- `gds/src/projection/eval/pipeline/node_property_predict_pipeline.rs`
- `gds/src/projection/eval/pipeline/node_pipeline/*`
- `gds/src/projection/eval/pipeline/link_pipeline/*`

## Execution path (assembled -> running)

### Predict execution template

Main template:
- `gds/src/projection/eval/pipeline/predict_pipeline_executor.rs`

Flow:
1. build graph filter for step execution,
2. validate pipeline before execution,
3. execute node property steps,
4. validate feature properties,
5. execute prediction,
6. cleanup intermediate properties.

### Node property step execution

Executor:
- `gds/src/projection/eval/pipeline/node_property_step_executor.rs`

Responsibilities:
- validate context labels and relationship types,
- resolve feature input labels/types,
- execute steps in order,
- cleanup intermediate mutate properties.

## Where PipelineApplications fits now

`PipelineApplications` is currently the bridge between procedure facade calls and projection/eval execution primitives.

It already wires concrete node train/predict paths (classification/regression) and uses pipeline runtime structures directly. It is therefore not separate from pipeline assembly in practice; it is an orchestration layer feeding assembled pipelines into execution.

File:
- `gds/src/procedures/pipelines/pipeline_applications.rs`

## Embeddings and pipeline assembly: current truth

The system currently has two realities:

1. Direct embeddings procedure path exists and is active.
- Dispatch and handlers live under:
  - `gds/src/applications/services/algorithms_dispatch.rs`
  - `gds/src/applications/algorithms/embeddings/*`
  - `gds/src/procedures/embeddings/*`

2. Pipeline assembly knows embedding-like mutate algorithm names, but full execution parity is partial/deferred for some embedding steps.
- Example symbolic constant:
  - `FASTRP_MUTATE` in `gds/src/projection/eval/pipeline/node_property_step.rs`

Interpretation:
- assembly model can describe embedding-derived node-property steps,
- execution coverage is uneven across embedding algorithms/modes,
- mutate/write parity for embeddings inside assembled pipelines is not yet complete.

## Dataset and SemDataset implications

To reach concrete SemDataset wiring, assembly/execution must converge on a stable dataset substrate with clear ownership and mutability rules.

Current blockers/risk points:
- split between direct embedding procedure flows and pipeline-integrated embedding flows,
- uneven mutate/write support across embedding algorithms,
- mutable graph access still constrained by Arc ownership patterns at key boundaries,
- dataset lifecycle semantics are not yet unified around SemDataset abstractions.

## No-clone policy direction (current)

The current policy direction is explicit:
- do not clone graph stores for mutate pipeline procedures,
- mutate in place through catalog-mediated mutable access,
- fail fast when ownership is shared.

This was introduced to keep semantics honest and prepare for stronger dataset/state wiring.

## Java parity posture

This codebase already has parity notes and deep-dive references:
- `gds/doc/PIPELINE_FACADE_ARCH_NOTE.md`
- `gds/doc/PIPELINE_3C.md`
- `gds/doc/ML-EVAL-DEEP-DIVE.md`

This document is intended as the synthesis layer above those docs.

## Practical reading order for full-system understanding

1. `gds/src/procedures/pipelines/facade.rs`
2. `gds/src/procedures/pipelines/pipeline_applications.rs`
3. `gds/src/projection/eval/pipeline/node_property_step_factory.rs`
4. `gds/src/projection/eval/pipeline/node_property_step.rs`
5. `gds/src/projection/eval/pipeline/node_property_step_executor.rs`
6. `gds/src/projection/eval/pipeline/predict_pipeline_executor.rs`
7. `gds/src/procedures/embeddings/*` and `gds/src/applications/algorithms/embeddings/*`

## Immediate review checklist for upcoming Java parity pass

- confirm canonical algorithm-name normalization and mutate-mode constraints,
- verify pipeline step context validation parity against Java error shape,
- enumerate embedding algorithms by mode support (stream/stats/mutate/write),
- identify every place where assembled pipeline can reference embedding algorithms,
- identify every missing runtime hook from assembled embedding step to executable mutate/write implementation,
- map dataset split/filter semantics needed before SemDataset integration.

## Closing note

Pipeline assembly is already the conceptual center of gravity, even if execution remains partially split by legacy/direct procedure routes. The integration path is now clear: unify dataset semantics and embedding execution paths under the same assembled pipeline runtime contract.
