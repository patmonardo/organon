# Foundation Part A Workbook (V2)

Part: `A. IDENTITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact section order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `identity.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Chunk boundaries must follow complete sentence groups (no mid-sentence start/end).

## Working template

### Entry <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-18 (fresh Part A pass)

Scope:

- file: `identity.txt`
- initial range: lines `1-64` (opening movement for A. Identity)

Decision:

- Fresh TopicMap analysis from source text only.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in section headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Keep this pass in Dhyana-stage discipline: no Ground handoff claims are accepted from Part A without contradiction-resolution gates in downstream workbooks.

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
