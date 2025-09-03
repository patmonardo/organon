import type { Chunk, LogicalOperation } from './index'

/**
 * Reflection — B. Difference (part 1: Absolute Difference)
 * Conservative chunking + focused logical operations. Additional parts (diversity, opposition)
 * will be appended as difference1 / difference2 in subsequent edits.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'diff-1-absolute-overview',
    title: 'Absolute difference — overview',
    text: `Difference is the negativity that reflection possesses in itself: the nothing invoked in identity discourse. It is the essential moment of identity that determines itself and is thus differentiated from difference.`
  },
  {
    id: 'diff-2-difference-in-for-itself',
    title: 'Difference in and for itself (self-referring difference)',
    text: `This difference is absolute: not difference through something external but self‑referring and simple. The simple "not" constitutes the difference of reflection — a determinateness in itself rather than otherness of mere existence.`
  },
  {
    id: 'diff-3-contrast-existence-reflection',
    title: 'Contrast: reflection vs existence',
    text: `In existence, otherness lies outside and each existence remains immediate for itself. In reflection, by contrast, the other is other in and for itself — difference is posited as internal determinateness and not external otherness.`
  }
]

// appended: Difference — part 3 (unity of identity & difference → diversity)
CANONICAL_CHUNKS.push(
  {
    id: 'diff-4-moments-unity',
    title: 'Difference includes identity and difference as posited moments',
    text: `Difference contains both moments, identity and difference, each a posited determinateness. In this positedness each refers to itself: identity functions as the moment of immanent reflection while difference is itself reflected.`
  },
  {
    id: 'diff-5-positedness-self-reference',
    title: 'Positedness and self-reference of the moments',
    text: `Both identity and difference are reflections into themselves; each retains its referential character within the posited whole. Their mutual self-reference is the structural condition for diversity.`
  },
  {
    id: 'diff-6-diversity-emerges',
    title: 'Diversity as the result of dual reflective moments',
    text: `Because difference has these two reflective moments that are themselves self-referring, difference becomes diversity: a determinate multiplicity grounded in the unity of difference and identity.`
  }
)

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'diff-op-1-declare-absolute',
    chunkId: 'diff-1-absolute-overview',
    label: 'Declare difference as reflective negativity',
    clauses: ['difference = negativityOfReflection', 'identity.includes = differenceAsMoment'],
    predicates: [{ name: 'IsReflectiveDifference', args: [] }],
    relations: [{ predicate: 'constitutes', from: 'difference', to: 'identityMoment' }]
  },
  {
    id: 'diff-op-2-self-referring',
    chunkId: 'diff-2-difference-in-for-itself',
    label: 'Formalize self-referring / simple not',
    clauses: ['difference.selfReferring = true', 'difference.simpleNot = true', 'notExternalMediation = true'],
    predicates: [{ name: 'IsAbsoluteDifference', args: [] }],
    relations: [{ predicate: 'distinguishes', from: 'absoluteDifference', to: 'externalOtherness' }]
  },
  {
    id: 'diff-op-3-contrast-existence',
    chunkId: 'diff-3-contrast-existence-reflection',
    label: 'Contrast existential otherness with reflective difference',
    clauses: ['existence.otherness = external', 'reflection.otherness = internal', 'determinateness = inItself'],
    predicates: [{ name: 'ContrastsExistenceReflection', args: [] }],
    relations: [{ predicate: 'contrastsWith', from: 'reflectionDifference', to: 'existentialOtherness' }]
  },
  {
    id: 'diff-op-4-moments-declare',
    chunkId: 'diff-4-moments-unity',
    label: 'Declare identity + difference as mutual posited moments',
    clauses: [
      'difference.includes = {identity, difference}',
      'each.moment = positedDeterminateness',
      'each.refersTo = itself'
    ],
    predicates: [{ name: 'HasPositedMoments', args: ['difference'] }],
    relations: [{ predicate: 'composes', from: 'difference', to: 'moments(identity,difference)' }]
  },
  {
    id: 'diff-op-5-self-reference-formalize',
    chunkId: 'diff-5-positedness-self-reference',
    label: 'Formalize self-reference of moments',
    clauses: [
      'identity.refersTo = identity',
      'difference.refersTo = difference',
      'mutualSelfReference => structuralCondition(diversity)'
    ],
    predicates: [{ name: 'IsMutualSelfReference', args: [] }],
    relations: [{ predicate: 'grounds', from: 'mutualSelfReference', to: 'diversity' }]
  },
  {
    id: 'diff-op-6-diversity-result',
    chunkId: 'diff-6-diversity-emerges',
    label: 'Encode diversity emergence from internal duality',
    clauses: [
      'if difference.hasDualReflectiveMoments then result = diversity',
      'diversity = determinateMultiplicityGroundedInUnity'
    ],
    predicates: [{ name: 'ProducesDiversity', args: ['difference'] }],
    relations: [{ predicate: 'yields', from: 'internalDuality', to: 'diversity' }]
  }
]
