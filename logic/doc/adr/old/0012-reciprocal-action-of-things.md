---
adr: 0012
title: Reciprocal Action of Things — Property as Connecting Reference and Action
status: proposed
date: 2025-08-15
authors:
  - patmonardo
reviewers:
  - engineering
---

Context
-------

This ADR translates the passage "c. The reciprocal action of things" into operational proposals for the Form Processor. The passage emphasizes that things are constituted by their properties, that properties are the self-subsistent connecting reference, and that reciprocal determination (the middle term) is the immanent reflection by which things act on one another.

Goals
-----

- Capture "reciprocal action" as a first-class processor concern (Science of Action).
- Model properties as both determinations and causal connectors (property-as-cause).
- Add a non-invasive, provenance-first Action stage that computes reciprocal effects between entities without schema changes.
- Ensure idempotence and traceability via content-addressed signatures and event evidence.

Proposal
--------

1. Property-as-Connecting-Reference

- Treat properties as the primary locus of determinateness and connection: they are the medium through which entities relate and act.
- Do not change canonical schemas; represent this as computed views and events derived from existing `properties` and `relations`.

2. Action Stage (actionStage)

- Introduce an optional but recommended pipeline stage called `actionStage` (aka reciprocal/action/syn-actor) that:
  - Runs after `ground` (which derives relations) and after `synth` when available.
  - Consumes `EssenceGraph` (entities, properties, relations) and `ReflectResult` (if present) and emits a set of `Actions` (structured effects) and provenance evidence.
  - Does not mutate persisted schemas by default; it produces a `KriyaActionResult` object and may emit `knowledge.action` events for downstream actors.

- Action stage contract (minimal):
  - Input: { graph: EssenceGraph, reflect?: ReflectResult, opts?: ActionOptions }
  - Output: { actions: Action[], signatures: Record<string,string>, evidence: string[] }

  - Action shape (suggested):
    - id: string
    - source: { entityId, propertyId }
    - target: { entityId }
    - kind: string (e.g., influence, cause, enable)
    - payload: any (quantitative/qualitative delta)
    - confidence: number
    - provenance: { ruleId?, contextId?, timestamp, signature }

3. Idempotence & Signatures

- Use content-addressed stable hashes for action signatures derived from the source property signature, ruleId, and relevant context. This guarantees idempotence across repeated runs.
- Emit evidence lines like `action:<id>:sig:<hex>` and include them in `ReflectResult`/`KriyaResult` event traces.

4. Events & Knowledge Integration

- The Action stage should optionally emit `knowledge.vritti` or `knowledge.action` events containing the action, evidence, and signatures. Emission should be opt-in via `opts.event` to keep execution deterministic and testable.

5. Non-breaking & Progressive Adoption

- Implementation should be computed-only by default (no persistence). Persistence (materializing actions as relations or commitments) should be an explicit commit step, guarded by ADR-level policy.
- Keep the Action stage implementation separate and small; integrate it into `kriya.runCycle` as a post-ground stage when provided in `StageFns`.

Engineering Contract
-------------------

- Inputs: `Principles`, `EssenceGraph`, optional `ReflectResult`.
- Outputs: `KriyaActionResult` attached to `KriyaResult.action` and optionally emitted events.
- Error modes: transient errors should bubble as failed stage errors; rule misfires should be captured as `evidence` with `confidence: 0` rather than throwing.

Edge cases
----------

- Empty properties or degenerate entities: produce no actions.
- Cycles (A influences B and B influences A): detect cycles and cap transitive propagation via `maxDepth` in `ActionOptions`.
- Conflicting actions: actions carry `confidence` and `ruleId`; downstream aggregator or synthStage resolves conflicts.

Tests & Validation
------------------

- Unit tests for:
  - action signature stability
  - action emission for simple property-driven rules
  - cycle handling and depth capping

Integration
-----------

- Integrate `actionStage` into `StageFns` as optional function. `runCycle` will call it when provided and attach its `KriyaActionResult` to `KriyaResult.action`.
- Document expected inputs and outputs in `logic/docs/concepts/reciprocal-action.md`.

Next steps
----------

1. Add a concept note (done alongside this ADR).
2. Implement a minimal `actionStage` prototype that:
   - Implements a couple of rule heuristics (e.g., property-value thresholds cause `influence` actions).
   - Produces stable signatures and evidence.
   - Is non-persistent by default.
3. Add unit tests and wire optional emission to `knowledge` module.

Rationale
---------

The ADR preserves the metaphysical claim: the property is the locus of action and the ground for thinghood — while keeping the codebase stable and evolvable. It operationalizes "reciprocal action" as computed, traceable, and idempotent actions that can be materialized only when explicitly committed.
