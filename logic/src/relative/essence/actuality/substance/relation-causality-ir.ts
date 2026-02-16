import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { RELATION_CAUSALITY_TOPIC_MAP } from './sources/relation-causality-topic-map';

const state1: DialecticState = {
  id: 'sub-b-1',
  title: 'Substance as power posits cause and effect',
  concept: 'FormalCausality',
  phase: 'appearance',
  moments: [
    {
      name: 'cause',
      definition: 'Substance existing for itself as originative determination',
      type: 'determination',
    },
    {
      name: 'effect',
      definition: 'Sublated substantiality as positedness',
      type: 'negation',
      relation: 'opposite',
      relatedTo: 'cause',
    },
  ],
  invariants: [
    {
      id: 'sub-b-1-inv-1',
      constraint: 'cause and effect define formal causality',
      predicate: 'equals(formalCausality, relation(cause, effect))',
    },
    {
      id: 'sub-b-1-inv-2',
      constraint: 'cause is substance reflected into itself',
      predicate: 'equals(cause, reflectedPower(substance))',
    },
  ],
  forces: [
    {
      id: 'sub-b-1-force-1',
      description: 'Identity pressure extinguishes cause/effect distinction',
      type: 'reflection',
      trigger: 'cause.actualityInEffect = explicit',
      effect: 'causalDistinction.sublated = true',
      targetState: 'sub-b-5',
    },
  ],
  transitions: [
    {
      id: 'sub-b-1-trans-1',
      from: 'sub-b-1',
      to: 'sub-b-5',
      mechanism: 'reflection',
      description: 'From formal cause/effect to extinguished identity',
    },
  ],
  nextStates: ['sub-b-5'],
  previousStates: ['sub-a-7'],
  provenance: {
    topicMapId: 'sub-b-1-substance-power-cause',
    lineRange: { start: 3, end: 15 },
    section: 'The Relation of Causality',
    order: 1,
  },
  description: RELATION_CAUSALITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: RELATION_CAUSALITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'sub-b-5',
  title: 'Cause and effect identity extinguished into immediacy',
  concept: 'CausalityExtinguished',
  phase: 'appearance',
  moments: [
    {
      name: 'extinguishedDifference',
      definition: 'Cause and effect lose formal distinction in their identity',
      type: 'sublation',
    },
    {
      name: 'indifferentImmediacy',
      definition: 'Causality appears as immediate and externally contingent',
      type: 'externality',
      relation: 'transforms',
      relatedTo: 'extinguishedDifference',
    },
  ],
  invariants: [
    {
      id: 'sub-b-5-inv-1',
      constraint: 'cause extinguishes in effect',
      predicate: 'extinguishedIn(cause, effect)',
    },
    {
      id: 'sub-b-5-inv-2',
      constraint: 'immediacy is indifferent to causal relation',
      predicate: 'indifferentTo(immediacy, causalRelation)',
    },
  ],
  forces: [
    {
      id: 'sub-b-5-force-1',
      description: 'Finite determinate causality exposes contingent side-being',
      type: 'contradiction',
      trigger: 'content.formDifference = external',
      effect: 'reciprocalCausality.prepared = true',
      targetState: 'sub-b-9',
    },
  ],
  transitions: [
    {
      id: 'sub-b-5-trans-1',
      from: 'sub-b-5',
      to: 'sub-b-9',
      mechanism: 'contradiction',
      description:
        'From extinguished formal causality to contingent side-being',
    },
  ],
  nextStates: ['sub-b-9'],
  previousStates: ['sub-b-1'],
  provenance: {
    topicMapId: 'sub-b-5-identity-extinguished',
    lineRange: { start: 135, end: 146 },
    section: 'The Relation of Causality',
    order: 5,
  },
  description: RELATION_CAUSALITY_TOPIC_MAP.entries[4]?.description,
  keyPoints: RELATION_CAUSALITY_TOPIC_MAP.entries[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'sub-b-9',
  title: 'Contingent side-being drives to reciprocity',
  concept: 'FiniteCausalityLimit',
  phase: 'appearance',
  moments: [
    {
      name: 'contingentSideBeing',
      definition:
        'Cause includes contingent factors not entering causal identity',
      type: 'appearance',
    },
    {
      name: 'causalInsufficiency',
      definition:
        'Single-line causality fails in living and spiritual determination',
      type: 'negation',
      relation: 'negates',
      relatedTo: 'contingentSideBeing',
    },
  ],
  invariants: [
    {
      id: 'sub-b-9-inv-1',
      constraint: 'remote and aggregate causes undermine linear causality',
      predicate: 'undermines(linearCausality, remoteAndAggregateCauses)',
    },
    {
      id: 'sub-b-9-inv-2',
      constraint: 'occasion and stimulus point beyond finite causality',
      predicate: 'pointsBeyond(occasionStimulus, finiteCausality)',
    },
  ],
  forces: [
    {
      id: 'sub-b-9-force-1',
      description: 'Finite causality sublates into reciprocity of action',
      type: 'passover',
      trigger: 'finiteCausality.selfContradiction = explicit',
      effect: 'reciprocity.emerges = true',
      targetState: 'sub-c-1',
    },
  ],
  transitions: [
    {
      id: 'sub-b-9-trans-1',
      from: 'sub-b-9',
      to: 'sub-c-1',
      mechanism: 'passover',
      description: 'From finite causality to reciprocity of action',
    },
  ],
  nextStates: ['sub-c-1'],
  previousStates: ['sub-b-5'],
  provenance: {
    topicMapId: 'sub-b-9-contingent-side-being',
    lineRange: { start: 227, end: 340 },
    section: 'The Relation of Causality',
    order: 9,
  },
  description: RELATION_CAUSALITY_TOPIC_MAP.entries[8]?.description,
  keyPoints: RELATION_CAUSALITY_TOPIC_MAP.entries[8]?.keyPoints,
};

export const relationCausalityIR: DialecticIR = {
  id: 'relation-causality-ir',
  title:
    'Relation Causality IR: Formal Causality, Extinguishing, Reciprocity Passover',
  section: 'ESSENCE - C. ACTUALITY - Absolute Relation - b. Causality',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'relation-causality.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'sub-b-1': 'appearance',
      'sub-b-5': 'appearance',
      'sub-b-9': 'appearance',
    },
  },
};

export const relationCausalityStates = {
  'sub-b-1': state1,
  'sub-b-5': state2,
  'sub-b-9': state3,
};
