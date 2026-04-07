# Quality Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `quality/` folder.
- Read it to follow the folder-level architectonic from `being/`, through `existence/`, into `being-for-itself/`.
- Its task is to preserve the high-level spine of the Quality section within the Sphere of Being.
- Use the readable subchapter files [BEING-WORKBOOK.md](BEING-WORKBOOK.md), [EXISTENCE-WORKBOOK.md](EXISTENCE-WORKBOOK.md), and [BEING-FOR-ITSELF-WORKBOOK.md](BEING-FOR-ITSELF-WORKBOOK.md) for the detailed readable walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the folder-level sequence in which pure immediacy collapses into becoming, becomes determinate existence, and completes itself as being-for-itself.
- Second question: what is the central operator of the folder as a whole?
  Answer: negation becoming increasingly immanent to being.
- Third question: where does the folder lead?
  Answer: into quantity.

## Authority + format lock (must persist)

- Working extraction references: `quality-idea.txt`, `being/being-idea.txt`, `existence/existence-idea.txt`, `being-for-itself/being-for-itself-idea.txt`, and `QUALITY-DISTILLATION.md`
- Upstream source authority: `quality-idea.txt`, `being/being-idea.txt`, `existence/existence-idea.txt`, `being-for-itself/being-for-itself-idea.txt`
- This workbook covers the whole `quality/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not collapse the folder-level container into one of its child chapter-clusters.
- Do not duplicate the chapter-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (quality-folder container pass)

Scope:

- files:
  - `quality-idea.txt`
  - `being/being-idea.txt`
  - `existence/existence-idea.txt`
  - `being-for-itself/being-for-itself-idea.txt`
  - `QUALITY-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `quality/` folder.
- Give each major child chapter its own readable artifact pair at the folder root.
- Keep the emphasis on the folder sequence: `being/`, `existence/`, `being-for-itself/`.
- Preserve the older local compiler, idea, part, and notebook stacks as denser legacy support rather than replacing them.

### Entry hegel-being-quality-folder — Marker `Quality`

- sourceFiles:
  - `quality-idea.txt`
  - `being/being-idea.txt`
  - `existence/existence-idea.txt`
  - `being-for-itself/being-for-itself-idea.txt`
- lineSpans:
  - `quality-idea.txt:1-24`
  - `being/being-idea.txt:1-4`
  - `existence/existence-idea.txt:1-53`
  - `being-for-itself/being-for-itself-idea.txt:1-47`
- summary: The `quality/` folder unfolds as one high-level movement from pure being, through determinate existence and infinity, into being-for-itself as the completed form of quality.

Key points: (KeyPoint)

- k1. Quality is the first sphere of determinateness.
- k2. The first field is being.
- k3. The middle field is existence.
- k4. The final field is being-for-itself.
- k5. Quality culminates in the transition to quantity.

Claims: (Claim)

- c1. id: hegel-being-quality-folder-c1
  - subject: quality_folder
  - predicate: unfolds_through
  - object: being_existence_and_being_for_itself
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quality-idea.txt:1-24`

- c2. id: hegel-being-quality-folder-c2
  - subject: quality_folder
  - predicate: culminates_in
  - object: quantity_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `being-for-itself/being-for-itself-idea.txt:41-47`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-being-quality-being-folder
  - targetWorkbook: `QUALITY-WORKBOOK.md`
  - note: quality begins with pure being and becoming.
  - sourceClaimIds: [`hegel-being-quality-folder-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-being-quality-being-folder-c1`, `hegel-being-quality-being-folder-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-being-quality-existence-folder
  - targetWorkbook: `QUALITY-WORKBOOK.md`
  - note: quality then develops as determinate existence and infinity.
  - sourceClaimIds: [`hegel-being-quality-folder-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-being-quality-existence-folder-c1`, `hegel-being-quality-existence-folder-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-being-quality-bfs-folder
  - targetWorkbook: `QUALITY-WORKBOOK.md`
  - note: quality completes itself as being-for-itself and passes into quantity.
  - sourceClaimIds: [`hegel-being-quality-folder-c1`, `hegel-being-quality-folder-c2`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`hegel-being-quality-bfs-folder-c1`, `hegel-being-quality-bfs-folder-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the top-level `quality/` surface as a true container artifact.

### Entry hegel-being-quality-being-folder — `Quality`: the `being/` opening field

- sourceFiles:
  - `being/being-idea.txt`
  - `being/being.txt`
  - `being/nothing.txt`
  - `being/becoming.txt`
- lineSpans:
  - `being/being-idea.txt:1-4`
  - `being/being.txt:1-19`
  - `being/nothing.txt:1-21`
  - `being/becoming.txt:1-110`
- summary: The `being/` subchapter gives pure immediacy as being, shows it identical with nothing, and establishes becoming as the truth that passes into existence.

Key points: (KeyPoint)

- k1. Pure being is empty immediacy.
- k2. Nothing is the same emptiness.
- k3. Becoming is their truth.
- k4. Becoming turns into existence.

Claims: (Claim)

- c1. id: hegel-being-quality-being-folder-c1
  - subject: being_subchapter
  - predicate: captures
  - object: immediacy_as_being_nothing_and_becoming
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/being.txt:1-19`
    - `being/nothing.txt:1-21`
    - `being/becoming.txt:1-110`

- c2. id: hegel-being-quality-being-folder-c2
  - subject: being_subchapter
  - predicate: turns_toward
  - object: existence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/becoming.txt:100-110`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-being-quality-existence-folder
  - targetWorkbook: `QUALITY-WORKBOOK.md`
  - note: the quiescent result of becoming is existence.
  - sourceClaimIds: [`hegel-being-quality-being-folder-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-being-quality-existence-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-quality-being
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: the detailed readable surface for this subchapter is the Being workbook.
  - sourceClaimIds: [`hegel-being-quality-being-folder-c1`, `hegel-being-quality-being-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-quality-being-c1`, `hegel-quality-being-c2`, `hegel-quality-being-c3`]

Review outcome:

- review_pending
- notes: this entry keeps the first quality field at chapter scale.

### Entry hegel-being-quality-existence-folder — `Quality`: the `existence/` field of finitude and infinity

- sourceFiles:
  - `existence/existence-idea.txt`
  - `existence/constitution.txt`
  - `existence/alternating-infinity.txt`
  - `existence/affirmative-infinity.txt`
- lineSpans:
  - `existence/existence-idea.txt:1-53`
  - `existence/constitution.txt:1-410`
  - `existence/alternating-infinity.txt:1-288`
  - `existence/affirmative-infinity.txt:1-427`
- summary: The `existence/` subchapter gives determinate being as something, constitution, finitude, and infinity, and turns the true infinite into being-for-itself.

Key points: (KeyPoint)

- k1. Existence is determinate being.
- k2. Quality becomes finitude and alterability.
- k3. The bad infinite is overcome in the true infinite.
- k4. True infinity turns into being-for-itself.

Claims: (Claim)

- c1. id: hegel-being-quality-existence-folder-c1
  - subject: existence_subchapter
  - predicate: captures
  - object: quality_as_determinate_existence_finitude_and_infinity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/existence-idea.txt:1-27`
    - `existence/constitution.txt:1-410`
    - `existence/alternating-infinity.txt:1-288`

- c2. id: hegel-being-quality-existence-folder-c2
  - subject: existence_subchapter
  - predicate: turns_toward
  - object: being_for_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/existence-idea.txt:28-53`
    - `existence/affirmative-infinity.txt:1-427`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-being-quality-bfs-folder
  - targetWorkbook: `QUALITY-WORKBOOK.md`
  - note: oppositionless infinity becomes being-for-itself.
  - sourceClaimIds: [`hegel-being-quality-existence-folder-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-being-quality-bfs-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-quality-existence
  - targetWorkbook: `EXISTENCE-WORKBOOK.md`
  - note: the detailed readable surface for this subchapter is the Existence workbook.
  - sourceClaimIds: [`hegel-being-quality-existence-folder-c1`, `hegel-being-quality-existence-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-quality-existence-c1`, `hegel-quality-existence-c2`, `hegel-quality-existence-c3`]

Review outcome:

- review_pending
- notes: this entry keeps Existence at chapter scale over its larger internal stack.

### Entry hegel-being-quality-bfs-folder — `Quality`: the `being-for-itself/` field of completed quality

- sourceFiles:
  - `being-for-itself/being-for-itself-idea.txt`
  - `being-for-itself/being-for-itself.txt`
  - `being-for-itself/one-many.txt`
  - `being-for-itself/attraction.txt`
- lineSpans:
  - `being-for-itself/being-for-itself-idea.txt:1-47`
  - `being-for-itself/being-for-itself.txt:1-230`
  - `being-for-itself/one-many.txt:1-247`
  - `being-for-itself/attraction.txt:1-392`
- summary: The `being-for-itself/` subchapter gives quality as the one, the many, repulsion, and attraction, and completes the section by passing into quantity.

Key points: (KeyPoint)

- k1. Being-for-itself is self-related qualitative negation.
- k2. The one unfolds into many ones.
- k3. Repulsion and attraction mediate one another.
- k4. Their equilibrium turns quality into quantity.

Claims: (Claim)

- c1. id: hegel-being-quality-bfs-folder-c1
  - subject: being_for_itself_subchapter
  - predicate: captures
  - object: completed_quality_as_one_many_repulsion_and_attraction
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being-for-itself/being-for-itself-idea.txt:1-47`
    - `being-for-itself/being-for-itself.txt:1-230`
    - `being-for-itself/one-many.txt:1-247`

- c2. id: hegel-being-quality-bfs-folder-c2
  - subject: being_for_itself_subchapter
  - predicate: culminates_in
  - object: quantity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being-for-itself/being-for-itself-idea.txt:41-47`
    - `being-for-itself/attraction.txt:1-392`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-quality-bfs
  - targetWorkbook: `BEING-FOR-ITSELF-WORKBOOK.md`
  - note: the detailed readable surface for this subchapter is the Being-for-itself workbook.
  - sourceClaimIds: [`hegel-being-quality-bfs-folder-c1`, `hegel-being-quality-bfs-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-quality-bfs-c1`, `hegel-quality-bfs-c2`, `hegel-quality-bfs-c3`]

Review outcome:

- review_pending
- notes: this entry fixes the quality-to-quantity handoff at the chapter-cluster scale.
