import { Chunk, LogicalOperation } from './index';

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'a-chunk-1-reflective-shine',
    title: 'Reflective shine & independence',
    text: `At first, each of the two relatively unconditioned sides reflectively shines in the other; condition, as an immediate, is reflected in the form-connection of the ground, and this form in the immediate existence as its positedness; but each, apart from this reflective shine of its other in it, stands out on its own and has a content of its own.`
  },
  {
    id: 'a-chunk-2-condition-form-moments',
    title: 'Condition as immediate existence; two form-moments',
    text: `Condition is at first immediate existence; its form has these two moments: that of positedness, according to which it is, as condition, material and moment of the ground; and that of the in-itself, according to which it constitutes the essentiality of ground or its simple reflection into itself. Both sides of the form are external to immediate existence, for the latter is the sublated ground-connection.`
  },
  {
    id: 'a-chunk-3-existence-sublation',
    title: 'Existence sublates itself — being becomes essence',
    text: `But, first, existence is in it only this: to sublate itself in its immediacy and to founder, going to the ground. Being is as such only the becoming of essence; it is its essential nature to make itself into a positedness and into an identity which is an immediacy through the negation of itself. The form determinations of positedness and of self-identical in-itself, the form through which immediate existence is condition, are not, therefore, external to that existence; the latter is, rather, this very reflection.`
  },
  {
    id: 'a-chunk-4-being-as-condition-posited',
    title: 'Being as condition — positedness and in‑itself mediated',
    text: `Second: as condition, being is posited as what it essentially is — a moment and the being of another, and at the same time the in-itself of the other. It is in-itself only through its negation (the ground) and by self-sublation; the in-itself of being is thus itself posited and contains both essentiality-for-ground and immediate existence.`
  },
  {
    id: 'a-chunk-5-ground-self-subsistence',
    title: 'Ground as self-subsistent reflection and presupposing relation',
    text: `Likewise the ground is the self-subsistent, self-referring reflection of positing and thus self-identical; yet it is also presupposing reflection that posits its in-itself as an other. Condition, in both its in-itself and immediacy, is the ground-connection's own moment; the immediate existence is only through the ground and is a moment of the presupposing.`
  },
  {
    id: 'a-chunk-6-unity-of-form-and-content',
    title: 'Unity — one whole of form and content; existence as informed matter',
    text: `The result is one whole of form and one whole of content: the condition's content is essential only insofar as it is the self-identity of reflection in the form; existence is not mere formless material but informed matter that is identical in form-connection with the ground's content.`
  },
  {
    id: 'a-chunk-7-unity-and-unconditioned-fact',
    title: 'Unity of condition & ground — the unconditioned fact',
    text: `The two sides of the whole, condition and ground, are one essential unity as content and form. They pass into one another, posit themselves as sublated, and reciprocally presuppose each other. Their mutual presupposing amounts to presupposing one identity — the single substrate that is the truly unconditioned: the fact-in-itself.`
  },
  {
    id: 'a-chunk-8-relativity-and-sublation-of-condition',
    title: 'Relativity of condition; finite determinate and sublation',
    text: `Condition remains only relatively unconditioned: because any given condition is a finite determinate existence, one asks for another condition and so sets the regress in motion. This finitude is external to the concept of condition and is therefore sublated in the absolutely unconditioned (the fact in itself).`
  },

  {
    id: 'a-chunk-9-absolutely-unconditioned',
    title: 'Absolutely unconditioned — fact contains its moments',
    text: `The absolutely unconditioned (the fact-in-itself) contains within itself the two sides, condition and ground, as its moments; it is the unity to which they have returned. Together they constitute its form or positedness. The unconditioned fact is itself ground: it both conditions and grounds the two moments, and so its positing is the return and unity of what was divided.`
  },

  {
    id: 'a-chunk-10-two-shapes-and-rejoining',
    title: 'Two shapes (sublated manifold & inner ground) — reflective rejoining',
    text: `These two moments appear in two shapes: (1) as the sublated ground-connection — an immediate manifold void of unity that refers outward to the ground and constitutes its in-itself; (2) as an inner, simple form which is itself ground and which refers to the self-identical immediate as other, determining it as condition. They presuppose the totality that posits them, yet because they are identical, the relation of condition and ground dissolves into mere reflective shine. The absolutely unconditioned thus posits and presupposes in a movement where that shine sublates itself: the fact conditions itself, places itself as ground over its conditions, and in so doing rejoins them in itself.`
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'a-op-1-reflective-shine',
    chunkId: 'a-chunk-1-reflective-shine',
    label: 'Mutual reflective shine; independent content',
    clauses: [
      'condition.reflectsIn(ground) && ground.formReflectsIn(condition)',
      'condition.hasIndependentContent = true',
      'ground.hasIndependentContent = true'
    ],
    predicates: [
      { name: 'ReflectsIn', args: ['condition','ground'] },
      { name: 'HasIndependentContent', args: ['entity'] }
    ],
    relations: [
      { predicate: 'reflectsIn', from: 'condition', to: 'ground' },
      { predicate: 'reflectsIn', from: 'ground.form', to: 'condition' }
    ]
  },

  {
    id: 'a-op-2-condition-form-moments',
    chunkId: 'a-chunk-2-condition-form-moments',
    label: 'Condition form-moments: positedness & in-itself',
    clauses: [
      'condition.isImmediate = true',
      'condition.form.moments = [positedness, inItself]',
      'positedness => condition.materialFor(ground)',
      'inItSelf => constitutesEssentialityOf(ground)',
      'formMoments.externalTo(condition.immediacy) = true'
    ],
    predicates: [
      { name: 'IsImmediate', args: ['condition'] },
      { name: 'Positedness', args: ['condition'] },
      { name: 'InItself', args: ['condition'] }
    ],
    relations: [
      { predicate: 'materialFor', from: 'condition', to: 'ground' },
      { predicate: 'constitutes', from: 'condition.inItSelf', to: 'ground.essentiality' }
    ]
  },

  {
    id: 'a-op-3-existence-sublation',
    chunkId: 'a-chunk-3-existence-sublation',
    label: 'Existence sublates itself; being → becoming of essence',
    clauses: [
      'existence.sublatesSelfInImmediacy => goesTo(ground)',
      'being = becomingOf(essence)',
      'existence.makesItself.positedness = true',
      'identity.immediacy.throughNegation = true',
      'formDeterminations.areInternalTo(existence) = true'
    ],
    predicates: [
      { name: 'SublatesSelfInImmediacy', args: ['existence'] },
      { name: 'BecomingOfEssence', args: ['being'] },
      { name: 'MakesPositedness', args: ['existence'] }
    ],
    relations: [
      { predicate: 'sublatesTo', from: 'existence', to: 'ground' },
      { predicate: 'becomes', from: 'being', to: 'essence' }
    ]
  },

  {
    id: 'a-op-4-being-as-condition-posited',
    chunkId: 'a-chunk-4-being-as-condition-posited',
    label: 'Being as condition: posited in negation; mediated in-itself',
    clauses: [
      'forAny(x): if x.isCondition then x.isMomentOf(another)',
      'x.inItself = true onlyIf x.negatesSelfVia(ground)',
      'inItSelf(posited) && immediacy(coexist) -> bothSidesPresent'
    ],
    predicates: [
      { name: 'IsCondition', args: ['x'] },
      { name: 'MomentOf', args: ['x','other'] },
      { name: 'NegatesSelfVia', args: ['x','ground'] }
    ],
    relations: [
      { predicate: 'isMomentOf', from: 'condition', to: 'ground' },
      { predicate: 'inItSelfNegatedBy', from: 'condition', to: 'ground' }
    ]
  },

  {
    id: 'a-op-5-ground-self-subsistence',
    chunkId: 'a-chunk-5-ground-self-subsistence',
    label: 'Ground self-subsistent reflection; presupposing relation',
    clauses: [
      'ground.isSelfSubsistent = true',
      'ground.isSelfReferring = true',
      'ground.presupposesOutsideItself = true',
      'condition.isMomentOf(ground) && immediate.existsOnlyThrough(ground)'
    ],
    predicates: [
      { name: 'SelfSubsistent', args: ['ground'] },
      { name: 'SelfReferring', args: ['ground'] },
      { name: 'PresupposesOutside', args: ['ground'] }
    ],
    relations: [
      { predicate: 'selfSubsists', from: 'ground', to: 'ground' },
      { predicate: 'presupposes', from: 'ground', to: 'condition' }
    ]
  },

  {
    id: 'a-op-6-unity-of-form-and-content',
    chunkId: 'a-chunk-6-unity-of-form-and-content',
    label: 'Unity: one whole of form and content; existence as informed matter',
    clauses: [
      'whole.form = true && whole.content = true',
      'condition.content.isEssential iff condition.isSelfIdentityInForm',
      'existence.isInformedMatter = true (not mere formless material)',
      'condition.content == ground.content (identity-in-form)'
    ],
    predicates: [
      { name: 'WholeForm', args: ['whole'] },
      { name: 'WholeContent', args: ['whole'] },
      { name: 'IsInformedMatter', args: ['existence'] }
    ],
    relations: [
      { predicate: 'identityInForm', from: 'condition.content', to: 'ground.content' },
      { predicate: 'informs', from: 'form', to: 'content' }
    ]
  },

  {
    id: 'a-op-7-unity-unconditioned-fact',
    chunkId: 'a-chunk-7-unity-and-unconditioned-fact',
    label: 'Condition ⇄ Ground as one unity → Unconditioned substrate (Fact-in-itself)',
    clauses: [
      'condition.presupposes(ground) && ground.presupposes(condition)',
      'mutualPresupposition => singleIdentity(substrate)',
      'singleIdentity.isUnconditioned = true',
      'substrate == factInItself'
    ],
    predicates: [
      { name: 'MutualPresupposes', args: ['condition','ground'] },
      { name: 'SingleIdentity', args: ['substrate'] },
      { name: 'IsUnconditioned', args: ['substrate'] }
    ],
    relations: [
      { predicate: 'presupposes', from: 'condition', to: 'ground' },
      { predicate: 'presupposes', from: 'ground', to: 'condition' },
      { predicate: 'substrateOf', from: 'substrate', to: 'condition+ground' }
    ]
  },

  {
    id: 'a-op-8-relative-condition-sublation',
    chunkId: 'a-chunk-8-relativity-and-sublation-of-condition',
    label: 'Relatively unconditioned → regress; sublation in absolute',
    clauses: [
      'if condition.isFiniteDeterminate then askFor(newCondition)',
      'askingFor(newCondition) => regressAdInfinitum',
      'regressAdInfinitum is explainedBy condition.finitude',
      'absolutelyUnconditioned.sublates(condition) => terminationOfRegress'
    ],
    predicates: [
      { name: 'RelativelyUnconditioned', args: ['condition'] },
      { name: 'FiniteDeterminate', args: ['condition'] },
      { name: 'SublatedIn', args: ['absolutelyUnconditioned','condition'] }
    ],
    relations: [
      { predicate: 'requests', from: 'condition', to: 'furtherCondition' },
      { predicate: 'sublates', from: 'absolutelyUnconditioned', to: 'condition' }
    ]
  },

  {
    id: 'a-op-9-absolutely-unconditioned',
    chunkId: 'a-chunk-9-absolutely-unconditioned',
    label: 'Absolutely unconditioned contains moments; is itself ground',
    clauses: [
      'fact.contains(condition) && fact.contains(ground)',
      'fact.moments = [condition, ground]',
      'fact.isGround = true',
      'fact.posits(condition, ground) && fact.unifies(condition, ground)'
    ],
    predicates: [
      { name: 'ContainsAsMoment', args: ['fact','moment'] },
      { name: 'IsGround', args: ['fact'] },
      { name: 'Unifies', args: ['fact','moments'] }
    ],
    relations: [
      { predicate: 'contains', from: 'fact', to: 'condition' },
      { predicate: 'contains', from: 'fact', to: 'ground' },
      { predicate: 'grounds', from: 'fact', to: 'condition+ground' }
    ]
  },

  {
    id: 'a-op-10-two-shapes-rejoining',
    chunkId: 'a-chunk-10-two-shapes-and-rejoining',
    label: 'Two shapes (sublated manifold & inner ground) → reflective rejoining',
    clauses: [
      'shapeA = sublatedGroundConnection (immediateManifold, external, refersTo(ground))',
      'shapeB = innerSimpleForm (ground, selfIdentical, refersTo(immediate) as other)',
      'shapeA.presupposes(totality) && shapeB.presupposes(totality)',
      'if shapeA.identityWith(shapeB) then relation(condition,ground) -> reflectiveShine',
      'fact.positsAndPresupposes = movementWhere(shinethenSublatesItself)',
      'fact.selfConditions() && fact.placesItselfAs(ground) && fact.rejoins(condition,ground)'
    ],
    predicates: [
      { name: 'SublatedConnection', args: ['shapeA'] },
      { name: 'InnerSimpleGround', args: ['shapeB'] },
      { name: 'PresupposesTotality', args: ['shape'] },
      { name: 'ReflectiveShine', args: ['shape'] }
    ],
    relations: [
      { predicate: 'refersTo', from: 'shapeA', to: 'ground' },
      { predicate: 'refersTo', from: 'shapeB', to: 'immediate' },
      { predicate: 'identityWith', from: 'shapeA', to: 'shapeB' },
      { predicate: 'rejoins', from: 'fact', to: 'condition+ground' }
    ]
  }
]
