//@/core/being/schema/FormContext.ts
import { z } from 'zod';
import { BaseSchema } from './base';
import { FormEntityRefSchema } from './entity';

/**
 * FormContext Schema - The Third Moment of Being
 *
 * In Hegelian terms, FormContext represents the Synthesis (Concept) that unites
 * Entity (Thesis/Being) and Relation (Antithesis/Essence) in a concrete Universal.
 *
 * It embodies the dialectical movement where individual entities and their connections
 * are brought together in a meaningful totality.
 */

// FormContext types representing different modes of conceptual organization
export const CoreFormContextTypes = [
  'collection',    // Simple gathering - immediate unity
  'organization',  // Structured arrangement - mediated unity
  'project',       // Purposive structure - teleological unity
  'domain',        // Knowledge boundary - categorical unity
  'category',      // Classification system - logical unity
  'session',       // Temporal FormContext - processual unity
  'view',          // Perspective FormContext - phenomenological unity
  'workflow',      // Process FormContext - operational unity
  'scenario',      // Hypothetical FormContext - modal unity
  'generic'        // Universal FormContext - abstract unity
] as const;

// The core FormContext schema - represents the concrete universal
export const FormContextSchema = BaseSchema.extend({
  // Universal moment - what makes it this particular FormContext
  name: z.string(),
  type: z.string(),
  description: z.string().optional(),

  // Particular moment - the specific content it contains
  entities: z.array(FormEntityRefSchema).default([]),
  relations: z.array(z.string()).default([]), // IDs of relations

  // Individual moment - the unique properties that differentiate it
  properties: z.record(z.any()).optional(),

  // Quantitative determination - metrics of its internal structure
  metrics: z.object({
    entityCount: z.number().int().nonnegative(),
    relationCount: z.number().int().nonnegative(),
    density: z.number().min(0).max(1).optional() // Network density as a measure of completeness
  }).optional(),

  // Modal determination - its possibility and actuality
  valid: z.boolean().default(true),
  validFrom: z.date().optional(),
  validTo: z.date().optional(),

  // Scope determination - its boundaries
  scope: z.enum(['global', 'domain', 'local']).default('local'),
  domain: z.string().optional(),
});

export type FormContext = z.infer<typeof FormContextSchema>;

/**
 * Calculate graph density from entity and relation counts
 *
 * Represents the ratio of actual to possible relations in the FormContext.
 * This is a quantitative measure of the FormContext's completeness.
 */
export function calculateDensity(entityCount: number, relationCount: number): number | undefined {
  if (entityCount <= 1) return entityCount === 0 ? undefined : 1.0;

  // Maximum possible relations in a directed graph = n(n-1)
  const maxRelations = entityCount * (entityCount - 1);
  return maxRelations > 0 ? relationCount / maxRelations : 0;
}

/**
 * Create a new FormContext - the generative moment
 */
export function createFormContext(params: {
  name: string;
  type: string;
  description?: string;
  entities?: z.infer<typeof FormEntityRefSchema>[];
  relations?: string[];
  properties?: Record<string, any>;
  validFrom?: Date;
  validTo?: Date;
  valid?: boolean;
  scope?: z.infer<typeof FormContextSchema.shape.scope>;
  domain?: string;
}): FormContext {
  const now = new Date();
  const entities = params.entities || [];
  const relations = params.relations || [];

  return {
    id: crypto.randomUUID(),
    name: params.name,
    type: params.type,
    description: params.description,
    entities: entities,
    relations: relations,
    properties: params.properties || {},
    metrics: {
      entityCount: entities.length,
      relationCount: relations.length,
      density: calculateDensity(entities.length, relations.length)
    },
    valid: params.valid ?? true,
    validFrom: params.validFrom,
    validTo: params.validTo,
    scope: params.scope || 'local',
    domain: params.domain,
    createdAt: now,
    updatedAt: now
  };
}

/**
 * Helper to determine if a FormContext is active at a specific time
 *
 * Represents the temporal determination of the FormContext's actuality.
 */
export function isFormContextActiveAt(
  FormContext: FormContext,
  date?: Date | null,
  timeProvider = (): Date => new Date()
): boolean {
  // If FormContext is not valid, it's not active regardless of time
  if (!FormContext.valid) return false;

  // If no date specified, use current time
  const checkDate = date || timeProvider();

  // Check validFrom if specified
  if (FormContext.validFrom && checkDate < FormContext.validFrom) {
    return false;
  }

  // Check validTo if specified
  if (FormContext.validTo && checkDate > FormContext.validTo) {
    return false;
  }

  return true;
}

/**
 * Helper to add entities to a FormContext - the augmentation moment
 */
export function addEntitiesToFormContext(
  FormContext: FormContext,
  entities: z.infer<typeof FormEntityRefSchema>[]
): FormContext {
  // Filter out duplicates
  const existingEntityMap = new Map(
    FormContext.entities.map(e => [`${e.entity}:${e.id}`, e])
  );

  const newEntities = entities.filter(e =>
    !existingEntityMap.has(`${e.entity}:${e.id}`)
  );

  if (newEntities.length === 0) {
    return FormContext;
  }

  const updatedEntities = [...FormContext.entities, ...newEntities];

  return {
    ...FormContext,
    entities: updatedEntities,
    metrics: {
      entityCount: updatedEntities.length,
      relationCount: FormContext.relations.length,
      density: calculateDensity(updatedEntities.length, FormContext.relations.length)
    },
    updatedAt: new Date()
  };
}

/**
 * Helper to add relations to a FormContext - the connection moment
 */
export function addRelationsToFormContext(
  FormContext: FormContext,
  relationIds: string[]
): FormContext {
  // Filter out duplicates
  const newRelations = relationIds.filter(id => !FormContext.relations.includes(id));

  if (newRelations.length === 0) {
    return FormContext;
  }

  const updatedRelations = [...FormContext.relations, ...newRelations];

  return {
    ...FormContext,
    relations: updatedRelations,
    metrics: {
      entityCount: FormContext.entities.length,
      relationCount: updatedRelations.length,
      density: calculateDensity(FormContext.entities.length, updatedRelations.length)
    },
    updatedAt: new Date()
  };
}
