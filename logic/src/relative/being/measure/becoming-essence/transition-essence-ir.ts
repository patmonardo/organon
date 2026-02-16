import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { TRANSITION_ESSENCE_TOPIC_MAP } from './sources/transition-essence-topic-map';

const teEntryById = (id: string) =>
  TRANSITION_ESSENCE_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'becoming-essence-3',
  title: 'Self-sublation manifested',
  concept: 'ManifestSelfSublation',
  phase: 'quantity',
  moments: [
    {
      name: 'selfSublatingMovement',
      definition:
        'Measure-determinacy manifests itself as the movement of self-cancelling immediacy',
      type: 'sublation',
    },
    {
      name: 'determinationsAsMoments',
      definition:
        'Fixed determinacies are reduced to moments of one reflective process',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'selfSublatingMovement',
    },
    {
      name: 'reflectionThreshold',
      definition:
        'Being ceases to stand immediately and prepares reflective determination',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'becoming-essence-3-inv-1',
      constraint: 'determinacies persist only as moments',
      predicate: 'momentForm(determinacies)',
    },
    {
      id: 'becoming-essence-3-inv-2',
      constraint: 'self-sublation is explicit and objective',
      predicate: 'explicitSelfSublation(being)',
    },
  ],
  forces: [
    {
      id: 'becoming-essence-3-force-1',
      description:
        'Manifest self-sublation determines being explicitly as essence',
      type: 'passover',
      trigger: 'reflectionThreshold.reached = true',
      effect: 'beingAsEssence.explicit = true',
      targetState: 'becoming-essence-4',
    },
  ],
  transitions: [
    {
      id: 'becoming-essence-3-trans-1',
      from: 'becoming-essence-3',
      to: 'becoming-essence-4',
      mechanism: 'passover',
      description:
        'From manifest self-sublation to being determined as essence',
    },
  ],
  nextStates: ['becoming-essence-4'],
  previousStates: ['becoming-essence-2'],
  provenance: {
    topicMapId: 'be-c-2-self-sublation-manifested',
    lineRange: { start: 29, end: 57 },
    section: 'C. TRANSITION INTO ESSENCE',
    order: 2,
  },
  description: teEntryById('be-c-2-self-sublation-manifested')?.description,
  keyPoints: teEntryById('be-c-2-self-sublation-manifested')?.keyPoints,
};

const state2: DialecticState = {
  id: 'becoming-essence-4',
  title: 'Being determined as essence',
  concept: 'TransitionIntoEssence',
  phase: 'quantity',
  moments: [
    {
      name: 'beingDeterminedAsEssence',
      definition:
        'Result of measure’s self-sublation: being is determined as inward reflection',
      type: 'determination',
    },
    {
      name: 'reflectiveMediation',
      definition:
        'What was immediate is now mediated through itself as reflective ground',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'beingDeterminedAsEssence',
    },
    {
      name: 'essenceOpening',
      definition: 'The measure chapter closes by opening the Essence chapter',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'becoming-essence-4-inv-1',
      constraint: 'result is no longer immediate being',
      predicate: 'not(immediateBeing(result))',
    },
    {
      id: 'becoming-essence-4-inv-2',
      constraint: 'result has form of reflection-within-itself',
      predicate: 'reflectionWithinItself(result)',
    },
  ],
  forces: [
    {
      id: 'becoming-essence-4-force-1',
      description: 'Determinate transition introduces essence proper',
      type: 'passover',
      trigger: 'beingAsEssence.explicit = true',
      effect: 'essenceChapter.initiated = true',
      targetState: 'ess-1',
    },
  ],
  transitions: [
    {
      id: 'becoming-essence-4-trans-1',
      from: 'becoming-essence-4',
      to: 'ess-1',
      mechanism: 'passover',
      description: 'From transition-into-essence to essence proper',
    },
  ],
  nextStates: ['ess-1'],
  previousStates: ['becoming-essence-3'],
  provenance: {
    topicMapId: 'be-c-4-being-determined-as-essence',
    lineRange: { start: 78, end: 91 },
    section: 'C. TRANSITION INTO ESSENCE',
    order: 4,
  },
  description: teEntryById('be-c-4-being-determined-as-essence')?.description,
  keyPoints: teEntryById('be-c-4-being-determined-as-essence')?.keyPoints,
};

export const transitionEssenceIR: DialecticIR = {
  id: 'transition-essence-ir',
  title: 'Transition Essence IR',
  section: 'BEING - MEASURE - C. Becoming of Essence - Transition into Essence',
  states: [state1, state2],
  metadata: {
    sourceFile: 'transition-essence.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'becoming-essence-3': 'quantity',
      'becoming-essence-4': 'quantity',
    },
  },
};

export const transitionEssenceStates = {
  'becoming-essence-3': state1,
  'becoming-essence-4': state2,
};
