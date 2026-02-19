# World Part A (TopicMap) Workbook (V2)

Part: `A. THE LAW OF APPEARANCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `law.txt` as authority for Part A.
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

## Session: 2026-02-19 (initial full Part A pass)

Scope:

- file: `law.txt`
- active range: lines `125-451` (`A. THE LAW OF APPEARANCE` block)

Decision:

- Initialize and complete Part A in one numbered pass.
- Bind entry spans to explicit `1/2/3` boundaries.
- Preserve transition from Thing dissolution into Appearance law as explicit cross-workbook relation.

## Decomposition status

- completed: `wld-law-a-001` for `A.1` (line `127` to `231`)
- completed: `wld-law-a-002` for `A.2` (line `232` to `344`)
- completed: `wld-law-a-003` for `A.3` (line `345` to `451`)

### Entry wld-law-a-001 — Appearance as negative mediation whose positive identity is law

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/law.txt`
- lineStart: 127
- lineEnd: 231

Summary:

In `A.1`, appearance is developed as concrete existence mediated by negation such that its reciprocal non-subsistence yields a positive self-identity, and this reflected identity of two-sided subsistence is law.

Key points: (KeyPoint)

- k1. Appearance is concrete existence mediated through negation and negation-of-negation, so its self-subsistence is reflective shine.
- k2. Negative mediation immediately contains a positive self-identity: positedness refers to positedness and thereby to itself.
- k3. The reflected manifold of appearance is reduced to simple difference whose reciprocal positedness forms one subsistence.
- k4. This unity of reflected identity is the law of appearance.

Claims: (Claim)

- c1. id: wld-law-a-001-c1
  - subject: appearance
  - predicate: is
  - object: concrete_existence_mediated_by_negation_with_reflective_self_subsistence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [127-146] appearance is concrete existent mediated through negation and negation of negation.
    - [147-166] reciprocal grounding is reciprocal negation; subsistence is posited connection.

- c2. id: wld-law-a-001-c2
  - subject: negative_mediation_of_appearance
  - predicate: contains
  - object: positive_self_identity_as_reflection_into_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [167-184] in its negation, concrete existent refers to itself and is positive essentiality.
    - [185-196] essential content has two sides: external posited immediacy and self-identical positedness.

- c3. id: wld-law-a-001-c3
  - subject: reciprocal_positedness_of_differences
  - predicate: is
  - object: unity_named_law_of_appearance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [208-223] contradiction sublates into identity where positedness of one is positedness of the other.
    - [224-231] this unity is explicitly identified as the law of appearance.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: wld-law-a-002
  - sourceClaimIds: [wld-law-a-001-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: from the law as unity of reciprocal positedness to law as explicit positive element of mediation.

- r2. type: refines
  - targetEntryId: thg-dis-c-003
  - sourceClaimIds: [wld-law-a-001-c1, wld-law-a-001-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [thg-dis-c-003-c3]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection
  - note: the contradictory mediation of the thing is carried forward as appearance's lawful self-identity.

Review outcome:

- review_pending
- notes: `A.1` complete; cross-chapter handoff anchored to Thing dissolution result.

### Entry wld-law-a-002 — Law as posited essentiality sharing content with appearance

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/law.txt`
- lineStart: 232
- lineEnd: 344

Summary:

In `A.2`, law is presented as the positive and essential positedness of appearance, not beyond appearance but its substrate, sharing the same content while distinguishing itself from immediate unessential flux.

Key points: (KeyPoint)

- k1. Law is the positive element within appearance's negative mediation, the identical positedness of both sides.
- k2. Law is reflected immediacy as essential positedness (`Gesetz` as `Gesetztsein`) and thereby essential connection of differences.
- k3. Law and appearance share one content continuously, but appearance includes unessential immediacy and flux beyond the law's restful identity.
- k4. The kingdom of laws is immediately present in appearance as its own essential return to ground.

Claims: (Claim)

- c1. id: wld-law-a-002-c1
  - subject: law
  - predicate: is
  - object: positive_element_of_appearance_mediation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [232-247] law is positive persistence in mediation where each side exists in sublation of the other.
    - [248-263] positive element of dissolution is self-identity of what appears.

- c2. id: wld-law-a-002-c2
  - subject: law
  - predicate: has_form_as
  - object: essential_positedness_and_connection_of_differences
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [264-282] reflected immediacy is positedness over immediate being.
    - [283-295] essential differences are simple determinations that are only through each other.

- c3. id: wld-law-a-002-c3
  - subject: law_and_appearance
  - predicate: share
  - object: one_content_with_law_as_substrate_of_appearance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [296-317] law and appearance have one and the same content continuous from appearance to law.
    - [327-344] law is immediately present in appearance; appearing world itself is kingdom of laws with dissolving movement.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: wld-law-a-001
  - sourceClaimIds: [wld-law-a-002-c1, wld-law-a-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [wld-law-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `A.2` deepens `A.1` by specifying law as explicitly posited essential connection.

- r2. type: transitions_to
  - targetEntryId: wld-law-a-003
  - sourceClaimIds: [wld-law-a-002-c3]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: from identity of content toward the law's unresolved deficiency and its push to real/necessary form.

Review outcome:

- review_pending
- notes: `A.2` complete; prepares determination of the law's insufficiency as merely positive essentiality.

### Entry wld-law-a-003 — Essential appearance, deficiency of immediate law, and demand for necessary form

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/law.txt`
- lineStart: 345
- lineEnd: 451

Summary:

In `A.3`, law is affirmed as essential appearance but exposed as still immediate and internally unproved, since its connected sides remain indifferent in content; this shortcoming necessitates transition from merely positive to real negative form.

Key points: (KeyPoint)

- k1. Law is essential appearance as identity of itself with unessential concrete existence.
- k2. Appearance as flux and self-mutating negativity exceeds the law's restful content, making appearance the richer totality at this stage.
- k3. In law, connection of determinations remains initially immediate and external (as in empirical law), lacking explicit necessity.
- k4. Law is essential form but not yet real form reflected into content; this drives transition beyond Part A.

Claims: (Claim)

- c1. id: wld-law-a-003-c1
  - subject: law
  - predicate: is
  - object: essential_appearance_identity_of_essential_and_unessential_concrete_existence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [345-353] law is essential appearance, reflection into itself in positedness, identical content with unessential concrete existence.
    - [354-370] identity is immediate and law remains indifferent to concrete existence, which includes additional determinations.

- c2. id: wld-law-a-003-c2
  - subject: appearance_relative_to_law
  - predicate: is
  - object: richer_totality_as_restless_negative_form
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [371-392] appearance is law as negative, self-mutating movement of passing-over and return.
    - [393-395] therefore appearance, against law, is totality because it contains law plus self-moving form.

- c3. id: wld-law-a-003-c3
  - subject: law_in_its_current_determination
  - predicate: lacks
  - object: explicit_necessity_of_content_connection_and_real_negative_form
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [396-434] law's identity is at first inner/immediate; connected determinations are initially posited and externally mediated.
    - [435-451] law is only positive essentiality, not yet real form reflected into content.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-law-a-002
  - sourceClaimIds: [wld-law-a-003-c3]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [wld-law-a-002-c2]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: the law's posited essentiality is retained but shown insufficient as long as necessity is only inner.

- r2. type: transitions_to
  - targetEntryId: wld-wld-b-001
  - sourceClaimIds: [wld-law-a-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: unresolved immediacy of law drives the transition into `B. THE WORLD OF APPEARANCE AND THE WORLD-IN-ITSELF`.

Review outcome:

- review_pending
- notes: Part A complete (`1/2/3`); next cadence step is Part B entry extraction.
