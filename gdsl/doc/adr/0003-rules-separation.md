# ADR 0003 — Rules in GDSL: schema vs. query

Status: Proposed
Date: 2025-09-07

Context
- We now have two rule concepts:
  1) schema rules (dataset-time, canonicalization/transforms),
  2) query rules (query-time derivations for Cypher‑Lite).
- This risks duplication and confusion during an exploratory phase.

Decision
- Keep both, but clearly separate scope and mark query rules as experimental.
  - Schema rules: only for dataset/canonicalize pipelines (persisted or reproducible transforms). Public and stable.
  - Query rules: only for in-memory query derivation (non-persistent). Experimental and opt‑in.
- Pause further expansion of both until core primitives settle (GraphArtifact, Facets/Signatures, Dataset, Lens/Topic, search).

Implementation (short-term)
- Namespacing:
  - schema rules live under src/schema/* (unchanged).
  - query rules live under src/query/* as QueryRule (Rule) with a distinct AST.
- Packaging:
  - Do not re-export src/query/rules or src/query/ast from the public barrel yet.
  - Gate query rules behind an env flag in tests and CLI (GDSL_QUERY_RULES=1).
- Docs:
  - Document that query rules are in-memory, non-persistent derivations; schema rules are for canonicalization.

Consequences
- No breaking change to the public API.
- Tests can continue to cover query rules as experimental, while production code focuses on core dataset features.
- Future unification remains possible without churn.

Later (unification plan)
- Define a small Predicate IR (Atom, Literal, Rule) that both schema and query can lower to.
- Choose evaluation strategies:
  - schema: bottom-up (semi‑naive) during canonicalize.
  - query: hybrid (small closure for predicates referenced by the query plan).
- Provide a single evaluator with modes (dataset-time vs query-time).
- Deprecate one set of types once the IR stabilizes.

Actions (now)
- Keep query engine and rule evaluator tests under test/happy, but do not export query rules in the package barrel.
- Focus next work sessions on:
  - GraphArtifact + Facets/Signatures stability
  - Dataset API + canonicalize
  - Topic search utilities
