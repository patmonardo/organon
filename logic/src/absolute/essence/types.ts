/**
 * Minimal Active* type definitions used by ADRs and engines.
 * These are intentionally lightweight and permissive; engines may refine
 * or replace with stronger schema-backed types as the project evolves.
 */
export interface ActiveBase {
  id: string;
  kind?: string;
  provenance?: any;
  weight?: number; // optional weight used in groundScore
}

export interface ActiveShape extends ActiveBase {
  type?: string;
  // container-side driver specifics
}

export interface ActiveContext extends ActiveBase {
  name?: string;
}

export interface ActiveMorph extends ActiveBase {
  transform?: string;
}

export interface ActiveEntity extends ActiveBase {
  entityType?: string;
}

export interface ActiveProperty extends ActiveBase {
  key?: string;
  value?: any;
}

export interface ActiveRelation extends ActiveBase {
  particularityOf?: string; // absolute id this particular pertains to
  active?: boolean;
  revoked?: boolean;
  confidence?: number; // 0..1
}

export type ActiveAny =
  | ActiveShape
  | ActiveContext
  | ActiveMorph
  | ActiveEntity
  | ActiveProperty
  | ActiveRelation;

export default ActiveAny;
