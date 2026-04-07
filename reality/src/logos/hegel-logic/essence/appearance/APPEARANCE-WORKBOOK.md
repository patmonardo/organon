# Appearance Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `appearance/` folder.
- Read it to follow the folder-level architectonic from `thing/`, through `world/`, into `relation/`.
- Its task is to preserve the high-level spine of the appearance band within the Doctrine of Essence.
- Use the chapter-level files [THING-WORKBOOK.md](THING-WORKBOOK.md), [WORLD-WORKBOOK.md](WORLD-WORKBOOK.md), and [RELATION-WORKBOOK.md](RELATION-WORKBOOK.md) for the detailed readable walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the folder-level sequence in which essence first appears as thinghood, then stabilizes itself as lawful world-appearance, and finally becomes essential relation.
- Second question: what is the central operator of the folder as a whole?
  Answer: appearance as essence made concrete and made relational.
- Third question: where does the folder lead?
  Answer: out of appearance and into actuality.

## Authority + format lock (must persist)

- Working extraction references: `appearance-idea.txt`, `thing/thing-idea.txt`, `world/world-idea.txt`, `relation/relation-idea.txt`, and `APPEARANCE-DISTILLATION.md`
- Upstream source authority: `appearance-idea.txt`, `thing/thing-idea.txt`, `world/world-idea.txt`, `relation/relation-idea.txt`
- This workbook covers the whole `appearance/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not collapse the folder-level container into one of its child chapter-clusters.
- Do not duplicate the chapter-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-04 (appearance-folder container pass)

Scope:

- files:
  - `appearance-idea.txt`
  - `thing/thing-idea.txt`
  - `world/world-idea.txt`
  - `relation/relation-idea.txt`
  - `APPEARANCE-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `appearance/` folder.
- Give each major child folder its own chapter-level artifact pair.
- Keep the emphasis on the folder sequence: `thing/`, `world/`, `relation/`.
- Use this file as the structural model for the top layer of `actuality/`.

### Entry hegel-appearance-folder — Marker `Appearance`

- sourceFiles:
  - `appearance-idea.txt`
  - `thing/thing-idea.txt`
  - `world/world-idea.txt`
  - `relation/relation-idea.txt`
- lineSpans:
  - `appearance-idea.txt:1-82`
  - `thing/thing-idea.txt:1-209`
  - `world/world-idea.txt:1-123`
  - `relation/relation-idea.txt:1-111`
- summary: The `appearance/` folder unfolds as one high-level movement from thinghood, through the world of appearance and its law, into essential relation as the truth of appearance.

Key points: (KeyPoint)

- k1. Essence must appear.
- k2. The appearance band first takes the form of thinghood.
- k3. It then becomes world-appearance and law.
- k4. It culminates in essential relation and points toward actuality.

Claims: (Claim)

- c1. id: hegel-appearance-folder-c1
  - subject: appearance_folder
  - predicate: unfolds_through
  - object: thing_world_and_relation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `appearance-idea.txt:1-82`

- c2. id: hegel-appearance-folder-c2
  - subject: appearance_folder
  - predicate: mediates
  - object: from_essence_made_concrete_to_actuality_threshold
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `appearance-idea.txt:1-82`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-appearance-thing-folder
  - targetWorkbook: `APPEARANCE-WORKBOOK.md`
  - note: the folder begins with the immediate appearance of essence as thinghood.
  - sourceClaimIds: [`hegel-appearance-folder-c1`, `hegel-appearance-folder-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-appearance-thing-folder-c1`, `hegel-appearance-thing-folder-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-appearance-world-folder
  - targetWorkbook: `APPEARANCE-WORKBOOK.md`
  - note: appearance then stabilizes itself as lawful world-appearance.
  - sourceClaimIds: [`hegel-appearance-folder-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-world-folder-c1`, `hegel-appearance-world-folder-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-appearance-relation-folder
  - targetWorkbook: `APPEARANCE-WORKBOOK.md`
  - note: the folder culminates in essential relation.
  - sourceClaimIds: [`hegel-appearance-folder-c1`, `hegel-appearance-folder-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-appearance-relation-folder-c1`, `hegel-appearance-relation-folder-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the top-level `appearance/` surface as container artifact rather than leaving the folder without a readable root layer.

### Entry hegel-appearance-thing-folder — `Appearance`: the `thing/` opening field

- sourceFiles:
  - `thing/thing-idea.txt`
  - `thing/thing.txt`
  - `thing/dissolution.txt`
- lineSpans:
  - `thing/thing-idea.txt:1-209`
  - `thing/thing.txt:211-642`
  - `thing/dissolution.txt:2-111`
- summary: The `thing/` subfolder gives appearance first as concrete existent thinghood, then shows that the thing's truth lies in its dissolution into appearance proper.

Key points: (KeyPoint)

- k1. Concrete existence first determines itself as thing.
- k2. The thing externalizes itself in properties and reciprocal action.
- k3. The thing dissolves and gives way to appearance.

Claims: (Claim)

- c1. id: hegel-appearance-thing-folder-c1
  - subject: thing_subfolder
  - predicate: captures
  - object: immediate_appearance_as_thinghood
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `thing/thing.txt:211-238`
    - `thing/thing-idea.txt:1-209`

- c2. id: hegel-appearance-thing-folder-c2
  - subject: thing_subfolder
  - predicate: turns_toward
  - object: appearance_proper
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `thing/dissolution.txt:97-111`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-world-folder
  - targetWorkbook: `APPEARANCE-WORKBOOK.md`
  - note: the dissolution of the thing opens the world of appearance.
  - sourceClaimIds: [`hegel-appearance-thing-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-world-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-appearance-thing
  - targetWorkbook: `THING-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Thing workbook.
  - sourceClaimIds: [`hegel-appearance-thing-folder-c1`, `hegel-appearance-thing-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-appearance-thing-c1`, `hegel-appearance-thing-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the container file from duplicating the internal chapter movement.

### Entry hegel-appearance-world-folder — `Appearance`: the `world/` field of law and inversion

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
- summary: The `world/` subfolder develops appearance as law, then as the opposition of world of appearance and world-in-itself, and finally dissolves that opposition into essential relation.

Key points: (KeyPoint)

- k1. Law is the essential appearance.
- k2. Appearance divides into opposed worlds.
- k3. The opposition of worlds dissolves into relation.

Claims: (Claim)

- c1. id: hegel-appearance-world-folder-c1
  - subject: world_subfolder
  - predicate: captures
  - object: lawful_and_opposed_world_appearance
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `world/law.txt:2-327`
    - `world/world.txt:2-258`

- c2. id: hegel-appearance-world-folder-c2
  - subject: world_subfolder
  - predicate: turns_toward
  - object: essential_relation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `world/disappearance.txt:71-123`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-relation-folder
  - targetWorkbook: `APPEARANCE-WORKBOOK.md`
  - note: the dissolution of the two worlds gives essential relation.
  - sourceClaimIds: [`hegel-appearance-world-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-relation-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-appearance-world
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the World workbook.
  - sourceClaimIds: [`hegel-appearance-world-folder-c1`, `hegel-appearance-world-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-appearance-world-c1`, `hegel-appearance-world-c2`]

Review outcome:

- review_pending
- notes: this entry preserves the world-field as a single movement rather than as two detached oppositions.

### Entry hegel-appearance-relation-folder — `Appearance`: the `relation/` field of consummation

- sourceFiles:
  - `relation/relation-idea.txt`
  - `relation/whole-parts.txt`
  - `relation/force-expression.txt`
  - `relation/outer-inner.txt`
- lineSpans:
  - `relation/relation-idea.txt:1-111`
  - `relation/whole-parts.txt:2-262`
  - `relation/force-expression.txt:2-369`
  - `relation/outer-inner.txt:2-231`
- summary: The `relation/` subfolder is the truth of appearance, unfolding through whole and parts, force and expression, and outer and inner until appearance becomes actuality.

Key points: (KeyPoint)

- k1. Essential relation is the truth of appearance.
- k2. Whole and parts, force and expression, and outer and inner are one chapter movement.
- k3. The relation field is the threshold to actuality.

Claims: (Claim)

- c1. id: hegel-appearance-relation-folder-c1
  - subject: relation_subfolder
  - predicate: captures
  - object: truth_of_appearance_as_essential_relation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `relation/relation-idea.txt:1-111`
    - `relation/whole-parts.txt:2-262`

- c2. id: hegel-appearance-relation-folder-c2
  - subject: relation_subfolder
  - predicate: culminates_in
  - object: actuality_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `relation/outer-inner.txt:206-231`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-appearance-relation
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Relation workbook.
  - sourceClaimIds: [`hegel-appearance-relation-folder-c1`, `hegel-appearance-relation-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-appearance-relation-c1`, `hegel-appearance-relation-c2`, `hegel-appearance-relation-c3`]

Review outcome:

- review_pending
- notes: this entry fixes the final appearance chapter-cluster as the consummation of the folder.
