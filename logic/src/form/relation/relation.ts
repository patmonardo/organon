import {
  Relation as SchemaRelation,
  createRelation as createSchemaRelation,
  createBidirectionalRelation as createSchemaBidirectionalRelation,
  isRelationActiveAt as isSchemaRelationActiveAt,
  EntityRef as SchemaEntityRef,
} from "../../schema/relation";

export type FormRelationId = string;
export type FormRelationEntityRef = SchemaEntityRef;

export class FormRelation {
  private _schema: SchemaRelation;
  public metadata: Record<string, any> = {};

  constructor(schema: SchemaRelation, options?: { metadata?: Record<string, any> }) {
    this._schema = schema;
    if (options?.metadata) this.metadata = { ...options.metadata };
  }

  static create(params: Parameters<typeof createSchemaRelation>[0] & { metadata?: Record<string, any> }): FormRelation {
    const schema = createSchemaRelation(params);
    return new FormRelation(schema, { metadata: params.metadata });
  }

  static createBidirectional(params: Parameters<typeof createSchemaBidirectionalRelation>[0] & { metadata?: Record<string, any> }): FormRelation {
    const schema = createSchemaBidirectionalRelation(params);
    return new FormRelation(schema, { metadata: params.metadata });
  }

  static fromSchema(schema: SchemaRelation, options?: { metadata?: Record<string, any> }): FormRelation {
    return new FormRelation(schema, options);
  }

  get id(): FormRelationId { return this._schema.id; }
  get type(): string { return this._schema.type; }
  set type(v: string) { this._schema = { ...this._schema, type: v, updatedAt: new Date() }; }
  get source(): SchemaEntityRef { return this._schema.source; }
  get target(): SchemaEntityRef { return this._schema.target; }
  get properties(): Record<string, any> { return this._schema.properties ?? {}; }
  set properties(v: Record<string, any>) { this._schema = { ...this._schema, properties: v, updatedAt: new Date() }; }

  isActiveAt(date?: Date | null): boolean {
    return isSchemaRelationActiveAt(this._schema, date ?? null);
  }

  toSchema(): SchemaRelation { return this._schema; }
  toJSON() { return { ...this._schema, metadata: this.metadata }; }
}
