# FormProcessor — seed notes

This folder hosts the FormProcessor: a small, deterministic pipeline that turns Form-level inputs into a World snapshot plus lightweight indexes.

- Name: FormProcessor.
- Scope: Bridge schemas and wrappers under `form/` and `thing/` to produce an assembled `world` and minimal indexes for consumers.

## Contracts

- Inputs (`ProcessorInputs`):
  - `entities`: system.Entity[]
  - `relations`: system.Relation[]
  - `properties`: system.Property[]
  - `contexts`: system.Context[]
  - `morphs`: system.Morph[] (present; derivations to come)
- Output (`ProcessorSnapshot`):
  - `world`: canonical `system.World` document (things + relation edges)
  - `indexes`: minimal helper indexes:
    - `byThing`: { thingId -> { contexts: string[], properties: Record<string, unknown> } }
    - `byContext`: { contextId -> { things: string[] } }
    - `edgesByKind`: { kind -> count }

All contracts are Zod schemas, so inputs are sanitized and defaults applied.

## Pipeline (today)

1) Parse inputs with Zod (types + defaults)
2) Assemble world
   - Entities become Things (by id)
   - Relations become World edges
   - Dedupe edges by (kind, direction, endpoints)
   - Stable ordering for determinism (sort by id/kind)
3) Build minimal indexes
   - Context membership per thing
   - Property bag per thing (last write wins in input order)
   - Edge counts by kind
4) Emit snapshot (world + indexes)

Everything is deterministic given the same inputs.

## Philosophy (short)

- Form is more fundamental than World; World emerges from Matter (things) shaped by Form (relations, properties, contexts).
- The processor is the “7th” that unifies the six pillars (Entity/Context/Property/Morph/Relation/FormShape) into a coherent snapshot suitable for engines and UIs.

## Usage

```ts
import { FormProcessor } from "@organon/logic/src/processor";
import { createEntity, createRelation } from "@organon/logic/src/schema";

const p = new FormProcessor();
const e1 = createEntity({ type: "system.Entity" });
const e2 = createEntity({ type: "system.Entity" });
const r = createRelation({
  type: "system.Relation",
  kind: "related_to",
  source: { id: e1.shape.core.id, type: e1.shape.core.type },
  target: { id: e2.shape.core.id, type: e2.shape.core.type },
});

const snapshot = p.compute({ entities: [e1, e2], relations: [r] });
console.log(snapshot.world.shape.things.length); // 2
```

## Next steps

- Property projection improvements (type-aware, context-scoped bags)
- Morph-derived relations/properties (derive pass)
- Time/version horizons (multi-snapshot, diffs)
- Event-bus integration for incremental updates
- Stronger indexes (byKind -> edge lists, reverse maps)
- Tests: stable ordering and index correctness

---
Node/TS notes: NodeNext is used; import TS without extensions, emitted JS uses `.js`. Keep imports from the published barrels to avoid churn.
