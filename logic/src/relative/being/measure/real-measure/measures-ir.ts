import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { MEASURES_TOPIC_MAP } from './sources/measures-topic-map';

const mEntryById = (id: string) =>
  MEASURES_TOPIC_MAP.entries.find((entry) => entry.id === id);

const state1: DialecticState = {
  id: 'real-measure-1',
  title: 'Relation of independent measures',
  concept: 'IndependentMeasureRelation',
  phase: 'quantity',
  moments: [
    {
      name: 'selfSubsistentMeasures',
      definition:
        'Measures appear as relatively independent material unities in determinate relation',
      type: 'determination',
    },
    {
      name: 'measureRelationAsDetermining',
      definition:
        'Their relation is itself determinative and not reducible to external juxtaposition',
      type: 'mediation',
      relation: 'contains',
      relatedTo: 'selfSubsistentMeasures',
    },
    {
      name: 'electiveAffinityMoment',
      definition:
        'The relation includes selective affinity through which measure seeks fitting counterpart',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'real-measure-1-inv-1',
      constraint: 'independence and relation are inseparable in real measure',
      predicate: 'inseparable(independence, relation)',
    },
    {
      id: 'real-measure-1-inv-2',
      constraint: 'measure relation has qualitative weight',
      predicate: 'qualitativeWeight(measureRelation)',
    },
  ],
  forces: [
    {
      id: 'real-measure-1-force-1',
      description:
        'Direct relation requires exposition of inner exponental determination',
      type: 'mediation',
      trigger: 'relation.needsInnerDeterminer = true',
      effect: 'innerExponent.explicit = true',
      targetState: 'real-measure-1-exponent',
    },
  ],
  transitions: [
    {
      id: 'real-measure-1-trans-1',
      from: 'real-measure-1',
      to: 'real-measure-1-exponent',
      mechanism: 'mediation',
      description: 'From independent relation to inner exponent',
    },
  ],
  nextStates: ['real-measure-1-exponent'],
  previousStates: ['measure-3'],
  provenance: {
    topicMapId: 'rm-a-intro-self-subsistent-measures',
    lineRange: { start: 3, end: 11 },
    section: 'The Relation of Independent Measures',
    order: 1,
  },
  description: mEntryById('rm-a-intro-self-subsistent-measures')?.description,
  keyPoints: mEntryById('rm-a-intro-self-subsistent-measures')?.keyPoints,
};

const state2: DialecticState = {
  id: 'real-measure-1-exponent',
  title: 'Inner exponent and instability of relation',
  concept: 'InnerMeasureExponent',
  phase: 'quantity',
  moments: [
    {
      name: 'innerMeasureExponent',
      definition:
        'The exponent functions as inner measure of compound relation, disclosing non-accidental structure',
      type: 'determination',
    },
    {
      name: 'compoundAlteration',
      definition:
        'Alteration of compound relation reveals the non-stability of immediate measure configurations',
      type: 'negation',
      relation: 'contains',
      relatedTo: 'innerMeasureExponent',
    },
    {
      name: 'returnToDegree',
      definition:
        'Measure tends to return into degree-form while preserving the complexity of relation',
      type: 'process',
    },
  ],
  invariants: [
    {
      id: 'real-measure-1-exp-inv-1',
      constraint: 'inner exponent determines relation from within',
      predicate: 'innerDetermination(exponent, relation)',
    },
    {
      id: 'real-measure-1-exp-inv-2',
      constraint: 'immediate stability is undermined by internal alteration',
      predicate: 'internalAlterationUnderminesStability(measure)',
    },
  ],
  forces: [
    {
      id: 'real-measure-1-exp-force-1',
      description:
        'Instability pushes relation into exclusive measure-continuity (nodal line form)',
      type: 'sublation',
      trigger: 'measure.immediateStabilityBreaks = true',
      effect: 'nodalContinuity.explicit = true',
      targetState: 'real-measure-2',
    },
  ],
  transitions: [
    {
      id: 'real-measure-1-exp-trans-1',
      from: 'real-measure-1-exponent',
      to: 'real-measure-2',
      mechanism: 'sublation',
      description: 'From unstable relation to nodal continuity',
    },
  ],
  nextStates: ['real-measure-2'],
  previousStates: ['real-measure-1'],
  provenance: {
    topicMapId: 'rm-a-a-2-exponent-inner-measure',
    lineRange: { start: 63, end: 95 },
    section: 'The Relation of Independent Measures',
    order: 6,
  },
  description: mEntryById('rm-a-a-2-exponent-inner-measure')?.description,
  keyPoints: mEntryById('rm-a-a-2-exponent-inner-measure')?.keyPoints,
};

export const measuresIR: DialecticIR = {
  id: 'measures-ir',
  title: 'Measures IR: Relation of Independent Measures',
  section:
    'BEING - MEASURE - B. Real Measure - The Relation of Independent Measures',
  states: [state1, state2],
  metadata: {
    sourceFile: 'measures.txt',
    totalStates: 2,
    cpuGpuMapping: {
      'real-measure-1': 'quantity',
      'real-measure-1-exponent': 'quantity',
    },
  },
};

export const measuresStates = {
  'real-measure-1': state1,
  'real-measure-1-exponent': state2,
};
