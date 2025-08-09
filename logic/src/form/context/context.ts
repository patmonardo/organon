import {
  Context as SchemaContext,
  createContext as createSchemaContext,
  addEntitiesToContext as addEntitiesToSchemaContext,
  addRelationsToContext as addRelationsToSchemaContext,
  isContextActiveAt as isSchemaContextActiveAt,
} from "../../schema/context";
import { FormEntityRef as SchemaFormEntityRef } from "../../schema/entity";

export type FormContextId = string;
export type FormContextFormEntityRef = SchemaFormEntityRef;

export class FormContext {
  private _schema: SchemaContext;
  public metadata: Record<string, any> = {};

  constructor(schema: SchemaContext, options?: { metadata?: Record<string, any> }) {
    this._schema = schema;
    if (options?.metadata) this.metadata = { ...options.metadata };
  }

  static create(params: Parameters<typeof createSchemaContext>[0] & { metadata?: Record<string, any> }): FormContext {
    const schema = createSchemaContext(params);
    return new FormContext(schema, { metadata: params.metadata });
  }

  static fromSchema(schema: SchemaContext, options?: { metadata?: Record<string, any> }): FormContext {
    return new FormContext(schema, options);
  }

  get id(): FormContextId { return this._schema.id; }
  get name(): string { return this._schema.name; }
  set name(v: string) { this._schema = { ...this._schema, name: v, updatedAt: new Date() }; }
  get type(): string { return this._schema.type; }
  set type(v: string) { this._schema = { ...this._schema, type: v, updatedAt: new Date() }; }
  get description(): string | undefined { return this._schema.description; }
  set description(v: string | undefined) { this._schema = { ...this._schema, description: v, updatedAt: new Date() }; }
  get entities(): ReadonlyArray<SchemaFormEntityRef> { return this._schema.entities; }
  get relations(): ReadonlyArray<string> { return this._schema.relations; }
  get properties(): Record<string, any> { return this._schema.properties ?? {}; }
  set properties(v: Record<string, any>) { this._schema = { ...this._schema, properties: v, updatedAt: new Date() }; }
  get createdAt(): Date { return this._schema.createdAt; }
  get updatedAt(): Date { return this._schema.updatedAt; }
  get scope() { return this._schema.scope; }
  set scope(v: SchemaContext["scope"]) { this._schema = { ...this._schema, scope: v, updatedAt: new Date() }; }

  addEntities(refs: SchemaFormEntityRef[]): void {
    this._schema = addEntitiesToSchemaContext(this._schema, refs);
  }

  addRelations(ids: string[]): void {
    this._schema = addRelationsToSchemaContext(this._schema, ids);
  }

  isActiveAt(date?: Date | null): boolean {
    return isSchemaContextActiveAt(this._schema, date);
  }

  toSchema(): SchemaContext { return this._schema; }
  toJSON() { return { ...this._schema, metadata: this.metadata }; }
}
