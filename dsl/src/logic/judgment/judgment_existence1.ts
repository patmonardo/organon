import { type Chunk, type LogicalOperation } from './index';

/* appended: Negative Judgment — part 1 (critique of positive; particularity; determination of negation) */
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'j-neg-1-critical-form',
    title: 'Negative judgment — critique of positive form',
    text: `The positive judgment's form (singular↔universal) is insufficient for truth; its truth is attained only in the negative judgment because the purely formal positive abstracts extremes and conceals mediated determination.`,
  },
  {
    id: 'j-neg-2-particularity-emerges',
    title: 'Particularity as mediated determination',
    text: `The negative judgment mediates: the singular is shown to be a particular and the universal, in predication, is itself particular. This mediation reduces both propositions to "the singular is a particular."`,
  },
  {
    id: 'j-neg-3-negation-attached-to-predicate',
    title: 'Negation attaches to predicate → not-universal = particular',
    text: `The 'not' of the negative must be attached to the predicate; the negative-as-not-universal (when properly conceptualized) is determinate and becomes the particular rather than an indeterminate non-being.`,
  },
  {
    id: 'j-neg-4-conceptualize-not-vs-indeterminate-not',
    title: 'Conceptualized negation vs unconceptualized not',
    text: `An unconceptualized 'not' (not-white, not-triangular) is indeterminate and void; reflection transforms such non-being into determinate negation that refers essentially to a positive and is thereby circumscribed conceptually.`,
  },
  {
    id: 'j-neg-5-negation-as-positive-moment',
    title: 'Negative as positive-determined moment',
    text: `In the fluid continuity of the concept the 'not' becomes a positive moment; negation is taken up into universality and thus the non-universal is immediately the particular within the negative judgment.`,
  },
];

/* appended: Negative Judgment — part 2 (negation attached to predicate; universality retained; determinateness -> particularity) */
CANONICAL_CHUNKS.push(
  {
    id: 'j-neg-6-connection-preserves-subject',
    title: 'Negation preserves subject / connection',
    text: `Negation in the negative judgment attaches to the predicate; the subject, as immediate basis, remains untouched and retains its reference to universality. The judgment remains a relation of subject and predicate.`,
  },
  {
    id: 'j-neg-7-deny-determinateness-not-universality',
    title: 'Negation denies determinateness, not the universal sphere',
    text: `What is negated is the determinateness of the predicate (e.g. "the rose is not red"), not the universal sphere itself (color). The universal sphere remains standing while a different determinate is implied.`,
  },
  {
    id: 'j-neg-8-universal-retained-transform-determinateness',
    title: 'Universal retained; determinateness transformed into particularity',
    text: `The universal sphere is retained; determinateness is sublated into an indeterminate determinateness that is already particularity. Thus the negative judgment both preserves universality and mediates the rise of particularity.`,
  },
  {
    id: 'j-neg-9-positive-form-of-negative',
    title: 'Positive form of the negative: "the singular is a particular"',
    text: `The expression "the singular is a particular" is the positive articulation of the negative judgment: it shows the mediated determination that emerges when negation operates within the judgment connection.`,
  },
);

/* appended: Negative Judgment — part 3 (particularity as mediator; negation of negation; infinite judgment) */
CANONICAL_CHUNKS.push(
  {
    id: 'j-neg-10-particular-mediates',
    title: 'Particularity mediates singularity and universality',
    text: `The particularity that results from the negative judgment mediates singularity and universality, enabling the reflection of the judgment of existence back into itself. The negative judgment thus supplies the mediation for the subject's full determinateness.`,
  },
  {
    id: 'j-neg-11-negation-of-negation',
    title: 'Negation of negation — restoration of the concrete',
    text: `The negative judgment is itself a second negation: by negating the determinateness of the predicate and then sublating that negation, the subject's concrete totality is restored and the predicate attains absolute determinateness — the singular becomes singular.`,
  },
  {
    id: 'j-neg-12-infinite-judgment',
    title:
      'Infinite judgment: purifying universality and removing positive connection',
    text: `When the predicate's whole extent is negated there remains no positive connection to the subject — the universality is purified of limitation. This culminates in the infinite judgment, in which the predicate is negated entirely as a positive link and the self‑related universality stands reflected.`,
  },
);

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'j-op-neg-1-flag-positive-limit',
    chunkId: 'j-neg-1-critical-form',
    label: 'Flag limits of positive judgment; require mediation',
    clauses: [
      'if judgment.kind == positive then require(negativeMediation) for truth',
      'positive.form.abstractsExtremes => truthNotGiven',
    ],
    predicates: [{ name: 'PositiveRequiresNegative', args: [] }],
    relations: [
      {
        predicate: 'requires',
        from: 'positiveJudgment',
        to: 'negativeMediation',
      },
    ],
  },
  {
    id: 'j-op-neg-2-formalize-particularity',
    chunkId: 'j-neg-2-particularity-emerges',
    label: 'Formalize mediated emergence of particularity',
    clauses: [
      'on(negativeJudgment) => subject.isDeterminedAs(particular)',
      'predicate.asUniversal -> becomes(particularInPredication)',
    ],
    predicates: [{ name: 'MediatesToParticular', args: [] }],
    relations: [
      { predicate: 'yields', from: 'negativeJudgment', to: 'particularity' },
    ],
  },
  {
    id: 'j-op-neg-3-attach-negation-to-predicate',
    chunkId: 'j-neg-3-negation-attached-to-predicate',
    label:
      'Attach negation to predicate and interpret not-universal as determinate',
    clauses: [
      'negation.attachTo(predicate) => predicate = not-universal',
      'not-universal.conceptualized => particular',
    ],
    predicates: [{ name: 'NegationAsDeterminate', args: [] }],
    relations: [
      { predicate: 'transforms', from: 'negation', to: 'particular' },
    ],
  },
  {
    id: 'j-op-neg-4-conceptualize-not',
    chunkId: 'j-neg-4-conceptualize-not-vs-indeterminate-not',
    label: 'Distinguish indeterminate not from conceptual negation',
    clauses: [
      'if not is unconceptualized then indeterminateNonBeing = true',
      'reflection.conceptualize(nonBeing) => determinateNegation(refersToPositive)',
    ],
    predicates: [{ name: 'ConceptualizesNegation', args: [] }],
    relations: [
      { predicate: 'constrains', from: 'conceptualNegation', to: 'nonBeing' },
    ],
  },
  {
    id: 'j-op-neg-5-negation-as-positive',
    chunkId: 'j-neg-5-negation-as-positive-moment',
    label: 'Treat negation as positive-determined moment within universality',
    clauses: [
      'inConceptContinuity: negation => positiveMoment',
      'nonUniversal.immediately = particular',
    ],
    predicates: [{ name: 'NegationIntegrated', args: [] }],
    relations: [
      { predicate: 'identifies', from: 'nonUniversal', to: 'particular' },
    ],
  },
  {
    id: 'j-op-neg-6-preserve-subject',
    chunkId: 'j-neg-6-connection-preserves-subject',
    label: 'Negation attaches to predicate; subject preserved',
    clauses: [
      'negation.attachTo(predicate) => subject.unchanged',
      'subject.referenceTo(universal) = true',
      'judgment.connection.remains = true',
    ],
    predicates: [{ name: 'NegationPreservesSubject', args: [] }],
    relations: [
      { predicate: 'maintains', from: 'negation', to: 'subjectReference' },
    ],
  },
  {
    id: 'j-op-neg-7-deny-determinateness',
    chunkId: 'j-neg-7-deny-determinateness-not-universality',
    label: 'Formalize denial of determinateness (example: rose not red)',
    clauses: [
      'if judgment = "rose is not red" then universal(color).retained = true',
      'predicate.determinateness.denied => alternateDeterminatenessImplied',
    ],
    predicates: [{ name: 'DeniesDeterminateness', args: [] }],
    relations: [
      { predicate: 'implies', from: 'denial', to: 'alternateDeterminateness' },
    ],
  },
  {
    id: 'j-op-neg-8-universal-retained',
    chunkId: 'j-neg-8-universal-retained-transform-determinateness',
    label: 'Encode retention of universal and transformation to particularity',
    clauses: [
      'universalSphere.retained = true',
      'determinateness -> sublatedInto(particularity)',
      'negativeJudgment.yields = mediatedParticular',
    ],
    predicates: [{ name: 'UniversalRetainedParticularArises', args: [] }],
    relations: [
      { predicate: 'yields', from: 'negativeJudgment', to: 'particularity' },
    ],
  },
  {
    id: 'j-op-neg-9-articulate-positive-form',
    chunkId: 'j-neg-9-positive-form-of-negative',
    label:
      'Expose "the singular is a particular" as positive form of negative judgment',
    clauses: [
      'negativeJudgment.formulate -> "singular is particular"',
      'thisForm = mediatedDeterminationOfJudgment',
    ],
    predicates: [{ name: 'ArticulatesNegativeAsPositiveForm', args: [] }],
    relations: [
      {
        predicate: 'expresses',
        from: 'negativeJudgment',
        to: 'mediatedDetermination',
      },
    ],
  },
  {
    id: 'j-op-neg-10-mediates-third-step',
    chunkId: 'j-neg-10-particular-mediates',
    label:
      'Treat particularity as mediator for reflection of existence-judgment',
    clauses: [
      'particularity = mediator(singularity, universality)',
      'on(negativeJudgment) => enable(reflectionOfExistenceIntoItself)',
      'mediator.yields = fullerDeterminatenessOfSubject',
    ],
    predicates: [{ name: 'ParticularityMediates', args: [] }],
    relations: [
      { predicate: 'enables', from: 'particularity', to: 'selfReflection' },
    ],
  },
  {
    id: 'j-op-neg-11-second-negation',
    chunkId: 'j-neg-11-negation-of-negation',
    label: 'Formalize negation of negation restoring concrete identity',
    clauses: [
      'negativeJudgment = firstNegation(ofPredicateDeterminateness)',
      'sublationOfNegation => secondNegation -> restoration(concreteTotality)',
      'resultExpressions = {"singular is singular", "universal is universal"}',
    ],
    predicates: [{ name: 'NegationOfNegationRestores', args: [] }],
    relations: [
      { predicate: 'yields', from: 'sublation', to: 'concreteRestoration' },
    ],
  },
  {
    id: 'j-op-neg-12-assert-equality',
    chunkId: 'j-neg-11-negation-of-negation',
    label: 'Assert the resulting identity of extremes after mediation',
    clauses: [
      'after(secondNegation): subject.determinateness = predicate.determinateness',
      'judgmentStatements -> "singular is singular" and "universal is universal"',
      'these are moments of the mediated, purified determination',
    ],
    predicates: [{ name: 'AssertsPostMediationIdentity', args: [] }],
    relations: [
      {
        predicate: 'expresses',
        from: 'postMediationJudgment',
        to: 'identityMoment',
      },
    ],
  },
  {
    id: 'j-op-neg-13-form-infinite-judgment',
    chunkId: 'j-neg-12-infinite-judgment',
    label:
      'Formalize infinite judgment as purified universality and removed connection',
    clauses: [
      'if predicate.extent.negatedFully then positiveConnectionToSubject = none',
      'universality.purified = true',
      'this state = infiniteJudgment (self‑related universality)',
    ],
    predicates: [{ name: 'IsInfiniteJudgment', args: [] }],
    relations: [
      {
        predicate: 'constitutes',
        from: 'purifiedUniversality',
        to: 'infiniteJudgment',
      },
    ],
  },
];
