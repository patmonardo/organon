import {
  FormMorph as SchemaMorph,
  FormMorphPipeline as SchemaMorphPipeline,
  defineMorph as defineSchemaMorph,
  defineMorphPipeline as defineSchemaMorphPipeline,
} from "../../schema/morph";

export type FormMorphId = string;

export class FormMorph {
  private _schema: SchemaMorph;
  public metadata: Record<string, any> = {};

  private constructor(schema: SchemaMorph, options?: { metadata?: Record<string, any> }) {
    this._schema = schema;
    if (options?.metadata) this.metadata = { ...options.metadata };
  }

  static define(config: Parameters<typeof defineSchemaMorph>[0] & { metadata?: Record<string, any> }): FormMorph {
    const schema = defineSchemaMorph(config);
    return new FormMorph(schema, { metadata: config.metadata });
  }

  static fromSchema(schema: SchemaMorph, options?: { metadata?: Record<string, any> }): FormMorph {
    return new FormMorph(schema, options);
  }

  get id(): FormMorphId { return this._schema.id; }
  get name(): string { return this._schema.name; }
  get inputType(): string { return this._schema.inputType; }
  get outputType(): string { return this._schema.outputType; }
  get config(): Record<string, any> | undefined { return this._schema.config; }

  toSchema(): SchemaMorph { return this._schema; }
  toJSON() { return { ...this._schema, metadata: this.metadata }; }
}

export class FormMorphPipeline {
  private _schema: SchemaMorphPipeline;
  public metadata: Record<string, any> = {};

  private constructor(schema: SchemaMorphPipeline, options?: { metadata?: Record<string, any> }) {
    this._schema = schema;
    if (options?.metadata) this.metadata = { ...options.metadata };
  }

  static define(
    id: string,
    name: string,
    morphIds: string[],
    options: Parameters<typeof defineSchemaMorphPipeline>[3] & { metadata?: Record<string, any> } = {}
  ): FormMorphPipeline {
    const schema = defineSchemaMorphPipeline(id, name, morphIds, options);
    return new FormMorphPipeline(schema, { metadata: options.metadata });
  }

  static fromSchema(schema: SchemaMorphPipeline, options?: { metadata?: Record<string, any> }): FormMorphPipeline {
    return new FormMorphPipeline(schema, options);
  }

  get id(): string { return this._schema.id; }
  get name(): string { return this._schema.name; }
  get morphs(): string[] { return this._schema.morphs ?? []; }

  toSchema(): SchemaMorphPipeline { return this._schema; }
  toJSON() { return { ...this._schema, metadata: this.metadata }; }
}
