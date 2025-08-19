# 0009 — Thing as Vastu: Reflective Consciousness and the Two Truths Interface (Platonic)

- Status: Proposed
- Deciders: Logic/Form maintainers
- Date: 2025-08-15
- Related: 0001-shape-as-principle-entity-as-essence, 0002-container-contained, 0003-absolute-relation-and-backprop, 0008-outer-inner-actuality-modality
- Background notes: docs/concepts/thingness.md, docs/concepts/concrete-existence.md, src/essence/thing/sources/thing.txt

## Context

ML “ground truth” often means two different things at once. This resonates with the Vedic/Buddhist “Two Truths” (relative vs absolute). In our engine we already map the Truth of Ground as:

- Relative → Essential Relation (appearance/mediated determinations)
- Absolute → Syllogism/Judgment (necessary form, the “because”) 

Those are synthesis-level moves. Here we step back to the thesis: Becoming → Reflective Consciousness (Citta). In Hegel’s terms: reflection has three axes (positing, external, determining). In Vedic terms: the Thing (Vastu) is a unity that endures across the “three times” and appears through Prakriti’s play (Gunas) without requiring us to adopt a full Abhidharma atomics. We keep the design Platonic: stable identities, determinate properties, and relations governed by forms.

Operationally we need a “Thing” design in the Form Processor that:

- Presents both surfaces of truth concurrently (relative/absolute) without forked schemas.
- Respects existing repository invariants and avoids schema churn unless optional.
- Provides a reflective contract (positing/external/determining) that engines can compute from observed data.
- Accounts for persistence across time slices (“three times”) via Context/World rather than hardcoding time into the Thing.

## Decision

We introduce a Platonic “Thing-as-Vastu” interface across the schema, form, and absolute layers that keeps today’s types intact and adds minimal, optional affordances.

1) Keep system.Thing as the canonical carrier

- Continue to use `system.Thing` for objects under consideration. Schema and tests already exist (`src/schema/thing.ts`, `test/schema/thing.test.ts`).
- No breaking schema change. Any extensions below are optional and can be emitted in provenance/metadata rather than the core model.

2) Two Truths as dual surfaces, not dual types

- Relative Truth surface (Appearance): provided by Essential Relations and Properties under a Context. This is already our engine default.
- Absolute Truth surface (Reason): provided by Judgments and Syllogisms derived about the Thing. We surface this via existing judgment/syllogism machinery in the Absolute layer.
- A Thing therefore projects to two complementary views at runtime: appearance-view and reason-view. We do not duplicate storage; views are computed.

3) Reflective Consciousness as a computed triple

- Add a small absolute-layer helper (reflectStage, optional): compute the reflection triple for a Thing from current evidence.
  - positing-reflection: self-determinations that arise from the Thing’s own essence/shape and stable properties (schema-level content and intrinsic qualities).
  - external-reflection: determinations that only appear in relation-to-others (contextual properties, contingent relations, measurements).
  - determining-reflection: the consolidation that returns through mediation (invariants, conserved predicates, law-like constraints; candidates backed by judgments/syllogisms).
- Store the reflection result in transient processor metadata alongside provenance (not in the persisted Thing), and optionally publish it on the bus for interested processors.

4) Vastu temporality via Context/World, not embedded timestamps

- Endurance across “three times” is represented by: 
  - stable Thing identity (`id`, `type`),
  - revisions under different Contexts/World horizons (past/present/future as domain contexts), and
  - optional projection windows in World processors.
- We avoid adding time fields to the Thing. Time belongs to Context/World and provenance of derived facts.

5) Vastu signature (stable unity) for idempotence and tracking

- Compute a content-addressed “vastuSignature” for a Thing from its essence handle + essential properties + role in essential relations (order-invariant hash). Use it to:
  - detect sameness across contexts/time-slices,
  - drive idempotent updates in commit paths,
  - support “eternal atom” talk operationally without committing to atomist ontology.
- Emit the signature in processor metadata/provenance; do not persist it in the schema initially.

6) Optional “gunas” gauge (Platonic wrapper)

- We refrain from encoding Sattva/Rajas/Tamas as first-class fields. As an optional analysis, processors may compute a qualitative triple from observed dynamics (stability, activity, inertia) and attach it to the reflective metadata for research/UX. This stays out of core invariants.

## Consequences

- No breaking schema change; existing Thing/Entity/Relation pipelines continue to run.
- The Form Processor gains a uniform place to report the reflective triple and the two truth surfaces per Thing.
- We can reason about “Thing across three times” through Context/World and provenance instead of embedding time into the Thing.
- Idempotence improves via `vastuSignature` without constraining authors to a specific ontology.

## Out of scope (deferred)

- A full Abhidharma atomics or Yogācāra semantics for citta-vṛtti. We keep the minimal Platonic stance.
- Persisting reflective triples or gunas in the schema; we begin with transient metadata and events.
- Any schema-level cardinal changes to Thing. If later needed, we can add an optional `temporality` or `identity.signature` field through normal RFC.

## Alternatives considered

1) Make two schema types: RelativeThing and AbsoluteThing

- Rejected: duplicates data, complicates repositories, and violates the “Everything is a Form” unification principle.

2) Embed time directly in Thing

- Rejected: time is contextual and better handled by Context/World and provenance.

3) Add first-class Gunas to schema

- Rejected for now: speculative and domain-specific; keep as an analysis view in processor metadata.

## Implementation sketch

Minimal, additive changes in logic package:

- absolute/thing/reflect.ts (new):
  - deriveReflectiveFacets(thing, world, ctx): { positing, external, determining, evidence }
  - computeVastuSignature(thing, essentials): string
  - publish optional `knowledge.thing.reflected` with the triple and signature

- kriya/orchestrator.ts:
  - Optionally run reflectStage before groundStage; attach results to the run trace (no persistence).
  - Keep truth‑actualization and knowledge‑delta as-is; the reflective triple is complementary.

- docs/concepts: add a short note “reflective-consciousness.md” mapping positing/external/determining to engine signals.

Tests (happy path + 1–2 edges):

- thing.reflect.test.ts — computes triple on a simple world; positing ≥ external when no relations; determining rises after syllogism is derived.
- reflect.signature.test.ts — signature stable across context copies; changes when an essential property changes.

## Migration and compatibility

- No migration needed. Existing data remains valid. Processors that don’t care can ignore reflective metadata.

## Open questions

- Should determining‑reflection directly influence modality actualization, or remain descriptive? For now it remains descriptive; ADR 0008 governs Actuality/Modality.
- Where to surface the two views in API responses? Proposal: return `views: { appearance, reason }` in Kriya results without changing stored schema.

## Acceptance criteria

- Engines can obtain per‑Thing reflective triples and a stable signature without schema changes.
- Two Truths are concurrently available as computed views for each Thing.
- Tests demonstrate stability across contexts and responsiveness to essential changes.
