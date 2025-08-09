//@organon/logic/schema/FormEntity.ts
import { z } from 'zod';
import { BaseSchema } from './base';

/**
 * FormEntity Schema
 *
 * Core schema for entities in the membership system.
 * This schema represents the "Being" aspect of our conceptual triad.
 */

// Basic FormEntity reference schema
export const FormEntityRefSchema = z.object({
  entity: z.string(),
  id: z.string()
});

export type FormEntityRef = z.infer<typeof FormEntityRefSchema>;

// Namespaces represent categories of entities
export const CoreNamespaces = [
  'system',     // System entities
  'core',       // Core business entities
  'user',       // User-related entities
  'finance',    // Financial entities
  'document',   // Document entities
  'meta'        // Metadata entities
] as const;

// Core system FormEntity types
export const SystemFormEntityTypes = [
  'system.FormEntity',       // Base FormEntity definition
  'system.FormEntityType',   // FormEntity type definition
  'system.Relation',     // Relation definition
  'system.Context',      // Context definition
  'system.Registry'      // Registry definition
] as const;

// FormEntity status values
export const FormEntityStatusValues = [
  'active',     // FormEntity is active and usable
  'archived',   // FormEntity is archived but retrievable
  'deleted',    // FormEntity is soft-deleted
  'draft',      // FormEntity is in draft mode
  'template'    // FormEntity is a template
] as const;

// The core FormEntity schema
export const FormEntitySchema = BaseSchema.extend({
  // Core idFormEntity
  type: z.string(),  // The FormEntity type
  name: z.string(),  // Display name

  // Optional description
  description: z.string().optional(),

  // Extended data
  properties: z.record(z.any()).optional(),

  // System metadata
  status: z.enum(FormEntityStatusValues).default('active'),
  version: z.number().int().default(1)
});

export type FormEntity = z.infer<typeof FormEntitySchema>;

/**
 * Helper function to create a new FormEntity
 */
export function createFormEntity(params: {
  type: string;
  id?: string;
  name?: string;
  description?: string;
  properties?: Record<string, any>;
  status?: z.infer<typeof FormEntitySchema.shape.status>;
}): FormEntity {
  const id = params.id || crypto.randomUUID();
  const now = new Date();

  return {
    id,
    type: params.type,
    name: params.name || id,
    description: params.description,
    properties: params.properties || {},
    status: params.status || 'active',
    version: 1,
    createdAt: now,
    updatedAt: now
  };
}

// Update this function to use 'status' instead of 'valid'
export function updateFormEntity(
  FormEntity: FormEntity,
  updates: {
    name?: string;
    description?: string;
    properties?: Record<string, any>;
    status?: z.infer<typeof FormEntitySchema.shape.status>;
    version?: number;
  }
): FormEntity {
  return {
    ...FormEntity,
    ...(updates.name !== undefined ? { name: updates.name } : {}),
    ...(updates.description !== undefined ? { description: updates.description } : {}),
    ...(updates.status !== undefined ? { status: updates.status } : {}),
    ...(updates.version !== undefined ? { version: updates.version } : {}),
    properties: {
      ...FormEntity.properties,
      ...(updates.properties || {})
    },
    updatedAt: new Date()
  };
}

/**
 * Helper function to create an FormEntity reference
 */
export function createFormEntityRef(FormEntity: FormEntity): FormEntityRef {
  return {
    entity: FormEntity.type,
    id: FormEntity.id
  };
}

/**
 * Helper function to format an FormEntity key
 */
export function formatFormEntityKey(FormEntityOrRef: FormEntity | FormEntityRef): string {
  const type = 'type' in FormEntityOrRef ? FormEntityOrRef.type : FormEntityOrRef.entity;
  const id = FormEntityOrRef.id;
  return `${type}:${id}`;
}

/**
 * Helper function to parse an FormEntity key
 */
export function parseFormEntityKey(key: string): FormEntityRef {
  const [entity, id] = key.split(':');
  return { entity, id };
}

/**
 * Helper function to check if an FormEntity is a system FormEntity
 */
export function isSystemFormEntity(FormEntity: FormEntity): boolean {
  return FormEntity.type.startsWith('system.');
}

/**
 * Helper function to check if an FormEntity is protected
 */
export function isProtectedFormEntity(FormEntity: FormEntity): boolean {
  return Boolean(FormEntity.properties?.protected);
}
