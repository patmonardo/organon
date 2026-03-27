# Intro Logic Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes what transcendental logic is and how its field is divided before later analytic and dialectic modules are split further.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `INTRO-LOGIC-DISTILLATION.md`
- Upstream source authority: `intro-logic.md`
- This workbook covers only the introduction to transcendental logic.

## Clean-room rules

- Keep the pass on the Kant Transcendental Logic side.
- Do not confuse formal logic with transcendental, object-related cognition.
- Do not turn the critique of illusion into a positive ontology of objects beyond experience.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (first pass)

Scope:

- files:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- pass policy: 1 marker entry + 1 analytic entry

Decision:

- Keep this workbook introductory and architectonic.
- Fix the distinction between general logic and transcendental logic.
- Keep the analytic-dialectic split visible before later component workbooks are added.

### Entry kant-trans-intro — Marker `Transcendental Logic`

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `INTRO-LOGIC-DISTILLATION.md:7-55`
- summary: The introduction defines transcendental logic as object-related a priori logic and divides it into analytic and dialectic.

Key points: (KeyPoint)

- k1. Logic presupposes the distinction between sensibility and understanding.
- k2. Transcendental logic concerns a priori cognition as related to objects.
- k3. The introduction divides transcendental logic into analytic and dialectic.

Claims: (Claim)

- c1. id: kant-trans-intro-c1
  - subject: transcendental_logic
  - predicate: studies
  - object: laws_of_pure_understanding_and_reason_as_related_to_objects_a_priori
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `INTRO-LOGIC-DISTILLATION.md:20-29`

- c2. id: kant-trans-intro-c2
  - subject: transcendental_logic
  - predicate: divides_into
  - object: transcendental_analytic_and_transcendental_dialectic
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `INTRO-LOGIC-DISTILLATION.md:46-55`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: the analytic side first opens through the discovery of the pure concepts of understanding.
  - sourceClaimIds: [`kant-trans-intro-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-trans-concepts-c1`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-ideas
  - targetWorkbook: `INTRO-IDEAS-WORKBOOK.md`
  - note: the dialectical side later opens through the critique of transcendental illusion.
  - sourceClaimIds: [`kant-trans-intro-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-trans-ideas-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the architectonic role of the introduction before later analytic and dialectic workbooks are populated.

### Entry kant-trans-intro-critique — `Transcendental Logic`: truth, critique, and illusion

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `INTRO-LOGIC-DISTILLATION.md:33-55`
- summary: The introduction argues that formal logic gives only a negative criterion of truth and that transcendental logic must therefore distinguish lawful critique from illusion.

Key points: (KeyPoint)

- k1. General logic judges only formal consistency.
- k2. Formal consistency is not sufficient for objective truth.
- k3. Dialectic begins when logic is misused as an organon beyond possible experience.

Claims: (Claim)

- c1. id: kant-trans-intro-critique-c1
  - subject: general_logic
  - predicate: provides
  - object: negative_criterion_of_formal_truth
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `INTRO-LOGIC-DISTILLATION.md:33-42`

- c2. id: kant-trans-intro-critique-c2
  - subject: misuse_of_logic_as_organon
  - predicate: generates
  - object: dialectical_illusion_about_objects_beyond_experience
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `INTRO-LOGIC-DISTILLATION.md:33-42`
    - `INTRO-LOGIC-DISTILLATION.md:46-55`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: the analytic entry shows why the introduction must divide logic into truth-guiding analytic and illusion-critiquing dialectic.
  - sourceClaimIds: [`kant-trans-intro-critique-c1`, `kant-trans-intro-critique-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c1`, `kant-trans-intro-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the truth-question and the critique of illusion together at seed scale.
