import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'bfs-om-a-one-within',
    title: 'a) The one within',
    text: `Within it, the one just is;
this, its being, is not an existence,
not a determination as reference to an other,
not a constitution;
it is rather its having
negated this circle of categories.
The one is not capable, therefore,
of becoming any other;
it is unalterable.
It is indeterminate,
yet no longer like being;
its indeterminateness is
the determinateness of self-reference,
absolutely determined being;
posited in-itselfness.
As negation which, in accordance with its concept,
is self-referring, it has distinction in it:
it directs away from itself towards another,
but this direction is immediately reversed,
because, according to this moment of self-determining,
there is no other to which it would be addressed,
and the directing reverts back to itself.
In this simple immediacy,
even the mediation of existence and ideality,
and with it all diversity and manifoldness,
have vanished.
In the one there is nothing;
this nothing, the abstraction of self-reference,
is here distinguished from the in-itselfness of the one;
it is a posited nothing, for this in-itselfness
no longer has the simplicity of the something,
but, as mediation, has rather the determination of being concrete;
taken in abstraction, it is indeed identical with the one,
but different from its determination.
So this nothing, posited as in the one,
is the nothing as the void.
The void is thus the quality of
the one in its immediacy.`,
    concise:
      'The one is unalterable and not an existence; its indeterminateness is self-reference (posited in-itselfness). As self-referring negation it momentarily points outward but immediately reverts to itself. All mediation and manifoldness vanish; within the one there is a posited nothing, the void, as its immediate quality.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'bfs-op-om-a1-one-not-existence-unalterable',
    chunkId: 'bfs-om-a-one-within',
    label: 'The one is not an existence or constitution; it negates that circle and cannot become other (unalterable)',
    clauses: [
      'assert(is(TheOne,true))',
      'assert(isExistence(TheOne,false))',
      'assert(isReferenceToOther(TheOne,false))',
      'assert(isConstitution(TheOne,false))',
      'assert(negatesCircleOfCategories(TheOne,true))',
      'assert(canBecomeOther(TheOne,false))',
      'tag(Property,"unalterable")'
    ],
    predicates: [{ name: 'OneWithin_Def_NonExistence_Unalterable', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-a2-indeterminate-as-self-reference',
    chunkId: 'bfs-om-a-one-within',
    label: 'Indeterminateness as determinateness of self-reference; posited in-itselfness',
    clauses: [
      'assert(indeterminate(TheOne,true))',
      'assert(indeterminateLikeBareBeing(TheOne,false))',
      'assert(determinateness(TheOne,"self-reference"))',
      'assert(positedInItselfness(TheOne,true))',
      'tag(Determination,"absolutely-determined-being")'
    ],
    predicates: [{ name: 'OneWithin_SelfReference_InItself', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-a3-self-referring-negation-reversion',
    chunkId: 'bfs-om-a-one-within',
    label: 'Self-referring negation: distinction, outward directing, immediate reversion (no other present)',
    clauses: [
      'assert(hasDistinction(TheOne,true))',
      'assert(selfReferringNegation(TheOne,true))',
      'assert(directsOutward(TheOne,true))',
      'assert(otherPresent(false))',
      'assert(immediateReversionToSelf(TheOne,true))'
    ],
    predicates: [{ name: 'OneWithin_OutwardAndReturn', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-a4-mediation-and-manifoldness-vanish',
    chunkId: 'bfs-om-a-one-within',
    label: 'In simple immediacy, mediation (existence/ideality) and manifoldness vanish',
    clauses: [
      'assert(isSimpleImmediacy(TheOne,true))',
      'assert(mediationOf("existence",false))',
      'assert(mediationOf("ideality",false))',
      'assert(diversity(false))',
      'assert(manifoldness(false))'
    ],
    predicates: [{ name: 'OneWithin_VanishingMediation', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-a5-void-as-immediate-quality',
    chunkId: 'bfs-om-a-one-within',
    label: 'Within the one there is a posited nothing (void), distinct from in-itselfness yet abstractly identical',
    clauses: [
      'assert(contains(TheOne,"nothing-as-void"))',
      'assert(distinctFrom(TheOne,"in-itselfness-of-one"))',
      'assert(identicalInAbstraction("void",TheOne,true))',
      'assert(differentAsDetermination("void",TheOne,true))',
      'tag(Quality,"void-in-immediacy")'
    ],
    predicates: [{ name: 'OneWithin_Void', args: [] }],
    relations: []
  }
]
