# ADR 0007 — Hetu / Ground mapping in the FormProcessor

Status: Proposed
Date: 2025-08-15

Context
-------
The `FormProcessor` (also referred to as `groundStage` in `logic/src/absolute/ground.ts`) derives relations and properties from Morphs acting over Entities and Properties. During the design conversation, the team explored a metaphysical mapping linking Hegelian dialectic and Yogic categories to the system's runtime artifacts.

Decision
--------
Add lightweight, runtime provenance metadata to derived relations and properties that encodes the following mapping:

- Essential relation (particular / Sabija) → runtime relation with `kind: 'essential'` and `provenance.metaphysics` including `{ role: 'particular', faculty: 'ahamkara', intuition: 'inner' }`.
- Absolute relation (universal / Bija) → companion runtime relation with `kind: 'absolute'` that contains the essential relation; `provenance.metaphysics` includes `{ role: 'absolute', faculty: 'buddhi', intuition: 'intellectual' }`.
- Derived properties carry `provenance.metaphysics` with `{ role: 'derivedProperty', faculty: 'ahamkara', intuition: 'inner' }`.

Rationale
---------
- The FormProcessor is responsible for producing world-level structure from rules (Morphs). Encoding metaphysical roles in provenance keeps the ground semantics explicit and traceable without requiring schema changes.
- Using runtime metadata allows downstream systems (projects, controllers, UI) to interpret relations as 'particular' vs 'universal' for presentation or inference without altering persistence contracts.
- This preserves determinism and auditability: provenance includes ruleId and timestamp.

Consequences
------------
- No schema evolution is required; the metadata is optional and placed in `provenance` on derived artifacts.
- Tests should validate presence and basic semantics (idempotence, context propagation).
- Future enhancements may persist these annotations permanently via the triad commit step or expose query helpers.

Implementation
--------------
- See `logic/src/absolute/ground.ts` for the current implementation. The function `commitGroundResults` persists derived relations/properties.
- Example provenance fields:

```json
{
  "provenance": {
    "ruleId": "m:copula-A-to-R",
    "source": "ground",
    "timestamp": "2025-08-15T...Z",
    "metaphysics": { "role": "absolute", "faculty": "buddhi", "intuition": "intellectual" }
  }
}
```

Notes
-----
- This ADR captures a design decision that may be controversial; keep it reversible. If team governance prefers not to encode metaphysical language in runtime docs, we can replace `metaphysics` with a neutral `classification` object.

