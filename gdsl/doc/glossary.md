# DSL Glossary and Naming Standard

Purpose
- Disambiguate Hegelian terms from DSL mechanics.
- Standardize keys in TS-JSON modules.
- Keep clause grammar small and evaluable.

Core data keys (DSL “schema”)
- Chunk
  - id: string
  - title: string
  - text?: string
  - summary?: string  // paragraph; replaces legacy “concise”
- HLO (High-Level Operation)
  - id: string
  - chunkId: string
  - label: string
  - digest?: string | string[]  // one-liner or bullets
  - clauses?: string[]          // see Clause grammar
  - tokens?: string[]           // derived list of Symbols used (was “predicates”)
  - essentialRelations?: { predicate: string; from: string; to: string; props?: object }[]  // “witness” edges (was “relations”)

Clause grammar (v1)
- assert(Symbol(arg1, arg2, ...))
- tag(key, value)
Notes
- Only two verbs in v1: assert, tag.
- Deprecate annotate(...). Normalize to tag('meta', {...}) as needed.

Key terms (DSL mechanics)
- Clause: a single string instruction inside an HLO (assert(...) or tag(...)).
- Symbol (aka functor): the name inside assert(…); e.g., equalOnlyToItself, passesOver.
- Token: one occurrence of a Symbol in a clause; tokens[] is the set/list of symbols an HLO uses.
- Tag: metadata marker; tag('phase','quality'), tag('source', 'Hegel/SL/§...').
- Digest: short one-liner (or few bullets) describing the HLO.
- Summary: paragraph summary attached to a Chunk.
- Essential Relation: an explicitly modeled edge you want projected to the graph (your “witnessers”).

Reserved graph words (avoid Hegel overload)
- Property (graph): plain key–value on nodes/edges.
- Node, Edge, Label: graph-level constructs only.

Hegelian terms (conceptual, for docs/UI)
- Predicate (Hegel): the P in S–P (Judgment). Half of the dyad in S–P–U progression.
- Property (Eigenschaft): law/trait of appearance.
- Essential Relation: Identity/Difference, Ground, Cause, etc.

Mapping (Hegel ↔ DSL)
- Hegel Predicate (P) → can be represented by a Symbol applied to a Subject later; in the DSL we keep “Symbol/Token” to avoid confusion.
- Hegel Property (law) → document as Trait/Law; do not use “property” for DSL fields.
- Essential Relations → captured as essentialRelations[] and/or compiled edges in the graph.

Naming conventions
- ids: kebab-case, stable, section-prefixed. Example: being-1-pure-indeterminate-immediate; being-op-6-being-equals-nothing.
- labels: human-readable phrases (Title Case).
- symbols (inside assert): camelCase, verbs first. Example: equalOnlyToItself, excludes, equates, passesOver.
- tags: lower-kebab keys; JSON-friendly values. Example: tag('phase','quality'), tag('source',"Hegel/SL/Being A.1").

Deprecations (migrate over time)
- concise → summary (Chunk)
- predicates → tokens (HLO)
- relations → essentialRelations (HLO)
- annotate(...) → tag('meta', {...})

Minimal examples

Before
- predicates: [{ name: 'SelfEquality', args: [] }]
- relations: [{ predicate: 'equates', from: 'Being', to: 'Nothing' }]
- clauses: ["annotate(Being,{noDifference:'internal|external'})"]

After
- tokens: ['SelfEquality']
- essentialRelations: [{ predicate: 'equates', from: 'Being', to: 'Nothing' }]
- clauses: ["tag('meta', { noDifference: 'internal|external' })"]

Field intents
- Chunk.summary: readable paragraph for the source chunk.
- HLO.digest: a compact “what this HLO does” line; useful for clustering/dup-detection.
- HLO.clauses: executable/parsable substrate; assert(...) drives analysis; tag(...) carries metadata.
- HLO.tokens: derived or declared list of Symbols used; aids indexing/search.
- HLO.essentialRelations: explicit graph edges to materialize (witnessers).

Authoring checklist (per file)
- Ensure every Chunk has: id, title, summary (text optional in v1).
- Ensure every HLO has: id, chunkId, label, digest, clauses (assert/tag only).
- Prefer tags over annotate; keep clause verbs limited to assert/tag.
- Keep tokens in sync with assert(...) symbols (or generate; choose one path consistently).
- Use essentialRelations only for edges you want projected; don’t mirror every assert.

Glossary quick ref
- Chunk: source unit (like “document” in topic modeling).
- HLO: operation extracted from a chunk.
- Symbol: name inside assert(…).
- Token: an instance of a Symbol (and the symbol list per HLO).
- Tag: metadata mark.
- Digest: one-liner summary of an HLO.
- Summary: paragraph for the chunk.
- Essential Relation: explicit relation to project into the graph.

Rationale
- Keeps Hegelian “Predicate” unpolluted; “Symbol/Token” names the DSL mechanics.
- Small grammar → easy parsing/validation/export.
- digest vs summary separates 1-line HLOs from paragraph chunks.
