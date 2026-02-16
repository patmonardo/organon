import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { MEASURELESS_TOPIC_MAP } from './sources/measureless-topic-map';

const mlEntryById = (id: string) =>
  MEASURELESS_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'real-measure-3',
  title: 'Measureless progression and substantive persistence',
  concept: 'MeasurelessProgression',
  phase: 'quantity',
  moments: [
    {
      name: 'measurelessDrive',
      definition:
        'Measure is driven into a progression that exceeds any fixed finite measure-state',
      type: 'process',
    },
    {
      name: 'substantialMatter',
      definition:
        'Beneath changing measures persists a substantial self-sameness',
      type: 'determination',
      relation: 'contains',
      relatedTo: 'measurelessDrive',
    },
    {
      name: 'indifferenceThreshold',
      definition:
        'Progressive determination culminates in indifference as threshold to essence',
      type: 'mediation',
    },
  ],
  invariants: [
    {
      id: 'real-measure-3-inv-1',
      constraint: 'measureless progression remains determinate',
      predicate: 'determinateProgression(measureless)',
    },
    {
      id: 'real-measure-3-inv-2',
      constraint: 'substance persists through quantitative difference',
      predicate: 'persistence(substance)',
    },
  ],
  forces: [
    {
      id: 'real-measure-3-force-1',
      description: 'Measureless culmination passes over into becoming essence',
      type: 'passover',
      trigger: 'indifferenceThreshold.reached = true',
      effect: 'becomingEssence.initiated = true',
      targetState: 'becoming-essence-1',
    },
  ],
  transitions: [
    {
      id: 'real-measure-3-trans-1',
      from: 'real-measure-3',
      to: 'becoming-essence-1',
      mechanism: 'passover',
      description: 'From measureless progression to becoming essence',
    },
  ],
  nextStates: ['becoming-essence-1'],
  previousStates: ['real-measure-2-leap'],
  provenance: {
    topicMapId: 'rm-c-8-progressive-determination-summary',
    lineRange: { start: 132, end: 174 },
    section: 'The Measureless',
    order: 8,
  },
  description: mlEntryById('rm-c-8-progressive-determination-summary')
    ?.description,
  keyPoints: mlEntryById('rm-c-8-progressive-determination-summary')?.keyPoints,
};

export const measurelessIR: DialecticIR = {
  id: 'measureless-ir',
  title: 'Measureless IR: Progressive Determination',
  section: 'BEING - MEASURE - B. Real Measure - The Measureless',
  states: [state1],
  metadata: {
    sourceFile: 'measureless.txt',
    totalStates: 1,
    cpuGpuMapping: {
      'real-measure-3': 'quantity',
    },
  },
};

export const measurelessStates = {
  'real-measure-3': state1,
};
