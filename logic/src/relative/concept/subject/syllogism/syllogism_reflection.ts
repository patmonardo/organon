import type { Chunk, LogicalOperation } from './index';

/**
 * Syllogism of Reflection — Syllogism of Allness (part 1)
 * Conservative chunking and focused HLOs. Two more parts will be appended.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sr-all-1-overview',
    title: 'Allness — overview',
    text: `The syllogism of allness is the understanding's syllogism in a developed form: the middle is concrete (developed into its moments) but at first universality gathers singulars only externally; allness is not yet the concept's universality but external universality of reflection.`,
  },
  {
    id: 'sr-all-2-middle-concreteness',
    title: 'Middle developed into its moments',
    text: `The middle is no longer an abstract particularity but contains singularity and thus is concrete; this is an essential requirement for the concept, though not yet absolute reflection.`,
  },
  {
    id: 'sr-all-3-external-universality',
    title: 'External universality vs conceptual universality',
    text: `At the outset the form of allness collects singulars into a universal only externally; singular determinations still underlie that universality, so negation of immediacy here is only the first negation.`,
  },
  {
    id: 'sr-all-4-contingency-resolved',
    title: 'Contingency resolved by concrete middle',
    text: `Whereas existence-syllogisms were contingent because the middle was a single determinateness, the concrete middle of allness contains singularity so only predicates that concretely belong may be attached in conclusion.`,
  },
  {
    id: 'sr-all-5-examples-and-contrast',
    title: 'Examples: abstract vs concrete subjects',
    text: `Contrast "what is green is pleasing" (subject = abstraction of green) with "all things green are pleasing" (subject = concrete, green things with all their other properties). The concreted middle prevents arbitrary opposite predicates attaching.`,
  },
];

// appended: Syllogism of Allness — part 2 (reflective-perfection as illusion; major presupposes conclusion)
CANONICAL_CHUNKS.push(
  {
    id: 'sr-all-6-reflective-illusion',
    title: 'Reflective perfection as illusion',
    text: `The syllogism's reflective perfection makes it illusory: the middle ('all') already contains the predicate for each singular, so the subject possesses the predicate immediately rather than obtaining it through inference.`,
  },
  {
    id: 'sr-all-7-major-presupposes-conclusion',
    title: 'Major premise already presupposes the conclusion',
    text: `The major premise contains the conclusion within it; thus it is not an independent ground. Before accepting the major, one must check whether the conclusion is not a counter-instance — the major is true only insofar as its singular instances conform.`,
  },
  {
    id: 'sr-all-8-example-gaius',
    title: 'Example — Gaius and mortality',
    text: `In "All humans are mortal; Gaius is human; therefore Gaius is mortal," the major is correct only because the conclusion holds; if Gaius were not mortal, the major would fail. The conclusion must be independently secure to validate the major as universal.`,
  },
);

// appended: Syllogism of Allness — part 3 (reflection is semblance; singularity as middle; induction)
CANONICAL_CHUNKS.push(
  {
    id: 'sr-all-9-reflection-semblance',
    title: 'Reflection as empty semblance of inference',
    text: `The syllogism of reflection is an external, empty semblance: the major premise already contains the conclusion, so the inference is not the real ground of the predicate's attachment.`,
  },
  {
    id: 'sr-all-10-subjective-singularity-as-middle',
    title: 'Subjective singularity constitutes the middle',
    text: `The true essence of the inference rests on subjective singularity: the singular stands immediately connected to its predicate and thereby functions as the effective middle term, possessing universality only externally.`,
  },
  {
    id: 'sr-all-11-induction-identification',
    title: 'Identification with induction',
    text: `Because the union of singular and predicate is immediate and not derived, this form is properly the syllogism of induction rather than a genuine syllogistic mediation.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'sr-op-1-declare-allness',
    chunkId: 'sr-all-1-overview',
    label: 'Declare Allness and its provisional nature',
    clauses: [
      'figure = Allness',
      'middle.developed = true',
      'universality.initially = external',
    ],
    predicates: [{ name: 'IsAllness', args: [] }],
    relations: [
      { predicate: 'posits', from: 'allness', to: 'externalUniversality' },
    ],
  },
  {
    id: 'sr-op-2-middle-concrete',
    chunkId: 'sr-all-2-middle-concreteness',
    label: 'Middle contains singularity — concreteness constraint',
    clauses: ['middle.contains = singularity', 'middle.isConcrete = true'],
    predicates: [{ name: 'IsConcreteMiddle', args: ['middle'] }],
    relations: [
      { predicate: 'constrains', from: 'middle', to: 'permissiblePredicates' },
    ],
  },
  {
    id: 'sr-op-3-external-vs-conceptual',
    chunkId: 'sr-all-3-external-universality',
    label: 'Differentiate external universality from concept-universality',
    clauses: [
      'universality.external = true',
      'notYet = absoluteImmanentReflection',
    ],
    predicates: [{ name: 'IsExternalUniversality', args: ['universality'] }],
    relations: [
      {
        predicate: 'dependsOn',
        from: 'universality',
        to: 'underlyingSingularDeterminations',
      },
    ],
  },
  {
    id: 'sr-op-4-resolve-contingency',
    chunkId: 'sr-all-4-contingency-resolved',
    label: 'Concrete middle removes arbitrariness of predicates',
    clauses: [
      'if middle.isConcrete then only predicatesBelongingConcretelyAllowed',
      'contingency.reduced = true',
    ],
    predicates: [{ name: 'ReducesContingency', args: ['concreteMiddle'] }],
    relations: [
      {
        predicate: 'blocks',
        from: 'concreteMiddle',
        to: 'arbitraryOppositePredicates',
      },
    ],
  },
  {
    id: 'sr-op-5-formal-example',
    chunkId: 'sr-all-5-examples-and-contrast',
    label: 'Formalize example: abstract subject vs concrete subject',
    clauses: [
      'abstractSubject(green) -> conclusion may conflict with other determinates',
      'concreteSubject(allGreenThings) -> conclusion restricted to concrete totality',
    ],
    predicates: [
      { name: 'Illustrates', args: ['example', 'concretenessEffect'] },
    ],
    relations: [
      {
        predicate: 'contrasts',
        from: 'abstractSubject',
        to: 'concreteSubject',
      },
    ],
  },
  {
    id: 'sr-op-6-illusion-declare',
    chunkId: 'sr-all-6-reflective-illusion',
    label:
      'Declare reflective perfection as illusory (predicate pre-contained in middle)',
    clauses: [
      'middle = allSingulars',
      'major.attaches(predicate) => predicate.immediatelyIn(subject)',
      'inference.notNecessaryIf(predicate.immediate)',
    ],
    predicates: [{ name: 'IsReflectiveIllusion', args: ['allness'] }],
    relations: [
      {
        predicate: 'containsImmediately',
        from: 'middle',
        to: 'predicateForSingulars',
      },
    ],
  },
  {
    id: 'sr-op-7-major-presupposes',
    chunkId: 'sr-all-7-major-presupposes-conclusion',
    label:
      'Formalize: major premise presupposes its conclusion; need to check for counter-instances',
    clauses: [
      'major.includes = conclusionImplictly',
      'major.onlyValidIf(allInstancesConform)',
      'validateMajor => ensure(noCounterInstance)',
    ],
    predicates: [{ name: 'MajorPresupposesConclusion', args: ['major'] }],
    relations: [
      { predicate: 'requiresValidationBy', from: 'major', to: 'instanceCheck' },
    ],
  },
  {
    id: 'sr-op-8-gaius-example',
    chunkId: 'sr-all-8-example-gaius',
    label: 'Encode Gaius example as constraint on universality',
    clauses: [
      'example = {All humans are mortal, Gaius is human, therefore Gaius is mortal}',
      'major.validity ⇔ conclusion.holdsFor(Gaius)',
      'counterInstance(GaiusNotMortal) => major.invalid',
    ],
    predicates: [
      { name: 'IllustratesCounterInstanceRisk', args: ['GaiusExample'] },
    ],
    relations: [{ predicate: 'tests', from: 'example', to: 'majorValidity' }],
  },
  {
    id: 'sr-op-9-semblance-declare',
    chunkId: 'sr-all-9-reflection-semblance',
    label: 'Declare syllogism of reflection as empty semblance',
    clauses: [
      'major.contains = conclusionImplicitly',
      'inference.semblance = true',
      'realGround = notSyllogisticInference',
    ],
    predicates: [{ name: 'IsSemblance', args: ['reflectionSyllogism'] }],
    relations: [{ predicate: 'conceals', from: 'major', to: 'conclusion' }],
  },
  {
    id: 'sr-op-10-singularity-as-middle',
    chunkId: 'sr-all-10-subjective-singularity-as-middle',
    label:
      'Formalize: singularity functions as effective middle (universality only external)',
    clauses: [
      'singular.connectedImmediatelyTo(predicate)',
      'singularity.functionsAs = effectiveMiddle',
      'universality.inSingularity = externalOnly',
    ],
    predicates: [{ name: 'IsEffectiveMiddle', args: ['singularity'] }],
    relations: [
      { predicate: 'grounds', from: 'singularity', to: 'predicateAttachment' },
    ],
  },
  {
    id: 'sr-op-11-identify-induction',
    chunkId: 'sr-all-11-induction-identification',
    label: 'Identify this form with induction (immediate union, not mediated)',
    clauses: [
      'if predicateAttachedImmediatelyToSingular then form = induction',
      'induction != mediatedSyllogism',
    ],
    predicates: [{ name: 'IsInduction', args: ['form'] }],
    relations: [
      {
        predicate: 'contrastsWith',
        from: 'induction',
        to: 'syllogisticMediation',
      },
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
