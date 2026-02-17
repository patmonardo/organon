# Organon GDSL Specification (Draft)

> This document captures the current understanding of the Organon FormDB/GDSL alignment, Program Features, and how they interface with the GDS kernel and the TypeScript shape engines.

## 1. Purpose

- Define the _FormDB_ implementation surface that the TypeScript engines drive with the GDSL `Form/Context/Morph` shapes.
- Record how the GDSL Program Feature slots into FormDB, how the engines stay dialectic-agnostic, and how clients may inspect the dialectical IR through service APIs.
- Anchor the overall runtime design so each FormEval run becomes a deterministically ordered orchestration of Program Features backed by GDS graph algorithms.

## 2. Core GDSL Concepts

| GDSL Surface | Runtime Counterpart | Notes                                                                      |
| ------------ | ------------------- | -------------------------------------------------------------------------- |
| `Form`       | `Entity`            | Identity-bearing determinacy; what gets instantiated from `FormShape`.     |
| `Context`    | `Property`          | Conditions and admissibility horizons; represented by `ContextShape`.      |
| `Morph`      | `Aspect`            | Mediation and transformation; `MorphShape` drives transitions/aspect APIs. |

Each compiler module in `logic/src/relative/core/compiler` translates a dialectic IR into one of these GDSL shapes. The engines only expose these shapes (plus `FormShape` derivatives) to consumers.

## 3. FormDB Architecture

- **FactStore**: stores the canonical `ProgramFeature` records (PureForm or GivenForm) that describe how a FormEval run should proceed. Each record includes the `ProgramFeature` id, metadata, the requested `GraphAlgo` target, and any input/output bindings.
- **ProgramFeature**: the GDSL contract executed by FormEval. It does not require a separate SDSL; it _is_ the GDSL program spec. At minimum, a ProgramFeature holds:
  - `featureId` (e.g. `program-feat:grounding`)
  - `type` (`pure` | `given`)
  - `graphAlgo` reference (one of the ~50 GDS GraphAlgo entries)
  - `inputs` map (FactStore bindings, dataframes, datasets)
  - `outputs` map (slots for results that feed Form shapes)
  - `dialecticHints` (optional metadata for service-level diagnostics, e.g., IR provenance)
- **FormEval** sequence: loads the requested ProgramFeature from the FactStore, resolves its inputs/outputs, invokes the GDS kernel via JSON GraphAlgo calls, and finally materializes the emitted Form/Context/Morph shapes for the TS engines to consume.

## 4. Program Feature Lifecycle

1. _Definition_: ProgramFeature records are created from the GDSL spec, either manually or via DSL tooling. They describe which GraphAlgo to run and how to hydrate the pipeline inputs.
2. _Execution_: FormEval picks a ProgramFeature, feeds its inputs to the GDS kernel (JSON API), receives the resulting Model/Shape bundle, and converts it into the shape engine outputs used downstream.
3. _Observation_: FormService exposes a `programFeatureHistory` that points to the fact store records plus any dialectical `DirectCompute` traces for debugging.

## 5. Service and API Layer

- The TS Form Engines remain specification-driven and emit `FormShape`, `ContextShape`, `MorphShape`, etc.
- Dialectical runtime (moments, forces, transitions, IR) is exposed only through the **Form Service API**, which includes:
  - `DirectCompute` helpers per engine (e.g., `foundationContext.directCompute(stateId)`), returning validated dialectic summaries.
  - `ProgramFeatureService` for clients to query ProgramFeature definitions and invoke them asynchronously.
  - `FactStore` queries to inspect the stored ProgramFeature states and their GDS outputs.
- These services are the only surface that advertises the dialectical interpreter; the engines themselves keep the IR hidden.

## 6. Integration with GDS Kernel

- Every ProgramFeature triggers a GDS GraphAlgo invocation. The JSON payload includes:
  - `algoId` (e.g., `graph/resolve-properties`)
  - `parameters` (dataset ids, FactStore references)
  - `returnPath` (which Form/Context/Morph shapes to populate)
- GraphAlgo outputs are normalized into GDSL FormDB shapes, and the FormService feeds them to the engines.
- ProgramFeatures can chain: the outputs of one GraphAlgo (e.g., computed invariant graph) become inputs for another ProgramFeature, allowing complex ML or NLP pipelines, all orchestrated through FormEval.

## 7. SDSLs (Optional)

- SDSL specs are primarily for client DSLs that extend the base GDSL shapes (e.g., the Reach-focused extension of the React dashboard FormShape).
- The SDSL form definition merely references a base `FormShape` and adds client-specific metadata or UI bindings.
- Any client wanting to invoke a GraphAlgo directly can do so through the ProgramFeatureService rather than redefining the core GDSL.

## 8. Next Work Items

1. Finalize the `FormEval` spec that sequences ProgramFeatures, FactStore reads, and GraphAlgo calls.
2. Document the GraphAlgo JSON contract in the GDSL spec so TS engines know what to expect from the kernel.
3. Define how the FormService exposes DirectCompute/dialectic debugging under a `dialectic` namespace, keeping the core shapes clean.
4. Detail how React/Reach dashboards consume the Reach SDSL (extending the base FormShape) without touching the dialectic IR.
