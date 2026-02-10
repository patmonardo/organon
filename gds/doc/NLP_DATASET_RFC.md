# NLP-First Dataset RFC

Summary

- Propose an NLP-first Dataset architecture for `gds` that treats a Dataset as a semantic carrier (corpus-centric), with lightweight token/stem/parse pipelines in-core and heavier ML/NLP algorithms moved to a separate `ml/nlp` extension.

Goals and boundaries

- Keep `gds` Dataset DSL lightweight and data-centric (ingest, registry, corpus, tabular/graph views).
- Provide stable token/stem/parse primitives and N-gram LM helpers in `gds::collections::dataset` (already present: `Corpus`, `token`, `stem`, `functions::model::lm`).
- Expose a small prelude (`gds::collections::dataset::prelude`) for common workflows.
- Do NOT bundle heavy ML model libraries or app-style GUIs (e.g., avoid becoming an app like Lightning). Those belong in `ml/nlp` or other packages.

Core concepts / types

- Corpus: single-column text dataset (existing `Corpus`) — primary ingestion primitive.
- Dataset: tabular/graph-backed dataset with expression DSL and plan support (existing `Dataset`).
- Token / Stem / Parse: canonical lightweight types for tokenization/stemming/parse spans (`token::Token`, `stem::Stem`, `parse::Parse`).
- Feature structures: typed, queryable feature containers (`featstruct`, `feature`).
- Registry: `DatasetRegistry` for dataset families and artifacts.

Pipelines and APIs

- Pluggable pipeline pattern: small core pipeline traits and sync/async adapters:
  - Tokenizer: returns Vec<Token> or Series; default whitespace tokenizer provided in `Corpus`.
  - Stemmer: maps Token -> Stem; default no-op/porter-compat shim can be provided.
  - Parser: maps tokens/stems -> `Parse`/`ParseForest` objects.
  - Feature extractor: transforms Dataset -> FeatureSeries / FeatureStructs.

Examples (Rust sketch)

- Tokenize a `Corpus` and materialize tokens as a column via dataset expressions (use `Corpus` → expression helpers in `expr` / `frame`).

Extension separation: `ml/nlp`

- Create `ml/nlp` as a separate package for heavy algos and research code (e.g., advanced tokenizers, embeddings, neural models, NER, dependency parsers).
- `ml/nlp` depends on `gds` for dataset primitives and exposes dataset extension helpers (e.g., `Dataset::to_token_series()`, `Dataset::to_rdf_graph()`).

Skrub as Dataset stdlib

- Treat Skrub as the canonical dataset stdlib (collection of dataset builders, format adapters, and standardized corpora loaders) — library, not an app.
- Provide adapters in `gds` to load Skrub artifacts into `Corpus`/`Dataset` and register them via `DatasetRegistry`.

RDF / GraphFrame roadmap

- Provide an export path from `Dataset`/`Corpus` -> RDF graph (GraphFrame) to support semantic graphs and GNNs.
- Implement as adapter layer in `gds::collections::graphframe` or as an opt-in feature behind `graphframe`/`rdf` features.

Testing & examples

- Add unit tests for tokenizer/stemmer adapters and N-gram LM helpers (use existing tests in `functions::model::lm`).
- Provide small examples under `gds/examples` demonstrating: ingest text dir → corpus → tokenize → feature extraction → export RDF.

Next steps

- Implement a pluggable tokenizer trait and a default whitespace tokenizer in `gds` (`dataset::tokenizer`).
- Create the `ml/nlp` package scaffold and move advanced algorithms there.
- Add Skrub adapter functions under `gds` to register common datasets in `DatasetRegistry`.

Decision notes

- Keep the core dataset DSL stable and small; move heavy ML/NLP into `ml/nlp` to avoid coupling and to make `gds` suitable as a dataset stdlib.
