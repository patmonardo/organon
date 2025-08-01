/**
 * Controller Schema: Action–Rule (Active Concept)
 * ------------------------------------------------
 * In ORGANON, Controller is the dialectical advancement of Concept (Morphism:Relation) into activity:
 *   - Action: The projection or enactment of an Active Concept qua Morphism (transformation, operation)
 *   - Rule: The operationalized, active Relation that governs or constrains Actions
 * Controller = Action : Rule
 *
 * Philosophical Note:
 * Controller is not just a passive logic, but an active, agential mediation—
 * advancing Concept into the domain of agency, transformation, and governance.
 * This enables the system to move from static relations to dynamic, rule-governed actions.
 */

import { z } from 'zod';

// Action: The projection/enactment of an Active Concept (Morphism)
export const ActionSchema = z.object({
  type: z.string(), // e.g., 'create', 'update', 'delete', 'transform', etc.
  payload: z.any(), // the data or parameters for the action
  // Optionally, add metadata, actor, or context fields
});

// Rule: The operationalized, active Relation that governs Actions
export const RuleSchema = z.object({
  name: z.string(), // rule identifier
  condition: z.string().optional(), // e.g., an expression or reference
  effect: z.string().optional(), // description or reference to effect
  // Optionally, add priority, constraints, or logic fields
});

// Controller: Synthesis of Action and Rule
export const ControllerSchema = z.object({
  action: ActionSchema,
  rule: RuleSchema,
  // Optionally, add metadata, version, or context fields
}).catchall(z.any());
