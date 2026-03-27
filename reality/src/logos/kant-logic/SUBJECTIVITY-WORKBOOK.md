# Subjectivity Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes subjectivity as an architectonic container logic and names its inner triad.
- Use the distillation as the fast commentary path out of this workbook.
- Return to the source text when checking whether the workbook remains complete and correct.

## Quick orientation

- First question: what is subjectivity here?
  Answer: the formal side of discursive cognition, not empirical inwardness.
- Second question: how is the triad now being read?
  Answer: concept as universality-region, judgment as explicit-unity region, and syllogism as derivational region inside one formal container.
- Third question: how should this workbook be used?
  Answer: read the workbook for logical commitments, the distillation for quick commentary, and the source text for verification.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `subjectivity.md` and `SUBJECTIVITY-DISTILLATION.md`
- Upstream source authority: `subjectivity.md`
- This workbook covers only the subjectivity module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Do not collapse subjectivity into empirical psychology.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (alignment pass)

Scope:

- files:
  - `subjectivity.md`
  - `SUBJECTIVITY-DISTILLATION.md`
- pass policy: 1 marker entry + 1 analytic entry

Decision:

- Recast subjectivity as architectonic container logic rather than as a light prefatory index.
- State what subjectivity is, record the triad as internal formal regions, and preserve the restriction of logical reflection to form.
- Treat the distillation as commentary on workbooked logic rather than as a replacement for source authority.
- Leave detailed concept, judgment, and syllogism machinery to the linked component workbooks.

### Entry kant-subj — Marker `Subjectivity`

- sourceFiles:
  - `subjectivity.md`
  - `SUBJECTIVITY-DISTILLATION.md`
- lineSpans:
  - `subjectivity.md:1-96`
- summary: Subjectivity names the formal architectonic of discursive cognition rather than an empirical account of inner life.

Key points: (KeyPoint)

- k1. Subjectivity is formal rather than psychological.
- k2. It orders how consciousness relates representations to an object.
- k3. It contains concept, judgment, and inference as internal formal regions.

Claims: (Claim)

- c1. id: kant-subj-c1
  - subject: subjectivity
  - predicate: is
  - object: formal_architectonic_of_discursive_cognition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `subjectivity.md:1-9`
    - `subjectivity.md:51-69`

- c2. id: kant-subj-c2
  - subject: subjectivity
  - predicate: contains
  - object: concept_judgment_and_inference_as_internal_regions
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `subjectivity.md:1-96`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: kant-concept
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: the first internal region of subjectivity is concept.
  - sourceClaimIds: [`kant-subj-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-concept-c1`]

- r2. type: unfolds_to
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: subjectivity makes its unity explicit in judgment.
  - sourceClaimIds: [`kant-subj-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`]

- r3. type: unfolds_to
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: subjectivity completes its basic grammar in inference.
  - sourceClaimIds: [`kant-subj-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`]

Review outcome:

- review_pending
- notes: marker entry now treats subjectivity as architectonic container logic rather than as an introductory label.

### Entry kant-subj-triad — `Subjectivity`: triadic formal container

- sourceFiles:
  - `subjectivity.md`
  - `SUBJECTIVITY-DISTILLATION.md`
- lineSpans:
  - `subjectivity.md:1-96`
- summary: Subjectivity unfolds through concept, judgment, and inference as internal formal regions of discursive cognition, with inference itself distributed by faculty.

Key points: (KeyPoint)

- k1. Concept provides universality as the first formal region.
- k2. Judgment provides explicit unity as the second formal region.
- k3. Inference provides derivational transition as the third formal region.

Claims: (Claim)

- c1. id: kant-subj-triad-c1
  - subject: subjectivity
  - predicate: unfolds_through
  - object: concept_judgment_inference
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `subjectivity.md:1-96`

- c2. id: kant-subj-triad-c2
  - subject: concept_judgment_inference
  - predicate: functions_as
  - object: formal_container_logic_of_discursive_cognition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `subjectivity.md:1-96`

Relations: (Relation)

- r1. type: summarized_by
  - targetEntryId: kant-subj
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: the triad explicates the formal side named by the marker entry.
  - sourceClaimIds: [`kant-subj-triad-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-subj-c1`]

- r2. type: unfolds_to
  - targetEntryId: kant-concept
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: concept is the first explicitly articulated region inside subjectivity.
  - sourceClaimIds: [`kant-subj-triad-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`kant-concept-c2`]

- r3. type: unfolds_to
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: judgment is the second region, where unity of consciousness becomes explicit and formalized.
  - sourceClaimIds: [`kant-subj-triad-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-judgment-c2`]

- r4. type: unfolds_to
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: inference is the third region, where discursive cognition extends itself by derivation.
  - sourceClaimIds: [`kant-subj-triad-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-syllogism-c1`]

Review outcome:

- review_pending
- notes: the triad is now preserved as an internal architectonic rather than as a simple topical sequence.

### Entry kant-subj-logical-reflection — `Subjectivity`: logical reflection and formal restriction

- sourceFiles:
  - `subjectivity.md`
  - `SUBJECTIVITY-DISTILLATION.md`
- lineSpans:
  - `subjectivity.md:56-69`
- summary: Subjectivity fixes the specific restriction of general logic: judgment has matter and form, but logical reflection considers only the formal differences among judgments while abstracting from matter and from the content of concepts.

Key points: (KeyPoint)

- k1. Judgments have matter and form as essential constituents.
- k2. Logical reflection abstracts from the matter of judgments and the content of concepts.
- k3. General logic attends only to formal differences among judgments.

Claims: (Claim)

- c1. id: kant-subj-logical-reflection-c1
  - subject: logical_reflection
  - predicate: abstracts_from
  - object: matter_of_judgments_and_content_of_concepts
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `subjectivity.md:56-69`

- c2. id: kant-subj-logical-reflection-c2
  - subject: general_logic
  - predicate: attends_to
  - object: mere_formal_differences_among_judgments
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `subjectivity.md:63-69`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-subj
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: this entry fixes the methodological restriction under which general logic treats the subjectivity triad.
  - sourceClaimIds: [`kant-subj-logical-reflection-c1`, `kant-subj-logical-reflection-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-subj-c1`]

- r2. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: judgment is the place where the formal restriction of general logic is made explicit.
  - sourceClaimIds: [`kant-subj-logical-reflection-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`]

Review outcome:

- review_pending
- notes: this entry records the formal restriction that makes General Logic a bootstrap layer rather than a full ontology of cognition.
