# ADR: Semantic Dataset Architecture

## Status
Draft

## Context

The current `Dataset` struct in GDS is a minimal, Polars-backed wrapper for ergonomic, dataset-first workflows. However, as the project evolves, the notion of a "Dataset" is expected to become more semantic, domain-oriented, and feature-rich. The true value of a Dataset is not just as a DataFrame wrapper, but as a framework for building, managing, and reasoning over Corpus/Language Module (LM) duos.

## Decision

- The Dataset module will serve as the foundation for a semantic, corpus-centric architecture.
- Polars DataFrame is the efficient substrate, but not the conceptual center.
- The Dataset API will focus on providing access to and coordination of Corpus artifacts (documents, annotations, features) and LM vocabularies/models.
- Users who want raw DataFrame power can use the DataFrame DSL directly; the Dataset API is for corpus, LM, and semantic artifact workflows.
- The Dataset will remain lightweight and foundational, but is designed to evolve as the architecture and use cases mature.
- The linkage between Dataset and DataFrame is intentionally minimal and flexible, to avoid over-coupling and to support future semantic extensions.

## Consequences

- The Dataset module is not a universal or final definition of "Dataset"; it is a seed for future evolution.
- The architecture will support both granular (per-DataFrame) and holistic (domain-wide) analysis.
- The system is positioned to become a true semantic, linguistic, and analytic platform, not just a DataFrame wrapper.
- Future contributors should treat the Dataset as a coordination point for corpus/LM-centric workflows, and expect the API to evolve as the system's needs become clearer.

## Dataset Intelligence: Corpus + LM = DataFrame Mastery

The core intelligence of the Dataset module is realized through the interplay of Corpus artifacts and Language Modules (LMs):

- **Corpus**: Structured, annotated, and feature-rich data—documents, annotations, feature tables, and more. Corpora are the semantic surfaces on which analysis, modeling, and reasoning operate.
- **Language Module (LM)**: The vocabulary, n-gram model, or other language understanding asset that provides linguistic and semantic context for the domain.

Together, Corpus and LM enable:
- Building, managing, and orchestrating DataFrames as semantic artifacts.
- Analyzing, transforming, and reasoning over data using both DataFrame-native and semantic (corpus/LM) operations.
- Natural language interfaces (e.g., text2sql) that bridge human intent and DataFrame logic.
- Domain-aware analytics, feature engineering, and ML/NLP workflows that go beyond raw data manipulation.

This dual focus elevates the Dataset module from a simple data container to an active, intelligent agent for semantic data science and advanced NLP/ML pipelines.

## Feature Graphs vs. Property Graphs: Semantic Power and Normal Forms

A key architectural insight is the distinction between Feature Graphs and traditional Property Graphs (as in Cypher/Neo4j):

- **Property Graphs** are designed for flexible, schema-optional storage of nodes and relationships with arbitrary key-value properties. They excel at traversals and attribute filtering, but do not natively express or enforce relational concepts like normal forms, functional dependencies, or feature decomposition.
- **Feature Graphs**, as envisioned here, are semantic structures that can encode dependencies, normalization, and compositional logic—enabling both normalization (as in SQL) and advanced feature engineering. They can represent not just relationships, but also the semantic and structural constraints that underlie data organization and reasoning.
- Feature Graphs bridge the worlds of tabular (SQL/Polars), graph (property/Cypher), and semantic (feature, dependency, normalization) reasoning, providing a foundation for analytic, synthetic, and dialectical workflows.

This makes Feature Graphs uniquely powerful for semantic data science, as they can:
- Express and enforce normal forms and feature decompositions
- Support both normalized and denormalized views
- Enable advanced reasoning, annotation, and transformation of data structures

By making Feature Graphs a first-class concept in the Dataset, the architecture gains a new level of semantic expressiveness and analytic power, beyond what is possible with property graphs alone.

## Descriptive Graphs, Feature Graphs, and FeatStruct: Master Objects for Semantic Modeling

While Neo4j GDS and property graphs excel at large-scale graph algorithms and ML, our focus is on Descriptive Graphs and Feature Graphs as "master objects" for semantic modeling:

- **Descriptive Graphs** are designed to represent, relate, and reason about features, dependencies, and semantic structure—not just for computation, but for meaning and context.
- **Feature Graphs** generalize the idea of NLTK's FeatStruct, supporting rich, multi-relational, and hierarchical information. They are central to the semantic dataset, enabling advanced feature decomposition, annotation, and reasoning.
- These graphs serve as the organizing backbone in a distributed graphstore world, coordinating corpora, LMs, and semantic artifacts across the system.
- While they may not offer the same scale or algorithmic focus as Neo4j GDS, their semantic power and expressiveness make them uniquely valuable for NLP, knowledge representation, and domain modeling.

This approach positions Descriptive and Feature Graphs as foundational, "master objects"—bridging linguistic, tabular, and graph-based reasoning in a way that is both expressive and extensible.

## Metaphysical Perspective: Nama-Rupa and the Dual Graph Strategy

This architectural distinction has a metaphysical resonance:

- **Nama** (Name/Speech/Feature): The world of language, features, and meaning—prior to and foundational for the world of objects. Descriptive and Feature Graphs operate in this realm, focusing on semantic structure, relationships, and the articulation of meaning.
- **Rupa** (Form/Object/Property): The world of objects, large-scale structures, and analytic power. Property graphs and large-scale GML/Neo4j GDS operate here, enabling traversal, computation, and analysis at scale.

In this view, the Dataset architecture bridges Nama and Rupa: it grounds the analytic, object-centric world in the semantic, feature-centric world of language and meaning. This dual strategy is both practical and philosophically coherent, ensuring that the system is powerful, expressive, and deeply rooted in the logic of language and knowledge.

## Next Steps

- Continue to keep the Dataset module foundational and minimal, but document its intended evolution.
- As the architecture matures, extend the Dataset API to support corpus/LM management, feature graphs, and domain-level reasoning.
- Write onboarding and design docs to clarify the semantic, corpus/LM-centric vision for contributors and future codegen.
