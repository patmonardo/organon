import { z } from "zod";
import { BaseSchema } from "./base";
import { FormEntityRefSchema } from "./entity";
import { FormEntityRef } from "./entity";

/**
 * FormRelation Schema
 *
 * Core schema for Form entity relations in the membership system.
 * This schema represents the "Essence" aspect of our conceptual triad.
 */

// Common relation types
export const CoreRelationTypes = [
  // Structural relations
  "contains", // Whole-part relation
  "instance_of", // Type-instance relation
  "extends", // Extension relation
  "implements", // Implementation relation

  // Associative relations
  "references", // Simple reference
  "associates", // General association
  "depends_on", // Dependency relation
  "equivalent_to", // Equivalence relation

  // Ownership relations
  "owned_by", // Ownership relation
  "created_by", // Creation relation

  // Generic relation
  "related_to", // Generic relation
] as const;

// Relation direction types
export const RelationDirectionTypes = [
  "directed", // One-way relation: source → target
  "bidirectional", // Two-way relation: source ↔ target
] as const;

// The core relation schema
export const FormRelationSchema = BaseSchema.extend({
  // Connection endpoints
  source: FormEntityRefSchema,
  target: FormEntityRefSchema,

  // Relation characteristics
  type: z.string(),
  direction: z.enum(RelationDirectionTypes).default("directed"),

  // Metadata
  properties: z.record(z.any()).optional(),

  // Validity
  valid: z.boolean().default(true),
  validFrom: z.date().default(() => new Date()),
  validTo: z.date().optional(),

  // Relation strength (0-1)
  strength: z.number().min(0).max(1).default(1),
});

export type FormRelation = z.infer<typeof FormRelationSchema>;

/**
 * Create a relation between entities
 */
export function createFormRelation(params: {
  source: FormEntityRef;
  target: FormEntityRef;
  type: string;
  direction?: z.infer<typeof FormRelationSchema.shape.direction>;
  properties?: Record<string, any>;
  validFrom?: Date;
  validTo?: Date;
  strength?: number;
  valid?: boolean;
}): FormRelation {
  const id = crypto.randomUUID();
  const now = new Date();

  return {
    id,
    source: params.source,
    target: params.target,
    type: params.type,
    direction: params.direction || "directed",
    properties: params.properties || {},
    valid: params.valid ?? true,
    validFrom: params.validFrom || now,
    validTo: params.validTo,
    strength: params.strength ?? 1,
    createdAt: now,
    updatedAt: now,
  };
}

/**
 * Create a bidirectional relation between entities
 */
export function createBidirectionalRelation(
  params: Omit<Parameters<typeof createFormRelation>[0], "direction">
): FormRelation {
  return createFormRelation({
    ...params,
    direction: "bidirectional",
  });
}

// Export a function for getting the current time (easier to mock)
export const getCurrentTime = () => new Date();

/**
 * Helper to determine if a relation is active at a specific time
 */
export function isRelationActiveAt(
  relation: FormRelation,
  date: Date | null = null,
  timeProvider = getCurrentTime
): boolean {
  if (!relation.valid) return false;

  // Use provided date or get current time
  const checkDate = date || timeProvider();

  const afterStart = !relation.validFrom || relation.validFrom <= checkDate;
  const beforeEnd = !relation.validTo || relation.validTo >= checkDate;

  return afterStart && beforeEnd;
}

/**
 * Helper to format an entity key (for indexing)
 */
export function formatEntityKey(ref: FormEntityRef): string {
  return `${ref.entity}:${ref.id}`;
}

/**
 * Helper for migration from old link format
 */
export function linkToRelation(link: {
  sourceEntity: string;
  sourceId: string;
  targetEntity: string;
  targetId: string;
  relation: string;
  metadata?: Record<string, any>;
  established?: Date;
  expires?: Date;
}): FormRelation {
  return createFormRelation({
    source: { entity: link.sourceEntity, id: link.sourceId },
    target: { entity: link.targetEntity, id: link.targetId },
    type: link.relation,
    properties: link.metadata,
    validFrom: link.established,
    validTo: link.expires,
  });
}

/**
 * Helper to invert a relation (swap source and target)
 */
export function invertRelation(relation: FormRelation): FormRelation {
  // Can't invert a bidirectional relation (it's already bidirectional)
  if (relation.direction === "bidirectional") {
    return relation;
  }

  return {
    ...relation,
    id: crypto.randomUUID(), // Create a new ID for the inverted relation
    source: relation.target,
    target: relation.source,
    updatedAt: new Date(),
  };
}

/**
 * Helper to check if two relations connect the same entities
 */
export function relationsConnectSameEntities(
  a: FormRelation,
  b: FormRelation
): boolean {
  const sameDirection =
    a.source.entity === b.source.entity &&
    a.source.id === b.source.id &&
    a.target.entity === b.target.entity &&
    a.target.id === b.target.id;

  const oppositeDirection =
    a.source.entity === b.target.entity &&
    a.source.id === b.target.id &&
    a.target.entity === b.source.entity &&
    a.target.id === b.source.id;

  return sameDirection || oppositeDirection;
}
