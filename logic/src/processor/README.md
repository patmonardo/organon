# Logical Form Processor

A minimal, composable kernel that implements the BEC “Immediate-with-its-Mediacy” principle for Forms:
- Form:Entity = Thing (Immediate) that contains its World:Context (Mediacy).
- Context:Property = World (middleware of possibilities; OWL/SHACL-like at schema level).
- Morphism:Relation = TransForm (mediate immediacy; a typed transformation over Things) — to be added next.

This module operationalizes Being→Essence→Concept for the “Form” stratum and provides a tiny API to build worlds, forms, and entities, then assert and validate fields against intensional property constraints.

## Concepts and mapping

- World (Context:Property)
  - Middleware platform of properties available in a context.
  - Holds intensional property specs (OWL Property/SHACL-like constraints).
- Form (Being-in-this-World)
  - A closure over a World. “Immediate that contains its mediacy.”
  - open = all properties from World are allowed; closed = only enumerated properties are allowed.
- Entity (Thing/Appearance)
  - A concrete instance of a Form within a World; its fields are property assertions constrained by World and allowed by Form.
- Property (Intension)
  - Domain/range/cardinality/required — modeled via a simple adapter view (PropertySpec).

Philosophical alignment
- Kant/Hegel: Property as determination of the Thing lives concretely as the set of FormFields on an Entity; the schema-level FormProperty is intensional, not an instance.
- OWL/SHACL: FormProperty ≈ OWL Object/DataProperty + SHACL-like constraints; Form ≈ Class/Shape; Entity ≈ Individual; Field ≈ Property Assertion.

## Invariants (enforced by the engine)
1) Domain: Property must apply to the entity’s Form (domain includes the Form or is empty for global).
2) Range: Data values match declared value type; object links target entities (shape checks can be added).
3) Cardinality: min/max and functional (max=1) enforced on assignment and validate().
4) Required: Required properties (by World and applicable domain) must be present for a valid Entity.
5) Shape closure: Closed forms allow only enumerated properties; open forms allow any World property.

## Files
- types.ts — BEC-aligned types (WorldId/FormId/EntityId/PropertyId, RangeSpec, Cardinality, PropertySpec).
- principle.ts — Principle<I, M> = Immediate containing Mediacy.
- form.ts — World/Form/Entity structures and helpers; FormPrinciple ties Form (Immediate) with World (Mediacy).
- engine.ts — ProcessorEngine orchestrates World/Form/Entity lifecycle, field assignment, and validation.
- index.ts — Barrel exports for ergonomic imports.

## PropertyAdapter contract
The engine is agnostic to your existing property implementation; provide a thin adapter:

```ts
export interface PropertyAdapter {
  getProperty(propId: PropertyId): PropertySpec | undefined;
  listByWorld(worldId: WorldId): PropertySpec[]; // preload for World
}
```

PropertySpec is an adapter view; map your concrete FormProperty/Service/Engine types into:

```ts
export interface PropertySpec {
  id: PropertyId;
  name: string;
  domain: FormId[];            // forms it applies to (empty ⇒ global)
  range: { kind: 'value'; valueType: string } | { kind: 'entity'; shapeIds: FormId[] };
  cardinality?: { min?: number; max?: number; functional?: boolean };
  required?: boolean;
}
```

## API sketch

- World.create(id, name, properties?) → World
- Form.create(id, name, world, { open?, allowed? }) → FormPrinciple
- Entity.create(id, formPrinciple) → Entity
- ProcessorEngine
  - registerWorld(worldId, name) → World
  - createForm(formId, name, worldId, opts?) → FormPrinciple
  - createEntity(entityId, formId) → Entity
  - setField(entityId, propertyId, value) → void
  - validate(entityId) → { ok: boolean; errors: string[] }

## Usage example

```ts
import { ProcessorEngine } from './engine';
import type { PropertyAdapter, PropertySpec } from './engine';

// Example adapter (replace with logic/src/form/property service wiring)
const adapter: PropertyAdapter = {
  getProperty: (id) => props.get(id),
  listByWorld: (worldId) => Array.from(props.values()).filter(p => p.domain.length === 0 || p.domain.includes('Form:Any')),
};

const props = new Map<string, PropertySpec>([
  ['p:name', { id: 'p:name', name: 'name', domain: ['Form:Person'], range: { kind: 'value', valueType: 'string' }, cardinality: { functional: true }, required: true }],
  ['p:friend', { id: 'p:friend', name: 'friend', domain: ['Form:Person'], range: { kind: 'entity', shapeIds: ['Form:Person'] }, cardinality: { max: 5 } }],
]);

const engine = new ProcessorEngine(adapter);
const world = engine.registerWorld('World:default', 'Default');
const person = engine.createForm('Form:Person', 'Person', world.id, { open: false, allowed: new Set(['p:name', 'p:friend']) });
const e1 = engine.createEntity('E:1', person.immediate.id);

engine.setField(e1.id, 'p:name', 'Alice');
engine.setField(e1.id, 'p:friend', { entityId: 'E:2' });

const result = engine.validate(e1.id);
// result.ok or result.errors
```

## Design notes
- Open vs Closed Forms: Start closed for authoring precision; open for exploratory ingestion. The Form.allows() helper centralizes the policy.
- Validation layering: Keep engine checks light; delegate deeper type/shape validation to the property module/engines.
- Identity: Stable ids for World/Form/Property; Entity ids free-form (UUIDs, IRIs, etc.).
- Persistence: This layer is storage-agnostic. A Cypher/graph persistence adapter can be layered beneath without changing the API.

## What’s next
- TransForm (Morphism:Relation): typed transformations Entity(A) → Entity(B) with world-constrained pre/post conditions.
- Range refinement: Validate object-range links against target entities’ Forms.
- Property hierarchies: subPropertyOf and inherited constraints.
- World policies: closed shapes by default; closure exceptions; cross-field constraints.
- Persistence port: pluggable repository to Neo4j or other property graph.

---

This processor is intentionally small so the philosophical structure remains visible in code. Extend via adapters rather than inflating core types.
