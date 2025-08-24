# Services API (Form layer)

Common surface
- Event API: `on(kind, handler)`
- Reads: `get(id)` (repo-first if provided)
- Verbs: `create`, `delete`, `describe`, `setCore`, `setState`, `patchState`

ShapeService
- create({ type, name? })
- Events: `shape.created`, `shape.core.set`, `shape.state.set`, `shape.state.patched`, `shape.described`, `shape.deleted`

EntityService
- create({ type, name? })
- Same event set as Shape (entity.*)

ContextService
- create({ type, name? })
- Events: `context.*`

PropertyService
- create({ type, name?, key, contextId })
- setCore({ name?, type?, key? })
- Events: `property.*` (created payload includes key, contextId)

MorphService
- create({ type, name? })
- Events: `morph.*`

RelationService
- create({ type, name?, kind, source:{id,type}, target:{id,type}, direction? })
- setCore({ name?, type?, kind? })
- setEndpoints({ source:{id,type}, target:{id,type} })
- setDirection('directed'|'bidirectional')
- Events: `relation.*` plus `relation.endpoints.set`, `relation.direction.set`

Persistence
- Repository interface may vary; services prefer delete+create to avoid API mismatches.
- In-memory Map used when repo is absent.

Notes
- All service writes validate with the corresponding Zod schema (e.g., `RelationSchema.parse(...)`).
- `describe(id)` returns a compact DTO (core + state + key endpoints) and emits `*.described`.
