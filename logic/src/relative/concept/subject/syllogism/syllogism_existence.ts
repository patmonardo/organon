import type { Chunk, LogicalOperation } from './index'

/**
 * Syllogism of Existence — First Figure (part 1)
 * Chunkified and HLO-extracted. Two more parts will be appended.
 */

/* CHUNKS (concise, editable units) */
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'syll-ex1-1-overview',
    title: 'First figure — overview',
    text: `S-P-U is the general schema: singularity (S) connects to universality (U) through particularity (P). The extreme terms stand over against each other and are joined in a distinct third term (the particular).`
  },
  {
    id: 'syll-ex1-2-schema-and-relations',
    title: 'Schema and immediate relations',
    text: `Singularity is not universal immediately but via particularity; universality lowers itself to the singular through particularity. Extremes are determinateness and share universal determinateness in the particular.`
  },
  {
    id: 'syll-ex1-3-general-meaning-emergence',
    title: 'General meaning: emergence into existence',
    text: `The singular, an inward self-reference, emerges into existence via particularity into a universality where it stands in external conjunction; conversely, particularity concretizes singularity into a self-referring universal.`
  },
  {
    id: 'syll-ex1-4-objective-significance-not-yet-positied',
    title: 'Objective significance is initially superficial',
    text: `In the first figure the objective significance is only superficially present: determinations are not yet posited as the unity constituting syllogistic essence, so the syllogism remains subjective (abstract terms lack being-for-themselves).`
  },
  {
    id: 'syll-ex1-5-deficiency-and-form-relation',
    title: 'Deficiency not in form but in content richness',
    text: `The form-relation (singularity, particularity, universality) is essential; the deficiency is that each determination is not concurrently richer under the others — terms lack the fuller content required for true objectivity.`
  },
  {
    id: 'syll-ex1-6-aristotle-and-inherence',
    title: 'Aristotle’s inherence account (formalistic view)',
    text: `Aristotle formulates syllogism as repeated inherence relations (one extreme in middle; middle in other extreme) — a repetition of equal inherence rather than the determinateness of the three terms to one another.`
  },
  {
    id: 'syll-ex1-7-relation-of-figures-to-first',
    title: 'Other figures reduce to the first or develop it',
    text: `Other figure-relations are valid only insofar as they reduce to the original relation; when they deviate, they are transformations of the first abstract form, which then determines itself further and becomes totality.`
  },
  {
    id: 'syll-ex1-8-schema-restated-and-example',
    title: 'Restatement of S-P-U and example',
    text: `S is subsumed under P, P under U; thus S under U. The middle can be subject or predicate depending on perspective. The syllogism is not merely three judgments (e.g., 'All humans are mortal; Gaius is human; therefore Gaius is mortal').`
  },
  {
    id: 'syll-ex1-9-critique-of-formalism-and-conclusion',
    title: 'Critique of formalism; unity in fact',
    text: `Treating the inference as separate propositions is a subjective expedient that hides the unity of determinations. The syllogistic inference is the truth of judgment: the determinate relations are already a unity — "All things are a syllogism."`
  }
]

// appended: First Figure — part 2 (qualitative side, contingency of middles)
CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex1-10-qualitative-side',
    title: 'Qualitative side — terms as content',
    text: `Consider the syllogism where terms are concrete content: the terms are singular determinacies (properties, relations). The singular is an immediate concrete subject; particularity is one of its determinacies; universality is a more abstract determinateness within the particular.`
  },
  {
    id: 'syll-ex1-11-manifoldness-and-middles',
    title: 'Manifoldness of determinations; many possible middles',
    text: `A concrete subject has an indeterminate manifold of determinacies; any of these may serve as middle term and attach the subject to different universals. The same middle can link to several predicates; choice of middle is accidental.`
  },
  {
    id: 'syll-ex1-12-contingency-and-contradiction-examples',
    title: 'Contingency leads to contradictory yet correct inferences (examples)',
    text: `Different middle terms yield different, equally correct inferences that can contradict: e.g., a painted wall inferred blue from one middle, yellow from another; senses vs spirituality for human predicates; gravitation vs centrifugal force for celestial motion; sociability vs individuality for political ends.`
  },
  {
    id: 'syll-ex1-13-formal-syllogism-unsatisfactory',
    title: 'Why the formal syllogism is unsatisfactory',
    text: `The formal syllogism is unsatisfactory because the form's abstractness forces a one-sidedness: it treats a single quality of a concept as if it were exhaustive. Kant's antinomies are similar: picking different determinations as ground yields opposing yet formally necessary results.`
  },
  {
    id: 'syll-ex1-14-conclusion-contingency-in-form',
    title: 'Conclusion: contingency rooted in form, not content',
    text: `The insufficiency is not merely contental; it arises from the abstract form that permits only one-side determinateness. Thus when a subject is given, it is contingent which determinateness the formal syllogism will infer from it.`
  }
)

// appended: First Figure — part 3 (form-connections, regress, mediation-shift, singular-as-mediator)
CANONICAL_CHUNKS.push(
  {
    id: 'syll-ex1-15-content-vs-form',
    title: 'Determinations: contentual immediacy vs determinations of form',
    text: `The determinations appear as immediate, contentual qualities, but their essence is formal: they are connections. These connections are (1) extremes→middle (the two premises) and (2) extremes→extremes (the mediated conclusion).`
  },
  {
    id: 'syll-ex1-16-premises-and-conclusion',
    title: 'Premises (immediate connections) vs mediated conclusion',
    text: `The immediate connections (propositio major and minor) are judgments and thus contradict the syllogism's demand that unity be posited; the true connection is the mediated conclusion, the syllogistic truth of the judgment.`
  },
  {
    id: 'syll-ex1-17-infinite-regress-of-proving-premises',
    title: 'Proof-regress: premises would need proving → bad infinity',
    text: `If premises must be proved, each proof yields new premises, producing a geometric progression to infinity. This regress repeats the original deficiency and indicates the need to sublate the infinite progression.`
  },
  {
    id: 'syll-ex1-18-mediation-reshaped',
    title: 'Mediation must change form: P-S-U and S-U-P',
    text: `Mediation cannot simply replicate S-P-U; to mediate P-U use S (P-S-U), to mediate S-P use U (S-U-P). The mediation form must shift so it does not reproduce the same deficient immediacy.`
  },
  {
    id: 'syll-ex1-19-singular-becomes-mediator',
    title: 'Singular elevated: conclusion posits S as universal mediator',
    text: `When the first syllogism concludes S-U, the singular is posited as a universal (it becomes mediating). The singular thus unites particularity and universality by being both particular and, through conclusion, a universal.`
  }
)

/* HLOs / Logical operations (kept focused and not over-analytical) */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'syll-ex1-op-1-declare-schema',
    chunkId: 'syll-ex1-1-overview',
    label: 'Declare S-P-U schema and role of P',
    clauses: ['schema = S-P-U', 'P.isMiddleTerm()', 'S.connectsTo(U) via P'],
    predicates: [{ name: 'IsSchema', args: ['S-P-U'] }],
    relations: [{ predicate: 'mediates', from: 'P', to: 'S+U' }]
  },
  {
    id: 'syll-ex1-op-2-particularity-function',
    chunkId: 'syll-ex1-2-schema-and-relations',
    label: 'Particularity grounds the mediation between S and U',
    clauses: ['S.notUniversalImmediate', 'U.notSingularImmediate', 'P.enables(conjunction)'],
    predicates: [{ name: 'IsParticularity', args: ['P'] }],
    relations: [{ predicate: 'enables', from: 'P', to: 'S-U-conjunction' }]
  },
  {
    id: 'syll-ex1-op-3-emergence-into-existence',
    chunkId: 'syll-ex1-3-general-meaning-emergence',
    label: 'Singular emerges into universality through P; particularity concretes singular',
    clauses: ['singular.selfReference => via(P) -> universality', 'particularity.concretizes(singularity)'],
    predicates: [{ name: 'EmergesInto', args: ['singular','universality'] }],
    relations: [{ predicate: 'concretizes', from: 'P', to: 'S' }]
  },
  {
    id: 'syll-ex1-op-4-objective-significance',
    chunkId: 'syll-ex1-4-objective-significance-not-yet-positied',
    label: 'Objective significance not yet posited; syllogism remains subjective',
    clauses: ['determinations.notPositedAsUnity', 'syllogism.status = subjective'],
    predicates: [{ name: 'IsSubjective', args: ['syllogism'] }],
    relations: [{ predicate: 'lacks', from: 'terms', to: 'beingForThemselves' }]
  },
  {
    id: 'syll-ex1-op-5-deficiency',
    chunkId: 'syll-ex1-5-deficiency-and-form-relation',
    label: 'Form is correct; deficiency is lack of richer determinations',
    clauses: ['formRelation = valid', 'deficiency = singleDetermination.lacksRicherContent'],
    predicates: [{ name: 'HasDeficiency', args: ['term'] }],
    relations: [{ predicate: 'isDueTo', from: 'deficiency', to: 'insufficientContent' }]
  },
  {
    id: 'syll-ex1-op-6-aristotle-inherence',
    chunkId: 'syll-ex1-6-aristotle-and-inherence',
    label: 'Aristotle: inherence repetition versus determinateness relation',
    clauses: ['Aristotle.form = repeatedInherence', 'this.ignores = determinatenessBetweenThreeTerms'],
    predicates: [{ name: 'IsInherenceView', args: ['Aristotle'] }],
    relations: [{ predicate: 'contrastsWith', from: 'inherenceView', to: 'determinatenessRelation' }]
  },
  {
    id: 'syll-ex1-op-7-figures-reduction',
    chunkId: 'syll-ex1-7-relation-of-figures-to-first',
    label: 'Other figures reduce to or develop the first figure',
    clauses: ['otherFigures.validOnlyIf(reducibleToFirst)', 'deviation => developmentOfFirst'],
    predicates: [{ name: 'ReducesTo', args: ['figure','first'] }],
    relations: [{ predicate: 'developsInto', from: 'firstAbstractForm', to: 'totality' }]
  },
  {
    id: 'syll-ex1-op-8-schema-restated',
    chunkId: 'syll-ex1-8-schema-restated-and-example',
    label: 'Restate subsumption chain and warn against three-proposition formalism',
    clauses: ['S.subsumedUnder(P)', 'P.subsumedUnder(U)', 'therefore S.subsumedUnder(U)', 'example.formalism = misleading'],
    predicates: [{ name: 'IsSubsumptionChain', args: ['S','P','U'] }],
    relations: [{ predicate: 'misleads', from: 'formalisticPresentation', to: 'subjectiveIllusion' }]
  },
  {
    id: 'syll-ex1-op-9-unity-in-fact',
    chunkId: 'syll-ex1-9-critique-of-formalism-and-conclusion',
    label: 'Syllogistic inference is the truth of judgment; unity precedes propositional steps',
    clauses: ['inference.isTruthOfJudgment', 'unityOfDeterminations = priorToSeparatedPropositions'],
    predicates: [{ name: 'IsTruthOfJudgment', args: ['syllogism'] }],
    relations: [{ predicate: 'grounds', from: 'unityOfDeterminations', to: 'judgmentTruth' }]
  },
  {
    id: 'syll-ex1-op-10-declare-qualitative',
    chunkId: 'syll-ex1-10-qualitative-side',
    label: 'Terms as singular determinacies (qualitative reading)',
    clauses: ['terms.mode = qualitative', 'terms.are = singularDeterminacies', 'P := onePropertyOf(S)'],
    predicates: [{ name: 'IsQualitative', args: ['syllogism'] }],
    relations: [{ predicate: 'takesAsMiddle', from: 'P', to: 'S' }]
  },
  {
    id: 'syll-ex1-op-11-multiple-middles',
    chunkId: 'syll-ex1-11-manifoldness-and-middles',
    label: 'Multiple possible middles attach S to different universals',
    clauses: ['S.determinacies = indeterminateManifold', 'forEach(d in S.determinacies) => d.canBeMiddle', 'sameMiddle.canLeadTo(manyPredicates)'],
    predicates: [{ name: 'HasManifold', args: ['S'] }],
    relations: [{ predicate: 'attachesTo', from: 'middle', to: 'universalSet' }]
  },
  {
    id: 'syll-ex1-op-12-contingency-examples',
    chunkId: 'syll-ex1-12-contingency-and-contradiction-examples',
    label: 'Different middles yield different, possibly contradictory but formally correct conclusions',
    clauses: ['middle1 -> conclusionA (correct)', 'middle2 -> conclusionB (correct)', 'conclusionA may contradict conclusionB'],
    predicates: [{ name: 'CanContradict', args: ['conclusions'] }],
    relations: [{ predicate: 'illustrates', from: 'examples', to: 'contingencyIssue' }]
  },
  {
    id: 'syll-ex1-op-13-form-abstractness-fault',
    chunkId: 'syll-ex1-13-formal-syllogism-unsatisfactory',
    label: 'Form’s abstractness causes one-sidedness; Kant analogy',
    clauses: ['form.isAbstract = true', 'abstractForm => content.oneSidedness', 'KantAntinomies ~ choosingDifferentDeterminatesAsGround'],
    predicates: [{ name: 'IsAbstractForm', args: ['formalSyllogism'] }],
    relations: [{ predicate: 'resembles', from: 'formalSyllogism', to: 'KantAntinomies' }]
  },
  {
    id: 'syll-ex1-op-14-contingency-rooted-in-form',
    chunkId: 'syll-ex1-14-conclusion-contingency-in-form',
    label: 'Contingency is consequence of form, not a content defect',
    clauses: ['insufficiency.cause = form.abstractness', 'givenSubject => contingentWhichDeterminateIsInferred'],
    predicates: [{ name: 'IsContingencyOfForm', args: ['syllogism'] }],
    relations: [{ predicate: 'resultsFrom', from: 'contingency', to: 'abstractForm' }]
  },
  {
    id: 'syll-ex1-op-15-content-vs-form',
    chunkId: 'syll-ex1-15-content-vs-form',
    label: 'Distinguish contentual immediacy from formal connection (premises vs conclusion)',
    clauses: [
      'determinations.mode = immediateContent',
      'essence(determinations) = formConnections',
      'connections = {extremes->middle (premises), extremes->extremes (conclusion)}'
    ],
    predicates: [
      { name: 'IsImmediateContent', args: ['determination'] },
      { name: 'IsFormConnection', args: ['determination'] }
    ],
    relations: [
      { predicate: 'connects', from: 'extremeA', to: 'middle' },
      { predicate: 'mediates', from: 'middle', to: 'extremesPair' }
    ]
  },

  {
    id: 'syll-ex1-op-16-premises-are-judgments',
    chunkId: 'syll-ex1-16-premises-and-conclusion',
    label: 'Premises are immediate judgments and contradict syllogistic demand for posited unity',
    clauses: [
      'premises.type = propositions',
      'syllogism.requires = unityPositedInMiddle',
      'premises.asImmediate => contradictNatureOfSyllogism'
    ],
    predicates: [
      { name: 'IsProposition', args: ['premise'] },
      { name: 'RequiresUnity', args: ['syllogism'] }
    ],
    relations: [
      { predicate: 'contradicts', from: 'premises', to: 'syllogisticRequirement' }
    ]
  },

  {
    id: 'syll-ex1-op-17-infinite-regress',
    chunkId: 'syll-ex1-17-infinite-regress-of-proving-premises',
    label: 'Proving premises yields infinite regress (bad infinity) that must be sublated',
    clauses: [
      'prove(premise) => produces(newPremises)',
      'iteration => geometricProgressionToInfinity',
      'badInfinity => indicatesDeficiencyOfForm'
    ],
    predicates: [
      { name: 'IsBadInfinity', args: ['regress'] },
      { name: 'IndicatesDeficiency', args: ['form'] }
    ],
    relations: [
      { predicate: 'resultsIn', from: 'proofIteration', to: 'infiniteProgression' }
    ]
  },

  {
    id: 'syll-ex1-op-18-mediation-reshaped',
    chunkId: 'syll-ex1-18-mediation-reshaped',
    label: 'Mediation must change form (use P-S-U or S-U-P) to avoid replication',
    clauses: [
      'if mediation.repeats(S-P-U) then regressContinues',
      'mediate(P-U) via S => form = P-S-U',
      'mediate(S-P) via U => form = S-U-P'
    ],
    predicates: [
      { name: 'AvoidsReplication', args: ['mediation'] },
      { name: 'IsAlternativeMediation', args: ['P-S-U|S-U-P'] }
    ],
    relations: [
      { predicate: 'transforms', from: 'originalForm', to: 'alternativeForm' }
    ]
  },

  {
    id: 'syll-ex1-op-19-singular-as-mediator',
    chunkId: 'syll-ex1-19-singular-becomes-mediator',
    label: 'Conclusion elevates S to mediating universal (S-U) — singular unites determinations',
    clauses: [
      'conclusion(S-U) => S.positedAsUniversal',
      'S.inMinor = particularity',
      'S.inConclusion = unityOfExtremes'
    ],
    predicates: [
      { name: 'IsPositedAsUniversal', args: ['S'] },
      { name: 'UnitesDeterminations', args: ['S'] }
    ],
    relations: [
      { predicate: 'elevates', from: 'conclusion', to: 'singular' }
    ]
  }
]
