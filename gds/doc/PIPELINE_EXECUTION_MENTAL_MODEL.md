# Pipeline Execution Mental Model

A compact map of how training and prediction work in this codebase, where ProcedureExecutor is used, and what is needed for continual-learning loops.

## Terminology Guardrail

- Pipeline is the ML execution carrier (training or prediction orchestration).
- Plan is the semantic commitment layer (dataset semantics, form commitments, provenance-bearing intent).
- This document is runtime-engineering focused: it describes Pipeline behavior, not Plan semantics.
- When both appear in architecture discussions, treat Pipeline as one implementation surface that may execute parts of a Plan, but never as a synonym for Plan.

## 1) Train Execution Path

Training is model production. The orchestrator is PipelineTrainAlgorithm plus a concrete PipelineTrainer implementation.

```mermaid
flowchart TD
    A["Procedure train entrypoint"] --> B["TrainComputation<br/>parse config + fetch pipeline"]
    B --> C["NodeFeatureProducer context validation"]
    C --> D["Concrete TrainAlgorithm<br/>(classification/regression)"]
    D --> E["PipelineTrainAlgorithm.compute"]
    E --> F["Pipeline validation"]
    F --> G["PipelineTrainer.run"]
    G --> H["ResultToModelConverter.to_model"]
    H --> I["Catalog model artifact"]
```

Key idea:
- Training path is specialized pipeline orchestration, not the generic AlgorithmSpec runtime.

## 2) Predict Execution Path

Prediction is model use. Predict executors run node-property feature steps, then inference, then output as stream/mutate/write shape.

```mermaid
flowchart TD
    A["Procedure predict entrypoint"] --> B["PredictComputation<br/>load model + config"]
    B --> C["Concrete PredictPipelineExecutor<br/>classification/regression"]
    C --> D["PredictPipelineExecutor.compute"]
    D --> E["Validate pipeline + feature requirements"]
    E --> F["Execute nodePropertySteps"]
    F --> G["Executor.execute<br/>model inference"]
    G --> H["Result shaping<br/>stream or mutate or write"]
    H --> I["Cleanup intermediate properties"]
```

Key idea:
- Predict path is where model artifacts become operational outputs.

## 3) ProcedureExecutor Boundary

ProcedureExecutor appears in two important places:
- Generic AlgorithmSpec runtime.
- Nested inside pipeline node-property steps.

It is not the top-level orchestrator for pipeline training in the current architecture.

```mermaid
flowchart LR
    A["Pipeline training outer flow"] --> B["PipelineTrainAlgorithm + PipelineTrainer"]
    B -. "nested feature step calls" .-> C["NodePropertyStep execution"]
    C --> D["ProcedureExecutor.compute"]
    D --> E["AlgorithmSpec.execute"]

    F["Generic non-pipeline procedures"] --> D
```

Key idea:
- ProcedureExecutor is a shared execution kernel, but pipeline training has its own outer controller.

## 4) Continual-Learning Prerequisites

This architecture supports iterative retrain-and-serve loops, but production continual learning needs explicit control logic:

1. Data/change detection:
- Detect distribution drift, schema drift, and label freshness windows.

2. Retraining policy:
- Trigger rules (time-based, data-volume-based, performance-based).
- Training budget limits and concurrency limits.

3. Evaluation gate:
- Holdout evaluation against current champion model.
- Regression/classification metric thresholds and fail-fast criteria.

4. Model lifecycle:
- Versioning, promotion rules, rollback, and retention policy.

5. Safe deployment:
- Canary predict runs, shadow evaluation, and post-deploy monitoring.

6. Feedback loop:
- Capture outcomes for the next training cycle and audit traces.

```mermaid
flowchart TD
    A["Observe data + outcomes"] --> B["Drift or trigger check"]
    B -->|yes| C["Train candidate model"]
    C --> D["Evaluate vs champion"]
    D -->|pass| E["Promote + deploy"]
    D -->|fail| F["Keep champion"]
    E --> G["Monitor online behavior"]
    G --> A
    F --> A
```

## Practical Reading Order

1. gds/src/projection/eval/pipeline/pipeline_train_algorithm.rs
2. gds/src/procedures/pipelines/node_classification_train_computation.rs
3. gds/src/procedures/pipelines/node_regression_train_computation.rs
4. gds/src/projection/eval/pipeline/predict_pipeline_executor.rs
5. gds/src/procedures/pipelines/node_classification_predict_pipeline_executor.rs
6. gds/src/procedures/pipelines/node_regression_predict_pipeline_executor.rs
7. gds/src/projection/eval/pipeline/node_property_step_execution.rs
8. gds/src/projection/eval/algorithm/executor.rs
