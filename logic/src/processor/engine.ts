import { Entity, Form, FormPrinciple, World } from './form';
import { Cardinality, EntityId, FieldValue, FormId, PropertyId, PropertySpec, RangeSpec, WorldId } from './types.js';

/**
 * PropertyAdapter abstracts the existing logic/src/form/property service/engine.
 * Implement these to bridge into your current property module.
 */
export interface PropertyAdapter {
  getProperty(propId: PropertyId): PropertySpec | undefined;
  listByWorld(worldId: WorldId): PropertySpec[]; // optional optimization
}

export class ProcessorEngine {
  private worlds = new Map<WorldId, World>();
  private forms = new Map<FormId, FormPrinciple>();
  private entities = new Map<EntityId, Entity>();
  private prop: PropertyAdapter;

  constructor(propAdapter: PropertyAdapter) {
    this.prop = propAdapter;
  }

  // World as middleware
  registerWorld(worldId: WorldId, name: string): World {
    const properties = this.prop.listByWorld(worldId);
    const world = World.create(worldId, name, properties);
    this.worlds.set(worldId, world);
    return world;
  }

  // Form as closure over World
  createForm(formId: FormId, name: string, worldId: WorldId, opts?: { open?: boolean; allowed?: Iterable<PropertyId> }): FormPrinciple {
    const world = this.worlds.get(worldId);
    if (!world) throw new Error(`World ${worldId} not found`);
    const form = Form.create(formId, name, world, opts);
    this.forms.set(formId, form);
    return form;
  }

  // Thing (Entity) in this World/Form
  createEntity(entityId: EntityId, formId: FormId): Entity {
    const form = this.forms.get(formId);
    if (!form) throw new Error(`Form ${formId} not found`);
    const entity = Entity.create(entityId, form);
    this.entities.set(entityId, entity);
    return entity;
  }

  // Add or replace a field value (handles data/object ranges)
  setField(entityId: EntityId, propId: PropertyId, value: FieldValue): void {
    const entity = this.entities.get(entityId);
    if (!entity) throw new Error(`Entity ${entityId} not found`);
    const form = this.forms.get(entity.formId);
    if (!form) throw new Error(`Form ${entity.formId} not found`);

    // 1) World membership and Form allowance (Being contains Essence)
    if (!Form.allows(form, propId)) throw new Error(`Property ${propId} not allowed by Form ${form.immediate.name}`);

    // 2) Property semantics (domain/range/cardinality)
    const spec = this.getPropertyOrThrow(propId);
    this.assertDomain(spec, form.immediate.id);
    this.assertRange(spec.range, value);
    // If object range and target entity exists, ensure its form matches allowed shapes
    if (spec.range.kind === 'entity' && typeof value === 'object' && value && 'entityId' in value) {
      const target = this.entities.get((value as { entityId: EntityId }).entityId);
      if (target) {
        const allowed = spec.range.shapeIds;
        if (allowed.length > 0 && !allowed.includes(target.formId)) {
          throw new Error(`Link target form ${target.formId} not allowed for property ${spec.name}`);
        }
      }
    }

    // 3) Cardinality/functional
    const arr = entity.fields.get(propId) ?? [];
    const next = this.applyCardinality(arr, value, spec.cardinality);
    entity.fields.set(propId, next);
  }

  validate(entityId: EntityId): { ok: boolean; errors: string[] } {
    const entity = this.entities.get(entityId);
    if (!entity) throw new Error(`Entity ${entityId} not found`);
    const form = this.forms.get(entity.formId)!;
    const errors: string[] = [];

    // Required properties for this form/world
    const worldProps = this.prop.listByWorld(entity.worldId);
    for (const p of worldProps) {
      if (!this.appliesToForm(p, form.immediate.id)) continue;
      if (!p.required) continue;
      const vals = entity.fields.get(p.id) ?? [];
      if (vals.length === 0) {
        errors.push(`Required property missing: ${p.name}`);
      }
    }

    // Range checks on each field
    for (const [propId, values] of entity.fields) {
      const spec = this.getPropertyOrThrow(propId);
      if (!this.appliesToForm(spec, form.immediate.id)) {
        errors.push(`Property ${propId} does not apply to form ${form.immediate.name}`);
      }
      for (const v of values) {
        try {
          this.assertRange(spec.range, v);
        } catch (e: any) {
          errors.push(`Range failed for ${spec.name}: ${e.message}`);
        }
      }
      // Cardinality integrity
      try {
        this.assertCardinality(values.length, spec.cardinality);
      } catch (e: any) {
        errors.push(`Cardinality failed for ${spec.name}: ${e.message}`);
      }
    }

    return { ok: errors.length === 0, errors };
  }

  // ——— Internal helpers ———
  private getPropertyOrThrow(propId: PropertyId): PropertySpec {
    const spec = this.prop.getProperty(propId);
    if (!spec) throw new Error(`Property ${propId} not found in adapter`);
    return spec;
  }

  private appliesToForm(p: PropertySpec, formId: FormId): boolean {
    return p.domain.length === 0 || p.domain.includes(formId);
  }

  private assertDomain(spec: PropertySpec, formId: FormId) {
    if (!this.appliesToForm(spec, formId)) {
      throw new Error(`Property ${spec.name} does not apply to form ${formId}`);
    }
  }

  private assertRange(range: RangeSpec, value: FieldValue) {
    if (range.kind === 'value') {
      const t = typeof value;
      if (value !== null && t === 'object') throw new Error(`Expected value of type ${range.valueType}, got object`);
      // Leave detailed typing to the property engine; here we just guard obvious mismatch.
    } else {
      // entity range: expect { entityId }
      if (typeof value !== 'object' || value === null || !('entityId' in value)) {
        throw new Error(`Expected link to Entity for object property`);
      }
      // optional: check target entity has allowed shape
      // (requires looking up the entity to read its formId)
    }
  }

  private assertCardinality(count: number, card?: Cardinality) {
    const min = card?.min ?? 0;
    const max = card?.functional ? 1 : (card?.max ?? Number.POSITIVE_INFINITY);
    if (count < min) throw new Error(`min ${min}, have ${count}`);
    if (count > max) throw new Error(`max ${max}, have ${count}`);
  }

  private applyCardinality(existing: FieldValue[], next: FieldValue, card?: Cardinality): FieldValue[] {
    const max = card?.functional ? 1 : (card?.max ?? Number.POSITIVE_INFINITY);
    if (max === 1) return [next];
    const arr = existing.slice();
    arr.push(next);
    if (arr.length > max) throw new Error(`Exceeds max cardinality ${max}`);
    return arr;
  }
}
