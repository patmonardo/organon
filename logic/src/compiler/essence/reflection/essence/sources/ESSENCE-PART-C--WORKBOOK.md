# Essence Part C (TopicMap) Workbook (V2)

Part: `C. REFLECTION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.
- Do not introduce alternate entry styles, headings, or compressed formats.

## Clean-room rules

- Source authority is `reflection.txt` only.
- Claims must be line-anchored.
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

## Session: 2026-02-18 (generated)

Scope:

- file: `reflection.txt`
- focus: movement of reflection and the triad (`Positing`, `External`, `Determining`)
- fixed range (active in this pass): lines `4-481`

Decision:

- Use Part B workbook format unchanged.
- Working section alignment (provisional): prelude (`4-66`), then source-labeled movements `1. Positing reflection` (`69-228`), `2. External reflection` (`232-314`), `3. Determining reflection` (`317-481`).
- Prelude handling: entries for `4-66` are deferred (candidate move to IDEA workbook) and are not included in this Part C workbook pass.
- Entry grouping for review: positing = `ess-ref-c-001..004`; external = `ess-ref-c-005..007`; determining = `ess-ref-c-008..010`.
- Determining-reflection internal labels respected in spans: `1` -> `ess-ref-c-008` (`317-371`), `2` -> `ess-ref-c-009` (`373-441`), `3` -> `ess-ref-c-010` (`443-481`).
- Section labels are tracked in this Decision block and spans, not encoded in entry headings.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in entry headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Align this pass with `WORKBOOK-CONTRACT-V1.md` Path-forward principle (`LogoLogia`) and KG-first direction.
- Keep `Key points` and `Claims` first-order, source-restricted, and line-anchored to `reflection.txt`.
- Treat `Relations` as second-order modeling expanded across iterative cycles (`Reflection -> Appearance -> relation-expansion cycles`).
- Keep `review_pending` where relation semantics remain provisional.
- Treat relation-building as a proof-trace of the passage from Essence/Appearance into Concept, to inform the Meta Compiler Generation system.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay for this workbook pass; keep claims unchanged.

### Entry ess-ref-c-001 — Positing reflection as self-referring negativity

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 69
- lineEnd: 106

Summary:

Positing reflection is developed as self-negating negativity whose coincidence with itself is immediacy that is simultaneously negative of itself.

Key points: (KeyPoint)

- k1. Shine as lack of essence has being as equality-with-self, not in an other.
- k2. Self-referring negativity is both negativity and sublated negativity in one unity.
- k3. Reflection's transition is sublated transition, not passage into alien being.
- k4. Immediacy here is self-negating equality whose being is to be what it is not.

Claims: (Claim)

- c1. id: ess-ref-c-001-c1
  - subject: shine_nothingness
  - predicate: has_being_as
  - object: own_equality_with_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [69-74] nothingness does not shine in another; its being is equality with itself as absolute reflection.

- c2. id: ess-ref-c-001-c2
  - subject: self_referring_negativity
  - predicate: is
  - object: unity_of_negativity_and_sublated_negativity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [76-84] negativity negates itself and is both negative and simple self-equality.

- c3. id: ess-ref-c-001-c3
  - subject: reflective_immediacy
  - predicate: is
  - object: self_negating_equality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [86-106] reflection as nothing-to-nothing; immediacy is negative of itself and is what it is not.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-c-002
  - note: establishes self-negating immediacy presupposed by the turning-back account.
  - sourceClaimIds: [`ess-ref-c-001-c1`, `ess-ref-c-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ess-ref-c-002-c1`, `ess-ref-c-002-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-002
  - note: moves from self-negating equality to positedness/turning-back determinateness.
  - sourceClaimIds: [`ess-ref-c-001-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`ess-ref-c-002-c1`, `ess-ref-c-002-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: no comparative additions in first-order blocks.

### Entry ess-ref-c-002 — Turning-back immediacy as positedness

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 107
- lineEnd: 127

Summary:

The self-reference of the negative is turning-back into itself, where immediacy is positedness and reflection exists only in the return that begins and ends in that return.

Key points: (KeyPoint)

- k1. Self-reference of the negative is turning-back into itself.
- k2. Immediacy is self-sublating and thus positedness.
- k3. Reflective movement cannot begin from a prior immediacy.
- k4. Reflection exists only as this turning in which start and return coincide.

Claims: (Claim)

- c1. id: ess-ref-c-002-c1
  - subject: negative_self_reference
  - predicate: is
  - object: turning_back_into_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [107-113] self-reference is turning back and self-sublating immediacy.

- c2. id: ess-ref-c-002-c2
  - subject: positedness
  - predicate: is
  - object: immediacy_as_determinateness_of_shine
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [113-119] positedness as immediacy purely as determinateness/self-reflecting.

- c3. id: ess-ref-c-002-c3
  - subject: reflection
  - predicate: is
  - object: movement_existing_only_in_turning_back
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [120-127] no beginning from immediacy; reflection is only in turning where what starts/returns is.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-c-001
  - note: grounds self-negating immediacy in the explicit category of positedness.
  - sourceClaimIds: [`ess-ref-c-002-c1`, `ess-ref-c-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ess-ref-c-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-003
  - note: the turning-back form now develops into presupposing and self-repulsion.
  - sourceClaimIds: [`ess-ref-c-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`ess-ref-c-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: semantics of beginning/return can be enriched in relation cycles.

### Entry ess-ref-c-003 — Positing as presupposing and self-repulsion

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 128
- lineEnd: 166

Summary:

Positing reflection, precisely as turning-back, sublates its own positing into presupposing, yielding self-repulsion and inner-directed reflection.

Key points: (KeyPoint)

- k1. Positing has no prior other and exists only as return/negative-of-itself.
- k2. As sublating negativity, reflection is equally presupposing.
- k3. Immediacy is semblance of beginning negated by return.
- k4. Inner-directed reflection is presupposing that from which it turns back.

Claims: (Claim)

- c1. id: ess-ref-c-003-c1
  - subject: positing_reflection
  - predicate: is
  - object: turning_back_without_prior_other
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [128-133] no other beforehand; reflection is only as turning-back/negative of itself.

- c2. id: ess-ref-c-003-c2
  - subject: reflection
  - predicate: becomes
  - object: presupposing_by_sublating_its_own_positing
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [134-151] sublating negative and positing leads to presupposing.

- c3. id: ess-ref-c-003-c3
  - subject: inner_directed_reflection
  - predicate: is
  - object: presupposing_of_that_from_which_it_turns_back
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [158-165] immediacy as semblance of beginning; turning back as self-repulsion and essential presupposing.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-c-002
  - note: explains how turning-back implies presupposition without external substrate.
  - sourceClaimIds: [`ess-ref-c-003-c2`, `ess-ref-c-003-c3`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`ess-ref-c-002-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-004
  - note: proceeds to full account of presupposition/self-arrival and determination of external reflection.
  - sourceClaimIds: [`ess-ref-c-003-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`ess-ref-c-004-c1`, `ess-ref-c-004-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: presupposition paradox to be expanded in second-order relation notes.

### Entry ess-ref-c-004 — Presupposition, counter-repelling movement, and the threshold of external reflection

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 167
- lineEnd: 228

Summary:

Reflection presupposes itself and sublates that presupposition through an absolute internal counter-repelling movement, where immediate positedness is both found and produced, culminating in external reflection.

Key points: (KeyPoint)

- k1. Essence is equal to itself only through sublating its self-presupposition.
- k2. Reflection's transcending of the immediate is identical with arriving at it.
- k3. Reflection is self-movement in which positing and presupposing mutually imply each other.
- k4. Presupposed immediacy is only positedness/turning-back and thereby determines external reflection.

Claims: (Claim)

- c1. id: ess-ref-c-004-c1
  - subject: essence
  - predicate: presupposes_and_sublates
  - object: itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [167-173] essence presupposes itself; sublating presupposition is essence itself.

- c2. id: ess-ref-c-004-c2
  - subject: reflective_movement
  - predicate: is
  - object: absolute_internal_counter_repelling_self_movement
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [190-207] movement as internal counter-repelling where transcending immediate is arriving at immediate.

- c3. id: ess-ref-c-004-c3
  - subject: reflection_determinate
  - predicate: is
  - object: external_reflection_when_presupposition_is_taken_as_other
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [213-228] presupposed immediacy is positedness but determined as negative opposed-to-other; thus reflection is external reflection.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-c-003
  - note: provides extended derivation of presupposition dynamic implicit in positing reflection.
  - sourceClaimIds: [`ess-ref-c-004-c1`, `ess-ref-c-004-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`ess-ref-c-003-c2`, `ess-ref-c-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-005
  - note: threshold statement becomes explicit treatment of external reflection.
  - sourceClaimIds: [`ess-ref-c-004-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`ess-ref-c-005-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: core for proof-trace from positing to external is captured; relation thickening pending.

### Entry ess-ref-c-005 — External reflection and the sublation of externality

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 232
- lineEnd: 277

Summary:

External reflection first appears as doubled presupposing that takes immediacy as its starting other and posits determinations externally in the immediate.

Key points: (KeyPoint)

- k1. External reflection is doubled as presupposed immediate and negative self-reference.
- k2. It starts from the immediate as its other while negating it as its negative.
- k3. Determinations it posits in the immediate are external to it.
- k4. This reproduces the finite/infinite contrast known from the sphere of being.

Claims: (Claim)

- c1. id: ess-ref-c-005-c1
  - subject: external_reflection
  - predicate: is_doubled_as
  - object: presupposed_immediate_and_negative_self_reference
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [232-247] reflection presupposes itself as sublated and is doubled as immediate and negative self-reference.

- c2. id: ess-ref-c-005-c2
  - subject: external_reflection
  - predicate: starts_from_and_negates
  - object: immediate_as_its_other
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [249-263] reflection starts from immediate presupposition and negates it as its negative.

- c3. id: ess-ref-c-005-c3
  - subject: external_reflection
  - predicate: posits_as_external
  - object: determinations_in_the_immediate
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [264-277] determinateness as positedness is not its concern; determinations are external, with finite/infinite contrast.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-c-004
  - note: develops the threshold claim of external reflection into its initial doubled structure.
  - sourceClaimIds: [`ess-ref-c-005-c1`, `ess-ref-c-005-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ess-ref-c-004-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-006
  - note: external reflection next appears in explicit syllogistic form.
  - sourceClaimIds: [`ess-ref-c-005-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`ess-ref-c-006-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first external-reflection movement isolated as 232-277.

### Entry ess-ref-c-006 — External reflection as syllogistic mediation

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 278
- lineEnd: 285

Summary:

External reflection is explicitly articulated as a syllogism connecting immediate and reflection-into-self through determinate immediacy.

Key points: (KeyPoint)

- k1. External reflection is presented as a syllogism.
- k2. The extremes are immediate and reflection-into-self.
- k3. The middle term is connecting reference as determinate immediacy.

Claims: (Claim)

- c1. id: ess-ref-c-006-c1
  - subject: external_reflection
  - predicate: operates_as
  - object: syllogistic_mediation_between_immediate_and_reflection_into_self
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [278-285] external reflection presented as syllogism with immediate and reflection-into-self as extremes.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-c-005
  - note: specifies the formal mediation structure implicit in external reflection's doubled setup.
  - sourceClaimIds: [`ess-ref-c-006-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`ess-ref-c-005-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-007
  - note: from syllogistic form to the self-negating operation that sublates externality.
  - sourceClaimIds: [`ess-ref-c-006-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ess-ref-c-007-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: isolated formal paragraph 278-285 as its own entry.

### Entry ess-ref-c-007 — Sublation of externality into determining reflection

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 286
- lineEnd: 314

Summary:

Closer inspection shows external reflection as self-negating positing in which the immediate is posited as identical with reflection, thereby sublating externality into determining reflection.

Key points: (KeyPoint)

- k1. External reflection is both positing and sublating of positing.
- k2. The immediate is posited as the same as reflection.
- k3. Reflection's externality to the immediate is sublated.
- k4. The result is determining reflection.

Claims: (Claim)

- c1. id: ess-ref-c-007-c1
  - subject: external_reflection
  - predicate: posits_and_sublates
  - object: immediate_as_its_negative
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [286-297] external reflection posits immediate as negative and equally sublates this positing.

- c2. id: ess-ref-c-007-c2
  - subject: immediate
  - predicate: is_posited_as
  - object: same_as_reflection
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [297-308] immediate is posited as same as reflection through self-negating positing.

- c3. id: ess-ref-c-007-c3
  - subject: external_reflection
  - predicate: sublates_into
  - object: determining_reflection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [309-314] external reflection is not external but immanent, hence determining reflection.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-c-006
  - note: turns syllogistic mediation into explicit self-negating operation.
  - sourceClaimIds: [`ess-ref-c-007-c1`, `ess-ref-c-007-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`ess-ref-c-006-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-008
  - note: result of external reflection opens the first movement of determining reflection.
  - sourceClaimIds: [`ess-ref-c-007-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`ess-ref-c-008-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition to determining reflection isolated at 309-314.

### Entry ess-ref-c-008 — Determining reflection: positedness, existence, and superiority

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 317
- lineEnd: 371

Summary:

Determining reflection is introduced as unity of positing and external reflection, and positedness is established as the principle and superior truth of existence in the sphere of essence.

Key points: (KeyPoint)

- k1. Determining reflection is the unity of positing and external reflection.
- k2. In determining reflection, the posited is immediate only as self-negating and sublated.
- k3. Positedness corresponds to existence in essence and is its principle.
- k4. Positedness is superior to existence because existence as posited is negative self-reference.

Claims: (Claim)

- c1. id: ess-ref-c-008-c1
  - subject: determining_reflection
  - predicate: is
  - object: unity_of_positing_and_external_reflection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [317-319] determining reflection explicitly defined as their unity.

- c2. id: ess-ref-c-008-c2
  - subject: positedness
  - predicate: corresponds_to
  - object: existence_in_essence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [345-353] positedness corresponds to existence and is principle of essence of existence.

- c3. id: ess-ref-c-008-c3
  - subject: positedness
  - predicate: is_superior_to
  - object: existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [362-371] positedness superior because existence as posited is negative reference to turning-back.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-c-007
  - note: confirms external reflection's result by defining determining reflection's basic structure.
  - sourceClaimIds: [`ess-ref-c-008-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`ess-ref-c-007-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-009
  - note: moves from definition to the internal logic of determination-of-reflection versus quality.
  - sourceClaimIds: [`ess-ref-c-008-c2`, `ess-ref-c-008-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`ess-ref-c-009-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: superiority claim may need fine-grained relation links to Being chapter during cross-part cycle.

### Entry ess-ref-c-009 — Determination of reflection versus quality

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 373
- lineEnd: 441

Summary:

When positing unites with external reflection, positedness becomes determination of reflection, which unlike quality is grounded in immanent reflectedness and persists as essential determinateness.

Key points: (KeyPoint)

- k1. Positedness becomes determination of reflection when positing and external reflection are unified.
- k2. Determination of reflection differs from quality by its ground in immanent reflectedness.
- k3. Reflection-determination persists through self-equality of negatedness, not immediate being.
- k4. Determinations of reflection appear as fixed free essentialities/essential shine.
- k5. Determination has two sides: posited negation and immanent reflection.

Claims: (Claim)

- c1. id: ess-ref-c-009-c1
  - subject: positedness
  - predicate: becomes
  - object: determination_of_reflection
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [373-381] positing united with external reflection yields positedness as determination of reflection.

- c2. id: ess-ref-c-009-c2
  - subject: determination_of_reflection
  - predicate: differs_from
  - object: quality_by_immanent_reflected_ground
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [383-408] contrast with quality; persistence through reflection's self-equality and sublated negation.

- c3. id: ess-ref-c-009-c3
  - subject: determination_of_reflection
  - predicate: has_two_sides
  - object: posited_negation_and_immanent_reflection
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [426-441] two-sided structure: posited negation and reflected-into-self equality.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3, k4
- c3 -> k5

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-c-008
  - note: deepens determining reflection from structural definition to determinative content.
  - sourceClaimIds: [`ess-ref-c-009-c1`, `ess-ref-c-009-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`ess-ref-c-008-c1`, `ess-ref-c-008-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-c-010
  - note: culminates in the explicit synthesis of positedness and immanent reflection.
  - sourceClaimIds: [`ess-ref-c-009-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`ess-ref-c-010-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: fixed-essentiality language ready for second-order Concept mapping.

### Entry ess-ref-c-010 — Final synthesis: determination as infinite self-reference

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/reflection.txt`
- lineStart: 443
- lineEnd: 481

Summary:

The final synthesis states that determination of reflection is simultaneously positedness and immanent reflection, i.e., negation that has taken otherness back into itself as infinite self-reference.

Key points: (KeyPoint)

- k1. Determination of reflection is both positedness and immanently reflected reference.
- k2. Positedness is sublatedness of determination, while reflectedness is subsisting.
- k3. Reflection-determination includes within itself determinate side and reference to its negation.
- k4. Unlike quality's transition-to-other, reflection-determination internalizes otherness.
- k5. Essentiality is unity of negation with itself and its other in reflection-into-self.

Claims: (Claim)

- c1. id: ess-ref-c-010-c1
  - subject: determination_of_reflection
  - predicate: is
  - object: unity_of_positedness_and_immanent_reflection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [443-457] determination has both immanently reflected reference and positedness.

- c2. id: ess-ref-c-010-c2
  - subject: reflection_determinateness
  - predicate: includes
  - object: determinate_side_and_reference_to_its_negation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [465-468] determinate side and reference to negation are internal to the determination.

- c3. id: ess-ref-c-010-c3
  - subject: reflection_determination
  - predicate: is
  - object: essentiality_as_infinite_self_reference
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [471-481] otherness taken back into itself; unity with other; reflection into self as infinite self-reference.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4, k5

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-c-009
  - note: completes the two-sided structure by explicit synthesis.
  - sourceClaimIds: [`ess-ref-c-010-c1`, `ess-ref-c-010-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`ess-ref-c-009-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-d-001
  - note: carries forward from reflection synthesis into the next Appearance-phase entry in the concept proof-trace.
  - sourceClaimIds: [`ess-ref-c-010-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: suitable anchor for next-cycle mapping from Reflection toward Appearance and Concept proof-trace.
