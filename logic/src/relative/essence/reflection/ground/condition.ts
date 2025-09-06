import { Chunk } from './index';

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'chunk-1-ground-self-positing',
    title: 'Ground — self-positing & mediation',
    text: `Ground is the immediate,
and the grounded the mediated.
But ground is positing reflection;
as such, it makes itself into positedness
and is presupposing reflection;
as such it refers itself to itself
as to something sublated,
to an immediate through which
it is itself mediated.
This mediation, as an advance
from the immediate to the ground,
is not an external reflection
but, as we have seen, the ground's own doing
or, what is the same, the ground-connection,
as reflection into its self-identity,
is just as essentially self-externalizing reflection.
The immediate to which ground refers as
to its essential presupposition is condition;
real ground is accordingly essentially conditioned.
The determinateness that it contains is
the otherness of itself.`
  },
  {
    id: 'chunk-2-condition-conclusion',
    title: 'Conclusion — what condition is',
    text: `Condition is therefore,
first, an immediate, manifold existence.
Second, it is this existence referred to an other,
to something which is ground,
not of this existence but in some other respect,
for existence itself is immediate and without ground.
According to this reference, it is something posited;
as condition, the immediate existence is supposed to be
not for itself but for another.
But this, that it thus is for another, is at the same time
itself only a positedness;
that it is posited is sublated in its immediacy:
an existence is indifferent to being a condition.
Third, condition is something immediate in the sense
that it constitutes the presupposition of ground.`
  },
  {
    id: 'chunk-3-condition-content-form',
    title: 'Expansion — condition as content and form-connection',
    text: `In this determination, it is the form-connection of ground
withdrawn into self-identity, hence the content of ground.
But content is as such only the indifferent unity of ground,
as in the form: without form, no content.
It nevertheless frees itself
from this indifferent unity
in that the ground-connection,
in the complete ground,
becomes a connection external to its identity,
whereby content acquires immediacy.

In so far, therefore, as condition is
that in which the ground-connection has
its identity with itself,
it constitutes the content of ground;
but since this content is indifferent to form,
it is only implicitly the content of form,
is something which has yet to become content
and hence constitutes the material for the ground.`
  },
  {
    id: 'chunk-4-condition-becomes-unconditioned',
    title: 'Return — condition becomes relative unconditioned for ground',
    text: `Posited as condition,
and in accordance with the second moment,
existence is determined to lose its indifferent immediacy
and to become the moment of another.
By virtue of its immediacy, it is indifferent to this connection;
inasmuch as it enters into it, however,
it constitutes the in-itself of the ground
and is for it the unconditioned.
In order to be condition,
it has its presupposition in the ground
and is itself conditioned;
but this condition is external to it.`
  },
  {
    id: 'chunk-5-condition-essence-distinction',
    title: 'Essence — condition is not the ground; distinction of roles',
    text: `Something is not through its condition; its condition is not its ground.
Condition is for the ground the moment of unconditioned immediacy, but is not itself the positing-movement that refers itself to itself and makes itself into positedness.
Over against condition there stands the ground-connection: something has, besides its condition, also a ground.
The ground is the empty movement of reflection which nonetheless has immediacy as its presupposition outside it.`
  },
  {
    id: 'chunk-6-ground-connection-content',
    title: 'Ground-connection — self-subsisting reflection and content distinction',
    text: `The ground-connection is the whole form and the self-subsistent process of mediation; since this mediating refers itself to itself as positing it is also something immediate and unconditioned on that side.
As self-subsisting self-reference it has a content peculiar to itself (the informed content of ground) distinct from the immediate material content of the condition.
The condition’s content is immediate material which, while external, constitutes the in-itself of the ground and should become a moment of it (material-for-form).`
  },

  // New: part-3 chunks (two precise chunks: indifference / mediation & contradiction)
  {
    id: 'chunk-7-two-sides-indifference',
    title: 'The two sides — mutual indifference and unconditionedness',
    text: `The two sides of the whole, condition and ground, are thus, on the one hand, indifferent and unconditioned with respect to each other:
the one as the non-referred-to side, to which the connecting reference in which it is the condition is external;
the other as the connecting reference, or form, for which the determinate existence of the condition is only a material, something passive whose form, such as it possesses on its own account, is unessential.`
  },
  {
    id: 'chunk-8-mediation-contradiction',
    title: 'Mediation — condition as in-itself of ground and the internal contradiction',
    text: `On the other hand, the two sides are also mediated.
Condition is the in-itself of the ground; so much is it the essential moment of the ground-connection, that it is the simple self-identity of the ground.
But this is sublated; this in-itself is only something posited; immediate existence is indifferent to being a condition.
Likewise, the ground-connection has in its self-subsistence also a presupposition; it has its in-itself outside itself.
Consequently, each side is this contradiction: indifferent immediacy and essential mediation, both in one reference — independent subsistence and determination as only moments.`
  }
]

export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

/* Lightweight logical-op shape (TS-friendly) */
export type Predicate = { name: string; args?: string[] }
export type Relation = { predicate: string; from: string; to: string }

export type LogicalOperation = {
  id: string
  label?: string
  clauses: string[]
  predicates?: Predicate[]
  relations?: Relation[]
  chunkId?: string
}

/* Minimal mapping: one logical operation per canonical chunk */
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'op-1-ground-self-positing',
    chunkId: 'chunk-1-ground-self-positing',
    label: 'Ground self-positing / mediation',
    clauses: [
      'ground.immediate = true',
      'ground.posits(self) => ground.positedness',
      'mediation := ground.connection (self-activity), not external reflection',
      'condition = immediate presupposition referenced by ground',
      'ground.determinateness = othernessOf(self)'
    ],
    predicates: [
      { name: 'Immediate', args: ['ground'] },
      { name: 'Posits', args: ['ground'] },
      { name: 'MediationIsSelfDoing', args: ['ground'] }
    ],
    relations: [{ predicate: 'presupposes', from: 'ground', to: 'condition' }]
  },
  {
    id: 'op-2-condition-conclusion',
    chunkId: 'chunk-2-condition-conclusion',
    label: 'Condition — immediacy, for-another, positedness, presupposition',
    clauses: [
      'condition.isImmediate = true',
      'condition.referredTo = ground (for-another)',
      'condition.isPosited = true',
      'positedness.sublatedBy(immediacy) => existence.indifferentToBeingCondition',
      'condition.constitutesPresuppositionOf(ground)'
    ],
    predicates: [
      { name: 'Immediate', args: ['condition'] },
      { name: 'ForAnother', args: ['condition', 'ground'] },
      { name: 'Posited', args: ['condition'] }
    ],
    relations: [
      { predicate: 'for', from: 'condition', to: 'ground' },
      { predicate: 'constitutes', from: 'condition', to: 'ground.inItself' }
    ]
  },
  {
    id: 'op-3-condition-content-form',
    chunkId: 'chunk-3-condition-content-form',
    label: 'Form-connection withdrawn into self-identity → content of ground',
    clauses: [
      'formConnection(ground).withdrawnInto(selfIdentity) => content(ground)',
      'content.initial = indifferentUnity(ground)',
      'without form => no content',
      'completeGround: groundConnection.becomesExternal => content.acquiresImmediacy',
      'implicitContent => materialFor(groundForm)'
    ],
    predicates: [
      { name: 'WithdrawnInto', args: ['formConnection','selfIdentity'] },
      { name: 'IndifferentUnity', args: ['content','ground'] },
      { name: 'BecomesExternal', args: ['groundConnection'] }
    ],
    relations: [
      { predicate: 'yields', from: 'formConnection', to: 'ground.content' },
      { predicate: 'materialFor', from: 'ground.content', to: 'ground.form' }
    ]
  },
  {
    id: 'op-4-condition-becomes-unconditioned',
    chunkId: 'chunk-4-condition-becomes-unconditioned',
    label: 'Condition → relative unconditioned for ground',
    clauses: [
      'condition.posited && condition.entersConnection => condition.losesIndifferentImmediacy',
      'condition.entersConnection => condition.constitutes(ground.inItself)',
      'condition.isFor(ground) && condition.isConditioned(externalToItself)',
      'through entry, condition.functionsFor(ground) as relativeUnconditioned'
    ],
    predicates: [
      { name: 'LosesImmediacy', args: ['condition'] },
      { name: 'ConstitutesInItself', args: ['condition','ground'] },
      { name: 'IsFor', args: ['condition','ground'] }
    ],
    relations: [
      { predicate: 'constitutes', from: 'condition', to: 'ground.inItself' },
      { predicate: 'functionsAs', from: 'condition', to: 'ground.relativeUnconditioned' }
    ]
  },
  {
    id: 'op-5-condition-essence-distinction',
    chunkId: 'chunk-5-condition-essence-distinction',
    label: 'Distinction: condition ≠ ground; condition as moment of immediacy',
    clauses: [
      'forAny(x): x.notIdenticalTo(x.condition)', // something is not through its condition
      'condition.roleFor(ground) = momentOf(unconditionedImmediacy)',
      'condition.isNot(positingMovement)', // condition lacks self-positing
      'groundConnection.existsOppositeTo(condition)'
    ],
    predicates: [
      { name: 'NotThroughCondition', args: ['something','condition'] },
      { name: 'MomentOfImmediacy', args: ['condition','ground'] },
      { name: 'IsNotPositing', args: ['condition'] }
    ],
    relations: [
      { predicate: 'opposes', from: 'ground.connection', to: 'condition' },
      { predicate: 'hasMoment', from: 'ground', to: 'condition' }
    ]
  },

  {
    id: 'op-6-ground-connection-content',
    chunkId: 'chunk-6-ground-connection-content',
    label: 'Ground-connection as self-subsisting reflection; content vs material distinction',
    clauses: [
      'groundConnection.isSelfSubsistent = true',
      'groundConnection.hasIdentityOfReflection = true',
      'ground.content = informedContent (distinct)',
      'condition.content = immediateMaterial (external-to-ground)',
      'condition.entersConnection => condition.materialBecomesMomentOf(ground)'
    ],
    predicates: [
      { name: 'SelfSubsistent', args: ['groundConnection'] },
      { name: 'HasReflectionIdentity', args: ['groundConnection'] },
      { name: 'InformedContent', args: ['ground'] },
      { name: 'ImmediateMaterial', args: ['condition'] }
    ],
    relations: [
      { predicate: 'hasContent', from: 'groundConnection', to: 'ground.content' },
      { predicate: 'materialFor', from: 'condition.content', to: 'ground' },
      { predicate: 'constitutes', from: 'condition', to: 'ground.inItself' }
    ]
  },
  {
    id: 'op-7-two-sides-indifference',
    chunkId: 'chunk-7-two-sides-indifference',
    label: 'Two sides: mutual indifference; externality of reference',
    clauses: [
      'condition.indifferentTo(ground) && ground.indifferentTo(condition)',
      'condition.isNonReferredSide = true',
      'ground.isConnectingReference = true',
      'condition.content = immediateMaterial (externalToReference)'
    ],
    predicates: [
      { name: 'Indifferent', args: ['condition','ground'] },
      { name: 'NonReferredSide', args: ['condition'] },
      { name: 'ConnectingReference', args: ['ground'] },
      { name: 'ImmediateMaterial', args: ['condition'] }
    ],
    relations: [
      { predicate: 'externalTo', from: 'condition.content', to: 'ground.connection' },
      { predicate: 'isFormFor', from: 'ground', to: 'condition' }
    ]
  },

  {
    id: 'op-8-mediation-contradiction',
    chunkId: 'chunk-8-mediation-contradiction',
    label: 'Mediation & contradiction: in-itself posited; mutual presupposition',
    clauses: [
      'condition.isInItselfOf(ground) = true',
      'condition.inItselfIsPosited = true',
      'groundConnection.hasPresuppositionOutsideItself = true',
      'eachSide = (indifferentImmediacy && essentialMediation)'
    ],
    predicates: [
      { name: 'InItselfOf', args: ['condition','ground'] },
      { name: 'Posited', args: ['condition'] },
      { name: 'HasPresuppositionOutside', args: ['groundConnection'] },
      { name: 'ContradictoryDouble', args: ['side'] }
    ],
    relations: [
      { predicate: 'constitutesInItself', from: 'condition', to: 'ground.inItself' },
      { predicate: 'presupposesOutside', from: 'groundConnection', to: 'ground.inItself' }
    ]
  }
]

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS
}

export function getLogicalOpsForChunk(oneBasedIndex: number): LogicalOperation | null {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return null
  return LOGICAL_OPERATIONS.find(op => op.chunkId === chunk.id) ?? null
}
