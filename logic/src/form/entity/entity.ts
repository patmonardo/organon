import {
  Entity as SchemaEntity,
  EntityRef as SchemaEntityRef,
  createEntity as createSchemaEntity,
  updateEntity as updateSchemaEntity,
  createEntityRef as createSchemaEntityRef,
} from "../../schema/entity";

export type FormEntityId = string; // UUID preferred; engines may temporarily use non-UUID for system IDs

/**
 * FormEntity wraps the schema Entity to provide engine-friendly behavior while
 * keeping alignment with the canonical schema types.
 */
export type FormEntityRef = SchemaEntityRef;

export class FormEntity {
  // Canonical schema record (Being). Do not expose for mutation directly.
  private _schema: SchemaEntity;

  // Extra-engine metadata not in schema (execution markers, etc.). Optional.
  public metadata: Record<string, any> = {};

  // Optional association to a context (outside of schema domain)
  public contextId?: string;

  // Local in-memory registry to support legacy findOrCreate usage across engines
  private static registry: Map<FormEntityId, FormEntity> = new Map();

  // Public constructor for backward-compat (legacy new FormEntity({...}))
  constructor(configOrSchema: SchemaEntity | {
    id: FormEntityId;
    type: string;
    name?: string;
    description?: string;
    properties?: Record<string, any>;
    status?: SchemaEntity["status"];
    metadata?: Record<string, any>;
    contextId?: string;
  }) {
    if ("id" in configOrSchema && "type" in configOrSchema && !("createdAt" in configOrSchema)) {
      const cfg = configOrSchema;
      const schema = createSchemaEntity({
        id: cfg.id,
        type: cfg.type,
        name: cfg.name ?? cfg.id,
        description: cfg.description,
        properties: cfg.properties,
        status: cfg.status,
      });
      this._schema = schema;
      if (cfg.metadata) this.metadata = { ...cfg.metadata };
      if (cfg.contextId) this.contextId = cfg.contextId;
      return;
    }
    // Construct from schema entity (preferred path)
    const schema = configOrSchema as SchemaEntity;
    this._schema = schema;
    // extra options not available via this constructor; use fromSchema if needed
  }

  // --------- Factories ---------
  static create(params: {
    type: string;
    id?: string; // Prefer UUID; if omitted, schema factory will generate one
    name?: string;
    description?: string;
    properties?: Record<string, any>;
    status?: SchemaEntity["status"];
    metadata?: Record<string, any>;
    contextId?: string;
  }): FormEntity {
    // Note: schema's createEntity doesn't validate UUID; it will accept provided id.
    const schema = createSchemaEntity({
      type: params.type,
      id: params.id,
      name: params.name,
      description: params.description,
      properties: params.properties,
      status: params.status,
    });
    const instance = new FormEntity(schema);
    if (params.metadata) instance.metadata = { ...params.metadata };
    if (params.contextId) instance.contextId = params.contextId;
    return instance;
  }

  static fromSchema(schema: SchemaEntity, options?: { metadata?: Record<string, any>; contextId?: string; }): FormEntity {
    const instance = new FormEntity(schema);
    if (options?.metadata) instance.metadata = { ...options.metadata };
    if (options?.contextId) instance.contextId = options.contextId;
    return instance;
  }

  // Legacy compatibility: minimal in-memory findOrCreate for engines
  static findOrCreate(config: { id: string; type: string; name?: string; }): FormEntity {
    const existing = FormEntity.registry.get(config.id);
    if (existing) return existing;
    // Use provided id verbatim (may be non-UUID for system entities like engines)
    const entity = FormEntity.create({ type: config.type, id: config.id, name: config.name, metadata: { created: Date.now() } });
    FormEntity.registry.set(entity.id, entity);
    // Touch updated marker for legacy code paths that expect it
    entity.metadata.updated = Date.now();
    return entity;
  }

  /**
   * Optional direct lookup of the local registry (legacy). Prefer engine services.
   * @deprecated Use an Engine/Repository instead.
   */
  static getEntity(id: string): FormEntity | undefined {
    return FormEntity.registry.get(id);
  }

  /**
   * @deprecated Legacy helper for tests/bootstraps. Prefer engine-managed lifecycle.
   */
  static _registerEntity(entity: FormEntity): void {
    FormEntity.registry.set(entity.id, entity);
    if (!entity.metadata.created) entity.metadata.created = Date.now();
    entity.metadata.updated = Date.now();
  }

  // --------- Schema accessors ---------
  get id(): FormEntityId { return this._schema.id; }
  get type(): string { return this._schema.type; }
  set type(value: string) { this._schema = { ...this._schema, type: value, updatedAt: new Date() }; }
  get name(): string { return this._schema.name; }
  get description(): string | undefined { return this._schema.description; }
  get properties(): Record<string, any> { return this._schema.properties ?? {}; }
  get status(): SchemaEntity["status"] { return this._schema.status; }
  get version(): number { return this._schema.version; }
  get createdAt(): Date { return this._schema.createdAt; }
  get updatedAt(): Date { return this._schema.updatedAt; }

  set name(value: string) { this._schema = { ...this._schema, name: value, updatedAt: new Date() }; }
  set description(value: string | undefined) { this._schema = { ...this._schema, description: value, updatedAt: new Date() }; }
  set properties(value: Record<string, any>) { this._schema = { ...this._schema, properties: value, updatedAt: new Date() }; }

  /** Update via schema helper to preserve invariants. */
  update(updates: Partial<Pick<SchemaEntity, "name" | "description" | "properties" | "status" | "version">>): void {
    this._schema = updateSchemaEntity(this._schema, updates);
  }

  /** A stable schema view for persistence / transport. */
  toSchema(): SchemaEntity { return this._schema; }

  /** Convenience JSON including extra engine metadata/context. */
  toJSON() {
    return { ...this._schema, contextId: this.contextId, metadata: this.metadata };
  }

  /** Schema-level reference (type + id). */
  getRef(): SchemaEntityRef { return createSchemaEntityRef(this._schema); }
}

// Note: keep the file export shape stable for existing imports