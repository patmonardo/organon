import type { Chunk, LogicalOperation } from './index'

// Canonical chunks for "A. ABSOLUTE GROUND — a. Form and essence"
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ag-chunk-1-intro',
    title: 'Absolute Ground — introduction',
    text: `The determination of reflection that returns into ground is the first immediate existence from which beginning is made. Existence here still means positedness and presupposes ground: the immediate is posited while the ground is the non-posited.`
  },
  {
    id: 'ag-chunk-2-reflection-and-positedness',
    title: 'Reflection, positing and sublation',
    text: `The positing rebounds on the positor; the positing is a sublating of itself. As sublated determinateness the ground is essence determined through itself, yet determined as indeterminate — essence identical with itself in its negativity.`
  },
  {
    id: 'ag-chunk-3-twofold-determinateness',
    title: 'Twofold determinateness: ground and grounded',
    text: `Determinateness of essence is twofold: (1) essence as ground (non-posited), and (2) the grounded, the immediate as positedness. Both are identical with themselves but in the identity of the negative with itself; the ground is the self-identity of positedness.`
  },
  {
    id: 'ag-chunk-4-mediation-unity-of-reflections',
    title: 'Mediation: unity of pure and determining reflection',
    text: `This mediation is neither pure reflection nor purely determining reflection but their unity: determinations have a posited self-subsistence and that subsistence is itself posited. Thus the determinations are distinguished from simple identity and constitute form as against essence.`
  },
  {
    id: 'ag-chunk-5-essence-form-and-determinations',
    title: 'Essence, form, and determinate immediacy',
    text: `Essence has form and determinations of form; only as ground does it have fixed immediacy. Essence is inseparable from its movement and is not a substrate preceding reflection; determinations of form rest on essence as an indeterminate that is indifferent to them.`
  },
  {
    id: 'ag-chunk-6-form-determinations-and-ground-connection',
    title: 'Form-determinations: identity, difference, ground-connection',
    text: `Form-determinations (identity, difference, diversity/opposition) include the ground-connection: through it essence is posited while remaining the substrate. The immanent identity with ground does not belong to form; ground posits subsistence for form.`
  },
  {
    id: 'ag-chunk-7-reciprocal-reference-form-and-essence',
    title: 'Reciprocal connecting reference of form and essence',
    text: `Essence is the simple unity of ground and grounded and thereby becomes itself a moment of form. In this absolute reciprocal reference the substrate (essence) and determinate form mutually constitute one another: form is grounded and ground becomes a moment of form.`
  },
  {
    id: 'ag-chunk-8-form-completed-whole',
    title: 'Form as the completed whole of reflection',
    text: `Form is the completed whole of reflection; it contains the determination that it is sublated and is therefore a unity of its determining moments while being referred to a sublatedness outside itself.`
  },
  {
    id: 'ag-chunk-9-form-vs-essence-and-positionality',
    title: 'Form (positing) vs simple essence (inert substrate)',
    text: `As self-referring negativity, form is positing and determining; simple essence is an indeterminate, inert substrate in which form's determinations have their subsistence and are reflected into themselves. The distinction of form and essence is necessary but their distinguishing is itself their unity.`
  },
  {
    id: 'ag-chunk-10-form-as-absolute-negativity',
    title: 'Form as absolute negativity; identity of form and essence',
    text: `Form is the negative absolute self-identity by virtue of which essence is essence; abstractly taken, this identity appears as essence against form, yet in truth the whole self-referring negativity is simple essence — form has essence within its identity and essence has form in its negative nature.`
  },
  {
    id: 'ag-chunk-11-form-makes-determination-positedness',
    title: 'Form’s determining = making determination posited',
    text: `Form is the internal reflective shining of essence; in its determining, form makes determinations into positedness as positedness — the movement by which determinations acquire their positive, reflected status.`
  },

  // --- added: part 3 & transition
  {
    id: 'ag-chunk-12-form-sublates-distinguishing',
    title: 'Form sublates the distinguishing; determines essence by self-sublation',
    text: `Form does not stand prior to essence as an external presupposition; rather, by distinguishing it sublates that distinction and becomes the ground of its own determinations. Thus form determines essence by making the distinguishing itself into a sublated self-identity.`
  },

  {
    id: 'ag-chunk-13-form-as-groundful-essence',
    title: 'Form as groundful essence — self-identity in determination',
    text: `Form is the contradiction of being both posited (and sublated) and yet having subsistence in that sublatedness. It is ground as essence, self-identical through being determined or negated; the determination is the mode in which essence has subsistence.`
  },

  {
    id: 'ag-chunk-14-transition-form-essence-to-matter',
    title: 'Transition — form and essence as moments; essence determined as matter',
    text: `The distinctions of form and essence are moments of a single reference of form to itself. Determining form refers to itself as sublated positedness and thereby presupposes its identity; in this aspect essence appears as the indeterminate other, as formless identity — matter.`
  }
]

// HLO — Logical operations derived from the chunks
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ag-op-1-intro',
    chunkId: 'ag-chunk-1-intro',
    label: 'Reflection returns → immediate positedness presupposes ground',
    clauses: [
      'reflection.returnsInto(ground)',
      'immediate.isPositedness && positedness.presupposes(ground)',
      'ground = nonPosited'
    ],
    predicates: [
      { name: 'ReturnsInto', args: ['reflection','ground'] },
      { name: 'IsPositedness', args: ['immediate'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'immediate', to: 'ground' },
      { predicate: 'is', from: 'ground', to: 'nonPosited' }
    ]
  },

  {
    id: 'ag-op-2-reflection-positedness-sublation',
    chunkId: 'ag-chunk-2-reflection-and-positedness',
    label: 'Positing rebounds → positing is self-sublation; ground = essence-in-negativity',
    clauses: [
      'positing.reboundsOn(positor)',
      'positing == sublatingOfItself',
      'ground = essence.determinedThroughItself(asIndeterminate)'
    ],
    predicates: [
      { name: 'ReboundsOn', args: ['positing','positor'] },
      { name: 'IsSublation', args: ['positing'] },
      { name: 'EssenceDetermined', args: ['ground'] }
    ],
    relations: [
      { predicate: 'reboundOn', from: 'positing', to: 'positor' },
      { predicate: 'isSublated', from: 'ground', to: 'essence' }
    ]
  },

  {
    id: 'ag-op-3-twofold-determinateness',
    chunkId: 'ag-chunk-3-twofold-determinateness',
    label: 'Twofold determinateness → ground vs grounded identity',
    clauses: [
      'essence.asGround = nonPositedness',
      'grounded = immediate.asPositedness',
      'identityOfNegative && identityOfPositive => singleIdentity'
    ],
    predicates: [
      { name: 'IsGround', args: ['essence'] },
      { name: 'IsGrounded', args: ['immediate'] },
      { name: 'NegativeIdentity', args: ['ground','grounded'] }
    ],
    relations: [
      { predicate: 'contrasts', from: 'ground', to: 'positedness' },
      { predicate: 'identicalWith', from: 'ground', to: 'grounded' }
    ]
  },

  {
    id: 'ag-op-4-mediation-unity',
    chunkId: 'ag-chunk-4-mediation-unity-of-reflections',
    label: 'Mediation = unity of pure & determining reflection; determinations self-subsist as posited',
    clauses: [
      'mediation = pureReflection ∪ determiningReflection',
      'determinations.havePositedSubsistence',
      'positedSubsistence => determinationsDistinguishedFromSimpleIdentity'
    ],
    predicates: [
      { name: 'IsMediation', args: ['mediation'] },
      { name: 'HasPositedSubsistence', args: ['determinations'] }
    ],
    relations: [
      { predicate: 'unites', from: 'mediation', to: 'pureReflection+determiningReflection' },
      { predicate: 'givesSubsistence', from: 'mediation', to: 'determinations' }
    ]
  },

  {
    id: 'ag-op-5-essence-form-determinations',
    chunkId: 'ag-chunk-5-essence-form-and-determinations',
    label: 'Essence inseparable from movement; form determinations rest on indeterminate essence',
    clauses: [
      'essence.isNotPriorToMovement',
      'form.determinations.haveFoundationIn(indeterminateEssence)',
      'determinations.reflectedIntoThemselves'
    ],
    predicates: [
      { name: 'NotPrior', args: ['essence','movement'] },
      { name: 'FoundationIn', args: ['form','essence'] }
    ],
    relations: [
      { predicate: 'restsOn', from: 'form', to: 'essence' },
      { predicate: 'reflectsInto', from: 'determinations', to: 'themselves' }
    ]
  },

  {
    id: 'ag-op-6-form-determinations-ground-connection',
    chunkId: 'ag-chunk-6-form-determinations-and-ground-connection',
    label: 'Form determinations (identity/difference) and ground-connection as form-determination',
    clauses: [
      'form.determinations = {identity, difference, diversity, opposition}',
      'groundConnection ∈ form.determinations',
      'ground.positsSubsistenceFor(form)'
    ],
    predicates: [
      { name: 'FormDeterminations', args: ['form'] },
      { name: 'Includes', args: ['form','groundConnection'] }
    ],
    relations: [
      { predicate: 'includes', from: 'form', to: 'groundConnection' },
      { predicate: 'grounds', from: 'ground', to: 'form' }
    ]
  },

  {
    id: 'ag-op-7-reciprocal-reference',
    chunkId: 'ag-chunk-7-reciprocal-reference-form-and-essence',
    label: 'Reciprocal reference: essence = unity of ground + grounded → becomes moment of form',
    clauses: [
      'essence = unity(ground, grounded)',
      'in unity essence.distinguishesItselfAs(substrate) from form',
      'simultaneously essence.becomes(momentOf(form))'
    ],
    predicates: [
      { name: 'UnityOf', args: ['essence','ground+grounded'] },
      { name: 'BecomesMoment', args: ['essence','form'] }
    ],
    relations: [
      { predicate: 'distinguishesAs', from: 'essence', to: 'substrate' },
      { predicate: 'becomes', from: 'essence', to: 'form' }
    ]
  },

  {
    id: 'ag-op-8-form-completed-whole',
    chunkId: 'ag-chunk-8-form-completed-whole',
    label: 'Form = completed reflection; unity includes reference to sublatedness',
    clauses: [
      'form.isCompletedWholeOf(reflection)',
      'form.contains(determinationOfSublation)',
      'form.unity = unity(ofDeterminingMoments) referredTo(sublatedness)'
    ],
    predicates: [
      { name: 'IsCompletedWhole', args: ['form','reflection'] },
      { name: 'ContainsSublation', args: ['form'] }
    ],
    relations: [
      { predicate: 'contains', from: 'form', to: 'determinationOfSublation' },
      { predicate: 'refersTo', from: 'form', to: 'sublatedness' }
    ]
  },

  {
    id: 'ag-op-9-form-vs-essence',
    chunkId: 'ag-chunk-9-form-vs-essence-and-positionality',
    label: 'Form (positing) vs essence (substrate); their distinguishing is unity',
    clauses: [
      'form = selfReferringNegativity (positing, determining)',
      'essence = indeterminateSubstrate (inert, foundationForDeterminations)',
      'distinction(form, essence) => unityOfDistinguishing'
    ],
    predicates: [
      { name: 'IsSelfReferringNegativity', args: ['form'] },
      { name: 'IsIndeterminateSubstrate', args: ['essence'] },
      { name: 'DistinctionYieldsUnity', args: ['form','essence'] }
    ],
    relations: [
      { predicate: 'grounds', from: 'essence', to: 'determinations' },
      { predicate: 'distinguishes', from: 'form', to: 'essence' }
    ]
  },

  {
    id: 'ag-op-10-form-absolute-negativity',
    chunkId: 'ag-chunk-10-form-as-absolute-negativity',
    label: 'Form as negative absolute self-identity; mutual immanence of form and essence',
    clauses: [
      'form.isNegativeAbsoluteSelfIdentity',
      'identityAbstract(form, essence) => appearsAs(essenceAgainstForm)',
      'inTruth: selfReferringNegativity == simpleEssence',
      'form.contains(essence) && essenceContains(form) (mutual immanence)'
    ],
    predicates: [
      { name: 'IsNegativeAbsoluteIdentity', args: ['form'] },
      { name: 'MutualImmanence', args: ['form','essence'] }
    ],
    relations: [
      { predicate: 'contains', from: 'form', to: 'essence' },
      { predicate: 'imbues', from: 'essence', to: 'form' }
    ]
  },

  {
    id: 'ag-op-11-form-posits-determination',
    chunkId: 'ag-chunk-11-form-makes-determination-positedness',
    label: 'Form’s determining → makes determinations positedness',
    clauses: [
      'form = reflectiveShiningOf(essence)',
      'form.determining => make(determination, positedness)',
      'determination.positedness := determination.asPosited'
    ],
    predicates: [
      { name: 'ReflectiveShining', args: ['form','essence'] },
      { name: 'MakesPosited', args: ['form','determination'] }
    ],
    relations: [
      { predicate: 'makes', from: 'form', to: 'determination' },
      { predicate: 'posits', from: 'determination', to: 'positedness' }
    ]
  },

  // --- added HLOs: part 3 & transition
  {
    id: 'ag-op-12-form-sublates-distinguishing',
    chunkId: 'ag-chunk-12-form-sublates-distinguishing',
    label: 'Form sublates its distinguishing → determines essence by self-sublation',
    clauses: [
      'form.distinguishes(essence) => form.sublates(distinguishing)',
      'sublation => form.becomesGroundOf(itsDeterminations)',
      'determinationOfEssence := selfIdentityThroughSublation'
    ],
    predicates: [
      { name: 'SublatesDistinguishing', args: ['form'] },
      { name: 'BecomesGroundOf', args: ['form','determinations'] }
    ],
    relations: [
      { predicate: 'sublates', from: 'form', to: 'distinguishing' },
      { predicate: 'grounds', from: 'form', to: 'determinations' }
    ]
  },

  {
    id: 'ag-op-13-form-groundful-essence',
    chunkId: 'ag-chunk-13-form-as-groundful-essence',
    label: 'Form as groundful essence — self-identity within determination/negation',
    clauses: [
      'form.isPositedAndSublated && form.hasSubsistence',
      'form.asGround = essence.selfIdenticalInDetermination',
      'contradiction(positedness,susbistence) => formIsGround'
    ],
    predicates: [
      { name: 'IsPositedAndSublated', args: ['form'] },
      { name: 'SelfIdenticalInDetermination', args: ['essence'] }
    ],
    relations: [
      { predicate: 'isGroundAs', from: 'form', to: 'essence' },
      { predicate: 'sustains', from: 'form', to: 'subsistence' }
    ]
  },

  {
    id: 'ag-op-14-transition-form-essence-matter',
    chunkId: 'ag-chunk-14-transition-form-essence-to-matter',
    label: 'Transition: form refers to itself as sublated positedness → essence appears as matter',
    clauses: [
      'determiningForm.refersTo(selfAsSublatedPositedness)',
      'this.referring.presupposes(identity) => essence = indeterminateOther',
      'essence.asIndeterminate = matter (formless identity)'
    ],
    predicates: [
      { name: 'RefersToSelf', args: ['determiningForm'] },
      { name: 'EssenceAsMatter', args: ['essence'] }
    ],
    relations: [
      { predicate: 'refersTo', from: 'determiningForm', to: 'sublatedPositedness' },
      { predicate: 'appearsAs', from: 'essence', to: 'matter' }
    ]
  }
]
