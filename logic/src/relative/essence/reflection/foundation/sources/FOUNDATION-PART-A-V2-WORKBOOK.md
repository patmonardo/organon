# Foundation Part A (TopicMap) Workbook (V2)

Part: `A. IDENTITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `identity.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries must follow complete sentence groups (no mid-sentence start/end).

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Working template

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-18 (fresh Part A pass)

Scope:

- file: `identity.txt`
- active range: lines `3-64` (full A. Identity movement in this source)

Decision:

- Fresh TopicMap analysis from source text only.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in entry headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Keep this pass in Dhyana-stage discipline: no Ground handoff claims are accepted from Part A without contradiction-resolution gates in downstream workbooks.
- Apply short-section exception policy: do not force artificial over-segmentation for the short Identity source.
- Remark handling: no explicit remark block exists in `identity.txt`, so no remark exclusion decision is needed beyond source lock.

### Entry fnd-id-a-001 — Reflection as determined essence and pure identity

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/identity.txt`
- lineStart: 3
- lineEnd: 36

Summary:

Foundation opens by determining reflection as essence's self-shining mediation, whose first determination is pure identity as simple self-reference.

Key points: (KeyPoint)

- k1. Reflection is determined reflection and essence is determined essence.
- k2. Reflection is essence shining within itself.
- k3. Essence is negative simplicity and absolute mediation with itself.
- k4. The first determination is pure identity as simple self-reference.

Claims: (Claim)

- c1. id: fnd-id-a-001-c1
  - subject: reflection
  - predicate: is_determined_as
  - object: determined_reflection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [5-6] "Reflection is determined reflection; accordingly, essence is determined essence..."

- c2. id: fnd-id-a-001-c2
  - subject: reflection
  - predicate: is
  - object: essence_shining_within_itself
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [8-8] "Reflection is the shining of essence within itself."

- c3. id: fnd-id-a-001-c3
  - subject: essence
  - predicate: first_determination_is
  - object: pure_identity_simple_self_reference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [18-19] "First, essence is simple self-reference, pure identity."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: fnd-id-a-002
  - note: move from pure identity framing toward identity as immediacy of reflection.

Review outcome:

- review_pending
- notes: boundary locked to opening determinations; next entry should continue through identity as pure production.

### Entry fnd-id-a-002 — Identity as pure production and in-itself sublation of being

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/identity.txt`
- lineStart: 37
- lineEnd: 64

Summary:

Identity is determined as the immediacy of reflection and pure self-production, not abstract identity from external negation, because being and determinateness are sublated in themselves.

Key points: (KeyPoint)

- k1. Identity is the immediacy of reflection, produced from itself and in itself.
- k2. Essential identity is not abstract identity resulting from merely relative negation.
- k3. Being and every determinateness of being are sublated in themselves.
- k4. This simple negativity of being in itself is identity and remains the same as essence.

Claims: (Claim)

- c1. id: fnd-id-a-002-c1
  - subject: identity
  - predicate: is_determined_as
  - object: immediacy_of_reflection_and_pure_self_production
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [37-48] self-identity is immediacy of reflection and pure production from itself and in itself.

- c2. id: fnd-id-a-002-c2
  - subject: essential_identity
  - predicate: is_not
  - object: abstract_identity_from_relative_negation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [50-56] denial of abstract identity produced by relative negation that leaves the other outside.

- c3. id: fnd-id-a-002-c3
  - subject: being_and_its_determinateness
  - predicate: are_sublated_in_themselves_as
  - object: simple_negativity_identical_with_essence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [57-64] being and every determinateness are sublated in themselves; simple negativity is identity and still the same as essence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-id-a-001
  - note: deepens pure identity from opening framing into its explicit anti-abstract determination.

- r2. type: transitions_to
  - targetEntryId: fnd-dif-b-001
  - note: closure of Identity hands off to Difference as the next essentiality.

Review outcome:

- review_pending
- notes: full first-order coverage for available Identity movement in this source is complete.
