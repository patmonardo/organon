# Node Pipelines Model/Feature/Pipeline Review and Upgrade Plan

## Scope

This plan focuses on `gds/src/procedures/pipelines` for node workflows only.

- In scope: `node_classification_*`, `node_regression_*`, and shared pipeline procedure modules.
- Out of scope for this pass: `link_*` and `lp_*` modules.

Primary goal: wire Node pipeline procedures from Java-shaped facade/builder flow into real executor-backed train/predict mutate/stream/write execution against core system objects.

## Current State Snapshot

What is in good shape:

- Node classification and node regression execution modules are now mostly symmetric.
- Predict executors for both node families support selected-node prediction and compact-to-root expansion semantics.
- Train-side configs/algorithms now preserve relationship filters and concurrency.

What is still missing:

- Facade entrypoints route to `PipelineApplications` methods that still panic for executor-backed procedures.
- Procedure layer wiring to graph/model runtime dependencies is incomplete.
- Model persistence wrappers exist but are uneven between node classification and node regression.

## Module Map (Procedures/Pipelines)

### 1. Facade and Application Surface

- `facade.rs`
  - Java-shaped trait surface (`LinkPredictionFacade`, `NodeClassificationFacade`, `NodeRegressionFacade`, top-level facade).
  - Local implementation delegates to `PipelineApplications`.
  - Catalog/builder operations are wired; executor-backed operations currently defer to panics.

- `pipeline_applications.rs`
  - Request/user-scoped builder and catalog mutation logic.
  - Creates/replaces training pipelines and configures:
    - node property steps
    - feature steps
    - split configs
    - auto-tuning
    - training methods
  - Executor-backed methods for train/predict/mutate/stream/write still call fail-fast panics.

### 2. Pipeline Catalog and Naming

- `pipeline_repository.rs`
  - Typed repository over `projection::eval::pipeline::PipelineCatalog`.
  - Supports create/get/replace/drop/list/exists for link and node training pipelines.

- `pipeline_name.rs`
  - Validated `PipelineName` value object used by facade/applications/computation layers.

- `pipeline_catalog_result_transformer.rs`
- `pipeline_info_result_transformer.rs`
- `node_pipeline_info_result_transformer.rs`
  - Convert internal training pipelines to API result payloads.

### 3. Result Types and Shared Builders

- `types.rs`
  - Procedure API result DTOs.
  - Defines train/predict mutate/write/stream result shapes.

- `predicted_probabilities.rs`
  - Node classification helper to map compact prediction arrays into root-sized node property payloads.

### 4. Node Execution Modules

- Classification
  - `node_classification_train_computation.rs`
  - `node_classification_predict_computation.rs`
  - `node_classification_predict_pipeline_executor.rs`
  - mutate/stream/write result builders and steps
  - `node_classification_train_side_effects.rs`

- Regression
  - `node_regression_train_computation.rs`
  - `node_regression_predict_computation.rs`
  - `node_regression_predict_pipeline_executor.rs`
  - `node_regression_pipeline_result.rs`
  - mutate/stream result builders and steps
  - `node_regression_train_side_effects.rs`

### 5. Model Catalog/Persistence Adapters

- `model_persister.rs`
  - Generic in-memory model catalog set.
  - Disk persistence intentionally warned as not yet wired.

- `trained_nc_pipeline_model.rs`
  - Most complete typed path: converts classification train result to model and stores in catalog.

- `trained_nr_pipeline_model.rs`
- `trained_lp_pipeline_model.rs`
  - Currently untyped retrieval wrappers; not yet mirrored to classification-level typed conversion/store flow.

## How This Connects to the Bigger Platform

### Model / Feature / Pipeline in Core Flow

1. **Pipeline Form Construction**
   - Facade calls enter `PipelineApplications`.
   - `PipelineApplications` mutates typed training pipeline forms via `PipelineRepository` + `PipelineCatalog`.

2. **Feature and Node Property Semantics**
   - Feature selection is represented as node feature steps in training pipelines.
   - Node property steps are executable algorithm steps validated in train/predict flows.

3. **Train Runtime Path**
   - Train computation modules pull pipeline form + train config.
   - `NodeFeatureProducer` validates and materializes feature context.
   - Train algorithms run via `AlgorithmMachinery` and produce model artifacts.

4. **Predict Runtime Path**
   - Predict computation modules instantiate `PredictPipelineExecutor` implementations.
   - Executors derive prediction node universe, extract features, run inference.
   - Result builders/side effects materialize stream/mutate/write outcomes.

5. **Model Catalog Bridge**
   - `ModelPersister` and `trained_*_pipeline_model` bridge procedure outputs to `core::model` catalog API.

## Key Gaps to Close

### Gap A: Executor-backed Procedure Wiring in `PipelineApplications`

Currently unimplemented (panic placeholders):

- Node classification:
  - `node_classification_train`
  - `node_classification_predict_mutate`
  - `node_classification_predict_stream`
  - `node_classification_predict_write`
  - estimates
- Node regression:
  - `node_regression_train`
  - `node_regression_predict_mutate`
  - `node_regression_predict_stream`

### Gap B: Request-Scoped Runtime Dependencies

`LocalPipelinesProcedureFacade` currently wires user + pipeline catalog only. Real execution needs request-scoped access to:

- graph store catalog/facade
- model catalog facade
- logging/service context
- optional write context and memory estimation runtime

### Gap C: Node Regression Model Adapter Parity

- Classification has typed conversion/store adapter (`trained_nc_pipeline_model.rs`).
- Regression currently remains mostly untyped retrieval wrapper.
- Need typed regression model conversion/persist path parallel to classification.

### Gap D: Side-Effects and Persistence Contract Clarity

- Side-effect modules still contain explicit “not yet wired” posture for disk persistence.
- Need stable contract for:
  - in-memory model catalog persistence (required)
  - optional disk persistence (deferred but explicit)

## Upgrade Plan (Node-Only)

### Phase 1: Wire Runnable Node Execution in `PipelineApplications`

Objective: replace panic placeholders with real train/predict dispatch for node modules.

Actions:

1. Add runtime dependency handles to `PipelineApplications` constructor or a sibling execution service.
2. Implement:
   - `node_classification_train`
   - `node_classification_predict_mutate`
   - `node_classification_predict_stream`
   - `node_classification_predict_write`
   - `node_regression_train`
   - `node_regression_predict_mutate`
   - `node_regression_predict_stream`
3. Keep estimates either wired or explicitly marked deferred with non-panicking response.

### Phase 2: Complete Model/Feature/Pipeline Integration Contracts

Objective: make node train/predict path explicit and typed across procedure-model boundaries.

Actions:

1. Add regression typed model adapter parity with classification.
2. Normalize train side-effect persistence contracts for both node families.
3. Ensure predict config preprocessing/defaulting is consistently applied at computation entry.

### Phase 3: Tighten Facade/App Boundaries for Maintainability

Objective: retain Java-shaped API surface while reducing internal ceremony.

Actions:

1. Keep facade trait signatures stable.
2. Move repeated parse/default logic into shared helpers where possible.
3. Introduce small, explicit internal execution service boundary:
   - form mutation operations
   - executor-backed runtime operations

## Recommended First Execution Slice (Today/Tomorrow)

1. Implement `node_regression_train` and `node_classification_train` in `PipelineApplications` using existing `*_train_computation` modules.
2. Implement node stream paths (classification + regression) next, since they are low side-effect risk.
3. Implement mutate/write once stream/train are stable.

This ordering gets runnable graph application value quickly while keeping risk bounded.

## Architectural Positioning

For current project goals, treat GDS Model/Feature/Pipeline as standalone executable subsystem. Do not block node execution wiring on Semantic Dataset integration.

That preserves velocity and keeps integration concerns as a later, deliberate phase.
