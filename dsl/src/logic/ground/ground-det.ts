import type { Chunk, LogicalOperation } from './index'

// Canonical chunks for "B. DETERMINATE GROUND — a. Formal ground"
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'dg-chunk-1-intro',
    title: 'Formal ground — introduction',
    text: `The determinate ground is a ground with a determinate content. Form treats that determinateness as substrate: the simple immediate that stands against form's mediation. Formal ground is the negative self-referring identity that posits itself and thereby constitutes the mediating factor.`
  },
  {
    id: 'dg-chunk-2-content-as-substrate',
    title: 'Content as substrate and indifferent unity',
    text: `In the determinate ground the content is the identical element common to both ground and grounded. The content is indifferent to their distinction; it functions as the substrate or positive unity through which formal mediation operates.`
  },
  {
    id: 'dg-chunk-3-negative-self-reference',
    title: 'Negative self-reference and positedness',
    text: `The ground refers negatively to itself: in its negativity it is self-identical and makes itself into a positedness. That negative reference is the immediate determinateness by which the ground acquires determinate content.`
  },
  {
    id: 'dg-chunk-4-formal-mediation-unity',
    title: 'Formal mediation: unity of the two sides',
    text: `Formal mediation is the negative unity in which both sides (ground and grounded) pass into each other and mutually posit themselves into one sublated identity. This mediation is the form proper: unity of pure reflection and determining reference applied to the determinate content.`
  },
  {
    id: 'dg-chunk-5-two-sided-perspective',
    title: 'Two-sided consideration: ground vs grounded',
    text: `A determinate content can be considered from two perspectives: as ground and as grounded. Each side is equally the whole mediation; the transition may be taken from either side because the content itself remains identical and indifferent to form.`
  },
  {
    id: 'dg-chunk-6-sufficiency-and-limitation',
    title: 'Sufficiency of the ground (limited) and formal ground',
    text: `Because ground and grounded share identity of content and form, the ground is sufficient (relative to this relation): nothing in the grounded is not in the ground. But this sufficiency is limited: when determinateness is still simple and undifferentiated we have only a formal ground, with content external to form.`
  },
  {
    id: 'dg-chunk-7-formal-ground-conclusion',
    title: 'Conclusion: formal ground vs real/complete ground',
    text: `Formal ground is the present stage where determinateness is one simple content indifferent to form. It is the formal moment in the development toward real and complete ground — the basis for subsequent analyses (real ground, complete ground) and for syllogisms of reflection.`
  }
]

// HLO — Logical operations derived from "Formal ground"
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'dg-op-1-intro',
    chunkId: 'dg-chunk-1-intro',
    label: 'DeterminateGround = ground + determinateContent (formal)',
    clauses: [
      'determinateGround.has(content)',
      'form.treats(content) as substrate',
      'ground = negativeSelfReferringIdentity -> positedness'
    ],
    predicates: [
      { name: 'HasContent', args: ['determinateGround','content'] },
      { name: 'TreatsAsSubstrate', args: ['form','content'] }
    ],
    relations: [
      { predicate: 'has', from: 'determinateGround', to: 'content' },
      { predicate: 'treatsAs', from: 'form', to: 'substrate' }
    ]
  },

  {
    id: 'dg-op-2-content-substrate',
    chunkId: 'dg-chunk-2-content-as-substrate',
    label: 'Content = identical element; substrate indifferent to form',
    clauses: [
      'content.isIdenticalElementOf(ground, grounded)',
      'content.indifferentTo(form)',
      'content.constitutes(positiveUnity)'
    ],
    predicates: [
      { name: 'IsIdenticalElement', args: ['content','ground+grounded'] },
      { name: 'IsIndifferentTo', args: ['content','form'] }
    ],
    relations: [
      { predicate: 'constitutes', from: 'content', to: 'positiveUnity' }
    ]
  },

  {
    id: 'dg-op-3-negative-self-reference',
    chunkId: 'dg-chunk-3-negative-self-reference',
    label: 'Ground refers negatively to itself → positedness',
    clauses: [
      'ground.refersNegativelyTo(ground)',
      'negativity => selfIdentity && positedness',
      'posited.content = determinateContent'
    ],
    predicates: [
      { name: 'RefersNegativelyTo', args: ['ground','itself'] },
      { name: 'IsPositedness', args: ['ground'] }
    ],
    relations: [
      { predicate: 'refersNegatively', from: 'ground', to: 'itself' },
      { predicate: 'yields', from: 'negativity', to: 'positedness' }
    ]
  },

  {
    id: 'dg-op-4-formal-mediation',
    chunkId: 'dg-chunk-4-formal-mediation-unity',
    label: 'Formal mediation = negative unity; mutual positing',
    clauses: [
      'formalMediation = negativeUnity(form)',
      'sides.passIntoEachOther() => mutualPositing',
      'formalMediation.refersTo(determinateContent) as mediatingFactor'
    ],
    predicates: [
      { name: 'IsFormalMediation', args: ['formalMediation'] },
      { name: 'MutualPositing', args: ['sides'] }
    ],
    relations: [
      { predicate: 'mediates', from: 'formalMediation', to: 'determinateContent' }
    ]
  },

  {
    id: 'dg-op-5-two-sided-perspective',
    chunkId: 'dg-chunk-5-two-sided-perspective',
    label: 'Ground and grounded are both whole form; perspective reversible',
    clauses: [
      'content.consideredAs(ground) || content.consideredAs(grounded)',
      'eitherTransition.valid -> bothSides.wholeMediation',
      'form.isExternalTo(simpleDeterminateness)'
    ],
    predicates: [
      { name: 'IsEitherPerspective', args: ['content'] },
      { name: 'WholeMediation', args: ['sides'] }
    ],
    relations: [
      { predicate: 'reversibleFrom', from: 'transition', to: 'eitherSide' }
    ]
  },

  {
    id: 'dg-op-6-sufficiency-and-formal-limit',
    chunkId: 'dg-chunk-6-sufficiency-and-limitation',
    label: 'Ground sufficiency (relative) and formal ground limitation',
    clauses: [
      'ground.isSufficientRelativeTo(relation) => nothingIn(grounded) notIn(ground)',
      'if content.simpleDeterminateness then ground = formalGround',
      'formalGround.content.externalToForm'
    ],
    predicates: [
      { name: 'IsSufficientRelative', args: ['ground'] },
      { name: 'IsFormalGround', args: ['ground'] }
    ],
    relations: [
      { predicate: 'isSufficientFor', from: 'ground', to: 'grounded' },
      { predicate: 'limits', from: 'formalGround', to: 'form' }
    ]
  },

  {
    id: 'dg-op-7-conclusion-formal-ground',
    chunkId: 'dg-chunk-7-formal-ground-conclusion',
    label: 'Formal ground = stage before real/complete ground; basis for syllogisms of reflection',
    clauses: [
      'formalGround.present => nextAnalyse(realGround, completeGround)',
      'formalGround = basisFor(syllogismsOfReflection)'
    ],
    predicates: [
      { name: 'IsBasisFor', args: ['formalGround','syllogismsOfReflection'] }
    ],
    relations: [
      { predicate: 'prepares', from: 'formalGround', to: 'realCompleteGroundAnalysis' }
    ]
  }
]
