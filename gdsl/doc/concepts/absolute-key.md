# Absolute Knowing — the Key

Thesis
- Logogenesis is the explanatory framework of the Three Syllogisms of Necessity. Its precise specification reveals the exact pathway of the dialectic of Truth and Knowledge: the genetic principle of Truth giving rise to Knowledge, and the genetic principle of Knowledge revealing Truth.

Definition
- Absolute Knowing: the unification that sublates Idealism (models) and Realism (observations) as one navigable knowledge surface.
- Sublation: Truth (observed/committed facts with provenance) and Knowledge (derived/explained relations/models) co-present, each genetically producing/illuminating the other under Logogenesis.
- Practical reading for GDSL/GDS: forward derivations (Truth → Knowledge) and backward explanations (Knowledge → Truth) are explicit, canonical, and testable via transforms, facets, and lenses.

Design invariants (testable)
- Canon-first: every Lens operates on validated GraphArtifact.
- Provenance/modality are facetized and never dropped.
- Views explicit; no hidden inference; derivations carry source references.

Falsifiable claims (how we’ll test)
- Genetics (forward): given a set of truths and a lawful derivation, the produced knowledge artifacts are reproducible, labeled (modality=inferred), and idempotent.
- Genetics (backward): given a knowledge artifact with provenance, we can retrieve a minimal sufficient set of truths that explains it.
- Monotonicity: adding supportive facet weight for a query token does not decrease any candidate’s score.

Implications
- For GDS (kernel): provide stable graph/index primitives that preserve provenance and support reproducible derivations.
- For GDSL (agent SDK): expose canonicalize + facets + lenses so agents can trace and present both directions of genetics.
- For Logic–Model–Task: Logic specifies lawful derivations; Model calibrates/supports them; Task executes and records resulting truths/knowledge with provenance.

Examples (tiny)
- Dataset → Lens overlay: observed facts (Truth) plus derived edges (Knowledge) presented together; provenance visible.
- Topic find boosted by provenance facet: tokens tied to modality/provenance raise relevance without changing memberships.

Risks and boundaries
- Out of scope now: runtime metaphysics and unbounded recursion; complex negation/aggregation in rules.
- Future unification path: a single predicate IR shared by schema-time and query-time evaluators.

Next steps
- Encode modality/provenance facets in transforms; keep them through canonicalize.
- Land invariant tests; unskip as implementations mature.
- Revisit once core API is frozen.
