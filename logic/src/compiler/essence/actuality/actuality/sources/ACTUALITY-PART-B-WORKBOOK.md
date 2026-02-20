# Actuality Part B (TopicMap) Workbook (V1)

Part: `B. RELATIVE NECESSITY OR REAL ACTUALITY, POSSIBILITY, AND NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `../../absolute/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `relative-necessity.txt` as authority for Part B.
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

- file: `relative-necessity.txt`
- active range: lines `1-end` (`B. RELATIVE NECESSITY OR REAL ACTUALITY, POSSIBILITY, AND NECESSITY`)

Decision:

- Complete Part B in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `act-part-b-001` for lines `3-46`
- completed: `act-part-b-002` for lines `47-161`
- completed: `act-part-b-003` for lines `162-242`

### Entry act-part-b-001 — Real actuality as contentful form-unity within formal necessity

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/relative-necessity.txt`
- lineStart: 3
- lineEnd: 46

Summary:

Formal necessity, as immediate conversion of formal moments, yields real actuality: a contentful, self-subsistent actuality that manifests itself while immediately containing possibility.

Key points: (KeyPoint)

- k1. Formal necessity has formal moments unified immediately without self-subsistent shape.
- k2. This immediate unity is actuality with content, namely real actuality.
- k3. Real actuality is self-preserving immanent reflection, not mere dissolving appearance.
- k4. Actuality manifests itself in production and self-relation through another self-subsistent.
- k5. Real actuality immediately contains possibility, yet as distinguished form-moment.

Claims: (Claim)

- c1. id: act-part-b-001-c1
  - subject: formal_necessity
  - predicate: is
  - object: immediate_unity_of_formal_moments_without_self_subsistence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [3-10] "necessity... formal... immediate unity... lack the shape of self-subsistence..."

- c2. id: act-part-b-001-c2
  - subject: actuality_from_formal_necessity
  - predicate: is
  - object: real_actuality_with_indifferent_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [11-18] "this necessity is actuality... has a content... This actuality is real actuality."

- c3. id: act-part-b-001-c3
  - subject: real_actuality
  - predicate: is
  - object: self_subsistent_immanent_reflection_manifesting_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [20-38] "it is at the same time an in-itself and immanent reflection... self-subsistent... determinate essentiality in another self-subsistent."

- c4. id: act-part-b-001-c4
  - subject: real_actuality
  - predicate: contains
  - object: immediate_possibility_as_distinguished_form_moment
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [40-46] "real actuality likewise has possibility immediately present in it... distinguished... from... possibility."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: act-part-b-002
  - note: from immediate possibility in real actuality to explicit account of real possibility and conditions.
  - sourceClaimIds: [act-part-b-001-c4]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-b-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: act-part-a-003
  - note: develops Part A necessity into contentful real actuality rather than purely formal conversion.
  - sourceClaimIds: [act-part-b-001-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [act-part-a-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry act-part-b-002 — Real possibility as conditioned totality and self-rejoining movement

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/relative-necessity.txt`
- lineStart: 47
- lineEnd: 161

Summary:

Real possibility is not abstract non-contradiction but the concrete totality of conditions, whose self-sublating circle produces actuality as self-rejoining.

Key points: (KeyPoint)

- k1. Real possibility is contentful in-itself of real actuality, not merely formal abstract identity.
- k2. It is the immediately existent manifoldness of circumstances referring to a fact.
- k3. This manifold is both possibility and actuality, with form-difference set against indifferent content-identity.
- k4. Real possibility is contradictory in its developed conditioned multiplicity.
- k5. Complete conditions entail actuality; the circle of conditions sublates itself into self-rejoining actuality.

Claims: (Claim)

- c1. id: act-part-b-002-c1
  - subject: real_possibility
  - predicate: is
  - object: contentful_in_itself_distinct_from_formal_possibility
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [47-55] "real possibility... in-itself full of content... not [just] formal possibility..."

- c2. id: act-part-b-002-c2
  - subject: real_possibility_of_a_fact
  - predicate: is
  - object: immediately_existent_manifoldness_of_circumstances
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [57-64] "real possibility... immediate concrete existence... manifoldness of circumstances..."

- c3. id: act-part-b-002-c3
  - subject: conditioned_manifoldness
  - predicate: is
  - object: contradictory_and_self_sublating
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [94-107] "because it is manifold... it is contradictory... manifold... to sublate itself... only a possibility."

- c4. id: act-part-b-002-c4
  - subject: completeness_of_conditions
  - predicate: entails
  - object: actuality_of_fact
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [108-113] "Whenever all the conditions... are completely present, the fact is actually there..."

- c5. id: act-part-b-002-c5
  - subject: self_sublating_circle_of_conditions
  - predicate: becomes
  - object: actuality_as_self_rejoining_of_possibility
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [139-143] "movement... possibility is... a self-rejoining."
    - [149-155] "circle of conditions... makes itself into the in-itselfness... becomes actuality..."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: act-part-b-001
  - note: specifies the immediate possibility in real actuality as concrete totality of conditions.
  - sourceClaimIds: [act-part-b-002-c1, act-part-b-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [act-part-b-001-c4]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: act-part-b-003
  - note: self-rejoining of real possibility yields real necessity and its relative character.
  - sourceClaimIds: [act-part-b-002-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-b-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: c3 and c5 should be preserved for later contingency/necessity unification mapping.

### Entry act-part-b-003 — Real necessity as identity with possibility, yet still relative and contingent

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/relative-necessity.txt`
- lineStart: 162
- lineEnd: 242

Summary:

Real necessity is the self-identity of self-sublating real possibility, but remains relative because it presupposes contingency and retains content/form externality.

Key points: (KeyPoint)

- k1. Negation of real possibility is real necessity as recoiling self-identity.
- k2. Under complete real conditions, what is really possible cannot be otherwise.
- k3. Real necessity and real possibility are identical, only apparently distinguished.
- k4. This necessity is relative, beginning from contingent determinate actuality.
- k5. Real necessity contains contingency in content and form; their unity is absolute actuality.

Claims: (Claim)

- c1. id: act-part-b-003-c1
  - subject: negation_of_real_possibility
  - predicate: is
  - object: real_necessity_as_self_identity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [162-165] "The negation of real possibility... is real necessity."

- c2. id: act-part-b-003-c2
  - subject: really_possible_under_given_conditions
  - predicate: cannot_be_otherwise
  - object: practical_identity_with_necessity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [176-179] "what is really possible can no longer be otherwise..."

- c3. id: act-part-b-003-c3
  - subject: real_possibility_and_real_necessity
  - predicate: are
  - object: presupposed_identity_only_apparently_distinguished
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [179-183] "only apparently distinguished... identity... already presupposed..."

- c4. id: act-part-b-003-c4
  - subject: real_necessity
  - predicate: is
  - object: relative_because_it_begins_from_contingency
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [187-190] "this necessity is... relative... takes its start from the contingent."
    - [208-214] "necessity... begins from... not yet reflected into itself..."

- c5. id: act-part-b-003-c5
  - subject: real_necessity
  - predicate: contains
  - object: contingency_with_unity_named_absolute_actuality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [224-241] "real necessity is in itself also contingency... unity of necessity and contingency... called absolute actuality."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: act-part-b-002
  - note: elevates conditioned self-sublation to explicit necessity while preserving its presupposed movement.
  - sourceClaimIds: [act-part-b-003-c1, act-part-b-003-c3]
  - sourceKeyPointIds: [k1, k3]
  - targetClaimIds: [act-part-b-002-c5]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: refines
  - targetEntryId: act-part-a-003
  - note: specifies Part A necessity as relative necessity that still contains contingency.
  - sourceClaimIds: [act-part-b-003-c4, act-part-b-003-c5]
  - sourceKeyPointIds: [k4, k5]
  - targetClaimIds: [act-part-a-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: act-part-c-001
  - note: transition to absolute necessity via unity of necessity and contingency as absolute actuality.
  - sourceClaimIds: [act-part-b-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-c-001-c1, act-part-c-001-c2]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part C handoff claim targets are now resolved.
