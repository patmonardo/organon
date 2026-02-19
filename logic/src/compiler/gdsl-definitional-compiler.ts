import { z } from 'zod';

import {
  OrganonDefinitionalPackageSchema,
  type OrganonDefinitionalPackage,
} from './gdsl-definitional-forms';
import {
  RootAspectShapeSchema,
  RootEntityShapeSchema,
  RootPropertyShapeSchema,
} from '../../../schema/root-shapes';

const DeferredDefinitionSchema = z.object({
  id: z.string().min(1),
  kind: z.enum(['constraint', 'rule', 'vocabulary', 'provenance']),
  reason: z.string().min(1),
});
export type DeferredDefinition = z.infer<typeof DeferredDefinitionSchema>;

export const GdslCompilerArtifactsSchema = z.object({
  definitionalPackage: OrganonDefinitionalPackageSchema,
  loweredRootLawShapes: z.object({
    entities: z.array(RootEntityShapeSchema),
    properties: z.array(RootPropertyShapeSchema),
    aspects: z.array(RootAspectShapeSchema),
  }),
  rootLawShapes: z.object({
    entities: z.array(RootEntityShapeSchema),
    properties: z.array(RootPropertyShapeSchema),
    aspects: z.array(RootAspectShapeSchema),
  }),
  deferred: z.array(DeferredDefinitionSchema),
  stats: z.object({
    definitions: z.number().int().min(0),
    entities: z.number().int().min(0),
    properties: z.number().int().min(0),
    aspects: z.number().int().min(0),
    deferred: z.number().int().min(0),
  }),
});
export type GdslCompilerArtifacts = z.infer<typeof GdslCompilerArtifactsSchema>;

function entityShapeId(entityId: string): string {
  return `gdsl:entity:${entityId}`;
}

function propertyShapeId(propertyId: string): string {
  return `gdsl:property:${propertyId}`;
}

function mapEntityToRootShape(entity: {
  id: string;
  label: string;
  facets: Record<string, unknown>;
}): z.infer<typeof RootEntityShapeSchema> {
  return RootEntityShapeSchema.parse({
    id: entityShapeId(entity.id),
    type: 'gdsl.root.entity',
    name: entity.label,
    values: {
      definitionId: entity.id,
      facets: entity.facets,
    },
    state: 'grounded',
  });
}

function mapPropertyToRootShape(property: {
  id: string;
  valueKind: 'datatype' | 'object';
  datatype?: string;
  objectTypeId?: string;
  subjectTypeId: string;
  functional?: boolean;
  defaultValue?: unknown;
}): z.infer<typeof RootPropertyShapeSchema> {
  return RootPropertyShapeSchema.parse({
    id: propertyShapeId(property.id),
    type: 'gdsl.root.property',
    entityShapeId: entityShapeId(property.subjectTypeId),
    property: {
      key: property.id,
      valueType:
        property.valueKind === 'datatype'
          ? (property.datatype ?? 'unknown')
          : 'entity-ref',
      value: {
        valueKind: property.valueKind,
        objectTypeId: property.objectTypeId,
        functional: property.functional,
        defaultValue: property.defaultValue,
      },
    },
  });
}

function mapAspectToRootShape(aspect: {
  id: string;
  relationType: string;
  predicate?: string;
  polarity: 'affirmed' | 'negated' | 'mediated';
  subjectTypeId: string;
  objectTypeId?: string;
}): z.infer<typeof RootAspectShapeSchema> {
  return RootAspectShapeSchema.parse({
    id: `gdsl:aspect:${aspect.id}`,
    type: 'gdsl.root.aspect',
    propertyShapeId: propertyShapeId(aspect.id),
    aspect: {
      relationType: aspect.relationType,
      subjectEntityShapeId: entityShapeId(aspect.subjectTypeId),
      objectEntityShapeId: aspect.objectTypeId
        ? entityShapeId(aspect.objectTypeId)
        : undefined,
      predicate: aspect.predicate,
      polarity: aspect.polarity,
    },
  });
}

export function compileGdslDefinitionalPackage(
  packageInput: OrganonDefinitionalPackage,
): GdslCompilerArtifacts {
  const definitionalPackage =
    OrganonDefinitionalPackageSchema.parse(packageInput);

  const entities = definitionalPackage.definitions
    .filter((definition) => definition.kind === 'entity')
    .map((definition) =>
      mapEntityToRootShape({
        id: definition.id,
        label: definition.label,
        facets: definition.facets,
      }),
    );

  const properties = definitionalPackage.definitions
    .filter((definition) => definition.kind === 'property')
    .map((definition) =>
      mapPropertyToRootShape({
        id: definition.id,
        valueKind: definition.valueKind,
        datatype: definition.datatype,
        objectTypeId: definition.objectTypeId,
        subjectTypeId: definition.subjectTypeId,
        functional: definition.functional,
        defaultValue: definition.defaultValue,
      }),
    );

  const aspects = definitionalPackage.definitions
    .filter((definition) => definition.kind === 'aspect')
    .map((definition) =>
      mapAspectToRootShape({
        id: definition.id,
        relationType: definition.relationType,
        predicate: definition.predicate,
        polarity: definition.polarity,
        subjectTypeId: definition.subjectTypeId,
        objectTypeId: definition.objectTypeId,
      }),
    );

  const deferred = definitionalPackage.definitions
    .filter(
      (definition) =>
        definition.kind === 'constraint' ||
        definition.kind === 'rule' ||
        definition.kind === 'vocabulary' ||
        definition.kind === 'provenance',
    )
    .map((definition) =>
      DeferredDefinitionSchema.parse({
        id: definition.id,
        kind: definition.kind,
        reason:
          'Deferred in v0 compiler: not part of root law-shape lowering yet',
      }),
    );

  return GdslCompilerArtifactsSchema.parse({
    definitionalPackage,
    loweredRootLawShapes: {
      entities,
      properties,
      aspects,
    },
    rootLawShapes: {
      entities,
      properties,
      aspects,
    },
    deferred,
    stats: {
      definitions: definitionalPackage.definitions.length,
      entities: entities.length,
      properties: properties.length,
      aspects: aspects.length,
      deferred: deferred.length,
    },
  });
}
