import { makeInMemoryRepository } from "./memory";
import {
  ShapeSchema,
  EntitySchema,
  ContextSchema,
  PropertySchema,
  MorphSchema,
  RelationSchema,

} from "../schema";

export const Repos = {
  shape: () => makeInMemoryRepository(ShapeSchema),
  entity: () => makeInMemoryRepository(EntitySchema),
  context: () => makeInMemoryRepository(ContextSchema),
  property: () => makeInMemoryRepository(PropertySchema),
  morph: () => makeInMemoryRepository(MorphSchema),
  relation: () => makeInMemoryRepository(RelationSchema),
};
