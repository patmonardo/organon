# ADR 0013 — Actuality as Reciprocal Name/Form Eval

Status: Proposed
Date: 2026-02-20

## Context

- Current workbook process is intentionally staged: source-grounded review first (Summary/KeyPoints/Claims), then deeper inferential passes.
- We need a precise decision for the planned concluding engine (`Actuality`) without prematurely collapsing ongoing `Appearance` review.
- Existing architecture already treats form processing as lawful mediation rather than external, table-lookup inference.
- The required precision is a bidirectional criterion: Name (principle/intelligibility) and Form (appearance/articulation) must reciprocally determine one another.

## Problem

Without a strict reciprocity contract, `Eval` can degrade into one-sided matching:

1. Name-only validation (principle asserted without demonstrable form articulation), or
2. Form-only matching (surface correspondence without principled reconstruction), or
3. External if-then interpretation detached from immanent transitions.

Any of these breaks the intended meaning of `Actuality` as concluding closure.

## Decision

Define `Actuality` as a seventh concluding engine whose decisive operation is `Eval` as **reciprocal determination** between `Name` and `Form`.

### 1) Core criterion (normative)

`Eval` passes only when both directions hold under one protocol:

- `Name -> Form`: Name generates/necessitates the accepted Form articulation.
- `Form -> Name`: Form reconstructs the same Name-principle without essential semantic loss.

`Actuality` is achieved only when reciprocal closure is satisfied.

### 2) Reciprocity invariants (normative)

- **Non-loss**: no essential Name constraint is absent in Form.
- **Non-invention**: no essential Form determination is ungrounded in Name.
- **Polarity preservation**: negation/sublation direction remains consistent across both directions.
- **Transition necessity**: Form transitions are implied by Name protocol, not appended externally.
- **Meaning identity**: the evaluated semantic content is the same in both directions (no equivocation).

### 3) Operational protocol (normative)

1. Normalize `Name` into determinate constraints (`must`, `may`, `excludes`, `transitions`).
2. Normalize `Form` into determinate articulation (elements, relations, transitions, negations).
3. Run forward check (`Name -> Form`) for generated-vs-observed coherence.
4. Run reverse check (`Form -> Name`) for reconstructed-vs-source coherence.
5. Compute residuals (`missing_from_form`, `unsupported_in_name`, `polarity_conflicts`, `transition_breaks`).
6. Emit `Eval` judgment and only then allow `Actuality` conclusion.

### 4) Output contract (normative)

`EvalResult` must include at minimum:

- `status`: `pass | fail | indeterminate`
- `forwardConsistency`: number (0..1)
- `reverseConsistency`: number (0..1)
- `reciprocityScore`: number (0..1)
- `residuals`: structured mismatch report
- `actualityJudgment`: `achieved | not_achieved | deferred`
- `notes`: concise rationale

## Scope and staging

- This ADR is binding as architecture direction but **staged for implementation**.
- Immediate phase: keep this as review/inference guardrail while `Appearance` review is still in progress.
- Implementation phase begins after current review pass confirms stable Name/Form extraction criteria.

## Consequences

- Positive:
  - Prevents reduction of evaluation to external formalism or lookup behavior.
  - Aligns concluding engine behavior with reciprocal intelligibility.
  - Creates explicit acceptance gates between review, inference, and actuality conclusions.
- Risks:
  - Over-strict reciprocity could mark valid drafts as indeterminate early.
  - Ambiguous normalization rules could create false mismatches.
- Mitigations:
  - Use `indeterminate` as a first-class outcome during staged adoption.
  - Version normalization profiles and keep mismatch reports explicit.

## Adoption checklist

1. Add `EvalResult` schema in logic compiler/eval docs (JSON-first).
2. Define Name/Form normalization profile v1 for review-time use.
3. Add REVIEW handoff rule: only review-cleared claims enter inference/eval.
4. Add initial conformance tests for forward/reverse reciprocity and residual typing.
5. Enable `Actuality` conclusion only when reciprocity gates are satisfied.

## Out of scope

- Final scoring weights for `reciprocityScore`.
- Full implementation of the seventh engine in runtime code.
- Revising current Ground/Appearance source authorities.

## References

- `logic/src/compiler/essence/reflection/ground/sources/GROUND-REVIEW-WORKBOOK.ipynb`
- `logic/src/compiler/essence/reflection/ground/sources/GROUND-PART-A-V2-WORKBOOK.md`
- `logic/src/compiler/essence/reflection/ground/sources/GROUND-PART-B-V2-WORKBOOK.md`
- `logic/src/compiler/essence/reflection/ground/sources/GROUND-PART-C-V2-WORKBOOK.md`
