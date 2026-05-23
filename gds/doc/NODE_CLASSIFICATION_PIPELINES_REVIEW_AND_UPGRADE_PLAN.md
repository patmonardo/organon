# Node Classification Pipelines Review and Upgrade Plan

This note is a bottom-up map of the `node_classification_*` procedure pipeline surface in `gds/src/procedures/pipelines/`. It is intentionally restricted to node classification and leaves the broader `pipeline_applications` / facade machinery aside, because that layer is a separate and more abstract review.

The purpose is twofold:

1. Explain what each file is doing in the current Rust implementation.
2. Turn that analysis into a practical Java-parity review / upgrade plan.

## Big Picture

The node classification procedure surface is organized around four concerns:

- configuration parsing and normalization
- model-backed prediction orchestration
- output shaping for stream / mutate / write modes
- training result rendering and model persistence stubs

In practice, the execution flow is:

1. Parse procedure config into a typed predict/training config.
2. Load a trained model result.
3. Reconstruct the prediction pipeline snapshot from the model.
4. Run the `PredictPipelineExecutor` implementation.
5. Shape the result for stream, mutate, or write output.
6. Return model/training metadata for train mode.

## File-by-File Role Map

### `node_classification_predict_computation.rs`

Top-level prediction entrypoint for node classification.

What it does:

- accepts a base config, a user-visible label, and a trained `NodeClassificationModelResult`
- extracts class ids from the model metadata
- builds a `LocalIdMap` to convert internal class ids back to original ids
- creates a `TaskProgressTracker` for prediction progress
- extracts `ClassifierData` from the trained model
- constructs `NodeClassificationPredictPipelineExecutor`
- delegates actual inference to `executor.compute()`

Why it matters:

- this is the first place where the trained model becomes operational prediction logic
- it is the bridge from catalog/model artifact to prediction runtime

Upgrade note:

- this is one of the strongest Java parity anchors; it should be easy to compare against Java’s prediction procedure entrypoint and ensure the Rust call chain preserves the same semantics

### `node_classification_predict_pipeline_executor.rs`

Core prediction executor for node classification.

What it does:

- implements the generic `PredictPipelineExecutor<NodePropertyPredictPipeline, NodeClassificationPipelineResult>` trait
- holds the pipeline snapshot, config, graph store, progress tracker, classifier data, class-id map, and termination flag
- validates feature dimensions against model expectations
- extracts lazy features from the graph
- runs `NodeClassificationPredict`
- converts the raw prediction result into `NodeClassificationPipelineResult`

Why it matters:

- this is the actual runtime inference kernel for node classification predictions
- it is the execution point that turns feature extraction into predictions

Upgrade note:

- this file is central to Java-parity review because it contains the full predict-path semantics: pipeline validation, feature extraction, inference, and result conversion
- this should remain a thin, understandable wrapper around the common predict executor pattern

### `node_classification_train_computation.rs`

Top-level training entrypoint for node classification.

What it does:

- accepts a `PipelineRepository`, training config, and user identity
- resolves the training pipeline by name
- validates that the requested main metric is compatible with the pipeline’s training parameter space
- creates a `NodeFeatureProducer`
- validates node-property step context configuration
- creates `NodeClassificationTrain`
- wraps it in `NodeClassificationTrainAlgorithm`
- runs the algorithm through `AlgorithmMachinery::run_algorithms_and_manage_progress_tracker`

Why it matters:

- this is the training-side bridge between procedure input and the ML pipeline runtime
- it is the node classification counterpart to the prediction entrypoint

Upgrade note:

- this is a major Java-parity anchor because it shows how the Rust code mirrors Java’s train orchestration
- the review should confirm that model selection, validation, and result conversion are all happening in the intended order

### `node_classification_pipeline_result.rs`

Normalized prediction result container.

What it does:

- stores predicted classes as a `HugeLongArray`
- stores optional predicted probabilities as a `HugeObjectArray<Vec<f64>>`
- maps internal class ids back to original ids using `LocalIdMap`

Why it matters:

- this is the clean result object that later stream/mutate/write builders consume
- it preserves the distinction between internal model ids and user-facing ids

Upgrade note:

- a good Java-parity review target is whether this object carries exactly the fields the downstream procedure result builders need, and nothing more

### `node_classification_predict_config_pre_processor.rs`

Placeholder config pre-processor.

What it does:

- defines a Java-parity-shaped preprocessor
- currently does not enrich input with model-derived pipeline parameters
- exists mainly to preserve call shape

Why it matters:

- this is an intentional stub, but it marks a missing integration point

Upgrade note:

- if Java behavior depends on model metadata being injected into prediction config, this file is one of the first places to wire in real logic
- currently it is a structural placeholder rather than a functional part of the pipeline

### `node_classification_predict_pipeline_base_config.rs`

Base config for classification prediction.

What it does:

- defines `NodeClassificationPredictPipelineConfig`
- stores graph name, concurrency, model user/name, target labels, relationship types, and probability flag
- supports construction from a raw `AnyMap`
- serializes back to `AnyMap`
- normalizes `username()` / `model_user()` handling

Why it matters:

- this is the normalized config surface for prediction procedures
- it is the main place where procedure parameters become typed runtime behavior

Upgrade note:

- this file should be compared carefully to Java config parsing and defaulting behavior
- it is likely to be important when upgrading the public procedure API shape

### `node_classification_predict_pipeline_mutate_or_write_config.rs`

Shared config layer for mutate and write prediction modes.

What it does:

- wraps the base config
- adds an optional predicted-probability property name
- exposes `to_map()` for result metadata
- changes `include_predicted_probabilities()` based on whether the probability property is present

Why it matters:

- it is the shared mode-specific config bridge for output-shaping behavior

Upgrade note:

- ensure the semantics of predicted probability output match Java mode handling, especially around defaults and property naming

### `node_classification_predict_pipeline_mutate_config.rs`

Mutate-mode config.

What it does:

- wraps the shared mutate-or-write config
- adds `mutate_property` and `write_concurrency`
- validates that `mutateProperty` and `predictedProbabilityProperty` differ
- serializes mode metadata to `AnyMap`

Why it matters:

- this file defines the mutate-mode contract for node classification prediction
- it is the config side of mutating predicted classes back into the graph

Upgrade note:

- this should be checked against Java for exact parameter names, validation rules, and output metadata

### `node_classification_predict_pipeline_stream_config.rs`

Stream-mode config.

What it does:

- wraps the base config
- disables predicted probabilities for stream mode

Why it matters:

- stream mode should be the simplest prediction output path

Upgrade note:

- compare with Java’s stream result shape and ensure the probability flag is intentionally suppressed

### `node_classification_predict_pipeline_write_config.rs`

Write-mode config.

What it does:

- wraps the shared mutate-or-write config
- adds `write_property` and `write_concurrency`
- validates that `writeProperty` and `predictedProbabilityProperty` differ
- implements `WriteConfigLike` and `WritePropertyConfigLike`

Why it matters:

- this is the write-mode contract for persisting predictions

Upgrade note:

- check Java parity for output property naming and write-concurrency semantics

### `node_classification_predict_pipeline_mutate_result_builder.rs`

Result builder for mutate mode.

What it does:

- implements `MutateResultBuilder`
- packages timings and configuration into a `PredictMutateResult`
- records node-properties-written metadata
- zeroes counts when no result is produced

Why it matters:

- this is the bridge from prediction result into mutate procedure output

Upgrade note:

- verify whether Java accumulates identical timing fields and write counts in the same way

### `node_classification_predict_pipeline_stream_result_builder.rs`

Result builder for stream mode.

What it does:

- implements `StreamResultBuilder`
- converts prediction results into a row iterator
- resolves original node ids from internal graph ids
- emits predicted class and optional probability rows

Why it matters:

- this is the stream output surface for predictions

Upgrade note:

- check whether the row shape, ordering, and id mapping match Java exactly

### `node_classification_predict_pipeline_write_result_builder.rs`

Result builder for write mode.

What it does:

- implements `WriteResultBuilder`
- packages timings and configuration into a `WriteResult`
- records node-properties-written metadata
- zeroes counts when no result is produced

Why it matters:

- this is the write-mode result surface

Upgrade note:

- compare with Java write result contracts and any extra metadata fields

### `node_classification_predict_pipeline_mutate_step.rs`

Mutate step implementation.

What it does:

- implements `MutateStep<NodeClassificationPipelineResult, GraphStoreNodePropertiesWritten>`
- resolves target labels from config or wildcard semantics
- converts prediction results into node properties using `predicted_probabilities::as_properties`
- writes node properties back to the graph through `GraphStoreService`

Why it matters:

- this is where prediction becomes graph mutation

Upgrade note:

- this is a high-value parity check because Java often couples prediction with explicit mutate-step orchestration
- verify that wildcard label resolution and property naming match expected behavior

### `node_classification_predict_pipeline_write_step.rs`

Write step implementation.

What it does:

- implements `WriteStep<NodeClassificationPipelineResult, GraphStoreNodePropertiesWritten>`
- converts prediction results into node properties for database writes
- returns a derived count of written properties
- currently behaves as a stubbed / simplified write path rather than fully persisting through the graph facade

Why it matters:

- this is the write-mode side effect surface

Upgrade note:

- if Java write mode does real database persistence, this file is an important candidate for upgrade
- compare whether the Rust implementation is still a structural placeholder or a real write implementation

### `node_classification_train_side_effects.rs`

Training side effects.

What it does:

- implements `SideEffect<NodeClassificationModelResult, ()>`
- warns that model persistence is not yet wired into Rust pipelines
- delegates warning behavior to `ModelPersister`

Why it matters:

- this is the post-training model persistence hook

Upgrade note:

- if Java persists trained models into a catalog or disk-backed model store, this is a major parity gap to close

### `node_classification_pipeline_companion.rs`

Default config / companion helper for node classification pipelines.

What it does:

- provides default split configuration
- provides default training-parameter space configuration
- enumerates the classification training methods supported by the pipeline (`LogisticRegression`, `RandomForestClassification`, `MLPClassification`)

Why it matters:

- this defines the default shape of a node classification training pipeline

Upgrade note:

- this is a useful comparison point against Java default pipeline construction and default parameter handling

### `node_classification_predict_pipeline_estimator.rs`

Prediction memory estimator.

What it does:

- estimates memory for classification prediction using model feature dimension and class count
- provides a fallback empty estimate when the model is absent

Why it matters:

- this is the planning / sizing side of the prediction procedure

Upgrade note:

- check whether Java uses identical memory estimation heuristics or a different estimator surface

### `node_classification_predict_pipeline_constants.rs`

Classification predict constants.

What it does:

- defines `MIN_BATCH_SIZE`

Why it matters:

- this controls prediction batching behavior

Upgrade note:

- if Java has a different minimum batch size or adaptive batching rule, this is a likely parity diff

### `node_classification_train_result_renderer.rs`

Training result renderer.

What it does:

- implements `ResultRenderer<NodeClassificationModelResult, Vec<NodeClassificationPipelineTrainResult>, ()>`
- renders training result into catalog-facing train result structs
- formats model info, config, and training statistics into maps

Why it matters:

- this is the train-side output shaping layer

Upgrade note:

- compare the rendered model info / config structure against Java output fields

### `model_persister.rs`

Model persistence helper.

What it does:

- persists a model into the `ModelCatalogFacade`
- optionally warns if disk persistence is requested
- provides a generic unimplemented-model warning helper

Why it matters:

- this is the general persistence shim used by training side effects

Upgrade note:

- if Java has a richer model catalog or persistent model storage behavior, this file is a critical parity anchor

## What the Pattern Says About the Architecture

The node classification family is not just one pipeline. It is a mini-system with these layers:

- config parsing and defaults
- model-backed prediction computation
- generic predict executor
- result normalization
- mode-specific config wrappers
- mutate/stream/write result shaping
- graph mutation / write side effects
- training orchestration and result rendering
- model persistence stubs

That is why the file list feels large but understandable: each file is doing one job in a deliberately separated pipeline stack.

## Review / Upgrade Priorities

If we want to upgrade this family for Java parity, the order should be:

1. `node_classification_predict_computation.rs`
2. `node_classification_predict_pipeline_executor.rs`
3. `node_classification_train_computation.rs`
4. `node_classification_pipeline_result.rs`
5. `node_classification_predict_config_pre_processor.rs`
6. `node_classification_predict_pipeline_base_config.rs`
7. `node_classification_predict_pipeline_mutate_or_write_config.rs`
8. `node_classification_predict_pipeline_mutate_config.rs`
9. `node_classification_predict_pipeline_stream_config.rs`
10. `node_classification_predict_pipeline_write_config.rs`
11. `node_classification_predict_pipeline_mutate_result_builder.rs`
12. `node_classification_predict_pipeline_stream_result_builder.rs`
13. `node_classification_predict_pipeline_write_result_builder.rs`
14. `node_classification_predict_pipeline_mutate_step.rs`
15. `node_classification_predict_pipeline_write_step.rs`
16. `node_classification_train_side_effects.rs`
17. `node_classification_pipeline_companion.rs`
18. `node_classification_predict_pipeline_estimator.rs`
19. `node_classification_predict_pipeline_constants.rs`
20. `node_classification_train_result_renderer.rs`
21. `model_persister.rs`

Why this order:

- it starts with the operational train/predict spine
- then moves into the config and result-shaping surfaces
- then closes with the persistence and defaulting helpers

## Upgrade Questions to Answer While Reviewing Java Sources

When comparing to Java, the most useful questions are:

- Does Java inject model metadata into prediction config pre-processing, and if so, where is that wired?
- Is Java’s prediction executor using the same feature extraction and class-id remapping pattern?
- Does Java’s mutate/write path persist results in the same way, or is Rust still a placeholder?
- What training statistics and model info are rendered in Java’s train result output?
- Are default configs, batch sizes, and output property names identical?
- Does Java’s model persistence path have a real catalog/disk implementation that Rust is still missing?

## Short Summary

The node classification procedure pipeline surface is already a coherent stack. It has a clear train path, a clear predict path, output-specific builders, and explicit stubs where catalog or persistence wiring is still incomplete.

That makes it a good place to do a focused Java parity review because the file boundaries already tell us what each piece is supposed to do.

This note should be used as the review / upgrade guide for the node_classification* family before moving on to the broader procedure facade layer.
