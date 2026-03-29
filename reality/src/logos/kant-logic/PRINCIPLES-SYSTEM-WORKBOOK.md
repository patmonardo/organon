# Principles System Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the architectonic of the System of Principles before the individual principle families are split out.
- In the current presentation, it should be read as the system-level bridge from schematism to the later principle modules.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-SYSTEM-DISTILLATION.md`
- Upstream source authority: `principles-system.md`
- This workbook covers only the introductory architectonic of the System of Principles.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not let the principle of contradiction spill over as a sufficient principle of synthetic truth.
- Do not detach synthetic a priori validity from possible experience.
- Do not confuse principles of empirical use with any supposed principles of transcendental use.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-system.md`
  - `PRINCIPLES-SYSTEM-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook at the system and architectonic level rather than trying to absorb the later detailed principle families.
- Preserve the contrast between analytic and synthetic principles.
- Keep the empirical-use restriction explicit when the table of principles is introduced.
- Keep the mathematical versus dynamical distinction visible in advance of the I-IV modules.

### Entry kant-trans-principles-system — Marker `System of Principles`

- sourceFiles:
  - `principles-system.md`
  - `PRINCIPLES-SYSTEM-DISTILLATION.md`
- lineSpans:
  - `principles-system.md:9-41`
  - `principles-system.md:47-106`
  - `principles-system.md:112-194`
  - `principles-system.md:200-274`
- summary: The System of Principles organizes the pure understanding's a priori judgments by showing that synthetic validity rests on possible experience and that the principles divide into mathematical and dynamical families as forms of empirical use in accordance with the categories.

Key points: (KeyPoint)

- k1. The system of principles follows the categories and possible experience.
- k2. The principle of contradiction governs analytic cognition only.
- k3. Synthetic a priori validity rests on the possibility of experience.
- k4. The principles divide into mathematical and dynamical classes.

Claims: (Claim)

- c1. id: kant-trans-principles-system-c1
  - subject: system_of_principles
  - predicate: organizes
  - object: pure_understanding_judgments_a_priori_in_accord_with_categories_and_possible_experience_as_principles_of_empirical_use
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-system.md:9-41`
    - `principles-system.md:248-274`

- c2. id: kant-trans-principles-system-c2
  - subject: supreme_principle_of_synthetic_judgments
  - predicate: states
  - object: conditions_of_possibility_of_experience_are_conditions_of_possibility_of_objects_of_experience
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-system.md:186-194`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles
  - targetWorkbook: `PRINCIPLES-WORKBOOK.md`
  - note: once schematism has secured the valid application of categories, the system of principles can be laid out.
  - sourceClaimIds: [`kant-trans-principles-system-c1`, `kant-trans-principles-system-c2`]
  - sourceKeyPointIds: [`k1`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-c1`, `kant-trans-principles-c2`]

Review outcome:

- review_pending
- notes: marker entry fixes the whole architectonic before the principle families are split into separate modules.

### Entry kant-trans-principles-system-analytic — `System of Principles`: analytic principle and its limit

- sourceFiles:
  - `principles-system.md`
  - `PRINCIPLES-SYSTEM-DISTILLATION.md`
- lineSpans:
  - `principles-system.md:47-106`
- summary: The principle of contradiction is the universal and sufficient principle of analytic cognition, but it remains only a negative and merely logical criterion of truth.

Key points: (KeyPoint)

- k1. Contradiction annihilates cognition as cognition.
- k2. The principle is sufficient for analytic judgments.
- k3. It does not determine synthetic truth.
- k4. The temporalized formula distorts its purely logical sense.

Claims: (Claim)

- c1. id: kant-trans-principles-system-analytic-c1
  - subject: principle_of_contradiction
  - predicate: is
  - object: universal_and_sufficient_principle_of_all_analytic_cognition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-system.md:57-78`

- c2. id: kant-trans-principles-system-analytic-c2
  - subject: principle_of_contradiction
  - predicate: does_not_supply
  - object: determining_ground_of_truth_for_synthetic_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-system.md:72-78`
    - `principles-system.md:80-106`

Relations: (Relation)

- r1. type: contrasts_with
  - targetEntryId: kant-trans-principles-system-synthetic
  - targetWorkbook: `PRINCIPLES-SYSTEM-WORKBOOK.md`
  - note: the system first clears the analytic criterion in order to state the genuinely transcendental principle of synthetic cognition.
  - sourceClaimIds: [`kant-trans-principles-system-analytic-c1`, `kant-trans-principles-system-analytic-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-system-synthetic-c1`, `kant-trans-principles-system-synthetic-c2`]

Review outcome:

- review_pending
- notes: the temporalized misformulation is kept because it shows exactly how synthetic content can be accidentally smuggled into mere logic.

### Entry kant-trans-principles-system-synthetic — `System of Principles`: supreme principle of synthetic judgments

- sourceFiles:
  - `principles-system.md`
  - `PRINCIPLES-SYSTEM-DISTILLATION.md`
- lineSpans:
  - `principles-system.md:112-194`
- summary: Synthetic judgments require a third mediating ground in time, imagination, and apperception, and their objective validity rests solely on the possibility of experience.

Key points: (KeyPoint)

- k1. Synthetic judgment goes beyond identity and contradiction.
- k2. A third thing is needed for synthetic connection.
- k3. Possible experience gives objective reality to synthetic a priori cognition.
- k4. The supreme principle states that objects of experience stand under conditions of possible experience.

Claims: (Claim)

- c1. id: kant-trans-principles-system-synthetic-c1
  - subject: possibility_of_synthetic_judgments
  - predicate: rests_on
  - object: inner_sense_time_imagination_and_unity_of_apperception_as_mediating_ground
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-system.md:127-137`

- c2. id: kant-trans-principles-system-synthetic-c2
  - subject: possible_experience
  - predicate: gives
  - object: objective_reality_to_all_our_a_priori_cognitions_and_grounds_supreme_synthetic_principle
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-system.md:139-194`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-system
  - targetWorkbook: `PRINCIPLES-SYSTEM-WORKBOOK.md`
  - note: this entry states the governing principle from which the later principle families will be derived.
  - sourceClaimIds: [`kant-trans-principles-system-synthetic-c1`, `kant-trans-principles-system-synthetic-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-system-c2`]

Review outcome:

- review_pending
- notes: this is the system-level centerpiece and keeps the whole principles sequence tied back to possible experience.

### Entry kant-trans-principles-system-table — `System of Principles`: mathematical and dynamical division

- sourceFiles:
  - `principles-system.md`
  - `PRINCIPLES-SYSTEM-DISTILLATION.md`
- lineSpans:
  - `principles-system.md:200-274`
- summary: The pure understanding is the source of principles because it is the source of rules, and the resulting table of principles divides into mathematical and dynamical forms of empirical use.

Key points: (KeyPoint)

- k1. Pure understanding is the source of principles.
- k2. The table of principles follows the categories.
- k3. Axioms, anticipations, analogies, and postulates are the four principle families.
- k4. Mathematical and dynamical principles differ in evidence and mode of empirical use.

Claims: (Claim)

- c1. id: kant-trans-principles-system-table-c1
  - subject: table_of_principles
  - predicate: follows
  - object: empirical_use_of_categories_in_four_families_of_axioms_anticipations_analogies_and_postulates
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-system.md:248-255`

- c2. id: kant-trans-principles-system-table-c2
  - subject: principles_of_pure_understanding
  - predicate: divide_into
  - object: mathematical_and_dynamical_classes_distinguished_by_intuitive_vs_discursive_evidence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-system.md:234-246`
    - `principles-system.md:257-274`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: kant-trans-principles-i
  - targetWorkbook: `PRINCIPLES-I-WORKBOOK.md`
  - note: the system-level table opens into the first principle family, axioms of intuition.
  - sourceClaimIds: [`kant-trans-principles-system-table-c1`, `kant-trans-principles-system-table-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-i-c1`]

Review outcome:

- review_pending
- notes: the forward transition is now explicit because the system file exists precisely to launch the four principle-family modules.
