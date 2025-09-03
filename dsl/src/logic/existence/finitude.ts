import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'finitude-1a-something-other-symmetric',
    title: 'Something↔Other I: symmetry; “this” as external pointing',
    text: `a. Something and other:

1. Something and other are,
first, both existents or something.
Second, each is equally an other.
It is indifferent which is named first,
and just for this reason it is named something
(in Latin, when they occur in a proposition,
both are aliud, or “the one, the other,” alius alium;
in the case of an alternating relation,
the analogous expression is alter alterum).
If of two beings we call the one A and the other B,
the B is the one which is first determined as other.
But the A is just as much the other of the B.
Both are other in the same way.
“This” serves to fix the distinction
and the something which is to be taken in the affirmative sense.
But “this” also expresses the fact that the distinction,
and the privileging of one something,
is a subjective designation that falls outside the something itself.
The whole determinateness falls on the side of this external pointing;
also the expression “this” contains no distinctions;
each and every something is just as good a “this” as any other.
By “this” we mean to express something completely determinate,
overlooking the fact that language, as a work of the understanding,
only expresses the universal, albeit naming it as a single object.
But an individual name is something meaningless in the sense
that it does not express a universal.
It appears as something merely posited
and arbitrary for the same reason
that proper names can also be arbitrarily picked,
arbitrarily given as well as arbitrarily altered.`,
    concise:
      'Symmetry: both are existents; each is equally other. Any “this” that fixes one side is an external, subjective pointing; language names universals, so “this” and proper names lack intrinsic determination.'
  },
  {
    id: 'finitude-1b-otherness-as-external',
    title: 'Something↔Other II: otherness as alien/external (Third’s comparison)',
    text: `Otherness thus appears as a determination
alien to the existence thus pointed at,
or the other existence as outside this one existence,
partly because the one existence is determined as other
only by being compared by a Third,
and partly because it is so determined
only on account of the other which is outside it,
but is not an other for itself.
At the same time, as has been remarked,
even for ordinary thinking every existence
equally determines itself as an other existence,
so that there is no existence
that remains determined simply as an existence,
none which is not outside an existence
and therefore is not itself an other.`,
    concise:
      'Externalization: at first, “other” seems alien—assigned by a Third or by comparison to an outside. Yet, for ordinary thinking, every existence also determines itself as other; none is simply “just existence.”'
  },
  {
    id: 'finitude-1c-other-for-itself',
    title: 'Something↔Other III: sameness of determinations vs other-for-itself',
    text: `Both are determined as something as well as other:
thus they are the same and there is as yet
no distinction present in them.
But this sameness of determinations, too,
falls only within external reflection,
in the comparison of the two;
but the other, as posited at first,
though an other with reference to something,
is other also for itself apart from the something.`,
    concise:
      'Externally, both share “something/other,” seeming the same. But beyond external reflection, the other is also other-for-itself, not merely relative to a given something.'
  },
  {
    id: 'finitude-1d-to-heteron-nature',
    title: 'Other-in-itself: Plato’s to heteron; nature as other of spirit',
    text: `Third, the other is therefore to be taken in isolation,
with reference to itself, has to be taken abstractly as the other,
the 'to heteron' of Plato who opposes it to the one
as a moment of totality,
and in this way ascribes to the other a nature of its own.
Thus the other, taken solely as such,
is not the other of something,
but is the other within, that is, the other of itself.
Such an other, which is the other by its own determination,
is physical nature; nature is the other of spirit;
this, its determination, is at first
a mere relativity expressing not a quality of nature itself
but only a reference external to it.
But since spirit is the true something,
and hence nature is what it is within only in contrast to spirit,
taken for itself the quality of nature is just this,
to be the other within, that which-exists-outside-itself
(in the determinations of space, time, matter).`,
    concise:
      'Other-in-itself: as Plato’s to heteron—moment of totality—other is “other within,” other of itself. Nature, as other of spirit, exemplifies this: existing-outside-itself (space, time, matter).'
  },
  {
    id: 'finitude-1e-other-of-other-alteration-identity',
    title: 'Other-of-other: absolute inequality, alteration, and self-identity',
    text: `The other which is such for itself is the other within it,
hence the other of itself and so the other of the other;
therefore, the absolutely unequal in itself,
that which negates itself, alters itself.
But it equally remains identical with itself,
for that into which it alters is the other,
and this other has no additional determination;
but that which alters itself is not determined in
any other way than in this, to be an other;
in going over to this other, it only unites with itself.
It is thus posited as reflected into itself
with sublation of the otherness,
a self-identical something from which the otherness,
which is at the same time a moment of it, is therefore distinct,
itself not appertaining to it as something.`,
    concise:
      'Other-for-itself is other-of-itself (other-of-other): absolute inequality that negates/alter itself; yet the passage unites with itself (other adds nothing new). Hence reflected-into-itself: self-identical something with otherness as a distinct moment.'
  },
  {
    id: 'finitude-2a-preserves-in-nonbeing-bfo-bii',
    title: 'Something VI: preserves itself in non-being; being-for-other; being-in-itself',
    text: `2. The something preserves itself in its non-being;
it is essentially one with it, and essentially not one with it.
It therefore stands in reference to an otherness
without being just this otherness.
The otherness is at once contained in it
and yet separated from it;
it is being-for-other.
Existence as such is an immediate, bare of references;
or, it is in the determination of being.
However, as including non-being within itself,
existence is determinate being,
being negated within itself,
and then in the first instance an other;
but, since in being negated it preserves itself
at the same time, it is only being-for-other.
It preserves itself in its non-being and is being;
not, however, being in general but being with reference
to itself in contrast to its reference to the other,
as self-equality in contrast to its inequality.
Such a being is being-in-itself.`,
    concise:
      'Something preserves itself in its non-being: essentially one with it and not-one. Otherness is both contained and separated → being-for-other. As negated-within-itself, existence is determinate being and, at first, only being-for-other; as self-equality against the other, this is being-in-itself.'
  },
  {
    id: 'finitude-2b-two-pairs-moments-connection',
    title: 'Something VII: two pairs; truth as connection; mutual containment of moments',
    text: `Being-for-other and being-in-itself constitute
the two moments of something.
There are here two pairs of determinations:
(1) something and other;
(2) being-for-other and being-in-itself.
The former contain the non-connectedness of their determinateness;
something and other fall apart.
But their truth is their connection;
being-for-other and being-in-itself are
therefore the same determinations posited as
moments of one and the same unity,
as determinations which are connections
and which, in their unity,
remain in the unity of existence.
Each thus itself contains within it, at the same time,
also the moment diverse from it.
Being and nothing in their unity, which is existence,
are no longer being and nothing
(these they are only outside their unity);
so in their restless unity, in becoming,
they are coming-to-be and ceasing-to-be.
In the something, being is being-in-itself.
Now, as self-reference, self-equality,
being is no longer immediately,
but is self-reference only as the non-being of otherness
(as existence reflected into itself).
The same goes for non-being:
as the moment of something in this
unity of being and non-being:
it is not non-existence in general
but is the other, and more determinedly,
according as being is at the same time distinguished from it,
it is reference to its non-existence, being-for-other.`,
    concise:
      'Two pairs: (something|other) vs (being-for-other|being-in-itself). Truth = their connection: both are moments of one unity and mutually contain each other. In something: being = being-in-itself (self-reference as non-being of otherness); non-being = other, i.e., being-for-other (reference to non-existence).'
  },
  {
    id: 'finitude-2c-being-in-itself-dual',
    title: 'Something VIII: being-in-itself—dual character',
    text: `Hence being-in-itself is, first,
negative reference to non-existence;
it has otherness outside it and is opposed to it;
in so far as something is in itself,
it is withdrawn from being-other and being-for-other.
But, second, it has non-being also right in it;
for it is itself the non-being of being-for-other.`,
    concise:
      'Being-in-itself: (1) negative reference to non-existence—withdrawn from being-(for-)other; (2) equally has non-being in it—namely, as the non-being of being-for-other.'
  },
  {
    id: 'finitude-2d-being-for-other-dual',
    title: 'Something IX: being-for-other—dual character; reciprocal pointing',
    text: `But being-for-other is, first, the negation of
the simple reference of being to itself
which, in the first place, is supposed
to be existence and something;
in so far as something is in an other or for an other,
it lacks a being of its own.
But, second, it is not non-existence as pure nothing;
it is non-existence that points to being-in-itself
as its being reflected into itself,
just as conversely the being-in-itself points to being-for-other.`,
    concise:
      'Being-for-other: (1) negates simple self-reference (lack of its own being when “in/for” an other); (2) not pure nothing—its non-existence points to being-in-itself, and conversely being-in-itself points back to being-for-other.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'finitude-op-1-symmetric-otherness-and-this',
    chunkId: 'finitude-1a-something-other-symmetric',
    label: 'Symmetric otherness; “this” as external subjective designation',
    clauses: [
      'assert(bothAre(["Something","Other"],"existents"))',
      'assert(eachEquallyOther("Something","Other"))',
      'tag(This,"external-pointing")',
      'tag(This,"subjective-designation")',
      'annotate(Language,{expresses:"universal"})',
      'assert(properNamesArbitrary(true))'
    ],
    predicates: [{ name: 'SymmetricOtherness', args: [] }],
    relations: [
      { predicate: 'equallyOtherThan', from: 'Something', to: 'Other' },
      { predicate: 'equallyOtherThan', from: 'Other', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-2-externalization-by-third',
    chunkId: 'finitude-1b-otherness-as-external',
    label: 'Otherness as alien/external (comparison by a Third); yet self-othering in ordinary thinking',
    clauses: [
      'assert(determinedAsOtherByThird(Existence))',
      'assert(outsideOf(Other,Something))',
      'tag(Otherness,"alien")',
      'assert(everyExistenceSelfOthers(true))'
    ],
    predicates: [{ name: 'ExternalOtherness', args: [] }],
    relations: [
      { predicate: 'comparedBy', from: 'Existence', to: 'Third' },
      { predicate: 'outsideOf', from: 'Other', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-3-other-for-itself',
    chunkId: 'finitude-1c-other-for-itself',
    label: 'Beyond external reflection: the other-for-itself',
    clauses: [
      'tag(Other,"for-itself")',
      'assert(notMerelyRelativeTo(Other,Something))',
      'annotate(Reflection,{external:true})'
    ],
    predicates: [{ name: 'OtherForItself', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-4-to-heteron-nature-of-spirit',
    chunkId: 'finitude-1d-to-heteron-nature',
    label: 'Plato’s to heteron; nature as other of spirit (outside-itself)',
    clauses: [
      'tag(Other,"to-heteron")',
      'assert(momentOfTotality(Other))',
      'assert(otherOf(Nature,Spirit))',
      'tag(Nature,"exists-outside-itself")',
      'annotate(Nature,{determinations:["space","time","matter"]})'
    ],
    predicates: [{ name: 'OtherInItself', args: [] }],
    relations: [
      { predicate: 'otherOf', from: 'Nature', to: 'Spirit' }
    ]
  },
  {
    id: 'finitude-op-5-other-of-other-alteration-identity',
    chunkId: 'finitude-1e-other-of-other-alteration-identity',
    label: 'Other-of-other: absolute inequality; alteration that unites with itself',
    clauses: [
      'tag(Other,"other-of-itself")',
      'tag(Other,"other-of-other")',
      'assert(negatesItself(Other))',
      'assert(altersItself(Other))',
      'assert(remainsIdenticalThrough(Other,"passage-into-other"))',
      'assert(reflectedIntoItself(Something))',
      'assert(sublatesOtherness(Something))',
      'annotate(Something,{otherness:"distinct-moment"})'
    ],
    predicates: [{ name: 'AlterationAndIdentity', args: [] }],
    relations: [
      { predicate: 'reflectsIntoItself', from: 'Something', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-6-preserves-in-nonbeing',
    chunkId: 'finitude-2a-preserves-in-nonbeing-bfo-bii',
    label: 'Preserves itself in non-being; contained-and-separated otherness → being-for-other; toward being-in-itself',
    clauses: [
      'assert(preservesItselfIn(Something,NonBeing))',
      'assert(essentiallyOneWith(Something,NonBeing))',
      'assert(essentiallyNotOneWith(Something,NonBeing))',
      'tag(Otherness,"contained-and-separated")',
      'tag(BeingForOther,"defined")',
      'assert(determinateBeing(Existence))',
      'assert(negatedWithinItself(Existence))',
      'tag(Something,"at-first-being-for-other")',
      'tag(BeingInItself,"named-as-self-equality")'
    ],
    predicates: [{ name: 'PreservationInNonBeing', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-7-moments-and-connection',
    chunkId: 'finitude-2b-two-pairs-moments-connection',
    label: 'Two pairs; truth as connection; mutual containment; being↔non-being reinterpreted',
    clauses: [
      'assert(pairs([["Something","Other"],["BeingForOther","BeingInItself"]]))',
      'assert(truthIsConnection(["BeingForOther","BeingInItself"]))',
      'assert(momentOf(BeingForOther,Something))',
      'assert(momentOf(BeingInItself,Something))',
      'assert(containsMoment(BeingForOther,BeingInItself))',
      'assert(containsMoment(BeingInItself,BeingForOther))',
      'tag(Being,"as-being-in-itself")',
      'tag(NonBeing,"as-being-for-other")'
    ],
    predicates: [{ name: 'MomentsConnected', args: [] }],
    relations: [
      { predicate: 'momentOf', from: 'BeingForOther', to: 'Something' },
      { predicate: 'momentOf', from: 'BeingInItself', to: 'Something' }
    ]
  },
  {
    id: 'finitude-op-8-being-in-itself-dual',
    chunkId: 'finitude-2c-being-in-itself-dual',
    label: 'Being-in-itself: (1) negative reference to non-existence; (2) non-being of being-for-other',
    clauses: [
      'tag(BeingInItself,"negative-reference-to-non-existence")',
      'assert(hasOthernessOutside(BeingInItself))',
      'assert(withdrawnFrom(BeingInItself,["BeingOther","BeingForOther"]))',
      'assert(nonBeingOf(BeingInItself,BeingForOther))'
    ],
    predicates: [{ name: 'BeingInItselfDual', args: [] }],
    relations: []
  },
  {
    id: 'finitude-op-9-being-for-other-dual',
    chunkId: 'finitude-2d-being-for-other-dual',
    label: 'Being-for-other: (1) negation of simple self-reference; (2) points to being-in-itself (and back)',
    clauses: [
      'tag(BeingForOther,"negation-of-self-reference")',
      'assert(lacksOwnBeingWhenForOther(Something))',
      'tag(BeingForOther,"not-pure-nothing")',
      'assert(pointsTo(BeingForOther,BeingInItself))',
      'assert(pointsTo(BeingInItself,BeingForOther))'
    ],
    predicates: [{ name: 'BeingForOtherDual', args: [] }],
    relations: [
      { predicate: 'pointsTo', from: 'BeingForOther', to: 'BeingInItself' },
      { predicate: 'pointsTo', from: 'BeingInItself', to: 'BeingForOther' }
    ]
  }
]
