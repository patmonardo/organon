# Absolute Part B (TopicMap) Workbook (V1)

Part: `B. THE ABSOLUTE ATTRIBUTE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `attribute.txt` as authority for Part B.
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
- claims: (Claim) minimum 3, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-20 (initial full Part B pass)

Scope:

- file: `attribute.txt`
- active range: lines `1-end` (`B. THE ABSOLUTE ATTRIBUTE`)

Decision:

- Complete Part B in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `abs-part-b-001` for lines `3-42`
- completed: `abs-part-b-002` for lines `44-70`
- completed: `abs-part-b-003` for lines `71-96`

### Entry abs-part-b-001 — Attribute as relative absolute and total content

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/attribute.txt`
- lineStart: 3
- lineEnd: 42

Summary:

Attribute is introduced as the relative absolute: form-determined, yet carrying the whole content of the absolute while immediate subsistences are reduced to reflective shine.

Key points: (KeyPoint)

- k1. The “absolute absolute” names form returned into itself, while attribute is the absolute under form-determination.
- k2. Attribute nonetheless bears whole content/totality previously distributed in essential relation.
- k3. Opposed worlds retained immediate subsistence, but in the absolute this immediacy is reduced to reflective shine.
- k4. The totality of attribute is posited as true single subsistence; its determination is unessential subsistence.

Claims: (Claim)

- c1. id: abs-part-b-001-c1
  - subject: attribute
  - predicate: is
  - object: relative_absolute_as_form_determination
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [3-9] "the absolute absolute... form is equal to content... The attribute is just the relative absolute... in a form determination."

- c2. id: abs-part-b-001-c2
  - subject: attribute
  - predicate: has
  - object: whole_content_of_absolute_as_totality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [15-20] "the attribute is the whole content of the absolute... each of which is itself the whole."

- c3. id: abs-part-b-001-c3
  - subject: immediate_subsistences_of_opposed_worlds
  - predicate: are_reduced_to
  - object: reflective_shine_in_absolute
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [31-38] "these sides... immediate subsistence... In the absolute... reduced to a reflective shine."

- c4. id: abs-part-b-001-c4
  - subject: attribute_totality
  - predicate: is_posited_as
  - object: true_single_subsistence_with_form_determination_unessential
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [39-42] "the totality that the attribute is is posited as its true and single subsistence... determination... unessential subsistence."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: abs-part-b-002
  - note: from attribute as relative totality to the self-sublation of form-determination in reflection.
  - sourceClaimIds: [abs-part-b-001-c3, abs-part-b-001-c4]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [abs-part-b-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: abs-part-a-003
  - note: concretizes Part A’s handoff claim that absolute identity appears as attribute.
  - sourceClaimIds: [abs-part-b-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [abs-part-a-003-c2]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry abs-part-b-002 — Attribute’s positivity sublates itself into simple absolute

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/attribute.txt`
- lineStart: 44
- lineEnd: 70

Summary:

The attribute is posited from identity, yet because all determination (including reflection) is sublated, attribute as attribute is itself dissolved into the simple absolute.

Key points: (KeyPoint)

- k1. Absolute is attribute insofar as identity stands as determination.
- k2. In absolute identity, determinations and reflection itself are posited as sublated.
- k3. Attribute’s form determination is only reflective shine; negative is posited as negative.
- k4. The exposition’s positive shine through attribute sublates attributehood itself into simple absolute.

Claims: (Claim)

- c1. id: abs-part-b-002-c1
  - subject: absolute_as_simple_identity
  - predicate: is
  - object: attribute_in_determination_of_identity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [44-49] "The absolute is attribute because... in the determination of identity... other determinations can be attached."

- c2. id: abs-part-b-002-c2
  - subject: absolute_identity
  - predicate: posits
  - object: all_determinations_and_reflection_as_sublated
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [50-54] "not only all determinations have been sublated but... reflection itself... all determinations are thus posited... as sublated."

- c3. id: abs-part-b-002-c3
  - subject: attribute_form_determination
  - predicate: is
  - object: mere_reflective_shine
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [55-60] "attribute has the absolute for its content... form determination... posited immediately as mere reflective shine."

- c4. id: abs-part-b-002-c4
  - subject: positive_reflective_shine_of_exposition
  - predicate: sublates
  - object: attributehood_into_simple_absolute
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [61-69] "positive reflective shine... expands it into attribute; sublates precisely this, that the attribute is attribute... into the simple absolute."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: abs-part-b-001
  - note: specifies why attribute’s apparent positivity cannot stabilize as independent form.
  - sourceClaimIds: [abs-part-b-002-c2, abs-part-b-002-c3]
  - sourceKeyPointIds: [k2, k3]
  - targetClaimIds: [abs-part-b-001-c4]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: abs-part-b-003
  - note: after sublation of attributehood, reflection’s residual externality is diagnosed.
  - sourceClaimIds: [abs-part-b-002-c4]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [abs-part-b-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: preserve distinction between sublated determination and reflective nullity in later cycles.

### Entry abs-part-b-003 — Reflection’s failure to reach true absolute; form as null manner

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/attribute.txt`
- lineStart: 71
- lineEnd: 96

Summary:

When reflection returns only to identity without relinquishing externality, it reaches merely abstract identity, so attribute-form is reduced to null reflective manner.

Key points: (KeyPoint)

- k1. Reflection reverts to identity yet retains externality, missing the true absolute.
- k2. The reached identity is indeterminate/abstract identity in determinateness of identity.
- k3. Inner determination as attribute-form remains distinct from externality and does not penetrate absolute.
- k4. Therefore outer/inner form of attribute is null in itself: mere reflective shine and manner.

Claims: (Claim)

- c1. id: abs-part-b-003-c1
  - subject: reflection_reversion
  - predicate: fails_to_reach
  - object: true_absolute_due_to_retained_externality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [71-76] "reflection... has not... left its externality behind and has not arrived at the true absolute."

- c2. id: abs-part-b-003-c2
  - subject: reached_identity
  - predicate: is
  - object: indeterminate_abstract_identity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [77-80] "It has only reached... indeterminate, abstract identity... identity in the determinateness of identity."

- c3. id: abs-part-b-003-c3
  - subject: inner_determination_as_attribute
  - predicate: does_not
  - object: penetrate_absolute_and_stably_subsist
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [81-89] "inner form... still distinct from externality... does not penetrate the absolute... is to disappear into the absolute."

- c4. id: abs-part-b-003-c4
  - subject: form_of_attribute
  - predicate: is
  - object: null_external_reflective_shine_or_mere_manner
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [91-96] "form... whether outer or inner... posited as something null in itself, an external reflective shine, or a mere way and manner."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: abs-part-b-002
  - note: carries forward self-sublation by explicitly nullifying attribute-form as such.
  - sourceClaimIds: [abs-part-b-003-c4]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [abs-part-b-002-c3, abs-part-b-002-c4]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: abs-part-c-001
  - note: prepares Part C by moving from null attribute-form to mode as explicit determination of externality.
  - sourceClaimIds: [abs-part-b-003-c1, abs-part-b-003-c4]
  - sourceKeyPointIds: [k1, k4]
  - targetClaimIds: [abs-part-c-001-c2, abs-part-c-001-c3]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part C handoff claim targets are now resolved.
