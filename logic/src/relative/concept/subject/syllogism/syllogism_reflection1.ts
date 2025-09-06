import type { Chunk, LogicalOperation } from './index';

/**
 * Syllogism of Reflection — part: The Syllogism of Induction (b)
 * Conservative chunking and focused HLOs for insertion into the reflection module.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'sr-ind-1-overview',
    title: 'Induction — overview (schema U‑S‑P / second-figure reflection)',
    text: `Induction is the reflection-form under the second-figure schema (U‑S‑P). Its middle is completed singularity (singularity posited with universality). One extreme is a predicate common to many singulars; the other extreme is the immediate genus or exhausted species.`,
  },
  {
    id: 'sr-ind-2-middle-completed-singularity',
    title: 'Middle: singularity completed by universality',
    text: `The middle is not abstract singularity but singularity completed — it contains its opposite (universality). This completed singularity functions as the effective mediating term in induction.`,
  },
  {
    id: 'sr-ind-3-premise-configuration',
    title:
      'Premise structure: many singulars connecting to a universal and predicate',
    text: `One set of immediate premises consists of singular instances each bearing the predicate; these singulars are collected under a universal or genus which is thereby exhausted by the singulars. The configuration is U (genus) mediated by S (singularity) to P (predicate).`,
  },
  {
    id: 'sr-ind-4-infinite-collection-note',
    title: 'Infinite listing (schema illustration)',
    text: `Induction may be schematized as an (open) sequence of singular tokens (s s s ...) linked to U and thereby to P — a collection that in itself tends toward an indefinite plurality but is subsumed by the completed singular middle.`,
  },
  // appended: Syllogism of Induction — part 2 (PSU mismatch resolved; equality of extension; experience)
  {
    id: 'sr-ind-5-psu-mismatch',
    title: 'PSU mismatch and completed middle (why P-S-U does not fit)',
    text: `The formal second-figure P‑S‑U does not fit induction because S there did not subsume or act as predicate. In induction the middle is "all singulars" (completed singularity) so the previous deficiency is eliminated: the subject U‑S contains a predicate of equal or greater extension, making the relation externally identical.`,
  },
  {
    id: 'sr-ind-6-equality-extension',
    title: 'Equality of extension (singularity vs universality)',
    text: `The difference between singular occurrence and universal repetition is here indifferent in form: the same content appears as singulars and as universal extension. This equality of extension underwrites the posited result of the formal syllogism in reflection.`,
  },
  {
    id: 'sr-ind-7-experience-not-contingent',
    title: 'Induction as experience (not mere perception or contingency)',
    text: `Induction is not merely contingent perception (as the corresponding existence-figure could be) but the syllogism of experience: a subjective gathering of singulars into a genus, then conjoining the genus with a universal determinateness found in all singulars.`,
  },
  {
    id: 'sr-ind-8-objective-significance-inner-only',
    title: 'Objective significance: inner concept not yet posited',
    text: `Objectively the immediate genus determines itself through the totality of singularity as a universal property, but at first this objective significance remains the inner concept of the syllogism and is not yet posited as such within the inference.`,
  },
];

// appended: Syllogism of Induction — part 3 (subjectivity, bad-infinity, necessity of immediate universality → analogy)
CANONICAL_CHUNKS.push(
  {
    id: 'sr-ind-9-subjective-nature',
    title: 'Induction remains essentially subjective',
    text: `Induction is still a subjective syllogism: its middle-terms are immediate singulars gathered externally into a genus. Universality remains only a completeness-task because singular immediacy persists, so the intended unity is never fully achieved.`,
  },
  {
    id: 'sr-ind-10-bad-infinity-and-problematic-conclusion',
    title: 'Bad infinity and the problematic conclusion',
    text: `Because singulars remain immediate, induction recurs into a bad infinity: the genus is only formed in the infinite progression of instances, so the induction's conclusion remains problematic and never fully secured by finite experience.`,
  },
  {
    id: 'sr-ind-11-presupposition-and-immediacy',
    title:
      'Induction presupposes its conclusion; immediacy grounded in universal',
    text: `Induction in practice presupposes that the genus is already conjoined with its determinateness; the inference thus rests on an immediacy that is really the universal in itself, not merely isolated singulars — otherwise the middle would split and inference fail.`,
  },
  {
    id: 'sr-ind-12-analogy-as-true-form',
    title: 'Analogy: singularity immediately identical with universality',
    text: `The true valid form is the syllogism of analogy: the middle is a singularity that is immediately in itself universality. Only when the singular is immediately identical with the universal does induction become a legitimate syllogistic inference.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'sr-op-12-declare-induction',
    chunkId: 'sr-ind-1-overview',
    label: 'Declare induction as U‑S‑P and role of completed singularity',
    clauses: [
      'schema = U-S-P',
      'middle = singularityCompletedWithUniversality',
      'predicate.commonTo = manySingulars',
    ],
    predicates: [{ name: 'IsInduction', args: [] }],
    relations: [
      { predicate: 'implementsSchema', from: 'induction', to: 'U-S-P' },
    ],
  },
  {
    id: 'sr-op-13-middle-completion',
    chunkId: 'sr-ind-2-middle-completed-singularity',
    label: 'Formalize middle: singularity contains universality (completed)',
    clauses: [
      'middle.contains = {singularity, universality}',
      'middle.isMediator = true',
    ],
    predicates: [{ name: 'IsCompletedSingularity', args: ['middle'] }],
    relations: [
      {
        predicate: 'mediates',
        from: 'middle',
        to: 'singularInstances+universal',
      },
    ],
  },
  {
    id: 'sr-op-14-premise-structure',
    chunkId: 'sr-ind-3-premise-configuration',
    label:
      'Encode premise pattern: singular tokens → universal/genus → predicate',
    clauses: [
      'forEach(s in singulars): s.has(predicate)',
      'universal.exhaustedBy = singulars',
      'conclusion: universal -> predicate',
    ],
    predicates: [
      { name: 'IsExhaustiveCollection', args: ['universal', 'singulars'] },
    ],
    relations: [
      {
        predicate: 'supports',
        from: 'singulars',
        to: 'universalPredicateAttachment',
      },
    ],
  },
  {
    id: 'sr-op-15-infinite-collection',
    chunkId: 'sr-ind-4-infinite-collection-note',
    label:
      'Note potential openness: induction collects many singulars (not strictly finite)',
    clauses: [
      'singulars = sequence(s1,s2,...)',
      'collection.tendsTo = indefinitePlurality',
      'induction.reliesOn = concreteInstances',
    ],
    predicates: [{ name: 'IsOpenCollection', args: ['singularSeries'] }],
    relations: [
      { predicate: 'dependsOn', from: 'induction', to: 'instanceGathering' },
    ],
  },
  {
    id: 'sr-op-16-psu-mismatch-formalized',
    chunkId: 'sr-ind-5-psu-mismatch',
    label:
      'Formalize why P‑S‑U does not correspond and how induction resolves it',
    clauses: [
      'secondFigure(P-S-U).S.didNotSubsumedAsPredicate',
      'induction.middle = allSingulars (completed)',
      'U-S.predicate.extension >= subject.extension => externalIdentity',
    ],
    predicates: [{ name: 'ResolvesPSUMismatch', args: [] }],
    relations: [
      { predicate: 'replaces', from: 'P-S-U', to: 'U-S-P(induction)' },
    ],
  },
  {
    id: 'sr-op-17-equality-extension',
    chunkId: 'sr-ind-6-equality-extension',
    label: 'Equality of extension as indifferent formal determination',
    clauses: [
      'singularContent.presentAs = {singularity, universality}',
      'equalityOfExtension => formalIndifference(subject<->predicate)',
    ],
    predicates: [{ name: 'HasEqualExtension', args: ['subject', 'predicate'] }],
    relations: [
      { predicate: 'grounds', from: 'equalityOfExtension', to: 'formalResult' },
    ],
  },
  {
    id: 'sr-op-18-induction-as-experience',
    chunkId: 'sr-ind-7-experience-not-contingent',
    label: 'Encode induction as experiential gathering and its implication',
    clauses: [
      'induction.collects = concreteSingulars',
      'genus.determinedBy = totalityOfSingulars',
      'induction.is = syllogismOfExperience',
    ],
    predicates: [{ name: 'IsSyllogismOfExperience', args: ['induction'] }],
    relations: [
      { predicate: 'dependsOn', from: 'induction', to: 'instanceGathering' },
    ],
  },
  {
    id: 'sr-op-19-objective-significance-inner',
    chunkId: 'sr-ind-8-objective-significance-inner-only',
    label:
      'Objective significance exists as inner concept but is not yet posited',
    clauses: [
      'genus.determinesItselfVia = singularTotality',
      'objectiveSignificance = innerConcept (not yet posited)',
      'positedObjectiveSignificance => further transformationNeeded',
    ],
    predicates: [{ name: 'IsInnerObjectiveSignificance', args: ['genus'] }],
    relations: [
      { predicate: 'awaits', from: 'innerConcept', to: 'positedRealization' },
    ],
  },
  {
    id: 'sr-op-20-subjective-formalize',
    chunkId: 'sr-ind-9-subjective-nature',
    label:
      'Formalize subjectivity: singulars as immediate middles; universality incomplete',
    clauses: [
      'induction.middle.parts = immediateSingulars',
      'universality.status = completenessTask',
      'unity.intended => remainsOughtNotIs',
    ],
    predicates: [{ name: 'IsSubjectiveSyllogism', args: ['induction'] }],
    relations: [
      {
        predicate: 'failsToRealize',
        from: 'induction',
        to: 'immediateUniversality',
      },
    ],
  },
  {
    id: 'sr-op-21-bad-infinity',
    chunkId: 'sr-ind-10-bad-infinity-and-problematic-conclusion',
    label:
      'Encode bad infinity: genus formed only in infinite instance-collection',
    clauses: [
      'singulars.collect => tendsTo(infiniteProgression)',
      'conclusion.validity.requires = completedInfiniteCollection',
      'finiteExperience => conclusionProblematic',
    ],
    predicates: [{ name: 'ExhibitsBadInfinity', args: ['induction'] }],
    relations: [
      {
        predicate: 'renders',
        from: 'finiteInstances',
        to: 'insufficientGround',
      },
    ],
  },
  {
    id: 'sr-op-22-presupposition-immediacy',
    chunkId: 'sr-ind-11-presupposition-and-immediacy',
    label:
      'Induction presupposes its conclusion — immediacy is truly universal',
    clauses: [
      'induction.presupposes = genusConjoinedWithDeterminateness',
      'realGround = universalImmediacy (not isolated singular)',
      'if middle.splits then inferenceFails',
    ],
    predicates: [{ name: 'PresupposesConclusion', args: ['induction'] }],
    relations: [
      { predicate: 'dependsOn', from: 'inference', to: 'universalImmediacy' },
    ],
  },
  {
    id: 'sr-op-23-analogy-formalize',
    chunkId: 'sr-ind-12-analogy-as-true-form',
    label:
      'Define analogy: middle singularity identical with universality → valid inference',
    clauses: [
      'analogy.middle = singularity ∧ immediatelyUniversal',
      'if singularity == universality then induction -> validSyllogism',
      'analogy = syllogismOfAnalogy',
    ],
    predicates: [{ name: 'IsAnalogy', args: ['syllogism'] }],
    relations: [
      {
        predicate: 'transforms',
        from: 'induction(whenImmediateUniversal)',
        to: 'analogy',
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
