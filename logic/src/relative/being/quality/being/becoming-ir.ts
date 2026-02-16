import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { becomingTopicMap } from './sources/becoming-topic-map';

const state1: DialecticState = {
  id: 'becoming-1',
  title: 'Unity of being and nothing as immediate vanishing',
  concept: 'BecomingUnity',
  phase: 'quality',
  moments: [
    {
      name: 'sameYetDistinct',
      definition:
        'Being and nothing are absolutely distinct yet inseparable and immediately vanish into one another',
      type: 'mediation',
    },
    {
      name: 'immediateTransition',
      definition:
        'Their truth is not either side but the movement of passing-over',
      type: 'process',
      relation: 'contains',
      relatedTo: 'sameYetDistinct',
    },
  ],
  invariants: [
    {
      id: 'becoming-1-inv-1',
      constraint: 'truth is movement, not isolated being or isolated nothing',
      predicate: 'truthAsMovement(being, nothing)',
    },
    {
      id: 'becoming-1-inv-2',
      constraint:
        'distinction and inseparability are simultaneously maintained',
      predicate: 'distinctAndInseparable(being, nothing)',
    },
  ],
  forces: [
    {
      id: 'becoming-1-force-1',
      description:
        'Immediate vanishing requires articulation as concrete interpenetrating moments',
      type: 'mediation',
      trigger: 'movement.requiresArticulation = true',
      effect: 'interpenetratingMoments.formed = true',
      targetState: 'becoming-4',
    },
  ],
  transitions: [
    {
      id: 'becoming-1-trans-1',
      from: 'becoming-1',
      to: 'becoming-4',
      mechanism: 'mediation',
      description:
        'From abstract unity to determinate coming-to-be and ceasing-to-be',
    },
  ],
  nextStates: ['becoming-4'],
  previousStates: ['nothing-6'],
  provenance: {
    topicMapId: 'becoming-1',
    lineRange: { start: 11, end: 30 },
    section: 'C.1. Unity of being and nothing',
    order: 1,
  },
  description: becomingTopicMap[0]?.description,
  keyPoints: becomingTopicMap[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'becoming-4',
  title: 'Interpenetration of coming-to-be and ceasing-to-be',
  concept: 'InterpenetratingBecoming',
  phase: 'quality',
  moments: [
    {
      name: 'doubleDirection',
      definition:
        'Coming-to-be and ceasing-to-be are opposed directions that interpenetrate and paralyze each other',
      type: 'process',
    },
    {
      name: 'selfSublation',
      definition:
        'Each side sublates itself internally and thus contains its opposite in itself',
      type: 'sublation',
      relation: 'transitions',
      relatedTo: 'doubleDirection',
    },
  ],
  invariants: [
    {
      id: 'becoming-4-inv-1',
      constraint: 'each direction includes its opposite as internal moment',
      predicate: 'internalOpposition(comingToBe, ceasingToBe)',
    },
    {
      id: 'becoming-4-inv-2',
      constraint: 'becoming remains one process across both directions',
      predicate: 'singleProcess(becoming)',
    },
  ],
  forces: [
    {
      id: 'becoming-4-force-1',
      description:
        'Self-sublation collapses unrest into a quiescent result that can stand as immediate',
      type: 'sublation',
      trigger: 'selfSublation.complete = true',
      effect: 'quiescentResult.emerges = true',
      targetState: 'becoming-7',
    },
  ],
  transitions: [
    {
      id: 'becoming-4-trans-1',
      from: 'becoming-4',
      to: 'becoming-7',
      mechanism: 'sublation',
      description: 'From interpenetrating unrest to quiescent simplicity',
    },
  ],
  nextStates: ['becoming-7'],
  previousStates: ['becoming-1'],
  provenance: {
    topicMapId: 'becoming-4',
    lineRange: { start: 62, end: 82 },
    section: 'C.2. The moments of becoming',
    order: 2,
  },
  description: becomingTopicMap[3]?.description,
  keyPoints: becomingTopicMap[3]?.keyPoints,
};

const state3: DialecticState = {
  id: 'becoming-7',
  title: 'Quiescent simplicity as transition to existence',
  concept: 'ExistentResult',
  phase: 'quality',
  moments: [
    {
      name: 'quiescentUnity',
      definition:
        'The unity of being and nothing becomes simple and stands as immediate determination',
      type: 'determination',
    },
    {
      name: 'existentShape',
      definition: 'This immediate unity has the form of existence',
      type: 'quality',
      relation: 'transitions',
      relatedTo: 'quiescentUnity',
    },
  ],
  invariants: [
    {
      id: 'becoming-7-inv-1',
      constraint: 'result is not relapse into pure being or pure nothing',
      predicate: 'notRelapse(result, pureBeing, pureNothing)',
    },
  ],
  forces: [
    {
      id: 'becoming-7-force-1',
      description:
        'The quiescent unity passes over into the chapter of existence',
      type: 'passover',
      trigger: 'unity.asImmediate = true',
      effect: 'existenceChapter.initiated = true',
      targetState: 'existence-1',
    },
  ],
  transitions: [
    {
      id: 'becoming-7-trans-1',
      from: 'becoming-7',
      to: 'existence-1',
      mechanism: 'passover',
      description: 'Becoming sublates into existence',
    },
  ],
  nextStates: ['existence-1'],
  previousStates: ['becoming-4'],
  provenance: {
    topicMapId: 'becoming-7',
    lineRange: { start: 101, end: 108 },
    section: 'C.3. Sublation of becoming',
    order: 3,
  },
  description: becomingTopicMap[6]?.description,
  keyPoints: becomingTopicMap[6]?.keyPoints,
};

export const becomingIR: DialecticIR = {
  id: 'becoming-ir',
  title: 'Becoming IR: Unity of Being and Nothing',
  section: 'BEING - QUALITY - C. Becoming',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'becoming.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'becoming-1': 'quality',
      'becoming-4': 'quality',
      'becoming-7': 'quality',
    },
  },
};

export const becomingStateMap = {
  'becoming-1': state1,
  'becoming-4': state2,
  'becoming-7': state3,
};
