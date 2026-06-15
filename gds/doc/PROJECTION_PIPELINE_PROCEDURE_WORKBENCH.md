# Projection Pipeline Procedure Workbench

This workbench is a training surface for reducing perceived complexity in the eval/pipeline system by making the component combinatorics explicit.

Use it as an operator map, not as a one-time architecture note.

## 1) Why This Workbench Exists

You are not dealing with one mechanism called "pipeline." You are navigating a layered composition:

- facade layer (`procedures/pipelines`) for public orchestration and request shape
- eval layer (`projection/eval/pipeline`) for execution templates
- catalog layer (`PipelineCatalog`) for user-scoped pipeline symbols
- pipeline kind layer (link, node classification, node regression)
- execution mode layer (train, stream, mutate, write, estimate)

When these are flattened mentally, complexity spikes. When they are kept separate, the system becomes tractable.

## 2) Primary Domain Components (Combinatorics Core)

Think in an axis product:

$$
\text{Surface} = \text{PipelineKind} \times \text{ExecutionMode} \times \text{Layer}
$$

Where:

- $\text{PipelineKind} \in \{\text{LinkPrediction},\ \text{NodeClassification},\ \text{NodeRegression}\}$
- $\text{ExecutionMode} \in \{\text{Create},\ \text{Configure},\ \text{FeatureBuild},\ \text{Train},\ \text{Predict(Stream/Mutate/Write)},\ \text{Estimate}\}$
- $\text{Layer} \in \{\text{Facade},\ \text{EvalTemplate},\ \text{Catalog},\ \text{Renderer/Result}\}$

The practical rule is: identify your coordinates before coding.

## 3) "Where Am I?" Trait Discriminator

Use this checklist whenever you open a file:

1. If you are in facade traits and request parsing, you are in API orchestration.
2. If you are in `PipelineExecutor` or `PipelineTrainAlgorithm`, you are in execution templates.
3. If you are in `TrainingPipeline` or node/link pipeline structs, you are in pipeline state and validation law.
4. If you are in `PipelineCatalog`, you are in symbol persistence and user scoping.
5. If you are in result builders/renderers, you are in output projection only.

If two or more apply, you likely crossed a boundary that should be made explicit.

## 4) Eval/Pipeline Mechanism: Operational Skeleton

Train path (model production):

1. Procedure train entrypoint parses config and resolves pipeline.
2. Pipeline validation runs against graph and selected labels/types.
3. Trainer executes model selection and training.
4. Result is converted into model artifact and persisted.

Predict path (model application):

1. Procedure predict entrypoint loads model/config.
2. Dataset splits and feature input filters are generated.
3. Node property steps run.
4. Feature properties are validated.
5. Inference runs, then output is shaped as stream/mutate/write.
6. Cleanup executes regardless of success/failure.

This train/predict bifurcation is the core discriminant to preserve.

## 5) Facade and Builder Capability Frame

For long-term platform evolution, treat facades/builders as capability contracts:

- Facades define stable procedural speech acts.
- Builders/companions define legal construction of pipelines and executor contexts.
- Eval templates define invariant execution order.
- Pipeline structs define mutable state plus validation constraints.

A healthy evolution pattern:

1. Add capability by extending pipeline state + validation first.
2. Wire eval template behavior next.
3. Expose through facade only after invariants are test-covered.
4. Keep result shaping independent from pipeline mutation logic.

## 6) Projection Workbench Tracks (Proposed)

These tracks are designed as top-level domains that can later be reflected into Shell ProgramFeature and TaskFrame modeling.

1. `pe-map-surfaces`
   - Focus: classify any function by (kind, mode, layer).
   - Outcome: 30-second "where am I" orientation.
2. `pe-train-template-law`
   - Focus: internalize `PipelineTrainAlgorithm.compute` sequence and invariants.
   - Outcome: no accidental train path regressions.
3. `pe-predict-template-law`
   - Focus: internalize `PipelineExecutor.compute` sequence and cleanup law.
   - Outcome: no orphaned intermediate properties.
4. `pe-facade-boundary-discipline`
   - Focus: keep request/result orchestration in procedures, execution in eval.
   - Outcome: no layer inversion.
5. `pe-catalog-symbolics`
   - Focus: reason clearly about pipeline identity, type erasure, and typed retrieval.
   - Outcome: fewer runtime type mismatch surprises.
6. `pe-shell-taskframe-projection`
   - Focus: represent eval/pipeline tracks as ProgramFeature and TaskFrame slices.
   - Outcome: operator-facing shell menu for pipeline/procedure development.

## 7) Shell and TaskFrame Integration Hypothesis

Mirror the existing workbench catalog style used for Shell and Task:

- define stable track ids
- attach exemplar files
- expose catalog text rendering for operator inspection

Projection-specific shell payload can be modeled as:

$$
\text{ProgramFeature}_{PE} = \langle \text{TrackId},\ \text{ExecutionMode},\ \text{BoundaryLaw} \rangle
$$

TaskFrame payload can then bind concrete runtime commitments:

$$
\text{TaskFrame}_{PE} = \langle \text{InputGraph},\ \text{PipelineSymbol},\ \text{Mode},\ \text{OutputContract} \rangle
$$

This keeps conceptual architecture aligned with executable procedure surfaces.

## 8) Practice Protocol (How To Use This Weekly)

1. Pick one track id.
2. Choose one concrete pathway (for example: node classification train).
3. Walk the files in order: facade -> eval template -> pipeline state -> result builder.
4. Write one boundary assertion: "this file may do X, may not do Y."
5. Add or adjust one test that protects that assertion.

After 4-6 sessions, the architecture becomes pattern memory rather than a large text blob.

## 9) Source Anchors

- `gds/src/procedures/pipelines/facades.rs`
- `gds/src/projection/eval/pipeline/pipeline_executor.rs`
- `gds/src/projection/eval/pipeline/pipeline_train_algorithm.rs`
- `gds/src/projection/eval/pipeline/training_pipeline.rs`
- `gds/src/projection/eval/pipeline/pipeline_catalog.rs`
- `gds/doc/PIPELINE_EXECUTION_MENTAL_MODEL.md`
- `gds/doc/PIPELINE_FACADE_ARCH_NOTE.md`
- `gds/src/shell/workbench/catalog.rs`
- `gds/src/task/workbench/catalog.rs`
