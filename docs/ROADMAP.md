# Roadmap (Logic)

Near-term
- Docs: link ADR-0004 and Kriya Syllogism from docs index and package README.
- Tests: add negative-path cases for Property (missing key/contextId) and Relation (missing endpoint.type).
- Repo API: converge on create(doc), update(id, doc), get(id), delete(id); services can keep delete+create as fallback.

Mid-term
- Absolute Relation facet: document constraint carriers and provenance fields.
- Processor design note: fixpoint scheduling, invalidation on Context version bump.
- Examples: minimal world with 2 Entities, 1 Context, 1 Morph, Absolute Relation constraints.

Longer-term
- Derivation libraries (meets/joins) and fairness in propagation.
- Publish docs (TypeDoc + concepts/ADR) via CI.
