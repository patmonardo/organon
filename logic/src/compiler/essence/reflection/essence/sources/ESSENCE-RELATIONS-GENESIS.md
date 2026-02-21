# Essence Relations Overlay (Draft)

Status: draft
Authority: second-order modeling overlay only (non-authoritative vs workbook claims)
Inputs: `ESSENCE-PART-A-WORKBOOK.md`, `ESSENCE-PART-B-WORKBOOK.md`, `ESSENCE-PART-C--WORKBOOK.md` (for pending inherence anchors)

## Purpose

This file proposes compiler-oriented relation overlays using existing claim IDs only.
It does not alter first-order workbook claims.

## Protocol vocabulary (overlay-only)

- `protocol_mode`: `membership | consequentia | inherence`
- `presupposes_monad`
- `genetic_from`
- `inherence_of_dyad`
- `proof_path`
- `trans_location_of`
- `presupposes_root_location`

## Overlay relations

### or1 — Membership framing in early transition (Part A)

- type: `protocol_mode`
- sourceScope: `ess-ref-a-001 -> ess-ref-a-002`
- value: `membership`
- note: initial comparative split and external partition are modeled as disjunctive division framing.
- proof_path:
  - `ess-ref-a-001-c3`
  - `ess-ref-a-002-c1`
  - `ess-ref-a-002-c2`
- review: `pending`

### or2 — Consequentia turn into non-essence (Part A -> Part B)

- type: `protocol_mode`
- sourceScope: `ess-ref-a-003 -> ess-ref-b-001`
- value: `consequentia`
- note: absolute negativity resolves external split and yields shine as determinate result.
- proof_path:
  - `ess-ref-a-003-c1`
  - `ess-ref-a-003-c3`
  - `ess-ref-b-001-c1`
- review: `pending`

### or3 — Consequentia development in middle movement (Part B)

- type: `protocol_mode`
- sourceScope: `ess-ref-b-002 -> ess-ref-b-004`
- value: `consequentia`
- note: task-grounding develops through identity of determinations into self-sublation and moment-form.
- proof_path:
  - `ess-ref-b-002-c3`
  - `ess-ref-b-003-c2`
  - `ess-ref-b-003-c3`
  - `ess-ref-b-004-c3`
- review: `pending`

### or4 — Genetic dyad generation (Being-side to Essence-side)

- type: `genetic_from`
- sourceDyad: `being_immediacy_split`
- targetDyad: `essence_shine_determinateness_split`
- note: prior dyadic determination is modeled as generating a posterior dyadic structure under preserved presupposition.
- proof_path:
  - `ess-ref-a-001-c3`
  - `ess-ref-a-003-c3`
  - `ess-ref-b-001-c3`
  - `ess-ref-b-003-c3`
- review: `pending`

### or5 — Inherence of prior/posterior dyads

- type: `inherence_of_dyad`
- dyads:
  - `being_nothing_like_split` (modeled anchor)
  - `shine_determinateness_return` (modeled anchor)
- inWhat: `concept_being_divided` (modeled anchor)
- note: both dyads are treated as inhering moments in one conceptual unity.
- proof_path:
  - `ess-ref-b-004-c2`
  - `ess-ref-b-004-c3`
  - `ess-ref-b-005-c3`
- review: `pending`

### or6 — Presupposed root-location hierarchy

- type: `presupposes_root_location`
- sourceLocations:
  - `sphere_being`
  - `sphere_essence`
- rootLocation: `sphere_concept`
- note: transcendental-location hypothesis for compiler navigation; interpretive and second-order.
- proof_path:
  - `ess-ref-a-003-c1`
  - `ess-ref-b-002-c3`
  - `ess-ref-b-005-c3`
- review: `pending`

### or7 — Trans-location assignment for topic-series

- type: `trans_location_of`
- mapping:
  - `ess-ref-a-001..a-003 -> sphere_being_essence_boundary`
  - `ess-ref-b-001..b-005 -> sphere_essence_internal`
- note: topics are modeled as location-bearing nodes in a series/subseries.
- proof_path:
  - `ess-ref-a-001-c2`
  - `ess-ref-a-003-c1`
  - `ess-ref-b-003-c2`
  - `ess-ref-b-005-c3`
- review: `pending`

### or8 — Dyad-wheel append rule (A+B as one appended segment)

- type: `genetic_from`
- sourceScope: `part-a + part-b`
- targetScope: `dyad_wheel_append_segment_1`
- note: Part A and Part B are treated as one dyadic append operation within the wider wheel of dyads.
- proof_path:
  - `ess-ref-a-001-c3`
  - `ess-ref-a-003-c3`
  - `ess-ref-b-003-c3`
  - `ess-ref-b-005-c3`
- review: `pending`

### or9 — Inherence activation zone (Part C target)

- type: `protocol_mode`
- sourceScope: `essence_part_c`
- value: `inherence`
- note: Part C is modeled as the locus where inherence is explicitly active after the A/B triadic unfolding in essence.
- proof_path:
  - `pending_part_c_claim_anchors`
- review: `pending`

### or10 — Scope shift inside triadic protocol

- type: `protocol_mode`
- sourceScope: `sphere_being`
- value: `membership -> consequentia -> inherence (immediacy scope)`
- note: Being-Nothing-Becoming is modeled as full triadic protocol in immediacy.
- proof_path:
  - `pending_being_workbook_claim_anchors`
- review: `pending`

- type: `protocol_mode`
- sourceScope: `sphere_essence`
- value: `consequentia-dominant with inherence escalation toward Part C`
- note: in essence the same triadic protocol is retained but with altered logical scope and ordering emphasis.
- proof_path:
  - `ess-ref-b-002-c3`
  - `ess-ref-b-003-c3`
  - `ess-ref-b-005-c3`
  - `pending_part_c_claim_anchors`
- review: `pending`

## Promotion rule

Promote an overlay relation into workbook `Relations` only when:

1. all referenced claim IDs are stable,
2. the `proof_path` is minimally sufficient,
3. interpretation scope is explicitly marked in relation notes,
4. reviewer flips status from `pending` to `accepted`.

## Open anchors required

- Resolve `pending_being_workbook_claim_anchors` from the Being workbook before compiler export.
- Resolve `pending_part_c_claim_anchors` from Part C workbook entries before promoting `or9` and `or10`.
