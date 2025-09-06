import type {
  Chunk,
  LogicalOperation,
} from './index';

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'c3-chunk-1-absolute-ground-identity',
    title: 'Absolute ground identical with its condition',
    text: `The absolutely unconditioned is the absolute ground identical with its condition — the immediate fact as truly essential. As ground it refers negatively to itself and makes itself into a positedness that is a complete, self-identical reflection.`,
  },
  {
    id: 'c3-chunk-2-positedness-sublated-immediacy',
    title: 'Positedness as sublated ground; immediacy as side of conditions',
    text: `This positedness is first the sublated ground: the fact as immediacy void of reflection, the side of the conditions. It is the totality of the determinations of the fact, the fact thrown into the externality of being — the restored circle of being.`,
  },
  {
    id: 'c3-chunk-3-conditions-as-content-manifold',
    title: 'Conditions as whole content; manifold without unity',
    text: `Essence lets go of immanent unity and becomes immediacy that functions as a conditioning presupposition, constituting one side only. Thus conditions are the whole content of the fact (the unconditioned as formless being) yet appear as a manifold without unity, mingled with extra-essential elements and circumstances.`,
  },
  {
    id: 'c3-chunk-4-sphere-of-being-and-sublated-reflection',
    title: 'Sphere of being as condition; reflection in being',
    text: `For the absolute fact the sphere of being itself is the condition. The ground returning posits that sphere as first immediacy. This immediacy, as sublated reflection, is reflection in the element of being; form proliferates as determinateness of being distinct from the determination of reflection.`,
  },
  {
    id: 'c3-chunk-5-becoming-and-return',
    title: 'Becoming of being; return of essence to ground',
    text: `The unity of form sinks into immediacy as a passing-over of determinateness. The becoming of being is the coming-to-be of essence and a return to ground. Existence that constitutes the conditions makes itself into the moment of an other; its becoming is the doing of reflection.`,
  },
  {
    id: 'c3-chunk-6-truth-existence-reflective-shine',
    title:
      'Truth: existence is condition; immediacy/becoming are reflective moments',
    text: `The truth of existence is that it is condition; its immediacy is presupposed by the ground-connection's reflection (sublated). Immediacy and becoming are the reflective shine of the unconditioned and are only moments of form.`,
  },

  // --- added: part 2 of "Procession of the fact into concrete existence"
  {
    id: 'c3-chunk-7-ground-connection-form-vs-immediacy',
    title: 'Ground-connection: form as against immediacy of conditions',
    text: `The other side of the reflective shine is the ground-connection determined as form, opposed to the immediacy of the conditions and content. This side is the form of the absolute fact that unifies form with content, and by positing itself as form it sublates the diversity of the content, reducing it to a moment while granting itself immediacy of subsistence.`,
  },

  {
    id: 'c3-chunk-8-reflection-sublates-immediacy',
    title: 'Reflection of ground sublates immediacy; conditions become moments',
    text: `The reflection of the ground sublates the immediacy of the conditions by connecting them and making them moments within the fact's unity. The conditions are presupposed by the unconditioned fact, and the fact thereby sublates its own positing so that positing immediately converts into becoming.`,
  },

  {
    id: 'c3-chunk-9-positing-sublating-cycle-and-becoming',
    title: 'Positing ↔ Sublating ↔ Determination from within',
    text: `The reflection proceeds as presupposing, then immediately sublating itself into a positing that determines; that positing in turn sublates the presupposed and determines from within. The cycle is thus a becoming within itself — the mediation by negation has vanished into simple self-reflection and immanent becoming.`,
  },
  {
    id: 'c3-chunk-10-mediation-disappears-fact-self-staging',
    title: 'Mediation disappears; the fact self-stages in concrete existence',
    text: `Mediation has disappeared into reflective shining; the fact's movement of being posited through conditions and ground is the vanishing of mediation's shine. The process of positing is a coming-forth, the simple self-staging of the fact in concrete existence — the pure movement of the fact to itself.`,
  },

  // --- added: part 3 of "Procession of the fact into concrete existence"
  {
    id: 'c3-chunk-11-all-conditions-at-hand-recollection',
    title: 'All conditions at hand → internal recollection → fact steps into existence',
    text: `When all the conditions of a fact are at hand the scattered manifold recollects itself: the whole fact is present within its conditions and thereby steps into concrete existence. The totality of conditions constitutes the reflection of the fact and effects its internal recollection.`
  },

  {
    id: 'c3-chunk-12-essence-immediacy-twofold-presence',
    title: 'Fact antecedently: essence (unconditioned) and immediate existence (determinate)',
    text: `The fact is prior to concrete existence as essence (the unconditioned) and as immediate existence (determined). Immediate existence appears twofold — in the conditions and in the ground — and its determinations are determinations of reflection.`
  },

  {
    id: 'c3-chunk-13-posited-ground-sublated',
    title: 'Posited ground becomes sublated; groundless immediacy and foundering',
    text: `As absolute reflection the fact makes itself its presupposition, producing a groundless immediacy whose being is simply to be there. The recollecting of conditions is the foundering of immediate existence to ground and the coming-to-be of a posited ground, which, insofar as it is ground, is immediately sublated.`
  },

  {
    id: 'c3-chunk-14-coming-forth-disappearance-mediation',
    title: 'Coming-forth into concrete existence = disappearance of mediation',
    text: `If all conditions are present they sublate themselves as immediacy and presupposition and the ground is sublated; the reflective shine disappears. The coming-forth is thus the tautological movement of the fact to itself: its mediation through conditions and ground vanishes, and the fact's entry into concrete existence is immediate except for that disappearance.`
  },

  // --- added: final transition -> Idea of Concrete Existence
  {
    id: 'c3-chunk-15-transition-concrete-existence',
    title: 'Transition — Idea of Concrete Existence',
    text: `The fact proceeds from the ground; the positing is the outward movement of the ground to itself and the simple disappearing of it. Through union with the conditions the fact obtains external immediacy and the moment of being, not as alien addition but because the ground makes itself into a positedness and, by sublating its difference from that positedness, becomes simple essential immediacy. Thus the fact is both unconditioned and groundless: it arises only insofar as the ground has foundered and ceased to be a substrate. This immediacy — mediated by ground and condition but rendered self-identical through sublation — is concrete existence.`
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'c3-op-1-ground-identity',
    chunkId: 'c3-chunk-1-absolute-ground-identity',
    label:
      'AbsoluteGround ≡ Condition → positedness as self-identical reflection',
    clauses: [
      'absoluteGround.isIdenticalWith(condition)',
      'ground.refersNegativelyTo(ground)',
      'ground.positsItselfAs(positedness)',
      'positedness.isSelfIdentical(bothSides=true)',
    ],
    predicates: [
      { name: 'IsIdenticalWith', args: ['absoluteGround', 'condition'] },
      { name: 'RefersNegativelyTo', args: ['ground', 'itself'] },
      { name: 'PositsAs', args: ['ground', 'positedness'] },
    ],
    relations: [
      { predicate: 'identicalWith', from: 'absoluteGround', to: 'condition' },
      { predicate: 'posits', from: 'absoluteGround', to: 'positedness' },
    ],
  },

  {
    id: 'c3-op-2-positedness-sublation',
    chunkId: 'c3-chunk-2-positedness-sublated-immediacy',
    label: 'Positedness → sublated ground → immediacy (side of conditions)',
    clauses: [
      'positedness == sublatedGround',
      'sublatedGround.appearsAs(immediacyVoidOfReflection)',
      'immediacy.servesAs(sideOfConditions)',
      'fact.asTotality = determinations.thrownInto(externalBeing)',
    ],
    predicates: [
      { name: 'IsSublatedGround', args: ['positedness'] },
      { name: 'IsImmediacy', args: ['sublatedGround'] },
      { name: 'TotalityDeterminations', args: ['fact'] },
    ],
    relations: [
      { predicate: 'appearsAs', from: 'sublatedGround', to: 'immediacy' },
      { predicate: 'constitutes', from: 'immediacy', to: 'conditions' },
    ],
  },

  {
    id: 'c3-op-3-conditions-content-and-manifold',
    chunkId: 'c3-chunk-3-conditions-as-content-manifold',
    label:
      'Conditions = whole content; also manifold w/o unity and extra-essential admixture',
    clauses: [
      'conditions.areWholeContentOf(fact)',
      'conditions.presentAs(formlessBeing)',
      'conditions.include(extraEssentialElements, circumstances)',
      'manifoldWithoutUnity != unityOfForm',
    ],
    predicates: [
      { name: 'WholeContent', args: ['conditions', 'fact'] },
      { name: 'FormlessBeing', args: ['conditions'] },
      { name: 'Includes', args: ['conditions', 'extraEssentialElements'] },
    ],
    relations: [
      { predicate: 'constitutes', from: 'conditions', to: 'fact.content' },
      {
        predicate: 'contains',
        from: 'conditions',
        to: 'extraEssentialElements',
      },
    ],
  },

  {
    id: 'c3-op-4-sphere-being-reflection',
    chunkId: 'c3-chunk-4-sphere-of-being-and-sublated-reflection',
    label:
      'SphereOfBeing as condition; reflection-in-being creates determinateness',
    clauses: [
      'forAbsoluteFact: sphereOfBeing.isCondition = true',
      'ground.returning.posits(sphereOfBeing) as firstImmediacy',
      'sublatedReflection -> reflectionIn(elementOfBeing)',
      'form.proliferates => determinatenessOfBeing (distinctFromReflection)',
    ],
    predicates: [
      { name: 'SphereIsCondition', args: ['sphereOfBeing'] },
      { name: 'PositsAsImmediacy', args: ['ground', 'sphereOfBeing'] },
      { name: 'DeterminatenessOfBeing', args: ['form'] },
    ],
    relations: [
      { predicate: 'isCondition', from: 'sphereOfBeing', to: 'absoluteFact' },
      { predicate: 'posits', from: 'ground', to: 'sphereOfBeing' },
    ],
  },

  {
    id: 'c3-op-5-becoming-and-return',
    chunkId: 'c3-chunk-5-becoming-and-return',
    label: 'Becoming of being → coming-to-be of essence → return to ground',
    clauses: [
      'unityOfForm.sinksInto(immediacy) as passingOver',
      'becomingOfBeing => comingToBeOf(essence)',
      'existence.constitutingConditions makesItselfMomentOf(other)',
      'movementOfBecoming.isDoingOf(reflection)',
    ],
    predicates: [
      { name: 'PassingOver', args: ['unityOfForm', 'immediacy'] },
      { name: 'ComingToBe', args: ['essence'] },
      { name: 'DoesMovement', args: ['reflection'] },
    ],
    relations: [
      { predicate: 'sinksInto', from: 'unityOfForm', to: 'immediacy' },
      { predicate: 'returnsTo', from: 'essence', to: 'ground' },
    ],
  },

  {
    id: 'c3-op-6-truth-existence-reflective-shine',
    chunkId: 'c3-chunk-6-truth-existence-reflective-shine',
    label:
      'Truth: existence is condition; immediacy & becoming are reflective moments',
    clauses: [
      'truthOfExistence := existence.isCondition',
      'existence.immediacy.isPresupposedBy(groundConnectionReflection)',
      'immediacy.isMomentOf(form)',
      'becoming.isReflectiveShineOf(unconditioned)',
    ],
    predicates: [
      { name: 'IsConditionTruth', args: ['existence'] },
      { name: 'PresupposedImmediacy', args: ['immediacy', 'groundConnection'] },
      { name: 'ReflectiveShine', args: ['becoming'] },
    ],
    relations: [
      { predicate: 'isCondition', from: 'existence', to: 'truth' },
      { predicate: 'momentOf', from: 'immediacy', to: 'form' },
    ],
  },

  {
    id: 'c3-op-7-ground-connection-form-vs-immediacy',
    chunkId: 'c3-chunk-7-ground-connection-form-vs-immediacy',
    label: 'Ground-connection (form) vs immediacy (conditions): unifies and sublates content',
    clauses: [
      'groundConnection.isForm && conditions.areImmediacy',
      'groundConnection.unifies(form, content)',
      'groundConnection.sublates(diversityOfContent) => contentReducedTo(moment)',
      'form.grantsItself(immediacyOfSubsistence)',
    ],
    predicates: [
      { name: 'IsForm', args: ['groundConnection'] },
      { name: 'Unifies', args: ['groundConnection', 'content'] },
      { name: 'Sublates', args: ['groundConnection', 'content'] },
    ],
    relations: [
      { predicate: 'opposes', from: 'groundConnection', to: 'immediacy' },
      { predicate: 'reducesToMoment', from: 'groundConnection', to: 'content' },
    ],
  },

  {
    id: 'c3-op-8-reflection-sublates-immediacy',
    chunkId: 'c3-chunk-8-reflection-sublates-immediacy',
    label: 'Reflection sublates immediacy; conditions become moments of unity',
    clauses: [
      'reflectionOfGround.connects(conditions) => conditions.become(moments)',
      'fact.presupposes(conditions) && fact.sublates(itsPositing)',
      'positedness.convertsImmediatelyTo(becoming)',
    ],
    predicates: [
      { name: 'Connects', args: ['reflectionOfGround', 'conditions'] },
      { name: 'Presupposes', args: ['fact', 'conditions'] },
      { name: 'ConvertsTo', args: ['positedness', 'becoming'] },
    ],
    relations: [
      { predicate: 'connects', from: 'reflectionOfGround', to: 'conditions' },
      { predicate: 'sublates', from: 'fact', to: 'itsPositing' },
    ],
  },

  {
    id: 'c3-op-9-positing-sublating-cycle',
    chunkId: 'c3-chunk-9-positing-sublating-cycle-and-becoming',
    label: 'Presupposing → Positing → Sublating → Becoming (immanent cycle)',
    clauses: [
      'reflection.presupposes() -> presupposing',
      'presupposing.sublatesItselfImmediateAs(positing)',
      'positing.subsumes(presupposed) => determiningFromWithin',
      'cycle => becomingWithinItself (mediation disappears)',
    ],
    predicates: [
      { name: 'Presupposing', args: ['reflection'] },
      { name: 'Positing', args: ['reflection'] },
      { name: 'DeterminesFromWithin', args: ['positing'] },
    ],
    relations: [
      { predicate: 'sublatesInto', from: 'presupposing', to: 'positing' },
      { predicate: 'determines', from: 'positing', to: 'presupposed' },
    ],
  },

  {
    id: 'c3-op-10-mediation-disappears-self-staging',
    chunkId: 'c3-chunk-10-mediation-disappears-fact-self-staging',
    label: 'Disappearance of mediation → fact’s self-staging in concrete existence',
    clauses: [
      'mediation.asTurningBack.disappears -> mediation = simpleReflection',
      'fact.movement(positedThroughConditions, positedThroughGround) => disappearanceOfReflectiveShine',
      'processOfPositing == comingForth == fact.selfStaging()',
    ],
    predicates: [
      { name: 'Disappears', args: ['mediation'] },
      { name: 'SelfStaging', args: ['fact'] },
      { name: 'ComingForth', args: ['processOfPositing'] },
    ],
    relations: [
      { predicate: 'becomes', from: 'mediation', to: 'simpleReflection' },
      { predicate: 'selfStages', from: 'fact', to: 'concreteExistence' },
    ],
  },

  {
    id: 'c3-op-11-all-conditions-recollect',
    chunkId: 'c3-chunk-11-all-conditions-at-hand-recollection',
    label: 'All conditions at hand → recollection → fact steps into concrete existence',
    clauses: [
      'if conditions.totality.isPresent then manifold.recollectsInternally()',
      'recollection => fact.isPositedWithin(conditions) && fact.stepsInto(existence)',
      'totalityOfConditions == reflectionOfFact'
    ],
    predicates: [
      { name: 'TotalityPresent', args: ['conditions'] },
      { name: 'RecollectsInternally', args: ['manifold'] },
      { name: 'IsPositedWithin', args: ['fact','conditions'] }
    ],
    relations: [
      { predicate: 'recollects', from: 'conditions', to: 'fact' },
      { predicate: 'stepsInto', from: 'fact', to: 'concreteExistence' }
    ]
  },

  {
    id: 'c3-op-12-essence-and-immediacy-twofold',
    chunkId: 'c3-chunk-12-essence-immediacy-twofold-presence',
    label: 'Fact = essence (unconditioned) + immediate existence (determinate) — twofold determination',
    clauses: [
      'fact.existsAs(essence, immediateExistence)',
      'immediateExistence.appearsIn(conditions, ground)',
      'determinationsOfImmediate := determinationsOfReflection'
    ],
    predicates: [
      { name: 'ExistsAs', args: ['fact','modes'] },
      { name: 'AppearsIn', args: ['immediacy','locations'] },
      { name: 'DeterminationsOfReflection', args: ['determinations'] }
    ],
    relations: [
      { predicate: 'appearsAs', from: 'fact', to: 'essence' },
      { predicate: 'appearsAs', from: 'fact', to: 'immediateExistence' }
    ]
  },

  {
    id: 'c3-op-13-posited-ground-sublated',
    chunkId: 'c3-chunk-13-posited-ground-sublated',
    label: 'Posited ground (groundless immediacy) → sublation of ground when conditions total',
    clauses: [
      'fact.asAbsoluteReflection => presupposedUnconditioned = groundlessImmediacy',
      'if conditions.totality.present then immediate.recollected -> ground.positedButSublated',
      'ground.asPosited.isSublated -> reflectiveShine.disappears'
    ],
    predicates: [
      { name: 'GroundlessImmediacy', args: ['presupposedUnconditioned'] },
      { name: 'PositedButSublated', args: ['ground'] },
      { name: 'ReflectiveShineDisappears', args: ['ground'] }
    ],
    relations: [
      { predicate: 'isPresupposition', from: 'groundlessImmediacy', to: 'fact' },
      { predicate: 'sublates', from: 'ground', to: 'immediacy' }
    ]
  },

  {
    id: 'c3-op-14-coming-forth-disappearance-mediation',
    chunkId: 'c3-chunk-14-coming-forth-disappearance-mediation',
    label: 'Coming-forth into concrete existence = disappearance of mediation',
    clauses: [
      'conditions.totality.present => conditions.sublateAs(immediacy,presupposition)',
      'ground.sublated && conditions.sublated => reflectiveShine.disappears',
      'comingForth := tautologicalMovement(factToItself) = immediateEntryExceptFor(disappearanceOfMediation)'
    ],
    predicates: [
      { name: 'SublatesAs', args: ['conditions','immediacy'] },
      { name: 'ReflectiveShineVanishes', args: ['process'] },
      { name: 'TautologicalMovement', args: ['comingForth'] }
    ],
    relations: [
      { predicate: 'sublates', from: 'conditions', to: 'immediacy' },
      { predicate: 'vanishes', from: 'reflectiveShine', to: 'nothing' }
    ]
  },

  // --- added HLO: final transition -> Idea of Concrete Existence
  {
    id: 'c3-op-15-transition-concrete-existence',
    chunkId: 'c3-chunk-15-transition-concrete-existence',
    label: 'Fact proceeds from ground → positing = outward movement & disappearance → concrete existence',
    clauses: [
      'fact.proceedsFrom(ground)',
      'ground.positsItself() => outwardMovement && ground.disappearsAs(substrate)',
      'unionWith(conditions) => fact.obtains(externalImmediacy, momentOfBeing)',
      'ground.sublatesDifferenceWith(positedness) => essentialImmediacy',
      'fact.isUnconditioned && fact.isGroundless',
      'concreteExistence := immediacy.mediatedBy(ground,conditions) && immediacy.selfIdenticalBy(sublation)'
    ],
    predicates: [
      { name: 'ProceedsFrom', args: ['fact','ground'] },
      { name: 'OutwardMovement', args: ['ground'] },
      { name: 'EssentialImmediacy', args: ['ground','positedness'] },
      { name: 'ConcreteExistence', args: ['immediacy'] }
    ],
    relations: [
      { predicate: 'proceedsFrom', from: 'fact', to: 'ground' },
      { predicate: 'obtains', from: 'fact', to: 'externalImmediacy' },
      { predicate: 'sublates', from: 'ground', to: 'positedness' },
      { predicate: 'constitutes', from: 'immediacy', to: 'concreteExistence' }
    ]
  }
];

export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS;
}

export function getLogicalOpsForChunk(
  oneBasedIndex: number,
): LogicalOperation | null {
  const chunk = getChunk(oneBasedIndex);
  if (!chunk) return null;
  return LOGICAL_OPERATIONS.find((op) => op.chunkId === chunk.id) ?? null;
}
