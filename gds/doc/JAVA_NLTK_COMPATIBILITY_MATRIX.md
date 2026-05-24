# Java <-> NLTK compatibility matrix (model, feature, pipeline, property)

Purpose: keep Java/GDS ML pipeline semantics compatible with NLTK-style schema semantics used by this repo.

Scope:
- Java side is represented in this workspace by Rust interfaces explicitly marked as direct Java translations.
- NLTK side is represented by the schema-grounded definitions in NLP_SCHEMA_MODEL_NOTE.

## Canonical references

Java-parity interfaces and descriptors:
- gds/src/projection/eval/pipeline/pipeline_trait.rs
- gds/src/projection/eval/pipeline/feature_step.rs
- gds/src/projection/eval/pipeline/node_pipeline/node_feature_producer.rs
- reality/src/descriptors/ml/pipeline.rs
- reality/src/descriptors/ml/step.rs
- reality/src/descriptors/ml/model.rs

NLTK/schema definitions:
- gds/doc/NLP_SCHEMA_MODEL_NOTE.md

## Compatibility matrix

### 1) Pipeline

Java/GDS meaning:
- Pipeline is a two-stage runtime: node-property computation then feature extraction.
- Validation is property-centric: required input properties must exist or be produced by node-property steps.

NLTK/schema meaning:
- Pipeline should preserve schema semantics and provenance while composing model/feature/tree transforms.

Compatibility status:
- Compatible at orchestration level.
- Gap: Java-parity pipeline contracts do not encode provenance/reentrance constraints explicitly.

Compatibility rule:
- Treat pipeline execution as an ordering/runtime concern only.
- Treat schema grounding and provenance as an additional semantic contract layered on top of pipeline runtime.

### 2) Feature

Java/GDS meaning:
- FeatureStep consumes input node property names plus configuration.
- Descriptor feature types are Scalar/Array/Embedding with source properties.

NLTK/schema meaning:
- Features are typed, structured projections (FeatStruct paths, spans, graph relations), not only flat columns.

Compatibility status:
- Partially compatible.
- Gap: Java-parity contracts are property-list centric; NLTK features require structured path semantics.

Compatibility rule:
- Keep Java feature steps as execution carriers.
- Represent NLTK structure in feature naming/config conventions (path-like keys and typed metadata), preserving traceability to schema nodes/spans.

### 3) Model

Java/GDS meaning:
- Model descriptors are model_type + hyperparameters.
- Training config selects model candidates and validation metric.

NLTK/schema meaning:
- Model is schema->schema transform that emits traceable deltas (tags/parses/trees/attribute updates).

Compatibility status:
- Partially compatible.
- Gap: Java model descriptors are learner-centric artifacts, not semantic delta contracts.

Compatibility rule:
- Separate model identity from model effect:
  - Keep Java/GDS model descriptors for training and prediction selection.
  - Add or preserve a schema-delta layer for semantic output interpretation.

### 4) Property

Java/GDS meaning:
- Node properties are graph keys validated/extracted by pipeline runtime.
- Node property steps mutate graph with computed properties.

NLTK/schema meaning:
- Properties are typed attributes, often nested (FeatStruct), with reentrance and variable-binding semantics.

Compatibility status:
- Partially compatible.
- Gap: flat property keys are weaker than FeatStruct path/value semantics.

Compatibility rule:
- Use property keys as storage/runtime handles.
- Preserve richer semantics in typed value payloads and stable path conventions.

## Minimum compatibility contract

To remain compatible across Java/GDS and NLTK definitions:

1. Every feature used in a pipeline must be traceable to schema node/span origin.
2. Feature representations must support structured paths (not scalar-only assumptions).
3. Model outputs must be representable as schema updates (or reversible projections to them).
4. Pipeline validation may stay property-centric, but semantic validation should additionally check provenance and schema-shape invariants.

## Practical next checks

1. Add a semantic feature-key convention (for example, featstruct:path, span:start, graph:edgeType) to pipeline feature configs.
2. Add optional provenance payload in prediction/train outputs for node pipelines.
3. Add a semantic validator pass after existing property validation to check schema-grounded invariants.
4. Keep Java parity interfaces unchanged where needed, and layer semantic constraints in higher-level contracts.

## Semantic stance: restriction vs freedom

This repo can intentionally keep two compatible but distinct regimes:

- ML regime (restricted):
  - Property is a constrained runtime handle for computation, validation, and optimization.
  - Feature is often reduced to model-ready inputs under task constraints.

- Semantic dataset regime (freer):
  - Property is a reflective determination in the sphere of Essence.
  - Feature emerges in the sphere of Conceptual Freedom and should not be reduced to only constrained property vectors.
  - Model should remain schema-grounded and semantically expressive, not only a predictor artifact.

Operational consequence:

- Keep ML constraints where they are useful for computation.
- Preserve richer semantic contracts (provenance, structure, reentrance, typed paths) at the dataset and model-semantics layer.

Computation-platform note:

- ML is the real computation platform for heavy training/inference workflows.
- This is consistent with the broader NLP ecosystem (including NLTK practice of delegating stronger computation workloads to scikit-learn-style tooling).
- Computation can be treated as the restrictive/coercive layer needed to produce results under finite resources.
- In this architecture: computation power stays in ML systems, while semantic datasets preserve the freer, schema-grounded meaning layer.

Design maxim:

- In conceptual feature design, operate in the freer sphere of conceptual construction.
- In property/runtime design, require coercion, restrictions, and constraints for computability and reproducibility.
