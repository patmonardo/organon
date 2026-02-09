# StatFrame

StatFrame is the statistical ML surface that parallels the Dataset NS/Expr/Functions split.
It is meant to capture sklearn-style semantics without forcing an R-style workflow.

## Goals

- Provide a statistical modeling DSL that composes with `Feature` and `Dataset`.
- Separate concerns: expressions describe intent, functions implement semantics.
- Keep modeling contracts explicit: fit/transform/predict are first-class concepts.
- Avoid heavy containers; prefer thin adapters over concrete data frames.

## Module Layout

- `expressions/`
  - Declarative model specs, preprocessing specs, and evaluation specs.
- `functions/`
  - Concrete statistical utilities (scalers, encoders, splitters).
- `prelude` (future)
  - Stable re-exports for the statistical surface.

## Core Concepts (Proposed)

- `StatExpr`: expression tree for statistical pipelines.
- `StatEstimator`: trait with `fit`, `transform`, `predict`.
- `StatPipeline`: ordered composition of estimators and selectors.
- `StatSplit`: train/valid/test split configs.
- `StatMetric`: evaluation metric spec and runner.

## Integration Points

- `Feature`: StatFrame should consume or emit features, not own them.
- `Dataset`: statistical preprocessing should be expressible as dataset functions.
- `Model`: stat estimators should surface results as dataset model deltas.

## Initial Targets (Inspired by sklearn)

- preprocessing: standardize, normalize, one-hot, binning
- feature extraction: hashing, dict vectorization
- model selection: train/test split, k-fold, grid search (lightweight)
- linear models: ridge, lasso, logistic (lightweight API surface)
