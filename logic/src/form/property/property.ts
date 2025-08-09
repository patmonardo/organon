import {
  FormProperty as SchemaProperty,
  FormPropertySchema as _SchemaPropertySchema,
  FormPropertyDefinition as SchemaPropertyDefinition,
  FormPropertyDefinitionSchema as _SchemaPropertyDefinitionSchema,
} from "../../schema/property";

export type FormPropertyId = string;

export class FormProperty {
  private _schema: SchemaProperty;
  public metadata: Record<string, any> = {};

  constructor(schema: SchemaProperty, options?: { metadata?: Record<string, any> }) {
    this._schema = schema;
    if (options?.metadata) this.metadata = { ...options.metadata };
  }

  static fromSchema(schema: SchemaProperty, options?: { metadata?: Record<string, any> }): FormProperty {
    return new FormProperty(schema, options);
  }

  get id(): FormPropertyId { return this._schema.id; }
  get name(): string { return this._schema.name; }
  set name(v: string) { this._schema = { ...this._schema, name: v, updatedAt: Date.now() }; }
  get description(): string | undefined { return this._schema.description; }
  set description(v: string | undefined) { this._schema = { ...this._schema, description: v, updatedAt: Date.now() }; }
  get contextId(): string { return this._schema.contextId; }
  get entityId(): string | undefined { return this._schema.entityId; }
  get relationId(): string | undefined { return this._schema.relationId; }
  get staticValue(): any { return this._schema.staticValue; }
  set staticValue(v: any) { this._schema = { ...this._schema, staticValue: v, updatedAt: Date.now() }; }

  toSchema(): SchemaProperty { return this._schema; }
  toJSON() { return { ...this._schema, metadata: this.metadata }; }
}

export class FormPropertyDefinition {
  private _schema: SchemaPropertyDefinition;
  public metadata: Record<string, any> = {};

  constructor(schema: SchemaPropertyDefinition, options?: { metadata?: Record<string, any> }) {
    this._schema = schema;
    if (options?.metadata) this.metadata = { ...options.metadata };
  }

  static fromSchema(schema: SchemaPropertyDefinition, options?: { metadata?: Record<string, any> }): FormPropertyDefinition {
    return new FormPropertyDefinition(schema, options);
  }

  get id(): string { return this._schema.id; }
  get name(): string { return this._schema.name; }
  get scriptType(): SchemaPropertyDefinition["scriptType"] { return this._schema.scriptType; }
  get contextId(): string { return this._schema.contextId; }
  get propertyId(): string { return this._schema.propertyId; }

  toSchema(): SchemaPropertyDefinition { return this._schema; }
  toJSON() { return { ...this._schema, metadata: this.metadata }; }
}
