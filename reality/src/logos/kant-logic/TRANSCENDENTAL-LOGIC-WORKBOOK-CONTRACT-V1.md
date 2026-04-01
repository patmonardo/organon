# Transcendental Logic Workbook Contract (V1)

Status: active
Applies to: Kant Transcendental Logic workbooks in this folder
Priority: workbook markdown over generated projections

## Canonical authority

- Workbook markdown files are the authoritative Knowledge Graph artifacts for the Kant Transcendental Logic pass.
- Distillation markdown files in this folder are working synthesis guides for extraction, but not exclusive authorities.
- Source markdown remains the primary textual authority for operator capture and may be anchored directly wherever it is clearer or more precise.
- Any Cypher, DB, or graph projection is derivative and non-authoritative.

## Stability rules

- Keep workbook format stable across sessions.
- Keep heading order, field names, and claim structure stable unless a migration is explicitly approved.
- If migration is approved, apply it to every affected Transcendental Logic workbook in one pass.

## Human-scale extraction rule (Transcendental Logic)

- Primary unit: one workbook per Transcendental Logic component.
- Current component set: introduction, aesthetic, intro-logic, concepts, deduction-trans-a, deduction-trans-b, principles, principles-system, principles-I, principles-II, principles-III, principles-IV, principles-ground, principles-reflection, intro-ideas, ideas.
- Add a short plain-language reading note near the top so the workbook states what it is doing before the KG fields begin.
- For small modules, prefer 1 marker entry plus 1 analytic entry in the first pass.
- For large modules, expand to as many entries as needed to preserve all important logical operations, architectonic divisions, and major argumentative turns.
- Use source-aligned sectional grouping when one analytic entry would collapse distinct arguments.
- Maximum 2 claims per entry remains preferred, but entry count should increase rather than compressing distinct arguments into one claim.
- Relations should stay sparse and structural rather than exhaustive.

## KG notation discipline

- Use explicit node-label notation in section headers:
  - `Key points: (KeyPoint)`
  - `Claims: (Claim)`
  - `Relations: (Relation)`
- Keep key-point ids stable (`k1`, `k2`, ...) within entry scope.
- Keep claim ids stable (`<entry-id>-c1`, `<entry-id>-c2`, ...).
- Keep relation ids stable (`r1`, `r2`, ...).
- Every accepted claim must include line-anchored evidence using `file:start-end` notation.

## Session discipline

- Each workbook must include a short `Decision:` block describing current pass constraints.
- Session constraints remain binding unless explicitly revised.

## Source-guided distillation protocol

- In this seed phase, distillations should usually guide extraction because they remain easier narrative surfaces than the workbooks.
- `Key points` and `Claims` may be anchored to distillation files, source files, or both.
- Prefer direct source anchoring whenever the raw Kant text states an operator, distinction, or formal rule more precisely than the distillation.
- If distillation and source ever diverge in force or structure, the source text wins.
- Distillation means compression of wording, not omission of substance.
- Distillations must preserve every important logical operation, architectonic division, and formal distinction required to reconstruct the Kantian argument.
- If a source module is too large for one short distillation, split it into additional sections rather than collapsing operations into overview language.
- Distillations may register ontological or architectonic stakes in plain language, but workbook claims must remain conservative and source-trackable.
- Do not silently substitute later Hegelian vocabulary for Kantian arguments inside workbook claims.
- Stronger synthetic overlays belong to a later cycle after the Transcendental Logic workbook layer is stable.

## Review protocol

- If uncertain, mark `review_pending`.
- Prefer stable first-order claims over ambitious interpretive compression.
- Expand relations only after the macro entry layer is accepted.
