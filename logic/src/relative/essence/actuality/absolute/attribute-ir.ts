import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { ATTRIBUTE_TOPIC_MAP } from './sources/attribute-topic-map';

const state1: DialecticState = {
  id: 'att-1',
  title: 'Absolute absolute vs attribute — relative absolute',
  concept: 'AbsoluteAsAttribute',
  phase: 'appearance',
  moments: [
    {
      name: 'absoluteAbsolute',
      definition:
        'Absolute whose form has returned into itself, where form is content',
      type: 'determination',
    },
    {
      name: 'relativeAbsolute',
      definition: 'Attribute as absolute in form-determination',
      type: 'mediation',
      relation: 'opposite',
      relatedTo: 'absoluteAbsolute',
    },
  ],
  invariants: [
    {
      id: 'att-1-inv-1',
      constraint: 'attribute = absoluteInFormDetermination',
      predicate: 'equals(attribute, absoluteInFormDetermination)',
    },
    {
      id: 'att-1-inv-2',
      constraint: 'attribute.content = totality',
      predicate: 'equals(attribute.content, totality)',
    },
  ],
  forces: [
    {
      id: 'att-1-force-1',
      description:
        'Relative absolute drives to reflective-shine reduction of worlds',
      type: 'reflection',
      trigger: 'attribute.formDetermination = explicit',
      effect: 'worlds.reduceToShine = true',
      targetState: 'att-3',
    },
  ],
  transitions: [
    {
      id: 'att-1-trans-1',
      from: 'att-1',
      to: 'att-3',
      mechanism: 'reflection',
      description: 'From attribute distinction to reflective shine reduction',
    },
  ],
  nextStates: ['att-3'],
  previousStates: ['exp-7'],
  provenance: {
    topicMapId: 'abs-b-1-absolute-absolute-vs-attribute',
    lineRange: { start: 3, end: 20 },
    section: 'The Absolute Attribute',
    order: 1,
  },
  description: ATTRIBUTE_TOPIC_MAP.entries[0]?.description,
  keyPoints: ATTRIBUTE_TOPIC_MAP.entries[0]?.keyPoints,
};

const state2: DialecticState = {
  id: 'att-3',
  title: 'Attribute as identity determination — absolute totality',
  concept: 'AttributeTotality',
  phase: 'appearance',
  moments: [
    {
      name: 'identityDetermination',
      definition: 'Attribute as determination of identity in absolute totality',
      type: 'determination',
    },
    {
      name: 'sublatedDeterminations',
      definition: 'Determinations posited as sublated reflective shine',
      type: 'sublation',
      relation: 'contains',
      relatedTo: 'identityDetermination',
    },
  ],
  invariants: [
    {
      id: 'att-3-inv-1',
      constraint: 'absoluteTotality = sublated(determinations)',
      predicate: 'equals(absoluteTotality, sublated(determinations))',
    },
    {
      id: 'att-3-inv-2',
      constraint: 'attribute.formDetermination = reflectiveShine',
      predicate: 'equals(attribute.formDetermination, reflectiveShine)',
    },
  ],
  forces: [
    {
      id: 'att-3-force-1',
      description: 'Identity determination drives to form-nullity realization',
      type: 'negation',
      trigger: 'reflection.revertsToIdentity = true',
      effect: 'form.nullity = true',
      targetState: 'att-6',
    },
  ],
  transitions: [
    {
      id: 'att-3-trans-1',
      from: 'att-3',
      to: 'att-6',
      mechanism: 'negation',
      description: 'From attribute totality to form as nullity',
    },
  ],
  nextStates: ['att-6'],
  previousStates: ['att-1'],
  provenance: {
    topicMapId: 'abs-b-3-attribute-as-identity-determination',
    lineRange: { start: 44, end: 60 },
    section: 'The Absolute Attribute',
    order: 3,
  },
  description: ATTRIBUTE_TOPIC_MAP.entries[2]?.description,
  keyPoints: ATTRIBUTE_TOPIC_MAP.entries[2]?.keyPoints,
};

const state3: DialecticState = {
  id: 'att-6',
  title: 'Form as nullity — external reflective shine',
  concept: 'AttributeToMode',
  phase: 'appearance',
  moments: [
    {
      name: 'formNullity',
      definition: 'Form of attribute recognized as nullity-in-itself',
      type: 'negation',
    },
    {
      name: 'mereManner',
      definition: 'External reflective shine as mere way and manner',
      type: 'appearance',
      relation: 'transforms',
      relatedTo: 'formNullity',
    },
  ],
  invariants: [
    {
      id: 'att-6-inv-1',
      constraint: 'form = externalReflectiveShine',
      predicate: 'equals(form, externalReflectiveShine)',
    },
    {
      id: 'att-6-inv-2',
      constraint: 'attribute.form = nullInItself',
      predicate: 'equals(attribute.form, nullInItself)',
    },
  ],
  forces: [
    {
      id: 'att-6-force-1',
      description: 'Null-form attribute passes over into mode',
      type: 'passover',
      trigger: 'form.nullity = true',
      effect: 'mode.emerges = true',
      targetState: 'mod-1',
    },
  ],
  transitions: [
    {
      id: 'att-6-trans-1',
      from: 'att-6',
      to: 'mod-1',
      mechanism: 'passover',
      description: 'From attribute to mode of the absolute',
    },
  ],
  nextStates: ['mod-1'],
  previousStates: ['att-3'],
  provenance: {
    topicMapId: 'abs-b-6-form-nullity',
    lineRange: { start: 91, end: 96 },
    section: 'The Absolute Attribute',
    order: 6,
  },
  description: ATTRIBUTE_TOPIC_MAP.entries[5]?.description,
  keyPoints: ATTRIBUTE_TOPIC_MAP.entries[5]?.keyPoints,
};

export const attributeIR: DialecticIR = {
  id: 'attribute-ir',
  title:
    'Attribute IR: Relative Absolute, Identity Determination, Form Nullity',
  section: 'ESSENCE - C. ACTUALITY - C. The Absolute - b. The Attribute',
  states: [state1, state2, state3],
  metadata: {
    sourceFile: 'attribute.txt',
    totalStates: 3,
    cpuGpuMapping: {
      'att-1': 'appearance',
      'att-3': 'appearance',
      'att-6': 'appearance',
    },
  },
};

export const attributeStates = {
  'att-1': state1,
  'att-3': state2,
  'att-6': state3,
};
