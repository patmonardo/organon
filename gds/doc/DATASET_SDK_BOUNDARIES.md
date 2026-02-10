# Dataset SDK Boundaries

Scope: Dataset SDK (core)

- Text-first dataset primitives: Corpus, Token, Stem, Tag, Parse, Tree, DependencyGraph.
- Lightweight, deterministic tokenizers/parsers: whitespace, regex, markup (XML/HTML), JSON.
- Minimal taggers and metrics for fast iteration and evaluation.
- Polars-friendly adapters for graph-like structures (edges/nodes frames).

Scope: ML/NLP (extension)

- Research-grade tokenizers, taggers, and parsers that are model-heavy or domain-specific.
- Advanced metrics, full corpora readers, and large-scale model tooling.
- Knowledge graphs, RDF, and GNN/GNC workflows (GraphFrame + ML/NLP).

Dependency graphs in core

- Keep DependencyGraph as a small, index-friendly structure for tree-to-graph transitions.
- Treat it as a constrained, hierarchical graph that fits Dataset/Polars joins.
- Full knowledge graphs and semantic stores belong to GraphFrame and ML/NLP.

Rationale

- The Dataset SDK should be a fast, stable semantic carrier for text corpora.
- Heavy or model-specific NLP belongs in ML/NLP to keep core lightweight.
