# ADR 0004 — GDS⇄GDSL architecture and the Logic–Model–Task triad

Status: Accepted
Date: 2025-09-07

Decision
- GDS = core Graph Data Science kernel (algorithms, indices, q-k-v compute fabric).
- GDSL = lightweight, agent-facing SDK (Dataset canonicalization, Signatures/Facets, Topic search, AgentKit).
- UserLand triad: LOGIC (rational inference), MODEL (empirical/statistical), TASK (plans/execution). GDSL serves agents across this sequence.

Responsibilities
- GDS (no UI/CLI):
  - Graph structures, indices, algorithms (paths, centrality, community).
  - Execution primitives for pipelines/ML (future).
- GDSL (SDK + CLI):
  - Schemas: GraphArtifact, Facets/Signatures.
  - CanonRules: pure dataset-time transforms.
  - Topic/Lens utilities: index/glossary/find.
  - AgentKit: find, facet weights, mock GDS.
  - CLI: dataset canonicalize/facets, topic search.
  - Experimental: Cypher‑Lite behind GDSL_QUERY_RULES=1 (not exported).

Invariants
- Stable core surface: schemas, canonicalize, signatures, topic search, AgentKit.
- Feature gating: experimental query/rules not in public exports.
- Idempotent CanonRules; provenance preserved.

Roadmap (short)
- Freeze core API + docs.
- CI: build + test on PRs.
- CLI polish and examples.
- Cypher‑Lite MVP (gated), later.

Consequences
- Safe foundation for agents; clear extension path to advanced query/ML without risking core stability.
