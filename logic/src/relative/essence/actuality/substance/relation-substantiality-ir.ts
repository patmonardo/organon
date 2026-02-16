import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { RELATION_SUBSTANTIALITY_TOPIC_MAP } from './sources/relation-substantiality-topic-map';

const state1: DialecticState = {
  id: 'sub-a-1',
  title: 'Substance as being that is because it is',
  concept: 'SubstanceImmediateActuality',
  phase: 'appearance',
  moments: [
    {
      name: 'substance',
      definition: 'Final unity of essence and being as absolute self-mediation',
      type: 'determination',
    },
    {
      name: 'accidentalityShine',
      definition: 'Self-referring shining of positedness in and as substance',
      type: 'reflection',
      relation: 'contains',
      relatedTo: 'substance',
    },
  ],
  invariants: [
    {
      id: 'sub-a-1-inv-1',
      constraint: 'substance is immediate actuality reflected into itself',
      predicate: 'and(immediate(actuality), reflectedIntoItself(substance))',
    },
    {
      id: 'sub-a-1-inv-2',
      constraint: 'being and reflection are united in substance',
      predicate: 'equals(unity(being, reflection), substance)',
    },
  ],
  forces: [
    {
      id: 'sub-a-1-force-1',
      description: 'Substance unfolds as totality and power in accidentality',
      type: 'appearance',
      trigger: 'accidentality.manifoldness = explicit',
      effect: 'substantialPower.emerges = true',
      targetState: 'sub-a-4',
    },
  ],
  transitions: [
    {
      id: 'sub-a-1-trans-1',
      from: 'sub-a-1',
      to: 'sub-a-4',
      mechanism: 'appearance',
      description: 'From immediate substance to totality/power of accidents',
    },
  ],
  nextStates: ['sub-a-4'],
  previousStates: ['act-c-11', 'mod-9'],
  provenance: {
    topicMapId: 'sub-a-1-substance-being',
    lineRange: { start: 3, end: 23 },
    section: 'The Relation of Substantiality',
    order: 1,
  },
  description: RELATION_SUBSTANTIALITY_TOPIC_MAP.entries[0]?.description,
  keyPoints: RELATION_SUBSTANTIALITY_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'sub-a-4',
  title: 'Substance as totality and absolute power',
  concept: 'SubstantialPower',
  phase: 'appearance',
  moments: [
    {
      name: 'totalityOfAccidentality',
      definition: 'Substance embraces accidentality as its own whole',
      type: 'determination',
    },
    {
      name: 'creativeDestructivePower',
      definition:
        'Substance creates through actuality and destroys through possibility',
      type: 'process',
      relation: 'transforms',
      relatedTo: 'totalityOfAccidentality',
    },
  ],
  invariants: [
    {
      id: 'sub-a-4-inv-1',
      constraint: 'creating and destroying are one substantial activity',
      predicate: 'equals(creating, destroying)',
    },
    {
      id: 'sub-a-4-inv-2',
      constraint: 'possible and actual are differently determined same content',
      predicate: 'sameContent(possible, actual)',
    },
  ],
  forces: [
    {
      id: 'sub-a-4-force-1',
      description:
        'Formal power reveals insufficiency of substantiality relation',
      type: 'negation',
      trigger: 'accidents.haveNoOwnPower = true',
      effect: 'causalForm.emerges = true',
      targetState: 'sub-a-7',
    },
  ],
  transitions: [
    {
      id: 'sub-a-4-trans-1',
      from: 'sub-a-4',
      to: 'sub-a-7',
      mechanism: 'negation',
      description: 'From substantial power to passover into causality',
    },
  ],
  nextStates: ['sub-a-7'],
  previousStates: ['sub-a-1'],
  provenance: {
    topicMapId: 'sub-a-4-substance-totality-power',
    lineRange: { start: 69, end: 101 },
    section: 'The Relation of Substantiality',
    order: 4,
  },
  description: RELATION_SUBSTANTIALITY_TOPIC_MAP.entries[3]?.description,
  keyPoints: RELATION_SUBSTANTIALITY_TOPIC_MAP.entries[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'sub-a-7',
  title: 'Substantiality passes over into causality',
  concept: 'SubstantialityToCausality',
  phase: 'appearance',
  moments: [
    {
      name: 'formalPower',
      definition: 'Substance manifests only as formal power in accidentality',
      type: 'appearance',
    },
    {
      name: 'causalPassover',
      definition: 'Substance now posits itself as cause/effect structure',
      type: 'passover',
      relation: 'passesOver',
      relatedTo: 'formalPower',
    },
  ],
  invariants: [
    {
      id: 'sub-a-7-inv-1',
      constraint: 'substance and accidents mutually imply each other',
      predicate: 'and(in(substance, accidents), in(accidents, substance))',
    },
    {
      id: 'sub-a-7-inv-2',
      constraint: 'differences are not yet substantial in themselves',
      predicate: 'not(substantial(differences))',
    },
  ],
  forces: [
    {
      id: 'sub-a-7-force-1',
      description: 'Formal power crystallizes into cause/effect relation',
      type: 'passover',
      trigger: 'substance.positedAsPower = formal',
      effect: 'causeEffect.relation = explicit',
      targetState: 'sub-b-1',
    },
  ],
  transitions: [
    {
      id: 'sub-a-7-trans-1',
      from: 'sub-a-7',
      to: 'sub-b-1',
      mechanism: 'passover',
      description: 'From relation of substantiality to relation of causality',
    },
  ],
  nextStates: ['sub-b-1'],
  previousStates: ['sub-a-4'],
  provenance: {
    topicMapId: 'sub-a-7-substance-not-substance',
    lineRange: { start: 157, end: 178 },
    section: 'The Relation of Substantiality',
    order: 7,
  },
  description: RELATION_SUBSTANTIALITY_TOPIC_MAP.entries[6]?.description,
  keyPoints: RELATION_SUBSTANTIALITY_TOPIC_MAP.entries[6]?.keyPoints,
};

export const relationSubstantialityIR: DialecticIR = {
  id: 'relation-substantiality-ir',
  title: 'Relation Substantiality IR: Substance, Power, Causal Passover',
  section: 'ESSENCE - C. ACTUALITY - Absolute Relation - a. Substantiality',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'relation-substantiality.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'sub-a-1': 'appearance',
      'sub-a-4': 'appearance',
      'sub-a-7': 'appearance',
    },
  },
};

export const relationSubstantialityStates = {
  'sub-a-1': state1,
  'sub-a-4': state2,
  'sub-a-7': state3,
};
