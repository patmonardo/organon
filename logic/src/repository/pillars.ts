import { makeInMemoryRepository } from "./memory";
import {
  ShapeSchema,
  EntitySchema,
  ContextSchema,
  PropertySchema,
  MorphSchema,
  RelationSchema,
  type Entity,
  type Context,
  type Property,
  type Morph,
  type Relation,
  type Form,
} from "../schema";

export const Repos = {
  shape: () => makeInMemoryRepository(ShapeSchema),
  entity: () => makeInMemoryRepository(EntitySchema),
  context: () => makeInMemoryRepository(ContextSchema),
  property: () => makeInMemoryRepository(PropertySchema),
  morph: () => makeInMemoryRepository(MorphSchema),
  relation: () => makeInMemoryRepository(RelationSchema),
};

export type { Form, Entity, Context, Property, Morph, Relation };
