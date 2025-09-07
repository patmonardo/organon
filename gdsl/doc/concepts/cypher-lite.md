GitHub Copilot.

Short take: a serverless “Cypher‑Lite” that blends Datalog (rules/inference) with Cypher (graph pattern matching) is plausible. Start with a small AST + in‑memory planner/executor over GraphArtifact, add a Datalog rule pass (semi‑naive), then consider a parser.

Closest simple implementations and references (useful to study)

Cypher/openCypher (pattern matching, semantics, grammars)
- openCypher project: ANTLR grammars and semantics; good for a minimal parser subset.
- libcypher-parser (C): small, focused Cypher parser library (battle‑tested).
- RedisGraph (C): openCypher subset + in‑memory planner/executor; compact codebase.
- Memgraph (C++): openCypher engine; clear docs and examples of plans/operators.
- Apache AGE / AgensGraph (C/Postgres): openCypher in PG; useful for understanding plan lowering.
- Kùzu (C++): modern, embeddable graph DB with Cypher‑like language; tidy code, good reference for an engine.

Datalog (rules, semi‑naive evaluation, recursion)
- Datafrog (Rust): tiny, excellent semi‑naive Datalog implementation; the cleanest reference for bottom‑up evaluation.
- Soufflé (C++): high‑performance Datalog compiler; study for stratified negation and aggregates.
- Datomic/Datascript (Clojure/ClojureScript): practical Datalog semantics and indexes; Datascript is small and embeddable.
- Datomish (JS/Rust, archived): Datalog in JS; shows index layout and evaluation strategy.
- Asami/Datahike (Clojure/JS): graphy Datalog stores; approachable semantics docs.

Graph query research (unifying direction)
- G‑CORE paper: a core for future graph query languages (paths + property graphs) — foundation for ISO GQL.
- openCypher formal semantics (GRA/TPA): how patterns lower to algebra (joins, expands).
- Magic Sets for Datalog: classic optimization for goal‑driven evaluation (useful when mixing rules with pattern matching).

JS/TS tooling for a small parser/executor
- Chevrotain or ohm‑js: simple parser generators to grow a Cypher‑like subset.
- Comunica (JS SPARQL engine): shows planner/executor architecture in JS (indexes, iterators, streaming).

Practical path (incremental, testable)
- AST first, parser later:
  - Keep the small AST you started: MATCH, WHERE, RETURN, LIMIT; add optional rules: head <- body Datalog clauses.
- Planner/executor over GraphArtifact:
  - Operators: IndexScan (labels/id), Expand (edge type), Filter (WHERE), Project, Limit, Join.
  - Seed scans from facets/signatures to keep matches small.
- Rule pass (Datalog):
  - Bottom‑up semi‑naive: materialize derived edges/nodes referenced by the query’s head predicates; merge into transient indexes; then run MATCH.
  - Enforce stratification for negation; bound recursion with depth as needed initially.
- Tests first (Vitest):
  - Happy‑path executor tests (single pattern, typed edge, WHERE, RETURN, LIMIT).
  - Rule tests (derive transitive closure edge, then MATCH uses it).
  - Index seeding tests (facet token narrows candidates).

If you want next, I can:
- add a happy‑path test for the current QueryEngine (MATCH one hop with WHERE + RETURN), or
- sketch a tiny Rule AST and a semi‑naive evaluator that injects derived edges into your in‑memory indexes.
