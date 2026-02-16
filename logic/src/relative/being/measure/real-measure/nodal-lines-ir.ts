import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { NODAL_LINES_TOPIC_MAP } from './sources/nodal-lines-topic-map';

const nEntryById = (id: string) =>
  NODAL_LINES_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'real-measure-2',
  title: 'Nodal continuity of exclusive measures',
  concept: 'NodalContinuity',
  phase: 'quantity',
  moments: [
    {
      name: 'exclusiveMeasureContinuity',
      definition:
        'Measures form a continuous series while each remains exclusive and self-referential',
      type: 'process',
    },
    {
      name: 'qualitativeFoundation',
      definition:
        'Continuity rests on qualitative self-reference that grounds quantitative progression',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'exclusiveMeasureContinuity',
    },
    {
      name: 'selfRepellingMeasure',
      definition:
        'Measure repels itself into differentiated nodes within continuity',
      type: 'negation',
    },
  ],
  invariants: [
    {
      id: 'real-measure-2-inv-1',
      constraint: 'continuity and exclusivity coexist in nodal measure',
      predicate: 'unity(continuity, exclusivity)',
    },
    {
      id: 'real-measure-2-inv-2',
      constraint: 'quantitative progression bears qualitative articulation',
      predicate: 'qualitativelyArticulatedProgression(measure)',
    },
  ],
  forces: [
    {
      id: 'real-measure-2-force-1',
      description: 'Nodal articulation necessitates explicit qualitative leap',
      type: 'contradiction',
      trigger: 'continuity.reachesNode = true',
      effect: 'qualitativeLeap.explicit = true',
      targetState: 'real-measure-2-leap',
    },
  ],
  transitions: [
    {
      id: 'real-measure-2-trans-1',
      from: 'real-measure-2',
      to: 'real-measure-2-leap',
      mechanism: 'contradiction',
      description: 'From nodal continuity to qualitative leap',
    },
  ],
  nextStates: ['real-measure-2-leap'],
  previousStates: ['real-measure-1-exponent'],
  provenance: {
    topicMapId: 'rm-b-1-exclusive-measure-continuity',
    lineRange: { start: 3, end: 33 },
    section: 'Nodal Lines of Measure-Relations',
    order: 1,
  },
  description: nEntryById('rm-b-1-exclusive-measure-continuity')?.description,
  keyPoints: nEntryById('rm-b-1-exclusive-measure-continuity')?.keyPoints,
};

const state2: DialecticState = {
  id: 'real-measure-2-leap',
  title: 'Qualitative leap and the limit of gradualness',
  concept: 'NodalLeap',
  phase: 'quantity',
  moments: [
    {
      name: 'qualitativeLeap',
      definition:
        'At nodal points, quantitative progression turns into qualitative jump',
      type: 'sublation',
    },
    {
      name: 'gradualnessNegated',
      definition:
        'Pure gradual transition proves insufficient to account for determinate change',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'qualitativeLeap',
    },
    {
      name: 'progressiveSeries',
      definition:
        'Series of leaps opens a progressive chain beyond fixed measure-states',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'real-measure-2-leap-inv-1',
      constraint: 'leap is intrinsic to nodal measure, not accidental rupture',
      predicate: 'intrinsicLeap(nodalMeasure)',
    },
    {
      id: 'real-measure-2-leap-inv-2',
      constraint: 'gradualness and leap are dialectically linked',
      predicate: 'linked(gradualness, leap)',
    },
  ],
  forces: [
    {
      id: 'real-measure-2-leap-force-1',
      description:
        'Progressive leap-series drives measure into the measureless movement',
      type: 'passover',
      trigger: 'progressiveSeries.unbounded = true',
      effect: 'measurelessMovement.explicit = true',
      targetState: 'real-measure-3',
    },
  ],
  transitions: [
    {
      id: 'real-measure-2-leap-trans-1',
      from: 'real-measure-2-leap',
      to: 'real-measure-3',
      mechanism: 'passover',
      description: 'From nodal leap to measureless movement',
    },
  ],
  nextStates: ['real-measure-3'],
  previousStates: ['real-measure-2'],
  provenance: {
    topicMapId: 'rm-b-4-qualitative-leap',
    lineRange: { start: 68, end: 100 },
    section: 'Nodal Lines of Measure-Relations',
    order: 4,
  },
  description: nEntryById('rm-b-4-qualitative-leap')?.description,
  keyPoints: nEntryById('rm-b-4-qualitative-leap')?.keyPoints,
};

export const nodalLinesIR: DialecticIR = {
  id: 'nodal-lines-ir',
  title: 'Nodal Lines IR: Measure-Relations and Leaps',
  section: 'BEING - MEASURE - B. Real Measure - Nodal Lines',
  states: [state1, state2],
  metadata: {
    sourceFile: 'nodal-lines.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'real-measure-2': 'quantity',
      'real-measure-2-leap': 'quantity',
    },
  },
};

export const nodalLinesStates = {
  'real-measure-2': state1,
  'real-measure-2-leap': state2,
};
