/**
 * BEC-aligned basic types for the Form Processor.
 * - Form: possibility-space (shape) anchored in a World (Context).
 * - Entity: concrete Thing that instantiates a Form.
 * - Property: intensional relation (from form/property module).
 * - World: middleware of properties (Context).
 */
export type WorldId = string;
export type FormId = string;
export type EntityId = string;
export type PropertyId = string;

export type ValueAtom =
  | string
  | number
  | boolean
  | null
  | { iri: string }
  | { lang: string; text: string };

export type Cardinality = {
  min?: number; // default 0
  max?: number; // default Infinity
  functional?: boolean; // implies max = 1
};

export type RangeSpec =
  | { kind: 'value'; valueType: string } // scalar/enum/literal
  | { kind: 'entity'; shapeIds: FormId[] }; // object range

// Adapter view of a property (from @logic/form/property)
export interface PropertySpec {
  id: PropertyId;
  name: string;
  domain: FormId[]; // forms it applies to
  range: RangeSpec;
  cardinality?: Cardinality;
  required?: boolean;
}

export type FieldValue = ValueAtom | { entityId: EntityId };
