# DSL — KG Schema (draft)

Purpose
- Minimal, explicit schema to drive ingestion, analysis, and GDS pipelines.
- Authoring manifests (TS/JSON) map directly to Dataset artifacts.
- Keep names stable and unambiguous for tooling and docs.

Terminology (canonical)
- Dataset: named collection of Chunks, HLOs, Terms, Tokens, and metadata.
- Chunk: source unit (document/section). Fields: id, title, text, summary, source.
- HLO (High-Level Operation): id, chunkId, label, digest, clauses[], tokens[], witnessEdges[].
- Clause: raw string in HLO.clauses (kinds: assert | tag | annotate). Keep raw for provenance.
- Symbol (aka Functor): the name inside assert(...). Canonical symbol id = camelCase verb.
- Token: canonical symbol name (tokens[] lists Symbols used).
- Term: domain concept/constant (Being, PureBeing, Nothing). Node-level entity.
- Var: variable placeholder (reserved for rules/backward-chaining later).
- Annotation: structured metadata produced by annotate(...) or tag(...).
- WitnessEdge (preferred name): explicit edges you want materialized in the graph; formerly "essentialRelation".

Node types (shape)
- Dataset
  - id: string (dataset:...)
  - title?: string
  - provenance?: object
- Chunk
  - id: string
  - title?: string
  - summary?: string
  - text?: string
  - source?: string (module/file)
- HLO
  - id: string
  - chunkId: string
  - label?: string
  - digest?: string | string[]
  - clauses?: string[] (raw)
  - tokens?: string[] (derived or declared)
  - witnessEdges?: { type: string; from: string; to: string; props?: object }[] (optional)
- Clause
  - id: string
  - hloId: string
  - raw: string
  - kind: 'assert' | 'tag' | 'annotate' | 'unknown'
- Token
  - id: string (token:equalOnlyToItself)
  - token: string
  - arity?: number
  - doc?: string
- Term
  - id: string (Being)
  - label?: string
  - aliases?: string[]
  - notes?: string

Edge types (canonical)
- DATASET_HAS_CHUNK (Dataset -> Chunk)
- CHUNK_HAS_HLO (Chunk -> HLO)
- HLO_HAS_CLAUSE (HLO -> Clause)
- CLAUSE_ASSERTS_SYMBOL (Clause -> Token)
- HLO_TAGS (HLO -> Clause/Annotation)
- HLO_TAGS_TERM (HLO -> Term) — when tag value is a term id
- WITNESS_<TYPE> (Term -> Term) — materialized witness edges (e.g., WITNESS_EQUALS, WITNESS_EXCLUDES)
- TERM_ALIAS_OF (Term -> Term) — alias/subtype relationships
Notes
- Prefer explicit WITNESS_ prefix for graph-visible conceptual edges. Keep types UPPER_SNAKE for clarity.
- Edge props: { provenance?, score?, note?, ... }.

Clause grammar (v1)
- assert(Symbol(arg1,arg2,...))
- tag(key, value)
- annotate(key, { ... })  // allowed v1 metadata verb
Parser notes
- Extract Symbol names by parsing the functor in assert(...) (allow optional unary not(...)).
- Heuristics: Terms are capitalized identifiers within arg lists; capture as Term candidates.

Naming conventions
- ids: kebab-case or stable prefixing (dataset:..., hlo:..., chunk:..., term:...).
- Symbol names: camelCase verbs (equalsTruth, passesOver).
- Token node ids: prefix with `token:` to avoid collisions (token:equalsTruth).
- Term node ids: plain canonical id (Being, PureBeing) or prefixed by `term:` if preferred.

Artifact layout (first-pass JSON)
- dataset.json:
  - dataset: id
  - nodes: [{ id, label, props }]
  - edges: [{ id?, type, from, to, props }]
  - tokens: [string]
  - terms: [{ id, label, aliases }]
  - counts: { chunks, hlos, clauses, tokens }

Arrow layout (later)
- nodes table: node_id, label, props_json
- edges table: edge_id, type, from_id, to_id, props_json
- clauses table: clause_id, hlo_id, raw, kind
- tokens table: token, arity
- hlo_token_coo: hlo_id, token, count

Ingest flow (priority)
1. Load Dataset manifest (TS/JSON) and import referenced modules.
2. Canonicalize: produce Chunk, HLO, Clause rows; derive tokens and Terms.
3. Validate referential integrity (HLO.chunkId exists; clause parsing succeeded).
4. Upsert into MemoryGraphStore for QA.
5. Emit JSON artifact.
6. Optionally convert to Arrow (.feather) for analytics and cache in @core.

Approval checklist (action items to finalize)
- [ ] Accept node/edge names above (Dataset, Chunk, HLO, Clause, Token, Term).
- [ ] Accept edge vocabulary and WITNESS_ prefix (replace "EssentialRelation").
- [ ] Confirm clause verbs allowed in v1: assert, tag, annotate.
- [ ] Confirm tokens are derived (not hand-maintained predicates arrays).
- [ ] Confirm storage artifact formats: JSON first, Arrow next.

Next steps once approved
- Add `docs/schema.md` (this file) to repo.
- Implement ingest script (src/scripts/ingest/ingest-dataset.ts) to produce dataset JSON (scaffold ready).
- Create a short ADR noting "WITNESS_EDGE naming and tokens derived from clauses" for governance.

If you approve the node/edge names and the WITNESS_ rename, I will update glossary and generate a tiny ADR text for
