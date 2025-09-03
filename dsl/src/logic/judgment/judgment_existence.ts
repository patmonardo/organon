import type { Chunk, LogicalOperation } from '../syllogism/index';

/**
 * Judgment — A. Positive Judgment (part 1)
 * Chunked passage + HLOs for the positive judgment: subject / predicate immediacies,
 * abstract singularity / abstract universality, copula as immediate being.
 */

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-1-positive-overview',
    title: 'Positive judgment — overview',
    text: `The positive judgment names subject and predicate as immediate determinations; their mediation is only presupposed. The judgment is a posited determinate concept whose immediacies remain abstract.`,
  },
  {
    id: 'j-2-predicate-abstract-universal',
    title: 'Predicate as abstract universal',
    text: `The predicate functions as the abstract universal. Its universality is conditioned by mediation (sublation of singularity/particularity) which here remains only presupposed, not enacted.`,
  },
  {
    id: 'j-3-subject-abstract-singular',
    title: 'Subject as abstract singular',
    text: `The subject is the abstract singular — an immediate something-in-general. As the abstract side of judgment it shows the concept's externality prior to mediation.`,
  },
  {
    id: 'j-4-copula-immediacy',
    title: 'Copula as immediate being',
    text: `The copula ("is") connects subject and predicate as an immediate relation. Lacking mediation or negation, the copula signifies immediate abstract being — hence the designation "positive" for this judgment.`,
  },
  {
    id: 'j-5-positivity-and-mediation-presupposed',
    title: 'Positivity and presupposed mediation',
    text: `Although universality and mediation are presupposed for the predicate, in the positive judgment mediation is not yet realized; the determination is immediate and simple, establishing the groundwork for later determinate judgments.`,
  },
  /* appended: Positive Judgment — part 2 (the proposition: "the singular is universal", reciprocity, form vs content) */
  {
    id: 'j-6-prop-singular-universal',
    title: 'Proposition: the singular is universal',
    text: `The first pure expression of the positive judgment is the proposition "the singular is universal." This expresses the form: subject and predicate receive determinate content through the judgment rather than by their bare names.`,
  },
  {
    id: 'j-7-universal-resolves-into-singular',
    title: 'The universal resolves into the singular',
    text: `Objectively the proposition shows perishableness of singulars and the universal's subsistence in concept. The universal gives itself determinate existence by resolving into singular determinations (the universal is singular).`,
  },
  {
    id: 'j-8-subject-as-concrete',
    title: 'Subject as concrete (universal inhering in singular)',
    text: `The subject, though initially an immediate singular, is posited as concrete through reference to the universal: it contains manifold properties and is thereby the universal in itself; the predicate isolates one such determination.`,
  },
  {
    id: 'j-9-reciprocity-and-result',
    title: 'Reciprocity of determination: twofold result',
    text: `Juxtaposing reciprocal determinations yields: (1) subject becomes universal by predicate, and (2) predicate is particularized within the subject. These reciprocal moves produce the one positive judgment rather than two separate propositions.`,
  },
  {
    id: 'j-10-form-vs-content-unity',
    title: 'Form vs content: implicit difference and unity',
    text: `Form (the singular is universal) and content (the universal is singular) are present together: the positive judgment unites form and content immediately while implicitly containing the difference that later determinate judgment must resolve.`,
  },
  /* appended: Positive Judgment — part 3 (union of form & content, particularity, negation of the positive) */
  {
    id: 'j-11-union-particularity',
    title: 'Union of form and content → particularity (problematic)',
    text: `If form ("the singular is universal") and content ("the universal is singular") were united so both subject and predicate are the unity of singularity and universality, both would become particular — an inner determination arrived at only by external reflection, producing an empty identical proposition.`,
  },
  {
    id: 'j-12-form-vs-content-distinction',
    title: 'Distinction of form and content preserved in positive judgment',
    text: `The positive judgment still distinguishes form and content: the subject remains immediate (not truly universal) while the content shows the subject as a manifold concrete. This tension prevents immediate unification into particularity.`,
  },
  {
    id: 'j-13-positivity-becomes-negative',
    title:
      'From positive to negative: necessity of positing the judgment as negative',
    text: `Because the two propositions taken together reveal the inadequacy of the positive form, the positive judgment must be posited as negative: the implicit difference collapses the immediate positivy and forces the judgement into a determinative, mediated (negative) form.`,
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-op-1-declare-positive-judgment',
    chunkId: 'j-1-positive-overview',
    label: 'Declare the positive judgment structure and limits',
    clauses: [
      'judgment.kind = positive',
      'subject = abstractSingular',
      'predicate = abstractUniversal',
      'mediation.presupposed = true',
    ],
    predicates: [{ name: 'IsPositiveJudgment', args: [] }],
    relations: [{ predicate: 'connects', from: 'subject', to: 'predicate' }],
  },
  {
    id: 'j-op-2-formalize-predicate-universal',
    chunkId: 'j-2-predicate-abstract-universal',
    label:
      'Formalize predicate as abstract universal with presupposed mediation',
    clauses: [
      'predicate.role = universal',
      'predicate.mediation = presupposed (sublationOfSingularity)',
      'universality.inConcept = immediacyWithNegativityImplicit',
    ],
    predicates: [{ name: 'IsPredicateUniversal', args: [] }],
    relations: [
      { predicate: 'conditions', from: 'predicate', to: 'mediation' },
    ],
  },
  {
    id: 'j-op-3-formalize-subject-singular',
    chunkId: 'j-3-subject-abstract-singular',
    label: 'Formalize subject as abstract singular (immediate)',
    clauses: [
      'subject.role = singularImmediate',
      'subject.represents = externalSideOfConcept',
      'subject.lacks = enactedMediation',
    ],
    predicates: [{ name: 'IsSubjectSingular', args: [] }],
    relations: [
      { predicate: 'contrastsWith', from: 'subject', to: 'predicate' },
    ],
  },
  {
    id: 'j-op-4-copula-immediacy-op',
    chunkId: 'j-4-copula-immediacy',
    label: 'Encode the copula as immediate being (no mediation)',
    clauses: [
      'copula.meaning = immediateBeing',
      'copula.contains = noNegationNoMediation',
      'copula.role => labelsConnection(subject,predicate)',
    ],
    predicates: [{ name: 'IsImmediateCopula', args: [] }],
    relations: [
      { predicate: 'asserts', from: 'copula', to: 'subjectPredicateLink' },
    ],
  },
  {
    id: 'j-op-5-note-presupposed-mediation',
    chunkId: 'j-5-positivity-and-mediation-presupposed',
    label: 'Flag presupposition: mediation not yet enacted',
    clauses: [
      'positiveJudgment.presupposes = mediation',
      'mediation.mustBeRealizedForDeterminateJudgment',
      'positiveJudgment = groundworkForFurtherDetermination',
    ],
    predicates: [{ name: 'PresupposesMediation', args: [] }],
    relations: [
      {
        predicate: 'prepares',
        from: 'positiveJudgment',
        to: 'determinateJudgment',
      },
    ],
  },
  {
    id: 'j-op-6-declare-proposition',
    chunkId: 'j-6-prop-singular-universal',
    label: 'Declare canonical proposition of positive judgment',
    clauses: [
      'proposition.form = "singular is universal"',
      'A_is_B_form = insufficientWithoutDetermination',
      'judgment.asserts = singular→universal (form)',
    ],
    predicates: [{ name: 'IsPropositionSingularUniversal', args: [] }],
    relations: [
      {
        predicate: 'expresses',
        from: 'proposition',
        to: 'positiveJudgmentForm',
      },
    ],
  },
  {
    id: 'j-op-7-objective-meaning',
    chunkId: 'j-7-universal-resolves-into-singular',
    label:
      'Formalize objective meaning: perishability and universal subsistence',
    clauses: [
      'singular.perishable = true',
      'universal.subsistsInConcept = true',
      'judgment = resolution(universal -> singular)',
    ],
    predicates: [{ name: 'FormalizesObjectiveMeaning', args: [] }],
    relations: [{ predicate: 'resolves', from: 'universal', to: 'singular' }],
  },
  {
    id: 'j-op-8-subject-concreteness',
    chunkId: 'j-8-subject-as-concrete',
    label: 'Encode subject as concrete totality of properties',
    clauses: [
      'subject.contains = {manifoldProperties}',
      'predicate.is = oneIsolatedPropertyOf(subject)',
      'subject.refersTo = universalImmanently',
    ],
    predicates: [{ name: 'IsSubjectConcrete', args: [] }],
    relations: [{ predicate: 'contains', from: 'subject', to: 'properties' }],
  },
  {
    id: 'j-op-9-reciprocity-result',
    chunkId: 'j-9-reciprocity-and-result',
    label: 'Juxtapose reciprocal determinations to yield single judgment',
    clauses: [
      'if subject→universal and predicate→singular then positiveJudgment = unified',
      'twoPropositionsCollapse -> singlePositiveJudgment',
      'reciprocity = mutualDetermination(subject,predicate)',
    ],
    predicates: [{ name: 'IsReciprocalDetermination', args: [] }],
    relations: [
      { predicate: 'unifies', from: 'reciprocity', to: 'positiveJudgment' },
    ],
  },
  {
    id: 'j-op-10-form-content-flag',
    chunkId: 'j-10-form-vs-content-unity',
    label: 'Flag implicit difference of form vs content and its unity',
    clauses: [
      'form = singularIsUniversal (immediate)',
      'content = universalIsSingular (result of immanent reflection)',
      'positiveJudgment.includes = {form, content} (implicitly distinct)',
    ],
    predicates: [{ name: 'FlagsFormContentUnity', args: [] }],
    relations: [
      {
        predicate: 'containsImplicitly',
        from: 'positiveJudgment',
        to: 'formAndContent',
      },
    ],
  },
  {
    id: 'j-op-11-particularity-empty',
    chunkId: 'j-11-union-particularity',
    label: 'Flag attempted union → empty identical proposition',
    clauses: [
      'if form ∧ content combined externally then bothSides -> particular',
      'resultingProposition = "the particular is the particular" (empty)',
      'emptyIdentical => judgmentSublated',
    ],
    predicates: [{ name: 'IsEmptyIdentity', args: [] }],
    relations: [
      {
        predicate: 'signals',
        from: 'combinedFormContent',
        to: 'judgmentFailure',
      },
    ],
  },
  {
    id: 'j-op-12-preserve-distinction',
    chunkId: 'j-12-form-vs-content-distinction',
    label:
      'Preserve form/content distinction; recognize limits of positive judgment',
    clauses: [
      'positiveJudgment => retains(form, content) as distinct aspects',
      'subject.immediacy != universalInFact',
      'content.asUniversal -> subjectIsBadInfinitePlurality',
    ],
    predicates: [{ name: 'PreservesFormContentDistinction', args: [] }],
    relations: [
      {
        predicate: 'maintains',
        from: 'positiveJudgment',
        to: 'formContentTension',
      },
    ],
  },
  {
    id: 'j-op-13-promote-to-negative',
    chunkId: 'j-13-positivity-becomes-negative',
    label:
      'When reciprocity shows inadequacy, re‑pose the positive as negative (mediate)',
    clauses: [
      'if form-content-reciprocity => contradictionWithImmediacy then reclassify(judgment, negative)',
      'negativeJudgment = introduceMediationAndDetermination',
      'transition => enables determinate judgment',
    ],
    predicates: [{ name: 'RequiresNegativeRepose', args: [] }],
    relations: [
      {
        predicate: 'transitionsTo',
        from: 'positiveJudgment',
        to: 'negativeJudgment',
      },
    ],
  },
];
