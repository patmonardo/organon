import { z } from "zod";
import {
  RelationSchema,
  type Relation,
  RelationCore,
  RelationState,
  RelationDirection,
  createRelation as createRelationDoc,
  updateRelation as updateRelationDoc,
} from "../../schema/relation";
import { EntityRef } from "../../schema/entity";

export class FormRelation {
  private doc: Relation;

  private constructor(doc: Relation) {
    this.doc = doc;
  }

  // Factory: create from params (schema defaults/validation applied)
  static create(input: {
    type: z.input<typeof RelationCore.shape.type>;
    kind: z.input<typeof RelationCore.shape.kind>;
    source: z.input<typeof EntityRef>;
    target: z.input<typeof EntityRef>;
    id?: string;
    direction?: z.input<typeof RelationDirection>;
    name?: z.input<typeof RelationCore.shape.name>;
    description?: z.input<typeof RelationCore.shape.description>;
    state?: z.input<typeof RelationState>;
    version?: string;
    ext?: Record<string, unknown>;
  }): FormRelation {
    const doc = createRelationDoc(input as any);
    return new FormRelation(doc);
  }

  // Factory: wrap an existing schema doc (or input parsed to one)
  static from(input: z.input<typeof RelationSchema> | Relation): FormRelation {
    const doc = RelationSchema.parse(input as any);
    return new FormRelation(doc);
  }

  // Access underlying schema doc
  toSchema(): Relation {
    return this.doc;
  }
  toJSON(): Relation {
    return this.doc;
  }

  // Core getters
  get id(): string { return this.doc.shape.core.id; }
  get type(): string { return this.doc.shape.core.type as string; }
  get kind(): string { return this.doc.shape.core.kind as string; }
  get name(): string | undefined { return this.doc.shape.core.name; }
  get description(): string | undefined { return this.doc.shape.core.description; }
  get createdAt(): string { return this.doc.shape.core.createdAt; }
  get updatedAt(): string { return this.doc.shape.core.updatedAt; }

  // State/endpoints getters
  get status(): z.infer<typeof RelationState.shape.status> { return this.doc.shape.state.status; }
  get tags(): string[] { return [...this.doc.shape.state.tags]; }
  get meta(): Record<string, unknown> { return this.doc.shape.state.meta; }
  get strength(): number { return this.doc.shape.state.strength; }

  get source(): z.infer<typeof EntityRef> { return this.doc.shape.source; }
  get target(): z.infer<typeof EntityRef> { return this.doc.shape.target; }
  get direction(): z.infer<typeof RelationDirection> { return this.doc.shape.direction; }

  get revision(): number { return this.doc.revision; }
  get version(): string | undefined { return this.doc.version; }

  // Mutators (schema-safe via update helper)
  setName(name?: string): this {
    this.doc = updateRelationDoc(this.doc, { core: { name } });
    return this;
  }
  setDescription(description?: string): this {
    this.doc = updateRelationDoc(this.doc, { core: { description } });
    return this;
  }
  setKind(kind: z.input<typeof RelationCore.shape.kind>): this {
    this.doc = updateRelationDoc(this.doc, { core: { kind } });
    return this;
  }
  setStatus(status: z.input<typeof RelationState.shape.status>): this {
    this.doc = updateRelationDoc(this.doc, { state: { status } });
    return this;
  }
  addTag(tag: string): this {
    if (!this.doc.shape.state.tags.includes(tag)) {
      this.doc = updateRelationDoc(this.doc, { state: { tags: [...this.doc.shape.state.tags, tag] } });
    }
    return this;
  }
  removeTag(tag: string): this {
    if (this.doc.shape.state.tags.includes(tag)) {
      const next = this.doc.shape.state.tags.filter(t => t !== tag);
      this.doc = updateRelationDoc(this.doc, { state: { tags: next } });
    }
    return this;
  }
  patchMeta(patch: Record<string, unknown>): this {
    this.doc = updateRelationDoc(this.doc, { state: { meta: { ...this.doc.shape.state.meta, ...patch } } as any });
    return this;
  }
  setStrength(strength: number): this {
    this.doc = updateRelationDoc(this.doc, { state: { strength } });
    return this;
  }

  setSource(ref: z.input<typeof EntityRef>): this {
    this.doc = updateRelationDoc(this.doc, { source: EntityRef.parse(ref) });
    return this;
  }
  setTarget(ref: z.input<typeof EntityRef>): this {
    this.doc = updateRelationDoc(this.doc, { target: EntityRef.parse(ref) });
    return this;
  }
  setEndpoints(source: z.input<typeof EntityRef>, target: z.input<typeof EntityRef>): this {
    this.doc = updateRelationDoc(this.doc, { source: EntityRef.parse(source), target: EntityRef.parse(target) });
    return this;
  }
  setDirection(direction: z.input<typeof RelationDirection>): this {
    this.doc = updateRelationDoc(this.doc, { direction });
    return this;
  }

  invert(): this {
    if (this.doc.shape.direction === "bidirectional") return this;
    this.doc = updateRelationDoc(this.doc, { source: this.doc.shape.target, target: this.doc.shape.source });
    return this;
  }
}
