import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { MODE_TOPIC_MAP } from './sources/mode-topic-map';

const state1: DialecticState = {
  id: 'mod-1',
  title: 'Attribute two sides — extremes and middle term',
  concept: 'ModeEmergence',
  phase: 'appearance',
  moments: [
    {
      name: 'attributeExtremes',
      definition: 'Absolute identity and negative reflection as two extremes',
      type: 'determination',
    },
    {
      name: 'modeAsSelfExternality',
      definition:
        'Mode as self-externality where negativity is posited outwardly',
      type: 'externality',
      relation: 'transforms',
      relatedTo: 'attributeExtremes',
    },
  ],
  invariants: [
    {
      id: 'mod-1-inv-1',
      constraint: 'mode = selfExternality(absolute)',
      predicate: 'equals(mode, selfExternality(absolute))',
    },
    {
      id: 'mod-1-inv-2',
      constraint: 'attribute = middleTerm(absolute, determinateness)',
      predicate: 'equals(attribute, middleTerm(absolute, determinateness))',
    },
  ],
  forces: [
    {
      id: 'mod-1-force-1',
      description: 'Mode externality drives toward immanent turning back',
      type: 'mediation',
      trigger: 'mode.externality = posited',
      effect: 'immanentTurningBack.emerges = true',
      targetState: 'mod-5',
    },
  ],
  transitions: [
    {
      id: 'mod-1-trans-1',
      from: 'mod-1',
      to: 'mod-5',
      mechanism: 'mediation',
      description: 'From externality-posited mode to immanent turning back',
    },
  ],
  nextStates: ['mod-5'],
  previousStates: ['att-6'],
  provenance: {
    topicMapId: 'abs-c-1-attribute-two-sides',
    lineRange: { start: 3, end: 23 },
    section: 'The Mode of the Absolute',
    order: 1,
  },
  description: MODE_TOPIC_MAP.entries[0]?.description,
  keyPoints: MODE_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'mod-5',
  title: 'Mode as immanent turning back — absolute being',
  concept: 'ImmanentMode',
  phase: 'appearance',
  moments: [
    {
      name: 'immanentTurningBack',
      definition: 'Mode as reflective shine that turns back into itself',
      type: 'reflection',
    },
    {
      name: 'absoluteBeing',
      definition:
        'Absolute as being precisely in this self-dissolving reflection',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'immanentTurningBack',
    },
  ],
  invariants: [
    {
      id: 'mod-5-inv-1',
      constraint: 'mode ≠ mereExternality',
      predicate: 'not(equals(mode, mereExternality))',
    },
    {
      id: 'mod-5-inv-2',
      constraint: 'absoluteBeing = reflectionAsTurningBack',
      predicate: 'equals(absoluteBeing, reflectionAsTurningBack)',
    },
  ],
  forces: [
    {
      id: 'mod-5-force-1',
      description: 'Immanent mode drives toward absolute manifestation',
      type: 'reflection',
      trigger: 'mode.immanentTurningBack = true',
      effect: 'absoluteManifestation.emerges = true',
      targetState: 'mod-9',
    },
  ],
  transitions: [
    {
      id: 'mod-5-trans-1',
      from: 'mod-5',
      to: 'mod-9',
      mechanism: 'reflection',
      description: 'From immanent turning back to absolute manifestation',
    },
  ],
  nextStates: ['mod-9'],
  previousStates: ['mod-1'],
  provenance: {
    topicMapId: 'abs-c-5-mode-immanent-turning-back',
    lineRange: { start: 49, end: 54 },
    section: 'The Mode of the Absolute',
    order: 5,
  },
  description: MODE_TOPIC_MAP.entries[4]?.description,
  keyPoints: MODE_TOPIC_MAP.entries[4]?.keyPoints,
};

const state3: DialecticState = {
  id: 'mod-9',
  title: 'Absolute as actuality — absolute manifestation',
  concept: 'AbsoluteManifestationAsActuality',
  phase: 'appearance',
  moments: [
    {
      name: 'absoluteManifestation',
      definition: 'Absolute as expression and manifestation for itself',
      type: 'appearance',
    },
    {
      name: 'actuality',
      definition: 'Way and manner coincide with absolute identity as actuality',
      type: 'determination',
      relation: 'unified',
      relatedTo: 'absoluteManifestation',
    },
  ],
  invariants: [
    {
      id: 'mod-9-inv-1',
      constraint: 'content = expositionItself',
      predicate: 'equals(content, expositionItself)',
    },
    {
      id: 'mod-9-inv-2',
      constraint: 'absoluteManifestation = actuality',
      predicate: 'equals(absoluteManifestation, actuality)',
    },
  ],
  forces: [
    {
      id: 'mod-9-force-1',
      description: 'Absolute actuality passes to substantial relation',
      type: 'passover',
      trigger: 'actuality.absolute = true',
      effect: 'formalActuality.emerges = true',
      targetState: 'act-a-1',
    },
  ],
  transitions: [
    {
      id: 'mod-9-trans-1',
      from: 'mod-9',
      to: 'act-a-1',
      mechanism: 'passover',
      description: 'From mode of absolute to contingency/formal actuality',
    },
  ],
  nextStates: ['act-a-1'],
  previousStates: ['mod-5'],
  provenance: {
    topicMapId: 'abs-c-9-absolute-as-actuality',
    lineRange: { start: 106, end: 114 },
    section: 'The Mode of the Absolute',
    order: 9,
  },
  description: MODE_TOPIC_MAP.entries[8]?.description,
  keyPoints: MODE_TOPIC_MAP.entries[8]?.keyPoints,
};

export const modeIR: DialecticIR = {
  id: 'mode-ir',
  title:
    'Mode IR: Externality Posited, Immanent Turning Back, Absolute Manifestation',
  section: 'ESSENCE - C. ACTUALITY - C. The Absolute - c. The Mode',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'mode.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'mod-1': 'appearance',
      'mod-5': 'appearance',
      'mod-9': 'appearance',
    },
  },
};

export const modeStates = {
  'mod-1': state1,
  'mod-5': state2,
  'mod-9': state3,
};
