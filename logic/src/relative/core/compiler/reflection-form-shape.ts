import { z } from 'zod';

const bhasaPhaseEnum = z.enum(['pratibhasa', 'dharana', 'dhyana', 'nirbhasa']);
const cittaReflectionEnum = z.enum([
  'movement-prelude',
  'positing',
  'external',
  'determining',
]);

const reflectionMomentSchema = z.object({
  name: z.string(),
  definition: z.string(),
  type: z.enum(['process', 'negation', 'mediation', 'determination']),
  relation: z.string().optional(),
  relatedTo: z.string().optional(),
});

const reflectionForceSchema = z.object({
  id: z.string(),
  description: z.string(),
  type: z.string(),
  trigger: z.string(),
  effect: z.string(),
  targetState: z.string(),
});

const reflectionTransitionSchema = z.object({
  id: z.string(),
  from: z.string(),
  to: z.string(),
  mechanism: z.string(),
  description: z.string(),
});

const provenanceLineRangeSchema = z.object({
  start: z.number(),
  end: z.number(),
});

const provenanceSchema = z.object({
  topicMapId: z.string(),
  lineRange: provenanceLineRangeSchema,
  section: z.string(),
  order: z.number(),
});

const reflectionFormShapeBaseSchema = z.object({
  id: z.string(),
  refStateId: z.enum(['ref-2', 'ref-7', 'ref-10', 'ref-14']),
  reflectionMode: z.enum(['movement', 'positing', 'external', 'determining']),
  cittaReflection: cittaReflectionEnum,
  bhasa: z.object({
    phase: bhasaPhaseEnum,
    pratyaharaTrace: z.array(z.string()),
    dharanaTrigger: z.string().optional(),
    dhyanaAbiding: z.boolean().default(false),
    samadhiSignal: z
      .object({ ready: z.boolean(), reference: z.string() })
      .default({ ready: false, reference: '' }),
  }),
  moments: z.array(reflectionMomentSchema),
  invariants: z.array(
    z.object({ id: z.string(), constraint: z.string(), predicate: z.string() }),
  ),
  forces: z.array(reflectionForceSchema),
  transitions: z.array(reflectionTransitionSchema),
  provenance: provenanceSchema,
});

const reflectionRef2ShapeSchema = reflectionFormShapeBaseSchema.extend({
  refStateId: z.literal('ref-2'),
  reflectionMode: z.literal('movement'),
  cittaReflection: z.literal('movement-prelude'),
});

const reflectionRef7ShapeSchema = reflectionFormShapeBaseSchema.extend({
  refStateId: z.literal('ref-7'),
  reflectionMode: z.literal('positing'),
  cittaReflection: z.literal('positing'),
});

const reflectionRef10ShapeSchema = reflectionFormShapeBaseSchema.extend({
  refStateId: z.literal('ref-10'),
  reflectionMode: z.literal('external'),
  cittaReflection: z.literal('external'),
});

const reflectionRef14ShapeSchema = reflectionFormShapeBaseSchema.extend({
  refStateId: z.literal('ref-14'),
  reflectionMode: z.literal('determining'),
  cittaReflection: z.literal('determining'),
});

export const reflectionFormShapeSchema = z.discriminatedUnion('refStateId', [
  reflectionRef2ShapeSchema,
  reflectionRef7ShapeSchema,
  reflectionRef10ShapeSchema,
  reflectionRef14ShapeSchema,
]);

export const reflectionFormShapeFixtures = {
  'ref-2': reflectionFormShapeSchema.parse({
    id: 'formshape-ref-2',
    refStateId: 'ref-2',
    reflectionMode: 'movement',
    cittaReflection: 'movement-prelude',
    bhasa: {
      phase: 'pratibhasa',
      pratyaharaTrace: ['movementFromNothingToNothing'],
      dhyanaAbiding: false,
      samadhiSignal: { ready: false, reference: 'ref-19' },
    },
    moments: [
      {
        name: 'reflection',
        definition: 'Movement of becoming that remains within itself',
        type: 'process',
      },
    ],
    invariants: [
      {
        id: 'ref-2-inv-1',
        constraint: 'essence = reflection',
        predicate: 'is(essence, reflection)',
      },
    ],
    forces: [
      {
        id: 'ref-2-force-1',
        description: 'Absolute reflection determines itself as positing',
        type: 'reflection',
        trigger: 'reflection.absolute = true',
        effect: 'positingReflection.emerges = true',
        targetState: 'ref-7',
      },
    ],
    transitions: [
      {
        id: 'ref-2-trans-1',
        from: 'ref-2',
        to: 'ref-7',
        mechanism: 'reflection',
        description: 'From absolute reflection to positing reflection',
      },
    ],
    provenance: {
      topicMapId: 'ref-2',
      lineRange: { start: 10, end: 54 },
      section: '2. Reflection',
      order: 1,
    },
  }),
  'ref-7': reflectionFormShapeSchema.parse({
    id: 'formshape-ref-7',
    refStateId: 'ref-7',
    reflectionMode: 'positing',
    cittaReflection: 'positing',
    bhasa: {
      phase: 'dharana',
      pratyaharaTrace: ['positing', 'presupposing'],
      dharanaTrigger: 'presupposing.determinesImmediate = true',
      dhyanaAbiding: false,
      samadhiSignal: { ready: false, reference: 'ref-19' },
    },
    moments: [
      {
        name: 'positing',
        definition: 'Immediacy as turning back, no other beforehand',
        type: 'mediation',
      },
    ],
    invariants: [
      {
        id: 'ref-7-inv-4',
        constraint: 'positing ↔ presupposing',
        predicate: 'reciprocal(positing, presupposing)',
      },
    ],
    forces: [
      {
        id: 'ref-7-force-1',
        description: 'Presupposing drives transition to external reflection',
        type: 'reflection',
        trigger: 'presupposing.determinesImmediate = true',
        effect: 'externalReflection.emerges = true',
        targetState: 'ref-10',
      },
    ],
    transitions: [
      {
        id: 'ref-7-trans-1',
        from: 'ref-7',
        to: 'ref-10',
        mechanism: 'reflection',
        description: 'From positing reflection to external reflection',
      },
    ],
    provenance: {
      topicMapId: 'ref-7',
      lineRange: { start: 128, end: 165 },
      section: '2. Reflection',
      order: 2,
    },
  }),
  'ref-10': reflectionFormShapeSchema.parse({
    id: 'formshape-ref-10',
    refStateId: 'ref-10',
    reflectionMode: 'external',
    cittaReflection: 'external',
    bhasa: {
      phase: 'dhyana',
      pratyaharaTrace: ['externalReflection', 'syllogismStructure'],
      dharanaTrigger: 'reflection.unifies = true',
      dhyanaAbiding: true,
      samadhiSignal: { ready: false, reference: 'ref-19' },
    },
    moments: [
      {
        name: 'externalReflection',
        definition: 'Presupposes itself as sublated, negative of itself',
        type: 'mediation',
      },
    ],
    invariants: [
      {
        id: 'ref-10-inv-3',
        constraint: 'structure = syllogism',
        predicate: 'is(structure, syllogism)',
      },
    ],
    forces: [
      {
        id: 'ref-10-force-1',
        description: 'Unity drives toward determining reflection',
        type: 'reflection',
        trigger: 'reflection.unifies = true',
        effect: 'determiningReflection.emerges = true',
        targetState: 'ref-14',
      },
    ],
    transitions: [
      {
        id: 'ref-10-trans-1',
        from: 'ref-10',
        to: 'ref-14',
        mechanism: 'reflection',
        description: 'From external reflection to determining reflection',
      },
    ],
    provenance: {
      topicMapId: 'ref-10',
      lineRange: { start: 232, end: 247 },
      section: '2. Reflection',
      order: 3,
    },
  }),
  'ref-14': reflectionFormShapeSchema.parse({
    id: 'formshape-ref-14',
    refStateId: 'ref-14',
    reflectionMode: 'determining',
    cittaReflection: 'determining',
    bhasa: {
      phase: 'dhyana',
      pratyaharaTrace: ['determiningReflection', 'positedness'],
      dharanaTrigger: 'determining.complete = true',
      dhyanaAbiding: true,
      samadhiSignal: { ready: true, reference: 'ref-19' },
    },
    moments: [
      {
        name: 'determiningReflection',
        definition: 'Unity of positing and external reflection',
        type: 'mediation',
      },
    ],
    invariants: [
      {
        id: 'ref-14-inv-1',
        constraint: 'determiningReflection = unity(positing, external)',
        predicate: 'equals(determiningReflection, unity(positing, external))',
      },
    ],
    forces: [
      {
        id: 'ref-14-force-1',
        description: 'Determining reflection establishes determination',
        type: 'reflection',
        trigger: 'determining.complete = true',
        effect: 'determinationOfReflection.emerges = true',
        targetState: 'ref-19',
      },
    ],
    transitions: [
      {
        id: 'ref-14-trans-1',
        from: 'ref-14',
        to: 'ref-19',
        mechanism: 'reflection',
        description: 'From determining reflection to determination',
      },
    ],
    provenance: {
      topicMapId: 'ref-14',
      lineRange: { start: 317, end: 334 },
      section: '2. Reflection',
      order: 4,
    },
  }),
} as const;

export const reflectionFormShapeFixtureList = [
  reflectionFormShapeFixtures['ref-2'],
  reflectionFormShapeFixtures['ref-7'],
  reflectionFormShapeFixtures['ref-10'],
  reflectionFormShapeFixtures['ref-14'],
] as const;

export type ReflectionFormShape = z.infer<typeof reflectionFormShapeSchema>;
