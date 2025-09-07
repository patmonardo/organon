# Graph Query — Core Reading List

Standards and foundations
- ISO GQL (Graph Query Language) — overview on Wikipedia (good entry point).
- openCypher — grammar and semantics (pattern matching, RETURN/WHERE).
- G‑CORE paper — paths + property graphs (influential for ISO GQL).

Engines and compact references
- RedisGraph — openCypher subset with a tight planner/executor (C).
- Memgraph — openCypher engine; docs on plans/operators (C++).
- Kùzu — embeddable graph DB with Cypher‑like language (C++).
- Apache AGE / AgensGraph — openCypher on Postgres (C).

Datalog and rule evaluation
- Datafrog (Rust) — minimal semi‑naive Datalog (clean reference).
- Soufflé (C++) — high‑performance Datalog compiler (stratified negation).
- Datascript (Clojure/JS) — practical Datalog indexes and eval.

Other JS query engines (architecture references)
- Comunica (SPARQL in JS) — planner/executor, iterator model, indexes.

Vendor variants
- PGQL (Oracle, Java) — Cypher‑like; useful for grammar patterns and examples.

Notes
- Start small: AST → logical plan (IndexScan, Expand, Filter, Join, Project, Limit) → executor over GraphArtifact.
- Add a bottom‑up rules pass (semi‑naive) only for query predicates you actually use.
