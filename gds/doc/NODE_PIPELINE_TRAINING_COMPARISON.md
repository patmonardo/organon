# Node Pipeline Training Comparison

This note compares the current Rust node classification and node regression training pipeline modules. The filenames intentionally match because both modules implement the same top-level training-pipeline idea: load a user-scoped pipeline, produce features, split target nodes, train a model, evaluate it, retrain the selected model, and convert the raw train result into a catalog-ready model result.

The important reading stance is: classification and regression should look nearly identical at the orchestration boundary, while differing only where the target type and model family require it.

## Module Shape

Both modules have the same top-level roles.

| Role | Classification | Regression | Shared idea |
| --- | --- | --- | --- |
| Training pipeline | `node_classification_training_pipeline.rs` | `node_regression_training_pipeline.rs` | Owns node property steps, feature steps, training parameter space, auto-tuning config, and split config. |
| Train config | `node_classification_pipeline_train_config.rs` | `node_regression_pipeline_train_config.rs` | Names the pipeline, user, target labels, target property, random seed, and metrics. |
| Factory | `node_classification_train_pipeline_algorithm_factory.rs` | `node_regression_train_pipeline_algorithm_factory.rs` | Resolves the pipeline from `PipelineCatalog`, validates node property steps, creates `NodeFeatureProducer`, and builds the train algorithm. |
| Train algorithm | `node_classification_train_algorithm.rs` | `node_regression_train_algorithm.rs` | Thin adapter around `PipelineTrainAlgorithm`; it wires trainer, pipeline, converter, graph store, config, and progress tracker. |
| Core trainer | `node_classification_train.rs` | `node_regression_train.rs` | Extracts targets, splits examples, produces features, trains/evaluates/retrains, and returns a raw train result. |
| Raw result | `node_classification_train_result.rs` | `node_regression_train_result.rs` | Carries trained model and `TrainingStatistics`; classification also carries class mapping/counts. |
| Converter | `node_classification_to_model_converter.rs` | `node_regression_to_model_converter.rs` | Converts raw train result into catalog-style model result with version, model type, schema, config, model info, and statistics. |
| Model info | `node_classification_pipeline_model_info.rs` | `node_regression_pipeline_model_info.rs` | Stores best parameters, rendered metrics, and the prediction pipeline snapshot. |

## Shared Training Flow

At the top level both training pipelines follow the same rhythm.

1. The factory receives a `DefaultGraphStore`, train config, and progress tracker.
2. The factory looks up the configured training pipeline from `PipelineCatalog` using `(username, pipeline)`.
3. The factory creates a `NodeFeatureProducer` from the graph store and train config.
4. The core trainer scopes training to the configured target node labels.
5. The trainer validates split sizes against that target-node universe.
6. The trainer extracts a compact target array, indexed by target-node row rather than by every graph node.
7. `NodeSplitter` creates outer train/test and inner validation splits over the compact target universe.
8. `NodeFeatureProducer::procedure_features` executes node property steps, validates feature properties, and extracts feature rows for the same target labels.
9. The trainer evaluates metrics, fills `TrainingStatistics`, retrains the final model, and returns a raw train result.
10. The converter creates a catalog-ready model result with model metadata and a `NodePropertyPredictPipeline` snapshot.

This is the main parity point: both modules now read as the same controller pattern, and most differences are local to target representation, metric construction, and model training.

## Prediction Shape

Prediction is simpler than training because the prediction-side pipeline is mostly a frozen feature-production description. Training produces a `NodePropertyPredictPipeline` snapshot through the converter, using `NodePropertyPredictPipeline::from_pipeline(&pipeline)`.

That means prediction state should stay in the top-level result/converter/model-info layer as much as possible:

- the trained model data lives in the catalog-ready result;
- the original graph schema, model type, GDS version, train config, and model info travel with that result;
- the prediction pipeline snapshot records node property steps and feature properties;
- task-specific prediction execution can consume the model result without reconstructing training state.

Classification adds class IDs to model info because prediction must map local classifier outputs back to original class IDs. Regression does not need this class mapping because the model predicts a continuous value directly.

## Similarities

The strongest similarities are now structural.

- Both configs carry `username`, `pipeline`, `target_node_labels`, `target_property`, `random_seed`, and task-specific metrics.
- Both factories use user-scoped catalog lookup and delegate progress/memory construction into the core trainer.
- Both core trainers use target-label scoped node universes rather than blindly splitting all projected nodes.
- Both core trainers validate target property values before training.
- Both progress tasks have the same Java-shaped phases: node property steps, cross-validation/model selection, train best model, evaluation, and retrain best model.
- Both memory estimates are conservative and split node-property-step memory from training memory.
- Both converters preserve the original schema and attach model type/version metadata.
- Both model-info structs render best parameters, metrics, pipeline, node property steps, and feature properties.

## Differences

The differences are the places where classification and regression genuinely are not the same problem.

| Area | Classification | Regression |
| --- | --- | --- |
| Target type | Discrete labels stored as compact `HugeIntArray`. | Continuous values stored as compact `HugeDoubleArray`. |
| Target metadata | Builds `LocalIdMap` and `LongMultiSet` for class IDs and counts. | No class map or counts. |
| Metrics | Builds concrete classification metrics from `ClassificationMetricSpecification`, class IDs, and class counts. | Uses `RegressionMetric` values directly, defaulting to mean squared error if none are configured for the current direct trainer path. |
| Model family | Uses classifier trainers: logistic regression, random forest classification, and MLP classification. | Current Rust path trains a linear regression model directly; random forest regression is represented in the pipeline type but not yet fully selected in the trainer path. |
| Model selection | Classification currently collects candidate configs, runs `CrossValidation`, selects the winning candidate, evaluates it, and retrains. | Regression currently creates placeholder candidate statistics and trains/evaluates/retrains a default linear regression model. This is the largest remaining behavioral difference. |
| Progress task | Includes separate `Evaluate on train data` and `Evaluate on test data` leaves. | Has one `Evaluate on test data` leaf, matching the Java regression progress shape, even though the Rust evaluation records both outer-train and test scores. |
| Model info | Includes predicted `classes` in addition to best parameters, metrics, and prediction pipeline. | No classes field; model info is best parameters, metrics, and prediction pipeline. |
| Memory estimate | Includes class-count and classification-metric estimate components. | No class-count estimate; target memory is double-array based. |

## Obvious Implementation Gaps

The paired files now make the remaining regression gaps easier to see.

1. Regression training does not yet mirror classification's full `CrossValidation` path. It should eventually collect regression candidate configs from the pipeline, map them to ML trainer configs, run model selection, train the selected regressor, evaluate it, and retrain with the winning parameters.
2. Regression's progress task is Java-shaped, but the current training body does not actually execute a distinct cross-validation/model-selection phase yet.
3. Regression metric validation still accepts the enum value without a full registry-level `validateMainMetric` equivalent.
4. Classification has a richer model-specific metrics path because class-aware metrics need class IDs and class counts. Regression should remain simpler, but its trainer-selection path should still become symmetric.
5. Prediction should remain simpler than training: most prediction readiness should be carried by the catalog-ready result and `NodePropertyPredictPipeline`, not by duplicating training machinery.

## Reading Guide

For learning the code, read the paired modules in this order:

1. `node_*_pipeline_train_config.rs`: what the procedure asks for.
2. `node_*_training_pipeline.rs`: what the stored pipeline contains.
3. `node_*_train_pipeline_algorithm_factory.rs`: how config + catalog + graph become an executable algorithm.
4. `node_*_train_algorithm.rs`: the thin `PipelineTrainAlgorithm` adapter.
5. `node_*_train.rs`: the real training control flow.
6. `node_*_train_result.rs`: what raw training returns.
7. `node_*_to_model_converter.rs`: how raw training becomes catalog model state.
8. `node_*_pipeline_model_info.rs`: what metadata prediction and model listing can inspect later.

If the paired files are read in this order, the system becomes much less long: the outer machinery is shared, and the inner differences reduce to target encoding, metric construction, model trainer family, and model-info payload.
