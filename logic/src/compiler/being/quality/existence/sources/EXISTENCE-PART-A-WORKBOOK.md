# Existence Part A Workbook

Part: `A. EXISTENCE AS SUCH`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/existence/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `existence.txt` as authority.
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

### Entry (Topic) `id` — `title`

- span: `lineStart-lineEnd`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-22 (seed pass)

Scope:

- file: `existence.txt`
- fixed range: full file
- pass policy: lock 1 marker + triadic subdivision for each subspecies (`a`, `b`, `c`)

Decision:

- Three-level IDs for Part A internals:
  - Subspecies group IDs: `a-a`, `a-b`, `a-c`
  - Numbered entries within each group: `a-<letter>-<nnn>`
- Each subspecies is unfolded as a triadic movement (3 entries each).
- Numbered subentries are analytic segmentations, not additional source-labeled species.
- Preserve explicit transition target toward Part B marker `exi-b`.

### Entry exi-a — Marker `A`: Existence as such (`a`/`b`/`c` segmentation)

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 4
- lineEnd: 220

Summary:

Existence as such unfolds in three textual subtopics: existence in general, quality, and something.

Key points: (KeyPoint)

- k1. Subtopic `a` develops existence as immediate unity from becoming.
- k2. Subtopic `b` develops quality as determinate unity of being and non-being.
- k3. Subtopic `c` develops something as mediated self-equality through sublation of distinction.

Claims: (Claim)

- c1. id: exi-a-c1
  - subject: existence_as_such
  - predicate: has_subtopic
  - object: existence_in_general
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [15-17] explicit subtopic heading and opening statement.

- c2. id: exi-a-c2
  - subject: existence_as_such
  - predicate: has_subtopic
  - object: quality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [102-104] explicit subtopic heading and opening movement.

- c3. id: exi-a-c3
  - subject: existence_as_such
  - predicate: has_subtopic
  - object: something
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [157-160] explicit subtopic heading and opening distinction.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-a-001
  - note: marker to subtopic `a` entry.
  - sourceClaimIds: [`exi-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`a-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: a-b-001
  - note: marker to subtopic `b` entry.
  - sourceClaimIds: [`exi-a-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`a-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: a-c-001
  - note: marker to subtopic `c` entry.
  - sourceClaimIds: [`exi-a-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: segmentation now explicitly obeys text's own `a/b/c` structure.

### Entry a-a-001 — `a. Existence in general` I: from becoming to immediate existence

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 17
- lineEnd: 34

Summary:

Existence first appears as what proceeds from becoming: immediate unity of being and nothing with its mediation sublated behind it.

Key points: (KeyPoint)

- k1. Existence proceeds from becoming as simple oneness of being and nothing.
- k2. The becoming-mediation is sublated, so existence appears as an immediate first.
- k3. Existence initially appears under the one-sided determination of being.

Claims: (Claim)

- c1. id: a-a-001-c1
  - subject: existence
  - predicate: proceeds_from
  - object: becoming_as_unity_of_being_and_nothing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [17-20] existence proceeds from becoming and is simple oneness of being/nothing.

- c2. id: a-a-001-c2
  - subject: existence
  - predicate: appears_as
  - object: immediate_first_after_sublated_mediation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [21-24] becoming lies behind and has sublated itself; existence appears as first.

- c3. id: a-a-001-c3
  - subject: initial_determinacy_of_existence
  - predicate: is
  - object: one_sided_being_with_nothing_to_emerge_in_contrast
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [25-28] one-sided being first; nothing likewise comes up in contrast.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bei-c-003
  - note: explicates the becoming->existence transition announced in Being Part C.
  - sourceClaimIds: [`a-a-001-c1`, `a-a-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bei-c-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: a-a-002
  - note: moves from immediate origin to explicit determinateness as being-with-non-being.
  - sourceClaimIds: [`a-a-001-c1`, `a-a-001-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`a-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first moment of subspecies `a-a` triad.

### Entry a-a-002 — `a. Existence in general` II: determinateness as being-with-non-being

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 29
- lineEnd: 51

Summary:

Existence is developed as determinate being (`Dasein`): being with non-being taken into immediate unity as determinateness.

Key points: (KeyPoint)

- k1. Existence is not mere being but determinate being (`Dasein`).
- k2. As outcome of becoming, existence is being with non-being in simple unity.
- k3. Determinateness as such is this concrete unity in the form of immediacy.

Claims: (Claim)

- c1. id: a-a-002-c1
  - subject: existence
  - predicate: is
  - object: determinate_being_dasein_not_mere_being
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [29-34] not mere being but existence/Dasein.

- c2. id: a-a-002-c2
  - subject: existence
  - predicate: has_structure
  - object: being_with_non_being_taken_up_into_simple_unity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [38-43] being with non-being; non-being taken up into unity with being.

- c3. id: a-a-002-c3
  - subject: determinateness_as_such
  - predicate: is_constituted_by
  - object: concrete_unity_of_being_and_non_being_in_immediacy
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [44-51] concrete whole in form of being/immediacy constitutes determinateness.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-a-003
  - note: determinateness is further restricted by the methodological rule of what is posited.
  - sourceClaimIds: [`a-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second moment of subspecies `a-a` triad.

### Entry a-a-003 — `a. Existence in general` III: posited content vs external reflection

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 52
- lineEnd: 100

Summary:

The exposition fixes a methodological distinction between concept-posited determinacy and external reflection, while indicating that negation/something/other will become posited in the development.

Key points: (KeyPoint)

- k1. Only what is posited in the concept belongs to conceptual content.
- k2. External reflection can clarify but cannot ground what is to follow.
- k3. The one-sidedness of being is external now but will become posited through negation and something/other.

Claims: (Claim)

- c1. id: a-a-003-c1
  - subject: conceptual_elaboration_rule
  - predicate: includes_only
  - object: determinacies_posited_in_the_concept
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [59-64] only what is posited belongs to conceptual content.

- c2. id: a-a-003-c2
  - subject: external_reflection
  - predicate: has_status
  - object: clarificatory_or_anticipatory_but_not_constitutive
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [65-75] non-posited determinacies belong to reflection, including comparison.
    - [83-92] such reflections are not foundations of what follows.

- c3. id: a-a-003-c3
  - subject: one_sided_determinateness_of_being
  - predicate: will_become_posited_through
  - object: negation_something_and_other
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [77-80] one-sidedness is external reflection now.
    - [81-82] in negation/something/other it will become posited.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-b-001
  - note: methodological closure of `a` opens the explicit determination of quality.
  - sourceClaimIds: [`a-a-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third moment of subspecies `a-a` triad.

### Entry a-b-001 — `b. Quality` I: immediate unity of being and non-being

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 104
- lineEnd: 120

Summary:

Quality begins from immediate unity in which being and nothing do not overstep one another, and determinateness remains undetached from being.

Key points: (KeyPoint)

- k1. In existence, being and non-being are one in immediacy.
- k2. Determinateness does not detach from being and develops on unity-of-being/non-being basis.
- k3. No explicit differentiation of being and determinateness is yet posited.

Claims: (Claim)

- c1. id: a-b-001-c1
  - subject: existence
  - predicate: unifies
  - object: being_and_non_being_without_overstepping
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [104-108] neither oversteps the other; existent is equally non-being.

- c2. id: a-b-001-c2
  - subject: determinateness
  - predicate: remains
  - object: inseparable_from_being_on_basis_of_their_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [109-116] determinateness not detached and never detached from being.

- c3. id: a-b-001-c3
  - subject: current_connection_of_determinateness_and_being
  - predicate: is
  - object: immediate_unity_without_posited_differentiation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [117-120] immediate unity and no differentiation yet posited.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-b-002
  - note: from undifferentiated immediacy to quality as isolated existent determinateness.
  - sourceClaimIds: [`a-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first moment of subspecies `a-b` triad.

### Entry a-b-002 — `b. Quality` II: quality as simple immediate determinateness

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 121
- lineEnd: 134

Summary:

Quality is defined as existent determinateness isolated in its simplicity, while determinateness in general remains a broader genus that can develop further.

Key points: (KeyPoint)

- k1. Existent determinateness, taken in isolation, is quality.
- k2. Quality as such is totally simple and immediate.
- k3. Determinateness in general is more universal and can further become quantitative.

Claims: (Claim)

- c1. id: a-b-002-c1
  - subject: quality
  - predicate: is
  - object: existent_determinateness_in_isolated_form
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [121-123] determinateness isolated by itself as existent determinateness is quality.

- c2. id: a-b-002-c2
  - subject: quality_as_such
  - predicate: has_character
  - object: total_simplicity_and_immediacy
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [123-124] quality as simple, immediate.

- c3. id: a-b-002-c3
  - subject: determinateness_in_general
  - predicate: exceeds
  - object: quality_as_such_and_can_develop_quantitatively
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [125-128] determinateness is more universal and can be quantitative.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-b-003
  - note: simple quality is then re-inscribed into reality/negation differentiation.
  - sourceClaimIds: [`a-b-002-c1`, `a-b-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`a-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second moment of subspecies `a-b` triad.

### Entry a-b-003 — `b. Quality` III: quality differentiated as reality and negation

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 135
- lineEnd: 155

Summary:

Quality is posited in the determination of nothing and so differentiates as reality and negation, where negation is still an existent quality rather than abstract nothing.

Key points: (KeyPoint)

- k1. Existence itself measures and overcomes one-sided immediate quality.
- k2. Quality in distinct value appears as reality and as negation (limit/restriction).
- k3. Negation remains an existence-quality determined with non-being, not abstract nothing.

Claims: (Claim)

- c1. id: a-b-003-c1
  - subject: existence
  - predicate: repositions
  - object: quality_beyond_one_sided_immediacy
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [135-139] existence measures one-sidedness and posits quality also in determination of nothing.

- c2. id: a-b-003-c2
  - subject: quality_differentiation
  - predicate: unfolds_as
  - object: reality_and_negation_limit_restriction
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [140-143] distinct/reflected determinateness gives reality and negation.

- c3. id: a-b-003-c3
  - subject: negation
  - predicate: is
  - object: existent_quality_determined_with_non_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [153-155] negation is existence, a quality determined with non-being.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-c-001
  - note: reality/negation distinction is sublated into the determination of something.
  - sourceClaimIds: [`a-b-003-c2`, `a-b-003-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`a-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third moment of subspecies `a-b` triad.

### Entry a-c-001 — `c. Something` I: sublation of distinction into being-in-itself

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 159
- lineEnd: 187

Summary:

In something, reality and negation are retained and sublated so that existence becomes mediated self-equality, determined as being-in-itself.

Key points: (KeyPoint)

- k1. Reality and negation are both existences and their distinction is sublated, not omitted.
- k2. The result is mediated self-equal simplicity rather than undifferentiated beginning.
- k3. This sublated state is existence's own determinateness: existent being-in-itself, something.

Claims: (Claim)

- c1. id: a-c-001-c1
  - subject: reality_and_negation
  - predicate: are
  - object: moments_of_existence_sublated_in_their_distinction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [159-171] reality contains negation and negation is existence; distinction is unseparated.
    - [173-176] distinction cannot be left out, therefore it is sublated.

- c2. id: a-c-001-c2
  - subject: resulting_existence
  - predicate: is
  - object: self_equal_simplicity_mediated_through_sublation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [177-182] self-equal simplicity mediated through sublation of distinction.

- c3. id: a-c-001-c3
  - subject: existence_in_this_state
  - predicate: is_determined_as
  - object: being_in_itself_or_something
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [183-187] existence is being-in-itself; it is existent, something.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-c-002
  - note: being-in-itself is explicated as first negation of negation and mediated self-reference.
  - sourceClaimIds: [`a-c-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first moment of subspecies `a-c` triad.

### Entry a-c-002 — `c. Something` II: first negation-of-negation and mediation with itself

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 189
- lineEnd: 208

Summary:

Something is the first negation of negation: simple self-reference that already contains mediation with itself as a universal conceptual moment.

Key points: (KeyPoint)

- k1. Something is first negation of negation and simple existent self-reference.
- k2. This determination marks the beginning of subjectivity while still remaining indeterminate.
- k3. Mediation with itself is already present in something and in every concept.

Claims: (Claim)

- c1. id: a-c-002-c1
  - subject: something
  - predicate: is
  - object: first_negation_of_negation_as_simple_self_reference
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [189-190] something as first negation of negation and self-reference.

- c2. id: a-c-002-c2
  - subject: in_itselfness_of_something
  - predicate: has_status
  - object: beginning_of_subject_still_indeterminate
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [205-208] only beginning of subject; in-itselfness still indeterminate.

- c3. id: a-c-002-c3
  - subject: mediation_with_itself
  - predicate: is_present_in
  - object: something_and_every_concept
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [195-204] mediation with itself present and found everywhere in every concept.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-c-003
  - note: abstract self-mediation concretizes into transition, alteration, and otherness.
  - sourceClaimIds: [`a-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second moment of subspecies `a-c` triad.

### Entry a-c-003 — `c. Something` III: transition to other and concrete alteration

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 209
- lineEnd: 220

Summary:

Something, as concrete becoming, now carries an existent negative moment (other), so its becoming is alteration and transition.

Key points: (KeyPoint)

- k1. In something, one moment is existent and the other is existent as negative: an other.
- k2. Becoming at this level is concrete alteration between somethings.
- k3. Something initially maintains simple self-reference while its otherness remains only generally posited.

Claims: (Claim)

- c1. id: a-c-003-c1
  - subject: structure_of_something_as_becoming
  - predicate: includes
  - object: existent_moment_and_negative_existent_other
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [213-216] one moment existence/existent; other equally existent as negative, an other.

- c2. id: a-c-003-c2
  - subject: becoming_of_something
  - predicate: is
  - object: concrete_transition_or_alteration
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [216-217] as becoming, something is a transition and alteration.

- c3. id: a-c-003-c3
  - subject: initial_mode_of_something
  - predicate: is
  - object: simple_self_maintenance_with_other_posited_only_in_general
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [218-220] at first only self-maintaining in self-reference; negative as other in general.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-b
  - note: concrete alteration of something opens Part B (something/other and further determinations).
  - sourceClaimIds: [`a-c-003-c1`, `a-c-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third moment of subspecies `a-c` triad and handoff to Part B.
