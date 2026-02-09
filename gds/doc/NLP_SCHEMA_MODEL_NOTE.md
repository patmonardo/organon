# NLP Schema and Model/Feature/Tree Notes

This note is a seed design pass for the dataset DSL. The claim: our fundamental NLP object looks like an XML/HTML-style document schema, and the dataset layer is an analytic lens on top of that schema. The DSL surface (Token, Tag, Parse, Tree, FeatStruct) is closer to XML than to typical ML feature tables. NLTK is a deep source of NLP-first ideas that align with a Semantic Web orientation. The goal is parity with NLTK notions of models and features, without inheriting the narrow Statistical/NN framing that dominates modern ML.

## 1. Fundamental Schema (NLP-as-XML / RDF)

We treat the NLP object as a typed, nested document with explicit spans and attributes. This is not a storage format choice; it is the semantic ground truth. The same object can be viewed as XML-style trees or as RDF-style graphs.

- Node: typed element with attributes and children.
- Span: byte or token span for anchoring to raw text.
- Attribute: key/value pairs, possibly nested (FeatStruct).
- Reentrance: shared substructure by id (like XML ID/IDREF).
- Triple view: node + edge + node, with attributes on edges and nodes.

Schema sketch (conceptual, not syntax):

- Document
  - RawText (optional)
  - Nodes[]
- Node
  - Type (label)
  - Span? (byte or token)
  - Attrs (FeatStruct)
  - Children[]
  - Id? and Refs[]

The schema is a single graph with tree-like views. Every dataset artifact (token tables, tags, parses) is a projection of this schema.

This makes the dataset a semantic graph (RDF-like), where XML-style trees are just a convenient projection. Tokens, tags, and parses are not separate data sources; they are views on the same graph.

## 2. The Big Three: Models, Features, Trees

Upper-level design that binds the DSL to analytic workflows:

- Models: compute or infer structure over the schema. A model is an operator over the schema graph, not just a function from arrays to labels.
- Features: views of attributes and spans that can be surfaced as tables or expressions. Features are a stable, typed lens, not arbitrary columns.
- Trees: the canonical structural view. Trees are the grammar of the document graph and the object that aligns with NLTK, parse pipelines, and semantic annotation.

### Model (NLTK parity)

A model is a structured transformation from schema state to schema state. This matches NLTK's model notion: composable, inspectable, and grounded in explicit linguistic structure.

Model concept in this DSL:

- Input: schema graph + view selection (tokens, spans, nodes).
- Output: schema graph updates (new nodes, edges, or attributes).
- Traceability: model outputs are always linked back to the source spans and nodes.

Examples:

- Tagging model: adds or edits Tag nodes under a span.
- Parsing model: produces Tree nodes with typed edges.
- Semantic model: enriches FeatStruct attributes (e.g., role, sense, polarity).

Model output is always re-embeddable into the schema, not just stored separately. This prevents the ML pattern where models output detached labels that lose semantic grounding.

### Feature (NLTK parity)

A feature is a typed projection of schema content into a stable view. In NLTK, features are structured (FeatStruct) and not just scalar columns. We keep that: features are structured and traceable.

- Attribute features: FeatStruct fields and paths.
- Span features: token/byte offsets, length, overlap.
- Node features: label, position, adjacency, ancestry.
- Graph features: edge types, path constraints, reentrance patterns.

Feature projection is meant to be reversible or traceable back to the schema node. Feature extraction is not "lossy featurization"; it is a view of the schema.

### Tree

Trees are the dominant structural form. Tokens, tags, and parses are all tree slices:

- Tokens: leaves with spans and text.
- Tags: attributes on token or span nodes.
- Parses: nonterminal nodes with structure and attributes.

This mirrors XML with element nesting and attributes, with reentrance as IDREF. It also mirrors RDF in a constrained, ordered, tree-shaped projection.

## 3. Mapping DSL Types to the Schema

- Token: leaf node with span + text; attrs for kind, normalization.
- Tag: attribute or child node bound to span or token index.
- Parse: tree node with label + span + attrs; edges define hierarchy.
- Tree: explicit view of node graph with ordering and parentage.
- FeatStruct: attribute payload, including reentrance and variable bindings.

These are not independent types; they are views that preserve the schema core. The dataset DSL should enforce this invariance.

## 4. Why NLTK Matters

NLTK treats feature structures and trees as first-class, semantic objects. That is closer to a Semantic Web document than to a vectorized ML pipeline. The DSL should preserve:

- Typed nodes and attributes.
- Reentrance and shared structure.
- Unification and variable binding as schema-level operations.

## 5. Substrates: DataFrame and GraphFrame

DataFrame and GraphFrame are substrates, not foundations.

- DataFrame: convenient for projections, aggregation, and classical statistical workflows.
- GraphFrame: closer to the semantic core, since the schema is a tagged graph.

We should treat GraphFrame as the primary semantic substrate, with DataFrame as a projection backend. The GraphFrame DSL is the first extension of the DataFrame DSL, but it must remain centered on the schema graph.

## 6. NLTK Model/Feature Parity Checklist

Model parity:

- Models are schema-to-schema transforms.
- Models keep provenance via spans and node ids.
- Models compose cleanly and remain inspectable.
- Models can emit feature structures directly.

Feature parity:

- Features include FeatStruct paths and nested values.
- Features preserve reentrance and variable bindings where needed.
- Features can be viewed as tables without losing semantics.

## 7. Model Interface Sketch (Seed Pass)

This is a lightweight interface sketch that mirrors NLTK-style composable models
while preserving schema grounding.

- ModelSpec
  - id: stable model identifier
  - kind: tagger, parser, semantic, language_model, feature_model, composite
  - input/output: views over the schema (tokens, tags, parses, trees, features)

- ModelContext
  - options: key/value feature map (FeatStruct values)

- ModelDelta
  - tags: tag insertions or edits
  - parses: parse insertions or edits
  - trees: tree insertions or edits
  - attributes: FeatStruct path updates

- ModelResult
  - delta: ModelDelta
  - notes: warnings, trace info

- Model
  - spec() -> ModelSpec
  - apply(dataset, context) -> ModelResult

- Train/Fit hooks (optional)
  - fit(dataset, context) -> ModelState
  - train(dataset, context) -> ModelState
  - update(state, dataset, context) -> ModelState

- Evaluate hooks (optional)
  - score(dataset, context) -> ModelScore
  - evaluate(dataset, context) -> ModelReport

This keeps the model output as schema updates, not detached labels. The fit/train
hooks let models learn from dataset state while remaining schema-grounded. The
evaluate hooks allow parity with NLTK's scoring and diagnostics, without forcing
everything into a DataFrame pipeline.

## 8. Design Direction (Next Steps)

- Formalize the schema types as a minimal interface (Document, Node, Span, Attrs).
- Keep Token/Tag/Parse/Tree as views over the same core schema graph.
- Ensure Model outputs are always inserted back into the schema.
- Maintain FeatStruct as the canonical attribute payload for semantic content.
- Draft the GraphFrame DSL as a first-class semantic substrate.
- Add a standard library plan: NLTK extraction + Skrub integration as DSL utilities.

## 9. Boundary: Dataset DSL vs ML/NLP Subsystem

The Dataset DSL is the core semantic client and SDK. It owns the schema, text
views, and deterministic transforms. It uses DataFrame and GraphFrame as
substrates, not as foundations.

Dataset DSL (core, stays here):

- Corpus and text primitives as the canonical data root.
- Schema views: Token, Tag, Parse, Tree, FeatStruct.
- Feature extraction as structured views (schema projections).
- Model interface and deltas (schema-to-schema transforms).
- Deterministic text processors (tokenize, normalize, segment).
- GraphFrame and DataFrame projections, with GraphFrame as the semantic default.

ML/NLP Subsystem (separate, later):

- Statistical and neural algorithms.
- Training, optimization, checkpoints.
- Learned taggers/parsers/language models.
- Evaluation suites, metrics, hyperparameter search.
- NLTK/Keras algorithm parity as a dedicated library surface.

Guiding rule:

- If it defines or preserves schema structure, it lives in Dataset.
- If it learns parameters or performs heavy optimization, it moves to ML/NLP.

## 10. NLTK Parity Mapping to ML/NLP Subsystem

The ML/NLP subsystem will carry algorithmic parity with NLTK (and later
comparisons against GDS-native ML). The Dataset DSL remains the semantic
carrier; the ML/NLP subsystem implements the algorithms that operate on it.

Scope mapping (initial):

- Language models: ngram counts, smoothing, scoring, perplexity.
- Taggers: rule-based, statistical, and sequence taggers.
- Parsers: chart/earley/cyk style parsers and probabilistic variants.
- Classifiers: Naive Bayes, maxent/logistic, decision lists.
- Text processing: stemmers, tokenizers, segmenters as algorithmic modules.

Integration contract:

- Algorithms consume Dataset views (tokens, tags, parses, trees).
- Algorithms emit ModelDelta updates or learned ModelState artifacts.
- Evaluation returns ModelScore/ModelReport using Dataset-native metrics.

This keeps learned algorithms out of the Dataset core while preserving a clean
semantic pipeline.

## 11. Dataset Std Library (Skrub)

Skrub integration lives in the Dataset std library as a pragmatic bridge for
"normal dataset" workflows. It should provide:

- Feature and schema adapters (Skrub → Feature/FeatureSpace).
- DataFrame projection helpers for tabular pipelines.
- Low-friction preprocessing utilities (imputation, encoding, scaling).

Skrub stays in the Dataset layer only as a convenience adapter; learning
algorithms remain in the ML/NLP subsystem.

This note is intentionally short and open to critique. It should evolve into a formal schema spec and mapping guide for DSL components.
