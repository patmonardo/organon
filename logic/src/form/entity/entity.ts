import { z } from "zod";
import {
  EntitySchema,
  type Entity,
  EntityCore,
  EntityState,
  createEntity as createEntityDoc,
  updateEntity as updateEntityDoc,
  createEntityRef,
  type EntityRef,
  formatEntityKey,
} from "../../schema/entity";

export type FormEntityId = string;

export class FormEntity {
  private doc: Entity;

  private constructor(doc: Entity) {
    this.doc = doc;
  }

  // Factory: create from params (uses schema defaults/validation)
  static create(input: {
    type: z.input<typeof EntityCore.shape.type>;
    id?: string;
    name?: z.input<typeof EntityCore.shape.name>;
    description?: z.input<typeof EntityCore.shape.description>;
    state?: z.input<typeof EntityState>;
    version?: string;
    ext?: Record<string, unknown>;
  }): FormEntity {
    const doc = createEntityDoc(input as any);
    return new FormEntity(doc);
  }

  // Factory: wrap an existing schema doc (or input parsed to one)
  static from(input: z.input<typeof EntitySchema> | Entity): FormEntity {
    const doc = EntitySchema.parse(input as any);
    return new FormEntity(doc);
  }

  // Access underlying schema doc
  toSchema(): Entity {
    return this.doc;
  }
  toJSON(): Entity {
    return this.doc;
  }

  // Convenience getters
  get id(): string {
    return this.doc.shape.core.id;
  }
  get type(): string {
    return this.doc.shape.core.type as string;
  }
  get name(): string | undefined {
    return this.doc.shape.core.name;
  }
  get description(): string | undefined {
    return this.doc.shape.core.description;
  }
  get createdAt(): string {
    return this.doc.shape.core.createdAt;
  }
  get updatedAt(): string {
    return this.doc.shape.core.updatedAt;
  }
  get status(): z.infer<typeof EntityState.shape.status> {
    return this.doc.shape.state.status;
  }
  get tags(): string[] {
    return [...this.doc.shape.state.tags];
  }
  get meta(): Record<string, unknown> {
    return this.doc.shape.state.meta;
  }
  get revision(): number {
    return this.doc.revision;
  }
  get version(): string | undefined {
    return this.doc.version;
  }
  get key(): string {
    return formatEntityKey(this.toRef());
  }

  toRef(): EntityRef {
    return createEntityRef(this.doc);
  }

  // Mutators (schema-safe via update helper)
  setName(name?: string): this {
    this.doc = updateEntityDoc(this.doc, { core: { name } });
    return this;
  }

  setDescription(description?: string): this {
    this.doc = updateEntityDoc(this.doc, { core: { description } });
    return this;
  }

  setStatus(status: z.input<typeof EntityState.shape.status>): this {
    this.doc = updateEntityDoc(this.doc, { state: { status } });
    return this;
  }

  addTag(tag: string): this {
    if (!this.doc.shape.state.tags.includes(tag)) {
      this.doc = updateEntityDoc(this.doc, {
        state: { tags: [...this.doc.shape.state.tags, tag] },
      });
    }
    return this;
  }

  removeTag(tag: string): this {
    if (this.doc.shape.state.tags.includes(tag)) {
      const next = this.doc.shape.state.tags.filter((t) => t !== tag);
      this.doc = updateEntityDoc(this.doc, { state: { tags: next } });
    }
    return this;
  }

  patchMeta(patch: Record<string, unknown>): this {
    this.doc = updateEntityDoc(this.doc, {
      state: { meta: { ...this.doc.shape.state.meta, ...patch } } as any,
    });
    return this;
  }
}
