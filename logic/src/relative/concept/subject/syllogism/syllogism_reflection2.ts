import type { Chunk, LogicalOperation } from './index';

/**
 * Syllogism of Reflection — c. The Syllogism of Analogy
 * Conservative chunking + focused HLOs; append to reflection module.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sr-ana-1-overview',
    title: 'Analogy — overview (schema S-U-P)',
    text: `Analogy uses the third-figure schema S-U-P. Its middle is a singular taken in its universal nature: a universality that is the immanent reflection of a concreted term and thus identical with that concreted term.`,
  },
  {
    id: 'sr-ana-2-middle-as-immanent-universal',
    title: 'Middle: singular as immediately universal',
    text: `The middle is a singular whose nature is universality (the immanent reflection of a concreted term). As such the singular is at once a universal and a concrete singularity — it serves as true mediating unity.`,
  },
  {
    id: 'sr-ana-3-extreme-with-shared-universal-nature',
    title: 'Extremes share universal nature',
    text: `Another extreme is a distinct singular that shares the same universal nature as the middle. Because the middle is immediately universal in itself, the extremes are legitimately conjoined.`,
  },
  {
    id: 'sr-ana-4-example-earth-moon',
    title: 'Example: earth / moon inhabitants',
    text: `Example: "The earth has inhabitants; the moon is an earth; therefore the moon has inhabitants." The middle singular ("earth" as universal nature) grounds the inference because it is immediately universal.`,
  },
  {
    id: 'sr-ana-5-significance-and-transition',
    title: 'Significance: valid mediation and closure',
    text: `Analogy secures mediation by having the singular immediately identical with the universal; it thus completes the transition from subjective induction to an objective, reflected syllogism.`,
  },
  // appended: Syllogism of Analogy — part 2 (critique of superficiality, quaternio terminorum, indeterminacy)
  {
    id: 'sr-ana-6-superficiality-issue',
    title: 'Analogy superficial when universality is mere quality',
    text: `Analogy is superficial if the uniting universal is only a quality or subjective distinctive mark — then identity of extremes is treated as mere similarity, reducing logic to representation.`,
  },
  {
    id: 'sr-ana-7-form-as-content-fallacy',
    title: 'Fallacy: making the form the empirical content',
    text: `Presenting the major as "similarity in one mark implies similarity in others" turns the form into content and relegates empirical matter to the minor; logic requires the form remain distinct from empirical matter.`,
  },
  {
    id: 'sr-ana-8-form-determines-content',
    title: 'Form determining content is a necessary advance',
    text: `When form differentiations acquire content-determination this is an essential, formal development: such content cannot be treated as arbitrary empirical matter and must be retained in logical treatment.`,
  },
  {
    id: 'sr-ana-9-quaternio-concern',
    title: 'Quaternio terminorum and apparent four-term problem',
    text: `Analogy appears to have four terms (two singulars, a shared property, and inferred properties). This seems to complicate reduction to a formal syllogism but reflects the middle's dual nature (singular + immanent universality).`,
  },
  {
    id: 'sr-ana-10-source-indeterminacy',
    title: 'Indeterminacy: general nature vs particularity',
    text: `Even if extremes share a universal nature, it can be unclear whether an inferred property belongs by virtue of that general nature or by particularity of the first singular (e.g., inhabitants as property of 'earth' in general or that specific earth).`,
  },
  {
    id: 'sr-ana-11-analogy-as-reflection',
    title: 'Analogy remains reflective — predicate not guaranteed',
    text: `Because singularity and universality unite immediately in the middle, externality of reflection persists: the predicate of one singular is not automatically the predicate of the other despite shared genus.`,
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'sr-op-24-declare-analogy',
    chunkId: 'sr-ana-1-overview',
    label: 'Declare analogy schema and nature of middle',
    clauses: [
      'schema = S-U-P',
      'middle = singularWithUniversalNature',
      'analogy.is = validReflectionForm',
    ],
    predicates: [{ name: 'IsAnalogySchema', args: [] }],
    relations: [{ predicate: 'implements', from: 'analogy', to: 'S-U-P' }],
  },
  {
    id: 'sr-op-25-middle-immanent',
    chunkId: 'sr-ana-2-middle-as-immanent-universal',
    label: 'Formalize middle as immanent universal (singular ≡ universal)',
    clauses: [
      'middle.universal = immanentReflectionOf(concretedTerm)',
      'singular.isUniversal = true',
    ],
    predicates: [{ name: 'IsImmanentUniversal', args: ['middle'] }],
    relations: [
      { predicate: 'identifies', from: 'middle', to: 'concretedTerm' },
    ],
  },
  {
    id: 'sr-op-26-extremes-shared-nature',
    chunkId: 'sr-ana-3-extreme-with-shared-universal-nature',
    label: 'Formalize extremes sharing universal nature',
    clauses: [
      'extremeB.universalNature = middle.universalNature',
      'sharedNature => legitimateConjunction',
    ],
    predicates: [
      { name: 'SharesUniversalNature', args: ['extreme', 'middle'] },
    ],
    relations: [
      { predicate: 'grounds', from: 'sharedNature', to: 'conclusion' },
    ],
  },
  {
    id: 'sr-op-27-example-encode',
    chunkId: 'sr-ana-4-example-earth-moon',
    label: 'Encode example: earth / moon inference',
    clauses: [
      'premise1: earth.has(inhabitants)',
      "premise2: moon.is(earth)  // moon shares earth's universal nature",
      'conclusion: moon.has(inhabitants)',
    ],
    predicates: [{ name: 'IllustratesAnalogy', args: ['earthMoonExample'] }],
    relations: [{ predicate: 'derives', from: 'premises', to: 'conclusion' }],
  },
  {
    id: 'sr-op-28-significance',
    chunkId: 'sr-ana-5-significance-and-transition',
    label: 'State significance: analogy secures mediation; closes transition',
    clauses: [
      'if singular==universal then mediation.valid',
      'analogy -> completesTransitionToReflectedSyllogism',
    ],
    predicates: [{ name: 'SecuresMediation', args: ['analogy'] }],
    relations: [{ predicate: 'transforms', from: 'induction', to: 'analogy' }],
  },
  {
    id: 'sr-op-29-superficiality',
    chunkId: 'sr-ana-6-superficiality-issue',
    label: 'Identify superficiality when universal is mere subjective quality',
    clauses: [
      'if universal.isQualityOnly then analogy.isSuperficial',
      'similarity != identity',
    ],
    predicates: [{ name: 'IsSuperficialAnalogy', args: ['analogy'] }],
    relations: [
      { predicate: 'reducesTo', from: 'analogy', to: 'representation' },
    ],
  },
  {
    id: 'sr-op-30-form-vs-content-fallacy',
    chunkId: 'sr-ana-7-form-as-content-fallacy',
    label: "Flag the fallacy of making form the major's content",
    clauses: [
      'major.asFormContent => empiricalMatter.relegatedToMinor',
      'logic.requires = form/contentDistinction',
    ],
    predicates: [{ name: 'IsFormContentFallacy', args: [] }],
    relations: [
      {
        predicate: 'violates',
        from: 'majorAsFormContent',
        to: 'logicalRequirement',
      },
    ],
  },
  {
    id: 'sr-op-31-form-determines-content',
    chunkId: 'sr-ana-8-form-determines-content',
    label: 'Treat necessary advance: form determining content',
    clauses: [
      'form.determineContent = necessaryAdvance',
      'suchContent != arbitraryEmpiricalMatter',
    ],
    predicates: [{ name: 'IsFormDeterminedContent', args: ['form'] }],
    relations: [
      { predicate: 'qualifies', from: 'form', to: 'contentDetermination' },
    ],
  },
  {
    id: 'sr-op-32-quaternio-formalize',
    chunkId: 'sr-ana-9-quaternio-concern',
    label:
      'Formalize quaternio terminorum appearance and dual nature of middle',
    clauses: [
      'analogy.terms = {s1, s2, sharedProperty, inferredProperties}',
      'middle.dualNature = singular ∧ immanentUniversality',
    ],
    predicates: [{ name: 'HasQuaternioAspect', args: ['analogy'] }],
    relations: [
      { predicate: 'reflects', from: 'quaternio', to: 'middleDuality' },
    ],
  },
  {
    id: 'sr-op-33-indeterminacy-source',
    chunkId: 'sr-ana-10-source-indeterminacy',
    label: 'Encode indeterminacy whether property is general or particular',
    clauses: [
      'inferredPropertySource = {generalNature | particularity}',
      'uncertainty => weakensConclusion',
    ],
    predicates: [{ name: 'IsSourceIndeterminate', args: ['inferredProperty'] }],
    relations: [
      {
        predicate: 'reducesStrengthOf',
        from: 'sourceIndeterminacy',
        to: 'conclusion',
      },
    ],
  },
  {
    id: 'sr-op-34-analogy-reflection-status',
    chunkId: 'sr-ana-11-analogy-as-reflection',
    label:
      'Conclude: analogy remains a reflective syllogism; predicate not guaranteed by shared genus alone',
    clauses: [
      'if middle.unityImmediate then externalityPersists',
      'sharedGenus != automaticPredicateTransfer',
    ],
    predicates: [{ name: 'IsReflectiveRemnant', args: ['analogy'] }],
    relations: [
      { predicate: 'warns', from: 'sharedGenus', to: 'predicateNotGuaranteed' },
    ],
  },
];

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getLogicalOpsForChunk(oneBasedIndex: number) {
  const chunk = getChunk(oneBasedIndex);
  if (!chunk) return [];
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunk.id);
}
