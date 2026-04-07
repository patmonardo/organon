# World Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `world/`, not a replacement for the local Part A/B/C workbooks.
- Read it to follow the chapter-level spine from law, through the opposed worlds, into the dissolution of appearance.
- Use the local `world/WORLD-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which appearance becomes lawful, then world-opposed, and finally dissolves into essential relation.
- Second question: what is the chapter's central operator?
  Answer: the collapse of the distinction between the world of appearance and the world in itself.
- Third question: where does the subfolder lead?
  Answer: to essential relation.

## Authority + format lock (must persist)

- Working extraction references: `world/world-idea.txt`, `world/law.txt`, `world/world.txt`, `world/disappearance.txt`, and `WORLD-DISTILLATION.md`
- Upstream source authority: `world/world-idea.txt`, `world/law.txt`, `world/world.txt`, `world/disappearance.txt`
- This workbook covers the `world/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-04 (World-folder pass)

Scope:

- files:
  - `world/world-idea.txt`
  - `world/law.txt`
  - `world/world.txt`
  - `world/disappearance.txt`
  - `WORLD-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `world/` subfolder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: law, world-in-itself, dissolution into relation.

### Entry hegel-appearance-world — Marker `World`

- sourceFiles:
  - `world/world-idea.txt`
  - `world/law.txt`
  - `world/world.txt`
  - `world/disappearance.txt`
- lineSpans:
  - `world/world-idea.txt:1-123`
  - `world/law.txt:2-327`
  - `world/world.txt:2-258`
  - `world/disappearance.txt:2-123`
- summary: The `world/` subfolder develops appearance as law, as the opposition of appearing and essential worlds, and as the dissolution of that opposition into essential relation.

Key points: (KeyPoint)

- k1. Appearance first stabilizes as law.
- k2. It then opposes a world in itself to the world of appearance.
- k3. That opposition dissolves into essential relation.

Claims: (Claim)

- c1. id: hegel-appearance-world-c1
  - subject: world_chapter
  - predicate: unfolds_through
  - object: law_world_opposition_and_dissolution
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `world/law.txt:2-327`
    - `world/world.txt:2-258`
    - `world/disappearance.txt:2-123`

- c2. id: hegel-appearance-world-c2
  - subject: world_chapter
  - predicate: culminates_in
  - object: essential_relation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `world/disappearance.txt:71-123`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-appearance-law
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: the chapter begins from law as essential appearance.
  - sourceClaimIds: [`hegel-appearance-world-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`hegel-appearance-law-c1`, `hegel-appearance-law-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-appearance-world-opposition
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: law develops into the opposition of worlds.
  - sourceClaimIds: [`hegel-appearance-world-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-appearance-world-opposition-c1`, `hegel-appearance-world-opposition-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-appearance-disappearance-threshold
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: the chapter culminates in the dissolution of world-opposition.
  - sourceClaimIds: [`hegel-appearance-world-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-disappearance-threshold-c1`, `hegel-appearance-disappearance-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the world chapter as one movement rather than two isolated cosmologies.

### Entry hegel-appearance-law — `World`: law of appearance

- sourceFiles:
  - `world/law.txt`
- lineSpans:
  - `world/law.txt:2-327`
- summary: Law is the positive element of appearance's mediation, the enduring self-identity of what appears in the flux of immediate determinate being.

Key points: (KeyPoint)

- k1. Appearance is reflected concrete existence.
- k2. Law is the essential identity of what appears.
- k3. Law is immediately present in appearance itself.

Claims: (Claim)

- c1. id: hegel-appearance-law-c1
  - subject: law
  - predicate: is
  - object: essential_appearance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `world/law.txt:4-108`
    - `world/law.txt:109-221`

- c2. id: hegel-appearance-law-c2
  - subject: law
  - predicate: is_not_beyond
  - object: appearance_but_immediately_present_in_it
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `world/law.txt:222-327`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-world-opposition
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: the totalized law gives rise to the world in itself.
  - sourceClaimIds: [`hegel-appearance-law-c1`, `hegel-appearance-law-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`hegel-appearance-world-opposition-c1`]

Review outcome:

- review_pending
- notes: this entry keeps law from floating free of the appearing world.

### Entry hegel-appearance-world-opposition — `World`: world of appearance and world-in-itself

- sourceFiles:
  - `world/world.txt`
- lineSpans:
  - `world/world.txt:2-258`
- summary: The appearing world and the world existing in and for itself arise as opposed totalities, but their relation is one of inversion and mutual implication, not sheer separation.

Key points: (KeyPoint)

- k1. A world in itself rises over against the world of appearance.
- k2. The essential world is the negative unity of appearance.
- k3. The two worlds stand in inversion.

Claims: (Claim)

- c1. id: hegel-appearance-world-opposition-c1
  - subject: world_in_itself
  - predicate: arises_as
  - object: opposite_of_world_of_appearance
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `world/world.txt:4-99`

- c2. id: hegel-appearance-world-opposition-c2
  - subject: relation_of_the_two_worlds
  - predicate: is
  - object: inversion
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `world/world.txt:100-258`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-disappearance-threshold
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: once inversion is explicit, the fixed opposition of worlds dissolves.
  - sourceClaimIds: [`hegel-appearance-world-opposition-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-disappearance-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the two-world schema tied to its own collapse.

### Entry hegel-appearance-disappearance-threshold — `World`: dissolution into relation

- sourceFiles:
  - `world/disappearance.txt`
- lineSpans:
  - `world/disappearance.txt:2-123`
- summary: Each world turns out to contain the other, and their supposed self-subsistence lies only in the unity of form that binds them, so appearance becomes essential relation.

Key points: (KeyPoint)

- k1. Each world is the other's appearance.
- k2. Their fixed difference disappears.
- k3. The truth of the two worlds is essential relation.

Claims: (Claim)

- c1. id: hegel-appearance-disappearance-threshold-c1
  - subject: world_of_appearance_and_essential_world
  - predicate: each_contains
  - object: the_other
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `world/disappearance.txt:2-54`
    - `world/disappearance.txt:55-93`

- c2. id: hegel-appearance-disappearance-threshold-c2
  - subject: truth_of_world_opposition
  - predicate: is
  - object: essential_relation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `world/disappearance.txt:94-123`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-relation
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: the next architectonic field is essential relation.
  - sourceClaimIds: [`hegel-appearance-disappearance-threshold-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-relation-c1`]

Review outcome:

- review_pending
- notes: this threshold entry preserves the dissolution as a real transition rather than a decorative conclusion.
