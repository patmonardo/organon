import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'bfs-om-c-many-ones-repulsion',
    title: 'c) Many ones — Repulsion',
    text: `The one and the void constitute the first existence of being-for-itself.
Each of these moments has negation for its determination,
and is posited at the same time as an existence.
In accordance with this determination,
the one and the void are each the reference
of negation to negation as of an other to its other:
the one is negation in the determination of being; the void,
negation in the determination of non-being.
Essentially, however, the one
is only self-reference as referring negation,
that is, it is itself the same as the
void outside it is supposed to be.
Both are, however, also posited as
each an affirmative existence
(the one as being-for-itself as such,
the other as indeterminate existence in general)
and each as referring to the other as to an other existence.
Essentially, however, the being-for-itself of the one
is the ideality of the existence and of the other;
it does not refer to an other but only to itself.
But inasmuch as the being-for-itself is fixed as the one,
as existent for itself, as immediately present,
its negative reference to itself is
at the same time reference to an existent;
and since the reference is just as much negative,
that to which the being-for-itself refers remains
determined as an existence and as an other;
as essentially self-reference,
the other is not indeterminate negation like the void,
but is likewise a one.
The one is consequently a becoming of many ones.

Strictly speaking, however, this is not just a becoming;
for becoming is a transition of being into nothing;
the one, by contrast, becomes only a one.
The one, as referred to, contains
the negative as reference;
it has this reference, therefore, in it.
Hence, instead of a becoming,
the one's own immanent reference is,
first, present;
and, second, since this reference is negative
and the one is at the same time an existent,
the one repels itself from itself.
This negative reference of
the one to itself is repulsion.`,
    concise:
      'One-and-void = first existence of being-for-itself. One = negation-in-being; void = negation-in-non-being. As ideality, one refers only to itself; fixed as an existent one, its negative self-reference is also reference to an existent, so the other is likewise a one: many ones. This is not ordinary becoming (being→nothing); rather the one’s immanent negative reference makes it repel itself: repulsion.'
  },
  {
    id: 'bfs-om-c-repulsion-implicit-vs-external',
    title: 'c) Many ones - Repulsion (II)',
    text: `This repulsion, as thus the positing of many ones
but through the one itself, is the one's
own coming-forth-from-itself,
but to such outside it as are themselves only ones.
This is repulsion according to the concept,
as it exists implicitly in itself.
The second repulsion is distinguished from it.
It is the one that first occurs to
the representation of external reflection,
not as the generation of ones
but only as the mutual holding off of ones
which are presupposed as already there.
To be seen now is how the first repulsion
that exists in itself determines itself
as the second, the external repulsion.

We must first establish the determinations
that the many ones have as such.
The becoming of the many, or their being produced,
immediately vanishes as the product of a positing;
what is produced are the ones, not for another,
but as infinitely referring to themselves.
The one repels only itself from itself;
it does not come to be but it already is;
that which is represented as the repelled is
equally a one, an existent;
repelling and being repelled applies
in like manner to both, and makes no difference.

The ones are thus presupposed
with respect to each other posited through
the repulsion of the one from itself;
presupposed, posited as non-posited;
their being-posited is sublated,
they are existents with respect to each other,
such as refer only to themselves.`,
    concise:
      'Two repulsions: (1) implicit (conceptual) repulsion = the one posits many ones from itself; (2) external repulsion = mutual holding-off among presupposed ones. The becoming of the many vanishes as mere positing; what remains are self-referring ones. Repulsion is reflexive and symmetric (repelling/repelled make no difference). The many are presupposed as non-posited: their positing is sublated; as existents they refer only to themselves.'
  },
  {
    id: 'bfs-om-c-plurality-infinity-contradiction',
    title: 'c) Many ones - Plurality and infinity as contradiction',
    text: `Thus plurality appears not as an otherness,
but as a determination completely external to the one.
The one, in repelling itself, remains reference to itself,
just like that which is taken as repelled at the start.
That the ones are other to one another,
that they are brought together in
the determinateness of plurality,
does not therefore concern the one.
If the plurality were a
reference of the ones to one another,
the ones would then limit each other
and would have the being-for-other affirmatively in them.
Their connecting reference
(and this they have through their unity which is in itself),
as posited here, is determined as none;
it is again the previously posited void.
This void is their limit,
but an external limit in which
they are not supposed to be for one another.
The limit is that in which
the limited are just as much as are not;
but the void is determined as pure non-being,
and this alone constitutes the limit of the ones.

The repulsion of the one from itself is
the making explicit of what
the one is implicitly in itself;
but, thus laid out as one-outside-the-other,
infinity is here an infinity that has externalized itself,
and this it has done through the immediacy
of the infinite, of the one.
Infinity is just as much
the simple reference of the one to the one
as, on the contrary, the one's absolute lack of reference;
it is the former according to the simple affirmative reference
of the one to itself;
it is the latter according to the same reference as negative.
Or again, the plurality of the ones is
the one's own positing of the one;
the one is nothing but
the negative reference of the one to itself,
and this reference
[hence the one itself]
is the plural one.
But equally, plurality is utterly external to the one,
for the one is precisely the sublating of otherness;
repulsion is its self-reference and simple equality with itself.
The plurality of the ones is infinity as a contradiction
that unconstrainedly produces itself.`,
    concise:
      'Plurality is an external determination relative to the one; their only connecting reference is the void as an external limit. Repulsion makes explicit what the one is implicitly; infinity externalizes. Infinity is both simple self-reference (affirmative) and absolute lack of reference (negative). Plurality is the one’s own positing of the one (plural one), yet remains external. Thus plurality is infinity as a self-producing contradiction.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'bfs-op-om-c1-first-existence',
    chunkId: 'bfs-om-c-many-ones-repulsion',
    label: 'One and void constitute the first existence of being-for-itself; each determined by negation and posited as existence',
    clauses: [
      'assert(firstExistenceOf(BeingForItself,["one","void"]))',
      'assert(hasDetermination("one","negation"))',
      'assert(hasDetermination("void","negation"))',
      'assert(positedAsExistence("one",true))',
      'assert(positedAsExistence("void",true))'
    ],
    predicates: [{ name: 'OM_FirstExistence', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c2-negation-modes',
    chunkId: 'bfs-om-c-many-ones-repulsion',
    label: 'Reference of negation to negation: one = negation-in-being; void = negation-in-non-being',
    clauses: [
      'assert(negationRefersToNegation(true))',
      'assert(modeOfNegation("one","in-being"))',
      'assert(modeOfNegation("void","in-non-being"))'
    ],
    predicates: [{ name: 'OM_NegationModes', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c3-ideality-vs-existence-reference',
    chunkId: 'bfs-om-c-many-ones-repulsion',
    label: 'As ideality, one refers only to itself; yet as fixed existent it refers to an existent other (which remains other)',
    clauses: [
      'assert(idealityOf(BeingForItself,true))',
      'assert(refersOnlyToSelf("one",true))',
      'assert(fixedAsExistent("one",true))',
      'assert(negativeSelfReferenceIsAlsoReferenceToExistent("one",true))',
      'assert(otherRemainsExistenceAndOther(true))'
    ],
    predicates: [{ name: 'OM_Ideality_Reference', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c4-other-likewise-one-many-ones',
    chunkId: 'bfs-om-c-many-ones-repulsion',
    label: 'Given self-reference, the other is likewise a one; the one becomes many ones',
    clauses: [
      'assert(otherIsLikewiseOne(true))',
      'assert(becomesManyOnes("one",true))'
    ],
    predicates: [{ name: 'OM_ManyOnes', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c5-not-ordinary-becoming',
    chunkId: 'bfs-om-c-many-ones-repulsion',
    label: 'Not becoming (being→nothing); the one becomes only a one',
    clauses: [
      'assert(isOrdinaryBecoming(false))',
      'assert(transition("being","nothing"),false)',
      'assert(becomesOnly("one","one",true))'
    ],
    predicates: [{ name: 'OM_NotBecoming', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c6-repulsion',
    chunkId: 'bfs-om-c-many-ones-repulsion',
    label: 'Immanent negative reference present; since the one is existent, it repels itself from itself (repulsion)',
    clauses: [
      'assert(immanentReferencePresent("one",true))',
      'assert(referenceIsNegative(true))',
      'assert(isExistent("one",true))',
      'assert(repelsItselfFromItself("one",true))',
      'tag(Determination,"repulsion")'
    ],
    predicates: [{ name: 'OM_Repulsion', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c7-two-repulsions',
    chunkId: 'bfs-om-c-repulsion-implicit-vs-external',
    label: 'Two repulsions: implicit (concept) vs external (mutual holding-off among presupposed ones)',
    clauses: [
      'assert(repulsionImplicit(true))',
      'assert(positsManyOnesFromItself("one",true))',
      'assert(repulsionExternal(true))',
      'assert(mutualHoldingOffAmongPresupposedOnes(true))',
      'assert(determinesItselfFromImplicitToExternal(true))'
    ],
    predicates: [{ name: 'OM_Repulsion_TwoKinds', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c8-production-vanishes-self-ref-ones',
    chunkId: 'bfs-om-c-repulsion-implicit-vs-external',
    label: 'Production/becoming of many vanishes as mere positing; products are ones, not for another, but infinitely self-referring',
    clauses: [
      'assert(becomingOfManyVanishes(true))',
      'assert(productIsOnes(true))',
      'assert(forAnother(false))',
      'assert(infinitelySelfReferring("ones",true))'
    ],
    predicates: [{ name: 'OM_Repulsion_ProductionVanishes', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c9-reflexive-symmetric-repulsion',
    chunkId: 'bfs-om-c-repulsion-implicit-vs-external',
    label: 'The one repels only itself; the repelled is equally a one; repelling vs being repelled makes no difference',
    clauses: [
      'assert(repelsOnlyItself("one",true))',
      'assert(repelledIsOne(true))',
      'assert(repelSymmetry(true))',
      'assert(noDifferenceBetweenRepellingAndRepelled(true))'
    ],
    predicates: [{ name: 'OM_Repulsion_ReflexiveSymmetric', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c10-presupposed-non-posited-sublated',
    chunkId: 'bfs-om-c-repulsion-implicit-vs-external',
    label: 'Ones presupposed with respect to each other; posited as non-posited; their being-posited is sublated; as existents they self-refer',
    clauses: [
      'assert(presupposedWithRespectToEachOther("ones",true))',
      'assert(positedAsNonPosited("ones",true))',
      'assert(beingPositedSublated("ones",true))',
      'assert(existentsReferOnlyToThemselves("ones",true))'
    ],
    predicates: [{ name: 'OM_Repulsion_PresuppositionSublation', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c11-determination-implicit-to-external',
    chunkId: 'bfs-om-c-repulsion-implicit-vs-external',
    label: 'How implicit repulsion determines itself as external repulsion',
    clauses: [
      'assert(implicitRepulsionGroundsExternal(true))',
      'assert(externalAsAppearanceOfImplicit(true))'
    ],
    predicates: [{ name: 'OM_Repulsion_Determination', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c12-plurality-external-void-limit',
    chunkId: 'bfs-om-c-plurality-infinity-contradiction',
    label: 'Plurality is external to the one; their only connecting reference is the void as external limit',
    clauses: [
      'assert(pluralityAsExternalDetermination(true))',
      'assert(oneRemainsSelfReferenceUnderRepulsion(true))',
      'assert(connectingReferenceIsNone(true))',
      'assert(limitOfOnesIs("void"))',
      'assert(voidIsPureNonBeing(true))',
      'assert(notForOneAnotherInVoid(true))'
    ],
    predicates: [{ name: 'OM_Plurality_External_VoidLimit', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c13-repulsion-explicates-infinity-externalized',
    chunkId: 'bfs-om-c-plurality-infinity-contradiction',
    label: 'Repulsion explicates what the one is implicitly; infinity laid out as one-outside-the-other (externalized via immediacy of the one)',
    clauses: [
      'assert(repulsionMakesImplicitExplicit(true))',
      'assert(infinityExternalizedAsOneOutsideOther(true))',
      'assert(externalizationThroughImmediacyOfOne(true))'
    ],
    predicates: [{ name: 'OM_Infinity_Externalized', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c14-infinity-dual-reference',
    chunkId: 'bfs-om-c-plurality-infinity-contradiction',
    label: 'Infinity as dual: simple reference of one to one (affirmative) and absolute lack of reference (negative)',
    clauses: [
      'assert(infinityAsSimpleSelfReference(true))',
      'assert(infinityAsAbsoluteLackOfReference(true))',
      'assert(affirmativeVsNegativeReference(true))'
    ],
    predicates: [{ name: 'OM_Infinity_DualReference', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c15-plural-one',
    chunkId: 'bfs-om-c-plurality-infinity-contradiction',
    label: 'Plurality as the one’s own positing of the one; the one is negative self-reference (plural one)',
    clauses: [
      'assert(pluralityIsOnesOwnPositing(true))',
      'assert(oneIsNegativeSelfReference(true))',
      'assert(isPluralOne(true))'
    ],
    predicates: [{ name: 'OM_Plural_One', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-om-c16-contradiction-self-producing',
    chunkId: 'bfs-om-c-plurality-infinity-contradiction',
    label: 'Plurality remains external; repulsion is self-reference; plurality is infinity as a contradiction that self-produces',
    clauses: [
      'assert(pluralityUtterlyExternalToOne(true))',
      'assert(repulsionIsSelfReference(true))',
      'assert(infinityAsSelfProducingContradiction(true))'
    ],
    predicates: [{ name: 'OM_Infinity_Contradiction', args: [] }],
    relations: []
  }
]
