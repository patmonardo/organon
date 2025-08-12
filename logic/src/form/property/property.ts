import { z } from "zod";
import {
  PropertySchema,
  type Property,
  PropertyCore,
  PropertyState,
  ValueType,
  createProperty as createPropertyDoc,
  updateProperty as updatePropertyDoc,
} from "../../schema/property";
import { EntityRef } from "../../schema/entity";
import { Id } from "../../schema/base";

export class FormProperty {
  private doc: Property;

  private constructor(doc: Property) {
    this.doc = doc;
  }

  // Factory: create new Property (schema defaults/validation applied)
  static create(input: {
    type: z.input<typeof PropertyCore.shape.type>;
    key: z.input<typeof PropertyCore.shape.key>;
    id?: string;

    contextId: z.input<typeof Id>;
    entity?: z.input<typeof EntityRef>;
    relationId?: z.input<typeof Id>;

    name?: z.input<typeof PropertyCore.shape.name>;
    description?: z.input<typeof PropertyCore.shape.description>;

    state?: z.input<typeof PropertyState>;
    value?: unknown;
    valueType?: z.input<typeof ValueType>;
    version?: string;
    ext?: Record<string, unknown>;
  }): FormProperty {
    const doc = createPropertyDoc(input as any);
    return new FormProperty(doc);
  }

  // Factory: wrap an existing schema doc (or input parsed to one)
  static from(input: z.input<typeof PropertySchema> | Property): FormProperty {
    const doc = PropertySchema.parse(input as any);
    return new FormProperty(doc);
  }

  // Access underlying schema doc
  toSchema(): Property {
    return this.doc;
  }
  toJSON(): Property {
    return this.doc;
  }

  // Core getters
  get id(): string { return this.doc.shape.core.id; }
  get type(): string { return this.doc.shape.core.type as string; }
  get key(): string { return this.doc.shape.core.key; }
  get name(): string | undefined { return this.doc.shape.core.name; }
  get description(): string | undefined { return this.doc.shape.core.description; }
  get createdAt(): string { return this.doc.shape.core.createdAt; }
  get updatedAt(): string { return this.doc.shape.core.updatedAt; }

  // State + binding getters
  get status(): z.infer<typeof PropertyState.shape.status> { return this.doc.shape.state.status; }
  get tags(): string[] { return [...this.doc.shape.state.tags]; }
  get meta(): Record<string, unknown> { return this.doc.shape.state.meta; }

  get contextId(): string { return this.doc.shape.contextId; }
  get entity(): z.infer<typeof EntityRef> | undefined { return this.doc.shape.entity; }
  get relationId(): string | undefined { return this.doc.shape.relationId; }

  get value(): unknown { return this.doc.shape.value; }
  get valueType(): z.infer<typeof ValueType> | undefined { return this.doc.shape.valueType; }

  get revision(): number { return this.doc.revision; }
  get version(): string | undefined { return this.doc.version; }

  // Core/state mutators
  setName(name?: string): this {
    this.doc = updatePropertyDoc(this.doc, { core: { name } });
    return this;
  }
  setDescription(description?: string): this {
    this.doc = updatePropertyDoc(this.doc, { core: { description } });
    return this;
  }
  setStatus(status: z.input<typeof PropertyState.shape.status>): this {
    this.doc = updatePropertyDoc(this.doc, { state: { status } });
    return this;
  }
  addTag(tag: string): this {
    if (!this.doc.shape.state.tags.includes(tag)) {
      this.doc = updatePropertyDoc(this.doc, { state: { tags: [...this.doc.shape.state.tags, tag] } });
    }
    return this;
  }
  removeTag(tag: string): this {
    if (this.doc.shape.state.tags.includes(tag)) {
      const next = this.doc.shape.state.tags.filter(t => t !== tag);
      this.doc = updatePropertyDoc(this.doc, { state: { tags: next } });
    }
    return this;
  }
  patchMeta(patch: Record<string, unknown>): this {
    this.doc = updatePropertyDoc(this.doc, { state: { meta: { ...this.doc.shape.state.meta, ...patch } } as any });
    return this;
  }

  // Binding mutators
  bindToEntity(ref: z.input<typeof EntityRef>): this {
    this.doc = updatePropertyDoc(this.doc, { entity: EntityRef.parse(ref), relationId: null });
    return this;
  }
  bindToRelation(relationId: z.input<typeof Id>): this {
    this.doc = updatePropertyDoc(this.doc, { entity: null, relationId });
    return this;
  }
  clearBinding(): this {
    this.doc = updatePropertyDoc(this.doc, { entity: null, relationId: null });
    return this;
  }
  setContext(contextId: z.input<typeof Id>): this {
    this.doc = updatePropertyDoc(this.doc, { contextId });
    return this;
  }

  // Value mutators
  setValue(value: unknown): this {
    this.doc = updatePropertyDoc(this.doc, { value });
    return this;
  }
  clearValue(): this {
    this.doc = updatePropertyDoc(this.doc, { value: undefined });
    return this;
  }
  setValueType(valueType?: z.input<typeof ValueType>): this {
    this.doc = updatePropertyDoc(this.doc, { valueType });
    return this;
  }
}
