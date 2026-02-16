import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { ABSOLUTE_INDIFFERENCE_TOPIC_MAP } from './sources/absolute-indifference-topic-map';

const aiEntryById = (id: string) =>
  ABSOLUTE_INDIFFERENCE_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'becoming-essence-1',
  title: 'Absolute indifference as abstract unity',
  concept: 'AbsoluteIndifference',
  phase: 'quantity',
  moments: [
    {
      name: 'abstractIndifference',
      definition:
        'Measure culminates in an indifference where determinations are held together as one abstract substrate',
      type: 'determination',
    },
    {
      name: 'vanishingDeterminateness',
      definition:
        'Differentiations persist only as vanishing moments in the indifference-whole',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'abstractIndifference',
    },
    {
      name: 'latentDifference',
      definition:
        'Indifference contains an implicit difference that has not yet assumed explicit relational form',
      type: 'mediation',
    },
  ],
  invariants: [
    {
      id: 'becoming-essence-1-inv-1',
      constraint: 'indifference is unity of measure moments',
      predicate: 'unityOfMoments(indifference)',
    },
    {
      id: 'becoming-essence-1-inv-2',
      constraint: 'determinateness is present only as vanishing',
      predicate: 'vanishingDeterminatenessIn(indifference)',
    },
  ],
  forces: [
    {
      id: 'becoming-essence-1-force-1',
      description:
        'Latent difference in indifference posits itself as inverse ratio',
      type: 'contradiction',
      trigger: 'indifference.latentDifferenceSelfPosits = true',
      effect: 'inverseRatioStructure.explicit = true',
      targetState: 'becoming-essence-1-inverse-ratio',
    },
  ],
  transitions: [
    {
      id: 'becoming-essence-1-trans-1',
      from: 'becoming-essence-1',
      to: 'becoming-essence-1-inverse-ratio',
      mechanism: 'contradiction',
      description: 'From abstract indifference to inverse-ratio determination',
    },
  ],
  nextStates: ['becoming-essence-1-inverse-ratio'],
  previousStates: ['real-measure-3'],
  provenance: {
    topicMapId: 'be-a-1-being-abstract-indifference',
    lineRange: { start: 3, end: 12 },
    section: 'Absolute Indifference',
    order: 1,
  },
  description: aiEntryById('be-a-1-being-abstract-indifference')?.description,
  keyPoints: aiEntryById('be-a-1-being-abstract-indifference')?.keyPoints,
};

export const absoluteIndifferenceIR: DialecticIR = {
  id: 'absolute-indifference-ir',
  title: 'Absolute Indifference IR',
  section: 'BEING - MEASURE - C. Becoming of Essence - Absolute Indifference',
  states: [state1],
  metadata: {
    sourceFile: 'absolute-indifference.txt',
    totalStates: 1,
    cpuGpuMapping: {
      'becoming-essence-1': 'quantity',
    },
  },
};

export const absoluteIndifferenceStates = {
  'becoming-essence-1': state1,
};
