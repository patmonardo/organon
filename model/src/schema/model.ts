/**
 * Model Schema: State–Structure (Active Being)
 * --------------------------------------------
 * In ORGANON, Model is the dialectical advancement of Being (Form:Entity) into activity:
 *   - State: The ActiveForm, a dynamic configuration or manifestation of a Form
 *   - Structure: The configuration/schema that determines which fields/properties of an Entity are included/excluded for a particular Action
 * Model = State : Structure
 *
 * Philosophical Note:
 * State is an ActiveForm; Structure is the schema/configuration of an ActiveEntity.
 * This enables the Model to mediate between pure Being and Action, supporting dynamic inclusion/exclusion of properties for agency.
 */

import { z } from 'zod';

// State: An ActiveForm (dynamic configuration of a Form)
export const StateSchema = z.object({
  id: z.string(),
  form: z.string(), // reference to the Form type or name
  values: z.record(z.string(), z.any()), // dynamic values for the form fields
  // Optionally, add metadata or status fields
});

// Structure: Configuration/schema for an ActiveEntity
export const StructureSchema = z.object({
  entityType: z.string(), // reference to the Entity type or name
  include: z.array(z.string()).optional(), // fields to include for this structure
  exclude: z.array(z.string()).optional(), // fields to exclude for this structure
  // Optionally, add constraints, defaults, or other schema logic
});

// Model: Synthesis of State and Structure
export const ModelSchema = z.object({
  state: StateSchema,
  structure: StructureSchema,
  // Optionally, add metadata, version, or context fields
}).catchall(z.any());
