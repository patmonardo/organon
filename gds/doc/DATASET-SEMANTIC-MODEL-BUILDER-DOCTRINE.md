# Dataset Semantic Model Builder Doctrine

Date: 2026-05-01

This note records where the Dataset module stands and what it is trying to
become. The short version is this:

Dataset is no longer only a convenient wrapper around DataFrame. It is the
semantic model builder of GDS. DataFrame is the executable tabular body;
GraphFrame and GML/GNN systems are downstream rupa bodies; the Dataset layer is
the nama world where models, features, annotations, corpora, programs, and
lowering intent are named before they become runtime shape.

The scope is intentionally narrow. This is not a business-intelligence dataset
stack and not a general-purpose programming dataset toolkit. If the goal is not
AI model building, ontology engineering, and epistemic execution design, this
platform is probably the wrong tool.

That is the edge of the platform. We are not building one more graph machine
learning framework. We are building the meta platform that lets many model
worlds become readable, comparable, lowerable, and serviceable through one
semantic substrate.

## Thesis

A Dataset in this package means a Martin Fowler style Semantic Model Building
Dataset: a named semantic carrier that owns the language of a domain before it
chooses a runtime image.

This differs from the usual data science sense of dataset. A normal dataset is
a pile of rows, files, arrays, tables, or examples. A GDS Dataset is a semantic
program surface:

- It can still hold rows and columns through Polars-backed DataFrame tables.
- It also carries features, model specifications, corpus structure, annotations,
  feature structures, toolchains, compilation graphs, and generated artifacts.
- It treats DataFrame lowerings as generated images of semantic intent, not as
  the first or final truth of the system.
- It gives the Applications facade a stable TS-JSON service surface without
  forcing every user to think in Rust internals.
- It gives the GDS Program Feature system a place to materialize program images
  as Dataset artifacts.

The center of gravity is ontological and epistemological, not linguistic in a
general NLP-platform sense. NLP components are used because they provide strong
first principles for semantic marking and feature-structure algebra, but the
target is PureForm execution: ontologies that define how they execute.

The practical doctrine is simple: Dataset names and prepares the model world;
DataFrame executes the table image; GraphFrame/GML/GNN consume graph bodies when
the semantic model asks for them.

## Where The Module Stands

The current module already has the core organs of the Semantic Model Builder.

1. The Dataset wrapper is lightweight but real. `Dataset` wraps `GDSDataFrame`,
   carries an optional name, and tracks an artifact profile. It gives ordinary
   table operations a semantic boundary without forcing callers to use Polars
   directly.

2. The Dataset DSL shell mirrors the Polars top-level matrix: `Expr`,
   `LazyFrame`, `DataFrame`, and `Series` each receive dataset-flavored
   namespace access. This makes the API readable as modern Polars while keeping
   Dataset vocabulary separate from the lower DataFrame vocabulary.

3. The module has a curated prelude. The prelude already exports Dataset,
   Corpus, Plan, Feature, Model, ToolChain, registry, tokenizer/parser/tagger,
   tree, feature-structure, language-model, metrics, codegen, compilation, and
   ontology-image utilities. This is the right place for examples to read like
   the modern API.

4. The NLP-first layer is present. Token, tokenizer, stem, tag, tagger, parse,
   parser, tree, language model helpers, corpus readers, tgrep, probability,
   and NLTK-style preprocessing are not decoration. They are the first semantic
   modeling substrate.

5. Feature structures are the algebraic jewel. `FeatStruct`, `FeatValue`,
   reentrance, variables, unification, subsumption, parsing, and formatting give
   the Dataset layer a symbolic algebra that can express feature marks before
   they become columns or graph nodes.

6. Model and Feature have been recast beyond ordinary ML naming. The code now
   distinguishes compute models from R4 model-as-valuation, and treats Feature
   as a named typed address with projection, binder, reentrancy, and annotation
   roles. This is the correct semantic center.

7. Model preparation already has a three-box form processor shape: essence
   resolution, feature execution, and image realization. It uses FeatStruct
   unification to stamp features with modality before lowering. This is where
   the system becomes more than a DataFrame helper.

8. ToolChain is the current UserLand bridge. It holds SDSL specifications,
   genus/species classification, GDSL source, model refs, feature refs, logical
   engine intent, MVC engine intent, and declarative Dataset pipelines. It can
   lower operations into DataFrame expressions and build Dataset artifacts.

9. Compilation is already Dataset-centered. Program features can be compiled
   into artifact, relation, and property datasets, then registered. This is the
   bridge from GDS Program Feature into the semantic model builder.

10. Applications has a Dataset-first Collections facade. It serves catalog,
    ingest, schema, preview, feature evaluation, attention report, capabilities,
    GDSL compilation, and materialization over TS-JSON. That is the beginning of
    the non-Rust service surface.

11. GraphFrame is deliberately not the center. It is experimental and feature
    gated. That is correct. GraphFrame should be a generated or opt-in body for
    graph-shaped interpretations, not the semantic source of truth.

The old short doctrine, `SEMANTIC-DATASET.md`, still says something true: this
is an NLP extension of Polars. The newer truth is stronger: it is an NLP-first
semantic model builder that can generate DataFrame, GraphFrame, Applications,
and Program Feature images.

## Nama And Rupa

The platform should keep two poles distinct.

Nama is the name-forming, model-forming side:

- Corpus, Document, Source, Annotation
- Feature, Feature role, Feature structure, Feature schema
- Model as valuation, ModelSpec, ModelSpace
- Plan, ToolChain, SDSL specification, GDSL source
- ProgramFeature images and ontology DataFrame images
- provenance, modality, contradiction, attention, report

Rupa is the embodied runtime side:

- DataFrame columns and expressions
- LazyFrame execution plans
- GraphFrame node and edge tables
- GML/GNN input matrices, tensors, batches, loaders, and training loops
- external ML runtimes such as PyTorch Lightning style trainers
- serving surfaces and application records

The doctrine is not to dissolve one into the other. Nama should remain readable
as semantic intent. Rupa should remain efficient, inspectable, and generated
from the semantic intent. The system wins when a user can stand at the Dataset
boundary and see both: the meaning model and its runtime body.

## Relation To GML And GNN Work

This package should not try to become the one framework that absorbs GML, GNN,
knowledge graph, RDF, graph analytics, and neural training. That move would
flatten the architecture.

Instead:

- Dataset is the meta world where semantic model intent is authored.
- GraphFrame is one rupa image of that intent.
- GML/GNN pipelines are consumers of graph images, feature matrices, and
  metadata produced from Dataset specifications.
- The platform should preserve enough provenance and feature-structure metadata
  that graph learning results can be interpreted back through the Dataset model.
- GML/GNN models should be represented as model specs and generated artifacts,
  not as the privileged ontology of the whole system.

This is more interesting than an object-oriented graph platform because it does
not begin with objects or graphs. It begins with semantic articulation and lets
objects, tables, graphs, tensors, services, and reports appear as images.

## The Source Lineage

The module is a synthesis of several traditions, each with a specific job.

Polars contributes the runtime algebra: expressions, lazy plans, DataFrames,
selectors, joins, scans, grouping, projection, and columnar execution.

NLTK contributes the first semantic modeling world: corpora, tokenization,
tagging, parsing, trees, language models, feature structures, and feature
grammar. The FeatStruct algebra is especially important because it gives the
system a small symbolic logic that can be lowered without being erased.

Fowler-style DSL thinking contributes the separation between domain language,
semantic model, and runtime interpretation. Dataset is where the semantic model
is built before it is executed.

Skrub contributes the stdlib intuition: practical dataset builders, adapters,
normalization workflows, tabular cleaning, and pragmatic data preparation
should become reusable Dataset resources rather than one-off scripts.

PyTorch Lightning contributes the orchestration intuition: training, fitting,
evaluation, callbacks, reports, and reproducible runs should be expressed as a
clean lifecycle surface without making Dataset itself a neural framework.

GDS Program Feature contributes the kernel-facing program image: Dataset must
serve program features as materialized semantic artifacts that can be cataloged,
inspected, and lowered.

## Finish-Line Architecture

The finish line is not a giant rewrite. The module already has the outlines.
The finish line is a disciplined vertical integration.

1. Stabilize the modern prelude examples.
   Examples should read through `gds::collections::dataset::prelude::*` and,
   when needed, `gds::collections::dataset::namespace::*`. They should show
   Dataset as the semantic authoring layer and DataFrame as the analytic body.

2. Make `DatasetPipeline` the ordinary narrative API.
   Pipeline examples should show IO, metadata, text lifecycle, feature/model
   refs, projection, reporting, DataFrame lowering, and compilation. The reader
   should be able to infer the whole platform from a small example.

3. Promote FeatStruct marks into the visible model-building story.
   The examples should show feature structures as essence marks, unification as
   preparation, modality as the execution decision, and image realization as the
   record of what happened.

4. Close the three-box execution loop.
   Box 1 prepares model essence. Box 2 lowers executable marked features into
   DataFrame/LazyFrame plans. Box 3 realizes ontology image tables with
   provenance, constraints, contradictions, and feature rows. The code already
   has pieces of this. The next work is to make the path legible end-to-end.

5. Make Applications the reviewable service surface.
   The TS-JSON Collections facade should remain Dataset-first: catalog,
   preview, schema, eval, attention, capabilities, compile, materialize. The
   service should not leak GraphFrame or internal algo modules as the user's
   first vocabulary.

6. Treat GDS Program Feature as a served Dataset artifact.
   Program features should compile to Dataset artifact tables, relation tables,
   property tables, and ontology images that can be registered, listed,
   previewed, and projected.

7. Add GraphFrame/GML output as projection, not as center.
   The graph world should be reached by `project_graph` or equivalent pipeline
   stages. Its outputs should retain links back to Dataset feature names,
   model specs, source documents, annotations, and provenance.

8. Add Dataset stdlib adapters gradually.
   Skrub-like data preparation, common corpora, resource catalogs, and practical
   cleaning transforms should enter as Dataset resources and functions. Keep
   heavyweight ML/NLP models outside the core until the boundary is deliberate.

9. Keep heavy ML as extension territory.
   Advanced tokenizers, neural encoders, dependency parsers, embedding models,
   graph neural networks, and trainer frameworks should consume Dataset
   artifacts and return Dataset-compatible reports. They should not define the
   Dataset core.

10. Preserve the semantic trace.
    Every lowering should answer: which model, which feature, which corpus,
    which document span, which annotation provenance, which program feature,
    which modality, which output artifact?

## Example Upgrade Program

The examples should be the readable constitution of the module. The current set
is already close, but it should be curated into a sequence.

Recommended sequence:

1. `collections_dataset_frame_dsl.rs`
   Show the ergonomic modern API: Dataset name, `.ds()` namespace, pipeline,
   lowerings, compilation.

2. `collections_dataset_corpus_readers.rs`
   Show source ingestion and corpus as the first semantic object, not just raw
   text loading.

3. `collections_dataset_tree.rs`
   Show the NLTK inheritance clearly: trees, spans, parse values, and tree DSL.

4. `collections_dataset_featstruct_model.rs`
   Show FeatStruct parsing, unification, feature marks, model preparation, and
   modality stamping.

5. `collections_dataset_compile_ir.rs`
   Show ProgramFeature to DatasetCompilation to artifact datasets to registry.

6. `collections_dataset_gdsl_absolute_concept.rs`
   Show GDSL source lowering into program features and materialized artifacts.

7. `collections_dataset_applications_expository.rs`
   Show the TS-JSON service boundary as the user-facing platform story.

8. New example: `collections_dataset_graph_projection.rs`
   Show graph projection as a generated body with provenance preserved, without
   making GraphFrame the primary source.

Each example should answer one question and print enough interpretation to be
read as expository code. These are not just smoke tests. They are the living
manual for the modern API.

## Guardrails

- Keep Dataset as the semantic center above DataFrame.
- Keep DataFrame lowerings generated, inspectable, and ordinary Polars-shaped.
- Keep GraphFrame opt-in until the graph projection path is mature.
- Keep Applications calls procedure/service-oriented and do not expose internal
  algo modules from examples.
- Keep the prelude curated; do not export every internal builder just because it
  exists.
- Keep NLP primitives in core when they are lightweight semantic carriers.
- Move research-heavy NLP and neural methods to extension modules when they
  become runtime/model commitments.
- Keep feature structures first-class; do not collapse them into JSON blobs or
  anonymous columns.
- Keep provenance mandatory for annotations and visible in realized images.
- Keep Program Feature support catalogable and previewable through Dataset.

## Near-Term Batches

Batch 1: examples as doctrine

- Refresh the existing Dataset examples so they all use the modern prelude and
  namespace surface.
- Add the missing FeatStruct/Model preparation example.
- Ensure `cargo check -p gds --examples` stays green.

## May Execution Board

This doctrine launches the May work as an examples-first implementation arc.
The examples are the agent-facing entry into the platform: each one should make
one part of Absolute Interpretability executable and reviewable.

- [x] Create the central Semantic Model Builder doctrine.
- [x] Add `collections_dataset_featstruct_model.rs` as the Box 1 -> Box 2 ->
   Box 3 walkthrough for FeatStruct marks, modality, execution receipts, and
   ontology-image realization.
- [ ] Curate the existing Dataset examples into the recommended reading order.
- [ ] Modernize any older Dataset examples that bypass the prelude or namespace
   shell without a good reason.
- [ ] Add graph projection as a Dataset-generated body while preserving the
   Dataset model trace.
- [ ] Extend the Applications expository example so Program Feature artifacts
   are cataloged, previewed, and read as Dataset service outputs.
- [ ] Keep `cargo check -p gds --examples` green after each batch.

Batch 2: three-box vertical slice

- Connect `prepare_model` outputs to feature execution examples.
- Realize the image tables with modality/provenance/contradiction records.
- Make the trace from mark to DataFrame artifact obvious.

Batch 3: Program Feature service story

- Extend the Applications expository example so ProgramFeature materialization
  is visibly a Dataset service, not a side path.
- Add catalog previews for artifact, relation, and property datasets.

Batch 4: graph projection without graph capture

- Add graph projection as one Dataset pipeline output.
- Preserve source feature names, model IDs, provenance, and artifact profiles.
- Keep GML/GNN examples downstream of this projection.

Batch 5: stdlib and adapter ecology

- Add small Skrub-like data-prep adapters where they clarify Dataset workflows.
- Keep each adapter readable as Dataset authoring, not as generic ETL sprawl.

## Final Doctrine

Dataset is the semantic model builder of the GDS kernel. It is NLP-first because
language gives the system its first model of naming, marking, parsing,
annotation, feature structure, and valuation. It is DataFrame-generating because
semantic intent must become efficient tabular execution. It is GraphFrame-ready
because some semantic models need graph bodies. It is ProgramFeature-serving
because the kernel needs its own program images to become inspectable artifacts.

The module reaches the finish line when a user can author a semantic model,
prepare its feature essence, lower it to DataFrame and graph bodies, serve it
through Applications, and still read the original names, features, annotations,
provenance, and program commitments in the resulting artifacts.

That is the Meta Platform: not one model type, but the disciplined place where
model types can be named, built, lowered, served, and interpreted.
