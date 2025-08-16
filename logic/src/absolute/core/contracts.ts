import { z } from 'zod';
import { EntitySchema } from '../../schema/entity';
import { RelationSchema } from '../../schema/relation';
import { PropertySchema } from '../../schema/property';
import { ContextSchema } from '../../schema/context';
import { MorphSchema } from '../../schema/morph';
import { WorldSchema } from '../../schema/world';
import { ShapeSchema } from '../../schema/shape';
import { ContentSchema } from '../../schema/content';
import { ConceptSchema } from '../../schema/concept';
import { JudgmentSchema } from '../../schema/judgment';
import { SyllogismSchema } from '../../schema/syllogism';

// Inputs accepted by the Form Processor. Keep defaults for incremental adoption.
export const ProcessorInputs = z.object({
  shapes: z.array(ShapeSchema).default([]),
  entities: z.array(EntitySchema).default([]),
  properties: z.array(PropertySchema).default([]),
  contexts: z.array(ContextSchema).default([]),
  morphs: z.array(MorphSchema).default([]),
  relations: z.array(RelationSchema).default([]),
  content: z.array(ContentSchema).default([]),
  concepts: z.array(ConceptSchema).default([]),
  judgments: z.array(JudgmentSchema).default([]),
  syllogisms: z.array(SyllogismSchema).default([]),
});
export type ProcessorInputs = z.infer<typeof ProcessorInputs>;

const GrossByThing = z.record(z.number().int().nonnegative());
export const ProcessorSnapshot = z.object({
  world: WorldSchema,
  indexes: z
    .object({
      content: z
        .object({
          subtleWorldTotal: z.number().int().nonnegative().default(0),
          grossByThing: GrossByThing.default({}),
        })
        .default({ subtleWorldTotal: 0, grossByThing: {} }),
      // room for logic indexes later
    })
    .default({ content: { subtleWorldTotal: 0, grossByThing: {} } }),
});
export type ProcessorSnapshot = z.infer<typeof ProcessorSnapshot>;
