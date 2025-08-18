Triad Protocol — Minimal contract for control & dataflow
======================================================

This document defines a tight, minimal protocol for the Triad control surface used by the
Form Processor. The goal is to make the runtime dataflow explicit and easy to test, audit,
and wire into external systems (logging, tracing, analytics).

Core ideas
----------
- One canonical data shape: a carrier we call Shape — the runtime JSON object that holds
  core, state, endpoints and metadata for Entities, Properties and Relations.
- One control surface: the Triad — a small surface that mediates Commands -> Engines -> Events
  and provides idempotent commit semantics.

High-level pipeline
-------------------
1. Ground stage (drivers): produce GroundResult (relations + properties) by applying Morphs.
2. Convert to Active carriers: `toActiveFromGround(groundResult)` produces ActiveProperty[] and ActiveRelation[].
3. Engine.process(activeCarriers): validate & map carriers into a list of actions (upsert/delete) and a snapshot.
4. Engine.commit(actions, snapshot): idempotently apply actions to repositories and emit TriadEvents.
5. Triad bus (EventBus): collects events, optionally publishes them to external systems.

Triad command & event contract (minimal)
---------------------------------------
- TriadCommand

  - shape: { kind: string; payload: unknown; meta?: Record<string, unknown> }
  - examples: { kind: 'relation.create', payload: RelationShape }, { kind: 'relation.delete', payload: { id } }

- TriadEvent

  - shape: { kind: string; payload: unknown; meta?: { traceId?: string; spanId?: string; engine?: string; timestamp?: string } }
  - common event kinds: 'relation.created', 'relation.deleted', 'relation.updated', 'relation.described', 'relation.core.set', 'relation.endpoints.set'

Message semantics
-----------------
- Commands are idempotent where possible. Engines must accept an expectedRevision option for optimistic concurrency.
- Events are the canonical record of state changes; they should include trace metadata and an authoritative payload.
- Engines must validate incoming Active carriers with Zod (or similar) and fail fast with deterministic errors.

Example (Relation flow)
-----------------------

1. Ground produces a relation:

```json
{
  "id": "rs-1:thing:1->thing:2",
  "sourceId": "thing:1",
  "targetId": "thing:2",
  "type": "linked_to",
  "kind": "essential",
  "provenance": { "ruleId": "rs-1", "source": "ground" }
}
```

2. Convert to ActiveRelation (schema-validated):

```json
{
  "id": "rs-1:thing:1->thing:2",
  "kind": "essential",
  "particularityOf": "rs-1:thing:1->thing:2:absolute",
  "source": { "id": "thing:1", "type": "system.Thing" },
  "target": { "id": "thing:2", "type": "system.Thing" },
  "type": "linked_to",
  "active": true,
  "provenance": { "ruleId": "rs-1", "source": "ground" }
}
```

3. Engine.process returns actions: e.g. relation.upsert / relation.delete

4. Engine.commit maps actions to TriadCommands (relation.create, relation.setCore, relation.setEndpoints, relation.setDirection, relation.delete) and emits TriadEvents on the bus.

Extension points / Observability
--------------------------------
- Add a small, uniform tracing meta object on events: { traceId, spanId, parentSpanId, engine, timestamp }
- Log both raw carriers and normalized actions — the demo already follows this idea.
- Persist event snapshots for replay/debugging.

Next steps
----------
- Optionally codify this contract as TypeScript interfaces in `src/form/triad/protocol.ts` and export them for engines.
- Add a tiny event serializer for triad events to make them ingestible by an external analytics pipeline.
