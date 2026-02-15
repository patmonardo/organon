# GDSL examples (pure operations)

This folder is for **pure GDSL language operations** and avoids coupling to SDSL model/feature design definitions.

## Boundary doctrine

- **GDSL**: active ontology/program operations (Shape-Context-Morph + selected ApplicationForms).
- **SDSL**: design/specification input to compilers.
- **Dataset compiler**: may read SDSL as specification, but persisted graph/data snapshots must come from dataset projections, not model-feature definitions.

## Current intent

1. Author Program-level GDSL payloads directly.
2. Send `form_eval.evaluate` through TSJSON.
3. Keep dataset payload (`graph_store.put`) separate from GDSL program payload.
4. Use this path to define canonical transport for complete Shape-Context-Morph and Entity-Property-Aspect semantics.

## Starter example

Run:

- `npx tsx gdsl-form-program.ts`

The starter demonstrates a pure GDSL-oriented Form Program call with a separate dataset snapshot.
