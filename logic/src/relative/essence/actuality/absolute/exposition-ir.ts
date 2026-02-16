import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { EXPOSITION_TOPIC_MAP } from './sources/exposition-topic-map';

const state1: DialecticState = {
  id: 'exp-1',
  title: 'Absolute not being nor essence — absolute unity',
  concept: 'AbsoluteUnity',
  phase: 'appearance',
  moments: [
    {
      name: 'absoluteUnity',
      definition:
        'Absolute as unity of being and essence, ground of essential relation',
      type: 'determination',
    },
    {
      name: 'innerOuterIdentity',
      definition: 'Identity of inner and outer in one substantial ground',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'absoluteUnity',
    },
  ],
  invariants: [
    {
      id: 'exp-1-inv-1',
      constraint: 'absolute ≠ pureBeing ∧ absolute ≠ pureEssence',
      predicate:
        'and(not(equals(absolute, pureBeing)), not(equals(absolute, pureEssence)))',
    },
    {
      id: 'exp-1-inv-2',
      constraint: 'absolute = unity(being, essence)',
      predicate: 'equals(absolute, unity(being, essence))',
    },
  ],
  forces: [
    {
      id: 'exp-1-force-1',
      description:
        'Absolute unity drives toward explicit form/content articulation',
      type: 'mediation',
      trigger: 'absolute.unity = explicit',
      effect: 'absoluteFormContent.emerges = true',
      targetState: 'exp-4',
    },
  ],
  transitions: [
    {
      id: 'exp-1-trans-1',
      from: 'exp-1',
      to: 'exp-4',
      mechanism: 'mediation',
      description: 'From absolute unity to form/content exposition',
    },
  ],
  nextStates: ['exp-4'],
  previousStates: [],
  provenance: {
    topicMapId: 'abs-a-1-absolute-not-being-nor-essence',
    lineRange: { start: 3, end: 22 },
    section: 'The Exposition of the Absolute',
    order: 1,
  },
  description: EXPOSITION_TOPIC_MAP.entries[0]?.description,
  keyPoints: EXPOSITION_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'exp-4',
  title: 'Absolute does not determine itself — movement stands over',
  concept: 'ExpositoryNegativity',
  phase: 'appearance',
  moments: [
    {
      name: 'selfIdentity',
      definition:
        'Absolute as simple self-identity that does not self-determine as difference',
      type: 'determination',
    },
    {
      name: 'standingReflection',
      definition:
        'Movement of reflection stands over against absolute identity',
      type: 'reflection',
      relation: 'opposite',
      relatedTo: 'selfIdentity',
    },
  ],
  invariants: [
    {
      id: 'exp-4-inv-1',
      constraint: 'absolute.identity = simple',
      predicate: 'equals(absolute.identity, simple)',
    },
    {
      id: 'exp-4-inv-2',
      constraint: 'reflection.standsOver = absoluteIdentity',
      predicate: 'standsOver(reflection, absoluteIdentity)',
    },
  ],
  forces: [
    {
      id: 'exp-4-force-1',
      description: 'Standing reflection drives to explicit positive exposition',
      type: 'reflection',
      trigger: 'reflection.standsOver = true',
      effect: 'positiveExposition.emerges = true',
      targetState: 'exp-7',
    },
  ],
  transitions: [
    {
      id: 'exp-4-trans-1',
      from: 'exp-4',
      to: 'exp-7',
      mechanism: 'reflection',
      description: 'From standing reflection to positive exposition',
    },
  ],
  nextStates: ['exp-7'],
  previousStates: ['exp-1'],
  provenance: {
    topicMapId: 'abs-a-4-absolute-does-not-determine',
    lineRange: { start: 52, end: 75 },
    section: 'The Exposition of the Absolute',
    order: 4,
  },
  description: EXPOSITION_TOPIC_MAP.entries[3]?.description,
  keyPoints: EXPOSITION_TOPIC_MAP.entries[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'exp-7',
  title: 'Positive exposition as reflective shine — relative absolute',
  concept: 'RelativeAbsoluteAsAttribute',
  phase: 'appearance',
  moments: [
    {
      name: 'reflectiveShine',
      definition: 'Positive exposition as reflective shine of the absolute',
      type: 'reflection',
    },
    {
      name: 'relativeAbsolute',
      definition:
        'Absolute in determination as attribute (not yet absolutely absolute)',
      type: 'determination',
      relation: 'transforms',
      relatedTo: 'reflectiveShine',
    },
  ],
  invariants: [
    {
      id: 'exp-7-inv-1',
      constraint: 'exposition = absoluteOwnDoing',
      predicate: 'equals(exposition, absoluteOwnDoing)',
    },
    {
      id: 'exp-7-inv-2',
      constraint: 'absoluteIdentityInDetermination = attribute',
      predicate: 'equals(absoluteIdentityInDetermination, attribute)',
    },
  ],
  forces: [
    {
      id: 'exp-7-force-1',
      description: 'Relative absolute drives toward explicit attribute logic',
      type: 'passover',
      trigger: 'absolute.inDetermination = true',
      effect: 'attribute.emerges = true',
      targetState: 'att-1',
    },
  ],
  transitions: [
    {
      id: 'exp-7-trans-1',
      from: 'exp-7',
      to: 'att-1',
      mechanism: 'passover',
      description: 'From exposition to attribute of the absolute',
    },
  ],
  nextStates: ['att-1'],
  previousStates: ['exp-4'],
  provenance: {
    topicMapId: 'abs-a-7-positive-exposition-reflective-shine',
    lineRange: { start: 126, end: 167 },
    section: 'The Exposition of the Absolute',
    order: 7,
  },
  description: EXPOSITION_TOPIC_MAP.entries[6]?.description,
  keyPoints: EXPOSITION_TOPIC_MAP.entries[6]?.keyPoints,
};

export const expositionIR: DialecticIR = {
  id: 'exposition-ir',
  title:
    'Exposition IR: Absolute Unity, Negative/Positive Exposition, Relative Absolute',
  section: 'ESSENCE - C. ACTUALITY - C. The Absolute - a. The Exposition',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'exposition.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'exp-1': 'appearance',
      'exp-4': 'appearance',
      'exp-7': 'appearance',
    },
  },
};

export const expositionStates = {
  'exp-1': state1,
  'exp-4': state2,
  'exp-7': state3,
};
