import type { Chunk, LogicalOperation } from './index'

// Canonical chunks for "A. ABSOLUTE GROUND — b. Form and matter"
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'agm-chunk-1-matter-as-identity',
    title: 'Matter as the simple identity (form’s other)',
    text: `Essence becomes matter when its reflection relates itself to essence as the formless indeterminate. Matter is the simple identity void of distinction, the other of form, and the proper substrate for form's determinations — the self-subsistent term to which form refers for positive subsistence.`
  },
  {
    id: 'agm-chunk-2-abstraction-and-absolute-abstract',
    title: 'Abstraction → matter is the absolutely abstract',
    text: `If one abstracts from every determination and form, what remains is matter: the absolutely abstract. Determinate, perceptible things are always matter+form; matter emerges by the form's reduction of itself to simple identity, not by an external removal of form.`
  },
  {
    id: 'agm-chunk-3-mutual-presupposition',
    title: 'Mutual presupposition of form and matter',
    text: `Form presupposes a matter and matter presupposes form; they do not confront each other externally. Matter is indifferent to form yet is the determinateness of self-identity to which form returns as substrate. Each is not the ultimate ground of the other.`
  },
  {
    id: 'agm-chunk-4-matter-as-passive-form-as-active',
    title: 'Matter passive / Form active — implicit reciprocity',
    text: `Matter is the passive, indifferent substrate; form is the active, self-referring negative (contradictory, self-dissolving, self-determining). Matter implicitly contains form (sublated negativity) and is the absolute receptivity for form; form must inform matter and acquire subsistence there.`
  },
  {
    id: 'agm-chunk-5-form-and-matter-not-ground-each-other',
    title: 'Neither is ground of the other; indifference as common determination',
    text: `Form and matter are alike determined not to be posited by each other. Matter is the identity of ground and grounded as substrate facing form. Their shared determination of indifference constitutes their reciprocal reference rather than a unilateral grounding relation.`
  },
  {
    id: 'agm-chunk-6-informing-and-materializing',
    title: 'Informing and materializing: form must give itself subsistence in matter',
    text: `Form posits itself as sublated and thus presupposes matter; matter contains implicit form and must be informed. Form must materialize itself, giving itself self-identity or subsistence in matter so the determinations of form obtain concrete subsistence.`
  },

  // --- added: part 2 of "Form and matter"
  {
    id: 'agm-chunk-7-mutual-self-mediation',
    title: 'Mutual self-mediation: form determines matter and vice versa',
    text: `Form determines matter and matter is determined by form: each implicitly contains the other. The activity of form on matter and matter's reception of form are the sublation of their apparent indifference — a self-mediation of each through its own non-being that restores their original identity.`
  },

  {
    id: 'agm-chunk-8-form-self-sublation-two-sides',
    title: 'Form’s two-sided self-sublation and union with matter',
    text: `Form is self-subsisting yet self-sublating; its sublation has two sides: it sublates its self-subsistence (becoming posited in an other, i.e., matter) and sublates its reference to matter to give itself subsistence — achieving identity by passing into and uniting with matter.`
  },

  {
    id: 'agm-chunk-9-activity-of-form-as-negative-relating',
    title: 'Activity of form = negative relating to itself; matter shares movement',
    text: `The activity by which matter is determined is a negative relating of form to itself. This is also the movement that belongs to matter: matter contains implicit determination and absolute negativity, so form’s activity and matter’s internal movement are one and the same.`
  },

  {
    id: 'agm-chunk-10-matter-self-contradiction-and-subsistence',
    title: 'Matter’s self-contradiction; subsistence through determination',
    text: `Matter is indeterminate self-identity and absolute negativity; it is self-contradictory and sublates itself within, allowing the negativity to obtain subsistence. Matter thereby attains determination by form while remaining the locus of the original unity-in-difference.`
  },

  {
    id: 'agm-chunk-11-externality-and-self-reference',
    title: 'Externality of relating and original unity; self-reference as reference-to-other',
    text: `The relating of form and matter appears as external because their original unity posits and presupposes itself; self-reference therefore manifests as reference to the self as sublated — a reference to its other — so that positing and presupposing are the same movement.`
  },

  // --- added: part 3 & transition (finalizing "Form and matter")
  {
    id: 'agm-chunk-12-restored-and-posited-unity',
    title: 'Restored original unity and its posited character',
    text: `Through the movement of form and matter the original unity is restored and becomes a posited unity. Matter is self-determining as much as it is determined by form; form determines itself while containing the matter it determines. The activity and movement are one, yielding the unity of in-itself and positedness: form is material, matter is necessarily formed.`
  },

  {
    id: 'agm-chunk-13-finitude-and-unity-as-truth',
    title: 'Finitude of form and matter; only their unity is truth',
    text: `Because form presupposes matter it is finite (an active factor, not a ground); matter presupposing form is finite as well. Neither finite form nor finite matter has truth alone — their unity is the truth. The determinations return to that unity and sublate their self-subsistence; the unity proves to be their ground.`
  },

  {
    id: 'agm-chunk-14-self-mediation-of-essence-as-ground',
    title: 'Self-mediation: matter determined by form through ground-unity',
    text: `The act by which matter is determined by form is the self-mediation of essence as ground: through itself and through the negation of itself. Matter becomes determined only insofar as it is the absolute unity of essence and form, and form grounds the subsistence of its determinations only insofar as it is that unity.`
  },

  {
    id: 'agm-chunk-15-informed-matter-and-negative-unity',
    title: 'Informed matter; the absolute ground as negative, posited unity',
    text: `Informed matter (form possessing subsistence) is the absolute unity of ground with itself, now also posited. The restored unity, having exhibited self-sublation, withdraws and repels itself and thus is negative unity — a determinate substrate: formed matter that nonetheless remains indifferent because sublated and unessential. This determinate substrate is the content.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'agm-op-1-matter-identity',
    chunkId: 'agm-chunk-1-matter-as-identity',
    label: 'Matter = simple identity; substrate for form',
    clauses: [
      'matter = essence.asSimpleIdentity',
      'matter.isOtherOf(form)',
      'matter.servesAs(substrateFor(form.determinations))'
    ],
    predicates: [
      { name: 'IsSimpleIdentity', args: ['matter'] },
      { name: 'IsOtherOf', args: ['matter','form'] },
      { name: 'IsSubstrateFor', args: ['matter','formDeterminations'] }
    ],
    relations: [
      { predicate: 'isOtherOf', from: 'matter', to: 'form' },
      { predicate: 'substrateFor', from: 'matter', to: 'form.determinations' }
    ]
  },

  {
    id: 'agm-op-2-abstraction-matter-abstract',
    chunkId: 'agm-chunk-2-abstraction-and-absolute-abstract',
    label: 'Abstraction from form → matter (absolutely abstract)',
    clauses: [
      'abstract(allFormDeterminations) => remainder == matter',
      'perceivedObjects = matter + form',
      'form.reductionToSimpleIdentity => matter.derivedBy(form)'
    ],
    predicates: [
      { name: 'AbstractsForm', args: ['process'] },
      { name: 'PerceivedIs', args: ['perceived','matter+form'] }
    ],
    relations: [
      { predicate: 'derives', from: 'matter', to: 'form.reduction' },
      { predicate: 'composedOf', from: 'perceivedObject', to: 'matter+form' }
    ]
  },

  {
    id: 'agm-op-3-mutual-presupposition',
    chunkId: 'agm-chunk-3-mutual-presupposition',
    label: 'Form ↔ Matter mutual presupposition; no external confrontation',
    clauses: [
      'form.presupposes(matter)',
      'matter.presupposes(form) (implicitly)',
      'neither.form.nor.matter = ultimateGroundOf(theOther)'
    ],
    predicates: [
      { name: 'Presupposes', args: ['form','matter'] },
      { name: 'MutualPresupposition', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'form', to: 'matter' },
      { predicate: 'implicitlyPresupposes', from: 'matter', to: 'form' }
    ]
  },

  {
    id: 'agm-op-4-passive-active-reciprocity',
    chunkId: 'agm-chunk-4-matter-as-passive-form-as-active',
    label: 'Matter passive, Form active; matter contains implicit form',
    clauses: [
      'matter.isPassiveAndIndifferent',
      'form.isActiveSelfReferringNegative',
      'matter.contains(sublatedNegativity) => implicitFormInside(matter)',
      'form.mustInform(matter) => form.materializes()'
    ],
    predicates: [
      { name: 'IsPassive', args: ['matter'] },
      { name: 'IsActiveNegative', args: ['form'] },
      { name: 'ContainsImplicitForm', args: ['matter'] }
    ],
    relations: [
      { predicate: 'contains', from: 'matter', to: 'implicitForm' },
      { predicate: 'informs', from: 'form', to: 'matter' }
    ]
  },

  {
    id: 'agm-op-5-indifference-and-reciprocal-reference',
    chunkId: 'agm-chunk-5-form-and-matter-not-ground-each-other',
    label: 'Indifference common determination → reciprocal reference',
    clauses: [
      'form.matterRelation = indifference',
      'indifference = determinationOf(matter)',
      'indifference => reciprocalReference(form,matter)'
    ],
    predicates: [
      { name: 'IsIndifference', args: ['relation'] },
      { name: 'ReciprocalReference', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'determines', from: 'indifference', to: 'matter' },
      { predicate: 'references', from: 'form', to: 'matter' }
    ]
  },

  {
    id: 'agm-op-6-informing-materializing',
    chunkId: 'agm-chunk-6-informing-and-materializing',
    label: 'Form must inform matter; form gains subsistence in matter',
    clauses: [
      'form.positsItselfAs(sublated) => presupposes(matter)',
      'matter.isAbsoluteReceptivityFor(form)',
      'form.materializes() => form.givesSelfSubsistenceIn(matter)'
    ],
    predicates: [
      { name: 'PresupposesMatter', args: ['form'] },
      { name: 'IsReceptivity', args: ['matter'] },
      { name: 'Materializes', args: ['form'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'form', to: 'matter' },
      { predicate: 'materializesIn', from: 'form', to: 'matter' }
    ]
  },

  // --- added HLOs for part 2 of "Form and matter"
  {
    id: 'agm-op-7-mutual-self-mediation',
    chunkId: 'agm-chunk-7-mutual-self-mediation',
    label: 'Form ↔ Matter: mutual self-mediation restores original identity',
    clauses: [
      'form.containsImplicit(matter) && matter.containsImplicit(form)',
      'form.actOn(matter) && matter.receive(form) => sublationOfIndifference',
      'sublation => selfMediationEachThroughNonBeing && restorationOfIdentity'
    ],
    predicates: [
      { name: 'MutualContainment', args: ['form','matter'] },
      { name: 'SublationOfIndifference', args: ['relation'] },
      { name: 'RestoresIdentity', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'form', to: 'matter' },
      { predicate: 'restores', from: 'selfMediation', to: 'identity' }
    ]
  },

  {
    id: 'agm-op-8-form-self-sublation-two-sides',
    chunkId: 'agm-chunk-8-form-self-sublation-two-sides',
    label: 'Form self-sublates (two sides) → becomes posited and gives itself subsistence',
    clauses: [
      'form.isSelfSubsisting && form.isSelfSublating',
      'sideA: form.sublatesSelfSubsistence -> positednessIn(other=matter)',
      'sideB: form.sublatesReferenceTo(matter) -> givesSelfSubsistence',
      'union(form,matter) => formIsBothPositedAndSelfIdentical'
    ],
    predicates: [
      { name: 'IsSelfSubsisting', args: ['form'] },
      { name: 'SublatesToPositedness', args: ['form','matter'] }
    ],
    relations: [
      { predicate: 'sublatesInto', from: 'form', to: 'positedness' },
      { predicate: 'unitesWith', from: 'form', to: 'matter' }
    ]
  },

  {
    id: 'agm-op-9-activity-negative-relating',
    chunkId: 'agm-chunk-9-activity-of-form-as-negative-relating',
    label: 'Activity of form = negative self-relating; movement shared by matter',
    clauses: [
      'activityOfForm = negativeRelating(form, itself)',
      'matter.contains(implicitDetermination) && thusSharesMovement',
      'formActivity == matterMovement => singleContradictoryMovement'
    ],
    predicates: [
      { name: 'NegativeRelating', args: ['form'] },
      { name: 'SharesMovement', args: ['matter','form'] }
    ],
    relations: [
      { predicate: 'relatesNegatively', from: 'form', to: 'itself' },
      { predicate: 'sharesMovementWith', from: 'form', to: 'matter' }
    ]
  },

  {
    id: 'agm-op-10-matter-self-contradiction-subsistence',
    chunkId: 'agm-chunk-10-matter-self-contradiction-and-subsistence',
    label: 'Matter = indeterminate identity + absolute negativity → subsistence via determination',
    clauses: [
      'matter = indeterminateSelfIdentity && matter.hasAbsoluteNegativity',
      'matter.sublatesItselfWithin -> negativityObtainsSubsistence',
      'determinationBy(form) => matter.attainsDeterminationWhilePreservingUnity'
    ],
    predicates: [
      { name: 'IsIndeterminateIdentity', args: ['matter'] },
      { name: 'HasAbsoluteNegativity', args: ['matter'] }
    ],
    relations: [
      { predicate: 'sublatesWithin', from: 'matter', to: 'negativity' },
      { predicate: 'attainsDeterminationBy', from: 'matter', to: 'form' }
    ]
  },

  {
    id: 'agm-op-11-externality-self-reference',
    chunkId: 'agm-chunk-11-externality-and-self-reference',
    label: 'External relating = self-reference to sublated self; positing = presupposing',
    clauses: [
      'originalUnity.presupposesDifferentia -> relatingAppearsExternal',
      'selfReference -> referenceTo(selfAsSublated) == referenceToOther',
      'positing == presupposing (same movement)'
    ],
    predicates: [
      { name: 'AppearsExternal', args: ['relating'] },
      { name: 'SelfReferenceToSublated', args: ['self'] },
      { name: 'PositingEqualsPresupposing', args: ['movement'] }
    ],
    relations: [
      { predicate: 'appearsAs', from: 'relating', to: 'externality' },
      { predicate: 'isReferenceTo', from: 'selfReference', to: 'sublatedSelf' }
    ]
  },

  // --- added HLOs: part 3 & transition
  {
    id: 'agm-op-12-restored-posited-unity',
    chunkId: 'agm-chunk-12-restored-and-posited-unity',
    label: 'Movement of form & matter → restored unity & posited unity; matter/form interchange',
    clauses: [
      'movement(form, matter) => restore(originalUnity)',
      'restoredUnity.isPosited = true',
      'form.contains(matter) && matter.selfDetermines => unityOf(inItself, positedness)'
    ],
    predicates: [
      { name: 'RestoresUnity', args: ['movement'] },
      { name: 'IsPositedUnity', args: ['unity'] }
    ],
    relations: [
      { predicate: 'restores', from: 'movement', to: 'originalUnity' },
      { predicate: 'yields', from: 'restoredUnity', to: 'positedness' }
    ]
  },

  {
    id: 'agm-op-13-finitude-unity-truth',
    chunkId: 'agm-chunk-13-finitude-and-unity-as-truth',
    label: 'Form & matter finite; only their unity is truth; unity subsumes determinations',
    clauses: [
      'form.presupposes(matter) => form.isFinite',
      'matter.presupposes(form) => matter.isFinite',
      'truth := unity(form,matter)',
      'determinations.returnTo(unity) => sublationOfSelfSubsistence'
    ],
    predicates: [
      { name: 'IsFinite', args: ['formOrMatter'] },
      { name: 'UnityIsTruth', args: ['unity'] }
    ],
    relations: [
      { predicate: 'isFinite', from: 'form', to: 'true' },
      { predicate: 'isFinite', from: 'matter', to: 'true' },
      { predicate: 'subsume', from: 'unity', to: 'determinations' }
    ]
  },

  {
    id: 'agm-op-14-self-mediation-ground-unity',
    chunkId: 'agm-chunk-14-self-mediation-of-essence-as-ground',
    label: 'Determination of matter by form = self-mediation of essence as ground',
    clauses: [
      'act(determineMatterByForm) := selfMediation(essenceAsGround)',
      'thisMediation => determinationOnlyIf(essence.isAbsoluteUnity(form,matter))',
      'form.groundsSubsistenceOnlyIf(form.isThatUnity)'
    ],
    predicates: [
      { name: 'IsSelfMediation', args: ['act'] },
      { name: 'RequiresUnity', args: ['determination'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'essence', to: 'formMatterRelation' },
      { predicate: 'requires', from: 'determination', to: 'absoluteUnity' }
    ]
  },

  {
    id: 'agm-op-15-informed-matter-negative-unity',
    chunkId: 'agm-chunk-15-informed-matter-and-negative-unity',
    label: 'Informed matter/form = absolute ground posited; negative unity = formed matter as content',
    clauses: [
      'informedMatter := matterWith(form.subsistence)',
      'absoluteGround.exhibits(selfSublation) => withdrawnNegativeUnity',
      'resultingContent := formedMatter.indifferentBecauseSublated'
    ],
    predicates: [
      { name: 'InformedMatter', args: ['matter'] },
      { name: 'NegativeUnity', args: ['ground'] },
      { name: 'FormedMatterContent', args: ['content'] }
    ],
    relations: [
      { predicate: 'informs', from: 'form', to: 'matter' },
      { predicate: 'constitutes', from: 'negativeUnity', to: 'content' }
    ]
  }
]
