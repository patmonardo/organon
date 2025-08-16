# 0010 — Property as Grounded Reflection and Causality (Platonic translation of “b. Property”)

- Status: Proposed
- Deciders: Logic/Form maintainers
- Date: 2025-08-15
- Related: 0001-shape-as-principle-entity-as-essence, 0002-container-contained, 0003-absolute-relation-and-backprop, 0008-outer-inner-actuality-modality, 0009-thing-as-vastu-reflective-consciousness
- Sources: `src/essence/thing/sources/thing.txt` (b. Property), docs/concepts/thingness.md

## Context

We extend the “Two Truths + Reflection” interface from the Thing (ADR 0009) to Property. In the Hegelian text (b. Property):

- Property is the negativity of reflection — mediation that is “reference to itself as to an other.”
- A thing has properties as determinate references to others (external reflection / positedness), yet the property is also the thing’s own self-identical substrate (ground).
- Property maintains itself in transition; through its properties the thing becomes cause (preserving itself as effect).
- Thinghood itself is the ground-connection: property is not merely positedness but ground that has passed into externality and reflected back into itself.
- Determinateness of property is the self-external reflection of the ground; the whole is the ground referring itself to itself.

Design goal: encode these as engine behaviors without breaking existing schemas.

## Decision

Introduce a Property runtime interface and helpers that expose both surfaces of truth and the reflection dynamic, while keeping schema stable.

1) Keep current Property schema; add reflective/grounding views at runtime

- No schema change to `schema/property`. We continue to bind properties to `system.Thing` via existing helpers.
- The processor computes and attaches (transient) metadata per Property instance:
  - positedness: evidence that the property only appears in relation/measurement (external reflection).
  - groundness: evidence that the property is the thing’s own (self-identity in positedness): essential, conserved, or law-backed.
  - determining: consolidation-through-mediation (judgments/syllogisms/invariants supporting the property as necessary or conserved).

2) Two Truths for Property as computed views

- Relative (appearance): property values as they appear under a Context (observations, contingent relations).
- Absolute (reason): property claims supported by judgments/syllogisms or invariants (with confidence/modality per ADR 0008).
- We do not fork types (no “RelativeProperty/AbsoluteProperty”); we compute views over the same schema entity.

3) Property signatures and identity across transitions

- Compute `propertySignature = H(definition ◦ carrierThing.id ◦ essential-qualifiers)` (order-invariant hash).
- Use this signature for idempotent persistence and to assert that “the property maintains itself in transition” across contexts/time.
- Do not persist the signature initially; emit it in provenance/trace.

4) Property as causality/expression

- Through its properties the thing becomes cause: we standardize optional relation types the processor may emit:
  - `expresses` (Thing —expresses→ PropertyValue) for contextual expression.
  - `causes` (Property —causes→ Effect) when a property is the mediating ground of a change, if detectable.
- These are optional emissions in the commit path; projects may enable/disable via Kriya options. No schema changes required (reuse Relation schema/types).

5) Ground-connection aggregation per Thing

- For each Thing, the processor aggregates its essential properties into a `propertyGround` view: the cluster representing the “ground that has passed into externality and returned.”
- This view powers determining‑reflection and can feed modality actualization (still governed by ADR 0008 policies).

6) Qualitative gauge (optional, Platonic)

- Staying pre‑Abhidharma: processors MAY compute a qualitative gauge for properties (stability/activity/inertia) to assist analysis/UX (no schema fields added). This mirrors the optional Gunas gauge in ADR 0009 but remains a view.

## Consequences

- Properties gain clear runtime semantics: when they are merely posited (appearance), when they are grounded (self‑identical substrate), and when they act as causes (maintain themselves as effect).
- Idempotence and cross‑context tracking improve via `propertySignature`.
- No breaking changes; repositories and schema remain stable.

## Implementation sketch

- absolute/property/reflect.ts (new):
  - derivePropertyFacets(thing, property, world, ctx) → { positedness, groundness, determining, evidence }
  - computePropertySignature(property, thing) → string
  - Optionally publish `knowledge.property.reflected` and `knowledge.property.caused` events.

- absolute/kriya/orchestrator.ts:
  - Optionally run property reflectStage alongside thing reflectStage.
  - If enabled, emit `expresses/causes` relations in the commitGroundResults path (
    guarded by options).

- Tests:
  - property.reflect.test.ts — appearance vs groundness behavior; determining increases once a judgment supports the property.
  - property.signature.test.ts — signature stability across contexts; sensitivity to essential change.
  - property.causality.test.ts — expresses/causes emission under a simple rule.

## Open questions

- Causality scope: when to emit `causes` without overclaiming? Initial policy: only when a rule explicitly identifies a property as mediating ground and a downstream effect is observed or derived.
- Should `propertyGround` affect Actuality directly? Keep descriptive for now; ADR 0008 remains the gate for modality changes.

## Acceptance criteria

- For any Thing in a world, processors can expose per‑property: positedness, groundness, determining, signature.
- Two Truths (appearance/reason) are simultaneously available for properties without new schema types.
- Optional expressions/causes relations can be emitted and tested under controlled policies.
