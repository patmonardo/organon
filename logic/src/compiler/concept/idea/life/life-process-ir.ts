import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { LIFE_PROCESS_TOPIC_MAP } from './sources/life-process-topic-map';

const state1: DialecticState = {
  id: 'life-process-1',
  title: 'Need and pain as contradiction of life against objectivity',
  concept: 'NeedPainContradiction',
  phase: 'subject',
  moments: [
    {
      name: 'needAsObjectiveContradiction',
      definition:
        'Life opposes itself to an objective world and experiences need as contradiction between self-identity and external otherness',
      type: 'contradiction',
    },
    {
      name: 'painAsActualRupture',
      definition:
        'Pain is the lived actuality of this contradiction, where the concept feels its rupture in finite existence',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'needAsObjectiveContradiction',
    },
  ],
  invariants: [
    {
      id: 'life-process-1-inv-1',
      constraint:
        'need presupposes self-feeling certainty of the nullity of externality',
      predicate: 'presupposesSelfFeeling(need)',
    },
    {
      id: 'life-process-1-inv-2',
      constraint:
        'pain remains intrinsic to living contradiction and not to inert entities',
      predicate: 'intrinsicToLife(pain)',
    },
  ],
  forces: [
    {
      id: 'life-process-1-force-1',
      description:
        'Contradiction drives life to assimilate externality into interiority',
      type: 'sublation',
      trigger: 'need.seeksNegationOfOtherness = true',
      effect: 'assimilationProcess.starts = true',
      targetState: 'life-process-2',
    },
  ],
  transitions: [
    {
      id: 'life-process-1-trans-1',
      from: 'life-process-1',
      to: 'life-process-2',
      mechanism: 'sublation',
      description:
        'From need and pain to active assimilation of the objective world',
    },
  ],
  nextStates: ['life-process-2'],
  previousStates: ['idea-6'],
  provenance: {
    topicMapId: 'life-process-1-need-pain',
    lineRange: { start: 3, end: 66 },
    section: 'The Life-Process',
    order: 1,
  },
  description: LIFE_PROCESS_TOPIC_MAP.entries[0]?.description,
  keyPoints: LIFE_PROCESS_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'life-process-2',
  title: 'Assimilation and reproduction as turning externality inward',
  concept: 'AssimilativeReproduction',
  phase: 'subject',
  moments: [
    {
      name: 'appropriativeAssimilation',
      definition:
        'Life appropriates external objectivity, sublates its independence, and converts it into means of its own reproduction',
      type: 'mediation',
    },
    {
      name: 'selfIdenticalReproduction',
      definition:
        'Production turns into reproduction, preserving life through continual return into itself',
      type: 'reflection',
      relation: 'transitions',
      relatedTo: 'appropriativeAssimilation',
    },
  ],
  invariants: [
    {
      id: 'life-process-2-inv-1',
      constraint:
        'external purposiveness is canceled in immanent vital process',
      predicate: 'sublated(externalPurposiveness)',
    },
    {
      id: 'life-process-2-inv-2',
      constraint:
        'life asserts universality by permeating transformed processes',
      predicate: 'permeatesAsUniversality(life)',
    },
  ],
  forces: [
    {
      id: 'life-process-2-force-1',
      description:
        'Reproductive self-identity elevates individuality to universal life as genus',
      type: 'passover',
      trigger: 'reproduction.rejoinsObjectivityAsSelf = true',
      effect: 'genusMoment.emerges = true',
      targetState: 'life-process-3',
    },
  ],
  transitions: [
    {
      id: 'life-process-2-trans-1',
      from: 'life-process-2',
      to: 'life-process-3',
      mechanism: 'passover',
      description:
        'From assimilation to universalized singularity at the threshold of genus',
    },
  ],
  nextStates: ['life-process-3'],
  previousStates: ['life-process-1'],
  provenance: {
    topicMapId: 'life-process-2-assimilation',
    lineRange: { start: 68, end: 157 },
    section: 'The Life-Process',
    order: 2,
  },
  description: LIFE_PROCESS_TOPIC_MAP.entries[1]?.description,
  keyPoints: LIFE_PROCESS_TOPIC_MAP.entries[1]?.keyPoints,
};

const state3: DialecticState = {
  id: 'life-process-3',
  title: 'Universal life as genus in immediate form',
  concept: 'GenusEmergence',
  phase: 'subject',
  moments: [
    {
      name: 'actualSingularity',
      definition:
        'Through objective process the living being becomes self-identical in otherness as actual singularity',
      type: 'sublation',
    },
    {
      name: 'universalLifeAsGenus',
      definition:
        'The life-process discloses real universal life as genus beyond isolated individual disruption',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'actualSingularity',
    },
  ],
  invariants: [
    {
      id: 'life-process-3-inv-1',
      constraint: 'singularity is universalized through rejoined objectivity',
      predicate: 'universalizedThroughRejoining(singularity)',
    },
    {
      id: 'life-process-3-inv-2',
      constraint: 'genus appears as truth of external life-process',
      predicate: 'truthOfExternalProcess(genus)',
    },
  ],
  forces: [
    {
      id: 'life-process-3-force-1',
      description:
        'Universal life must determine itself explicitly as genus relation',
      type: 'mediation',
      trigger: 'universalLife.requiresExplicitSelfRelation = true',
      effect: 'genusDevelopment.starts = true',
      targetState: 'genus-1',
    },
  ],
  transitions: [
    {
      id: 'life-process-3-trans-1',
      from: 'life-process-3',
      to: 'genus-1',
      mechanism: 'mediation',
      description:
        'From universalized life-process to explicit genus development',
    },
  ],
  nextStates: ['genus-1'],
  previousStates: ['life-process-2'],
  provenance: {
    topicMapId: 'life-process-3-genus',
    lineRange: { start: 159, end: 179 },
    section: 'The Life-Process',
    order: 3,
  },
  description: LIFE_PROCESS_TOPIC_MAP.entries[2]?.description,
  keyPoints: LIFE_PROCESS_TOPIC_MAP.entries[2]?.keyPoints,
};

export const lifeProcessIR: DialecticIR = {
  id: 'life-process-ir',
  title: 'Life-Process IR: Need/Pain, Assimilation, Genus Threshold',
  section: 'CONCEPT - IDEA - A. Life - B. The Life-Process',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'life-process.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'life-process-1': 'subject',
      'life-process-2': 'subject',
      'life-process-3': 'subject',
    },
  },
};

export const lifeProcessStates = {
  'life-process-1': state1,
  'life-process-2': state2,
  'life-process-3': state3,
};
