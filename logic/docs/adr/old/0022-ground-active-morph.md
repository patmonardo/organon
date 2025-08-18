# ADR 0022 — Ground as ActiveMorph (Morph Driver)

Status: Accepted
Date: 2025-08-17

Context
- The engine needs a place to interpret Morph rules over the current working graph (Entities + Properties) and derive new Relations and Properties until a fixpoint.
- Earlier work wired Reflection (ActiveContext) to compute spectra and Kriya to orchestrate stages. Ground is positioned to be the Morph driver.

Decision
- `absolute/essence/ground.ts` is the ActiveMorph driver. It:
  - Interprets `Morph.ruleSpec` against a working copy of the graph.
  - Derives Essentials (particular relations) and their Absolute containers with stable, reproducible identifiers.
  - Derives Properties as effects of rules.
  - Runs to a bounded fixpoint (configurable) with idempotence guards.
  - Attaches structured provenance and optional advisory annotations (e.g., Reflection spectrum) without changing schema.
- Ground delegates Relation invariants and helpers to `absolute/essence/relation.ts` (ActiveRelation helpers).

Rationale
- Separates concerns cleanly: Reflect = Context; Ground = Morph; World = Aggregation/Truth; Kriya = Orchestration.
- Keeps schema authoritative while allowing rich engine metadata for explanation.
- Fixpoint evaluation models propagation succinctly and predictably.

Details (contract)
- Input: `{ entities: Entity[]; properties: Property[] }`, `principles: { morphs: Morph[] }`, `opts?: { fixpointMaxIters?: number, reflectResult?: any }`.
- Output: `{ relations: Relation[]; properties: Property[] }` where relations include pairs: an essential and its absolute container, linked by `particularityOf`.
- Determinism: rule id + endpoints produce stable relation ids; idempotence prevents duplicates.
- Provenance: relations/properties carry `{ ruleId, source: 'ground', timestamp, contextId?, contextVersion?, metaphysics }`.

Consequences
- Tests and processors can rely on `groundStage` as the single Morph driver.
- `relation.ts` becomes the shared place for invariants like “every essential points to an absolute container” and utilities like `findAbsoluteFor`.

References
- Implementation: `logic/src/absolute/essence/ground.ts`
- Relation helpers: `logic/src/absolute/essence/relation.ts`
- Prior ADRs: 0020 Reflect (ActiveContext), 0021 World (ActiveWorld)
