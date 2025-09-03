import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'bfs-a-existence-and-being-for-itself',
    title: 'a) Existence and being-for-itself',
    text: `As already mentioned, being-for-itself is
infinity that has sunk into simple being;
it is existence in so far as in the
now posited form of the immediacy
of being the negative nature of infinity,
which is the negation of negation,
is only as negation in general,
as infinite qualitative determinateness.
But in such a determinateness, wherein it is existence,
being is at once also distinguished
from this very being-for-itself
which is such only as infinite
qualitative determinateness;
nevertheless, existence is at the same time
a moment of being-for-itself,
for the latter certainly contains
being affected by negation.
So the determinateness which in existence as such is
an other, and a being-for-other,
is bent back into the infinite unity of being-for-itself,
and the moment of existence is present
in the being-for-itself as being-for-one.`,
    concise:
      'Being-for-itself is infinity sunk into simple being, so existence appears as infinite qualitative determinateness. The otherness of existence is bent back into the unity of being-for-itself; existence persists there as the moment being-for-one.'
  },
  {
    id: 'bfs-b-being-for-one',
    title: 'b) Being-for-one',
    text: `This moment gives expression to how the finite is
in its unity with the infinite or as an idealization.
Being-for-itself does not have negation in it as
a determinateness or limit,
and consequently also not as reference
to an existence other than it.
Although this moment is now being
designated as being-for-one,
there is yet nothing at hand for which it would be;
there is not the one of which it would be the moment.
There is in fact nothing of the sort
yet fixed in being-for-itself;
that for which something (and there is no something here)
would be, what the other side in general should be,
is likewise a moment,
itself only being-for-one,
not yet a one.
What we have before us, therefore,
is still an undistinguishedness of two sides
that may suggest themselves in the being-for-one;
there is only one being-for-another,
and since this is only one being-for-another,
it is also only being-for-one;
there is only the one ideality,
of that for which or in which
there should be a determination as moment,
and of that which should be the moment in it.
Being-for-one and being-for-itself do not therefore
constitute two genuine determinacies,
each as against the other.
Inasmuch as the distinction is momentarily assumed
and we speak of a-being-for-itself,
it is this very being-for-itself,
as the sublated being of otherness,
that refers itself to itself as to the sublated other,
is therefore for-one;
in its other it refers itself only to itself.
An idealization is necessarily for-one,
but it is not for an other;
the one, for which it is, is only itself.
The "I," therefore, spirit in general,
or God, are idealizations,
because they are infinite;
as existents which are for-themselves, however,
they are not ideationally different
from that which is for-one.
For if they were different,
they would be only immediate,
or, more precisely, they would only be
existence and a being-for-another;
for if the moment of being for-one did
not attach to them,
it is not they themselves
but an other that would be
that which is for them.
God is therefore for himself,
in so far he is himself
that which is for him.

Being-for-itself and being-for-one are not, therefore,
diverse significations of ideality
but essential, inseparable, moments of it.`,
    concise:
      'Being-for-one names the moment of ideality without an external One; the two sides are undistinguished. If distinguished, for-itself refers to itself in the other; I/spirit/God illustrate that for-itself is not different from for-one; both are inseparable moments.'
  },
  {
    id: 'bfs-c-the-one',
    title: 'c) The one',
    text: `Being-for-itself is the simple unity of
itself and its moments, of the being-for-one.
There is only one determination present,
the self-reference itself of the sublating.
The moments of being-for-itself have sunk into
an indifferentiation which is immediacy or being,
but an immediacy that is based on
the negating posited as its determination.
Being-for-itself is thus an existent-for-itself,
and, since in this immediacy its inner meaning vanishes,
it is the totally abstract limit of itself: the one.

Attention may be drawn in advance
to the difficulties that lie ahead
in the exposition of the development of the one,
and to the source of these difficulties.
The moments that constitute the concept of
the one as being-for-itself
occur in it one outside the other;
they are
(1) negation in general;
(2) two negations that are, therefore,
(3) the same,
(4) absolutely opposed;
(5) self-reference, identity as such;
(6) negative reference which is nonetheless self-reference.
These moments occur here apart because
the form of immediacy, of being, enters into
the being-for-itself as existent-for-itself;
because of this immediacy, each moment is posited
as a determination existent on its own,
and yet they are just as inseparable.
Hence, of each determination the opposite must equally be said;
it is this contradiction that causes the difficulty
that goes with the abstract nature of the moments.`,
    concise:
      'The One is being-for-itself as simple self-reference of sublating: an existent-for-itself whose immediacy makes it the abstract limit of itself. Its concept-moments appear apart (negation, two negations, sameness, absolute opposition, identity, negative-yet-self-reference) yet are inseparable; each bears its opposite, yielding the difficulty.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'bfs-op-a1-def-being-for-itself',
    chunkId: 'bfs-a-existence-and-being-for-itself',
    label: 'Definition: being-for-itself = infinity sunk into simple being; existence = infinite qualitative determinateness',
    clauses: [
      'assert(is(BeingForItself,"infinity-in-simple-being"))',
      'assert(exhibits(BeingForItself,"negation-of-negation-as-general"))',
      'assert(is(Existence,"infinite-qualitative-determinateness"))'
    ],
    predicates: [{ name: 'BFS_Definition', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-a2-existence-as-moment',
    chunkId: 'bfs-a-existence-and-being-for-itself',
    label: 'Existence is a moment of being-for-itself (being affected by negation)',
    clauses: [
      'assert(distinguishedFrom(Being,BeingForItself))',
      'assert(contains(BeingForItself,"being-affected-by-negation"))',
      'assert(isMomentOf(Existence,BeingForItself))'
    ],
    predicates: [{ name: 'BFS_ExistenceMoment', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-a3-bent-back-being-for-other-to-for-one',
    chunkId: 'bfs-a-existence-and-being-for-itself',
    label: 'Bent-back: being-for-other becomes being-for-one within being-for-itself',
    clauses: [
      'assert(is(ExistenceMode,"being-for-other"))',
      'assert(bentBackInto(ExistenceMode,BeingForItself))',
      'assert(presentAs(Existence, "being-for-one", BeingForItself))'
    ],
    predicates: [{ name: 'BFS_BentBack_ForOther_To_ForOne', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-b1-no-external-one-no-limit',
    chunkId: 'bfs-b-being-for-one',
    label: 'Being-for-one: no external One posited; no limit or reference to another existence',
    clauses: [
      'assert(hasLimit(BeingForItself,false))',
      'assert(hasReferenceToOtherExistence(BeingForItself,false))',
      'tag(Moment,"being-for-one-without-external-one")'
    ],
    predicates: [{ name: 'BFS_ForOne_NoExternal', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-b2-undistinguished-one-ideality',
    chunkId: 'bfs-b-being-for-one',
    label: 'Undistinguished sides: one being-for-another = being-for-one; one ideality',
    clauses: [
      'assert(undistinguishedSides(true))',
      'assert(equals(BeingForAnother,BeingForOne))',
      'tag(Ideality,"single")'
    ],
    predicates: [{ name: 'BFS_OneIdeality', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-b3-self-reference-in-other',
    chunkId: 'bfs-b-being-for-one',
    label: 'If distinguished: for-itself refers to itself as sublated other (hence for-one)',
    clauses: [
      'assert(refersToItselfAsSublatedOther(BeingForItself,true))',
      'assert(is(BeingForItself,BeingForOne))',
      'assert(isForOther(Ideality,false))'
    ],
    predicates: [{ name: 'BFS_SublatedOther', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-b4-idealizations-i-spirit-god',
    chunkId: 'bfs-b-being-for-one',
    label: 'I/Spirit/God: idealizations; as for-themselves not different from for-one; else regress',
    clauses: [
      'tag(Examples,["I","Spirit","God"])',
      'assert(idealizations(["I","Spirit","God"]))',
      'assert(notDifferentFromForOne(["I","Spirit","God"]))',
      'assert(regressIfDifferent("to-immediacy-and-being-for-another",true))',
      'assert(isForItself("God",true))',
      'assert(isThatWhichIsForHim("God","God"))'
    ],
    predicates: [{ name: 'BFS_Idealization_Examples', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-b5-inseparable-moments',
    chunkId: 'bfs-b-being-for-one',
    label: 'Being-for-itself and being-for-one are inseparable moments of ideality',
    clauses: [
      'assert(inseparableMoments(["BeingForItself","BeingForOne"],Ideality))'
    ],
    predicates: [{ name: 'BFS_Inseparable', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-c1-one-as-abstract-limit',
    chunkId: 'bfs-c-the-one',
    label: 'One = existent-for-itself as abstract limit; simple self-reference of sublating grounded in negation',
    clauses: [
      'assert(simpleUnity(BeingForItself,"being-for-one"))',
      'assert(selfReferenceOfSublating(true))',
      'assert(hasImmediacy(BeingForItself,true))',
      'assert(basedOnNegationAsDetermination(BeingForItself,true))',
      'assert(is(One,"existent-for-itself"))',
      'assert(is(AbstractLimit,One))'
    ],
    predicates: [{ name: 'BFS_One_AbstractLimit', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-c2-moments-apart-yet-inseparable-contradiction',
    chunkId: 'bfs-c-the-one',
    label: 'Moments listed: apart due to immediacy, yet inseparable; each bears its opposite (contradiction)',
    clauses: [
      'assert(momentsOfOne(["negation-in-general","two-negations","the-same","absolutely-opposed","identity-self-reference","negative-reference-yet-self-reference"]))',
      'assert(appearApartDueToImmediacy(true))',
      'assert(inseparable(true))',
      'assert(eachBearsOpposite(true))',
      'tag(Difficulty,"abstract-contradictory-moments")'
    ],
    predicates: [{ name: 'BFS_One_Moments_Contradiction', args: [] }],
    relations: []
  }
]
