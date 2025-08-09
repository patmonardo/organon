import type { PropertyAdapter } from './engine.js';
import type { FormId, PropertyId, PropertySpec, RangeSpec, WorldId } from './types.js';

// Minimal view of FormProperty schema used to derive defaults
// (We avoid importing the full schema to keep this processor self-contained.)
export type FormPropertySchemaLike = {
  id: string;
  name: string;
  contextId: string;
  quantitative?: {
    dataType?: 'string' | 'number' | 'boolean' | 'date' | 'object' | 'array';
    range?: { min?: any; max?: any };
  };
};

export type PropertyCatalogEntry = PropertySpec & { worldId: WorldId };

/**
 * PropertyAdapterRegistry
 * - In-memory registry of PropertySpec per world.
 * - Can derive a basic value range from FormProperty-like schema.
 */
export class PropertyAdapterRegistry implements PropertyAdapter {
  private worldIndex = new Map<WorldId, Map<PropertyId, PropertyCatalogEntry>>();

  add(entry: PropertyCatalogEntry): void {
    const bucket = this.worldIndex.get(entry.worldId) ?? new Map();
    bucket.set(entry.id, entry);
    this.worldIndex.set(entry.worldId, bucket);
  }

  addMany(entries: PropertyCatalogEntry[]): void {
    for (const e of entries) this.add(e);
  }

  getProperty(propId: PropertyId): PropertySpec | undefined {
    for (const bucket of this.worldIndex.values()) {
      const hit = bucket.get(propId);
      if (hit) return toSpec(hit);
    }
    return undefined;
  }

  listByWorld(worldId: WorldId): PropertySpec[] {
    const bucket = this.worldIndex.get(worldId);
    if (!bucket) return [];
    return Array.from(bucket.values()).map(toSpec);
  }

  // Helpers to build catalog entries
  static fromSchema(
    worldId: WorldId,
    props: FormPropertySchemaLike[],
    opts?: {
      domainByProperty?: Record<string, FormId[]>; // propertyId -> formIds
      requiredByProperty?: Record<string, boolean>;
      cardinalityByProperty?: Record<string, { min?: number; max?: number; functional?: boolean }>;
      entityRangeByProperty?: Record<string, FormId[]>; // propertyId -> allowed shapeIds
    }
  ): PropertyCatalogEntry[] {
    return props.map((p) => {
      const valueRange: RangeSpec = p.quantitative?.dataType
        ? { kind: 'value', valueType: p.quantitative.dataType }
        : { kind: 'value', valueType: 'string' };

      const range = (opts?.entityRangeByProperty && opts.entityRangeByProperty[p.id]?.length)
        ? ({ kind: 'entity', shapeIds: opts.entityRangeByProperty[p.id] } as RangeSpec)
        : valueRange;

      const domain = opts?.domainByProperty?.[p.id] ?? [];
      const required = opts?.requiredByProperty?.[p.id] ?? false;
      const cardinality = opts?.cardinalityByProperty?.[p.id];

      const spec: PropertyCatalogEntry = {
        worldId,
        id: p.id,
        name: p.name,
        domain,
        range,
        required,
        cardinality,
      };
      return spec;
    });
  }
}

function toSpec(e: PropertyCatalogEntry): PropertySpec {
  // Strip worldId for the engine view
  const { worldId: _w, ...spec } = e;
  return spec;
}
