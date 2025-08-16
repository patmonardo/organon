Purpose

This file documents recommended import patterns for the `logic/src/absolute`
processor layer to keep the boundary between schema and runtime clear.

Guiding principle

- Use schema types only at API boundaries (validation and persistence).
- Inside the processor and derivation stages prefer runtime aliases (ThingLike,
  PropertyLike, Loose*), or local lightweight interfaces to avoid leaking
  schema complexity into engine internals.

Examples

1) API boundary (validation / persistence)

```ts
import { ProcessorInputs } from "../core/contracts"; // schema-level parser
import type { Property } from "../../schema/property"; // use for persistence or strict typing
```

2) Processor internals (reflect / ground / orchestrator)

```ts
import type { ThingLike, PropertyLike } from "../essence/reflect"; // runtime-friendly
import type { KriyaOptions } from "../core/kriya"; // canonical runtime options
// use Loose* or `any` sparingly when integrating with external data
```

3) Persisting derived items

- Use repository triads (relation/property repo) exposed to the orchestrator
  via `opts.triad` and only use schema-level types when calling repo.create/update.

Rationale

- Keeping schema at the borders simplifies upgrades to the schema layer and
  prevents type leakage into the processor exports which are meant to be
  engine-focused.

Notes

- This is a living guideline. If you see common patterns that require a new
  runtime alias, add it to `essence/reflect.ts` (for ThingLike/PropertyLike) or
  create `absolute/types.ts` if the aliases become shared across modules.
