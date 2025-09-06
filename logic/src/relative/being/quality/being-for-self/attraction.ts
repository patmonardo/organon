import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'bfs-ra-a-exclusion-to-attraction',
    title: 'C. Repulsion and Attraction — a) From exclusion to attraction',
    text: `This consideration regarding the ones:
that from either side of their determination,
whether they just are or refer to one another,
they show themselves to be only one and the same,
indistinguishable, is a comparison that belongs to us.
Also to be seen, therefore, is what is posited in them
in their mutual reference itself.
They are (this much is presupposed in this reference)
and they are only inasmuch as
they negate themselves reciprocally
and at the same time keep away this ideality,
their being negated, from themselves, that is,
they negate the reciprocal negating.
But they are only inasmuch as they negate,
and so, since their reciprocal negating is negated,
their being is negated.
To be sure, since they are,
nothing would be negated through this negating
which for them is only something external;
this negating of the other rebounds off them,
coming their way only by striking their surface.
And yet, they turn back upon themselves
only by negating the others;
they are only as this mediation,
this turning back of theirs is their self-preservation
and their being-for- itself.
Since their negating is ineffectual
because of the resistance offered by the others,
whether as existents or as negating,
they do not return back to themselves,
do not preserve themselves, and so are not.
It was previously remarked that
the ones themselves are each a one like any other.
This is not just a matter of our
connecting them by way of reference,
of bringing them together externally;
repulsion is itself a referring;
the one that excludes the ones refers itself to them,
to the ones, that is, to itself.
The negative relating of the ones to one another is consequently
only a coming-together-with-oneself.
This identity in which their repelling crosses over is
the sublation of their diversity and externality
which they should have rather asserted with respect to
each other by excluding each other.

This self-positing-in-a-one of the many ones is attraction.`,
    summary:
      'Comparison shows indistinguishability, but the inner posit is this: the ones are only insofar as they negate. If they negate the reciprocal negating, they negate their being. Though negation seems “external,” their return-to-self is only via negating others (mediation); if ineffectual, they fail to preserve themselves. Repulsion is itself a referring that returns to self, sublating diversity/externality. The many self-posit in a one: attraction.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'bfs-op-ra-a11-comparison-vs-posited-reference',
    chunkId: 'bfs-ra-a-exclusion-to-attraction',
    label: 'Beyond comparison: see what is posited in mutual reference itself',
    clauses: [
      'assert(comparisonShowsIndistinguishable(true))',
      'assert(focusOnPositedInMutualReference(true))'
    ],
    predicates: [{ name: 'RA_Posited_In_Mutual_Reference', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-a12-negating-negation-negates-being',
    chunkId: 'bfs-ra-a-exclusion-to-attraction',
    label: 'They are only insofar as they negate; negating the reciprocal negating negates their being',
    clauses: [
      'assert(onlyInsofarAsNegate(true))',
      'assert(keepAwayIdealityOfBeingNegated(true))',
      'assert(negateReciprocalNegating(true))',
      'assert(therebyBeingIsNegated(true))'
    ],
    predicates: [{ name: 'RA_Negation_of_Negation', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-a13-mediation-or-failure-to-return',
    chunkId: 'bfs-ra-a-exclusion-to-attraction',
    label: 'Return-to-self only via negating others (mediation); if negation rebounds or is resisted, no preservation (are not)',
    clauses: [
      'assert(negationSeemsExternalRebound(true))',
      'assert(returnToSelfOnlyViaNegatingOthers(true))',
      'assert(beingIsMediationOfReturn(true))',
      'assert(resistanceMakesNegationIneffectual(true))',
      'assert(noReturnNoPreservationNoBeing(true))'
    ],
    predicates: [{ name: 'RA_Mediation_vs_Ineffectual', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-a14-repulsion-as-referring-identity-sublation',
    chunkId: 'bfs-ra-a-exclusion-to-attraction',
    label: 'Repulsion is a referring that returns to itself; negative relating is a coming-together-with-oneself; diversity/externality sublated',
    clauses: [
      'assert(repulsionIsReferring(true))',
      'assert(excludingOneRefersToOnesThusToItself(true))',
      'assert(negativeRelatingIsComingTogetherWithSelf(true))',
      'assert(identityAsSublationOfDiversityExternality(true))'
    ],
    predicates: [{ name: 'RA_Repulsion_Return_Identity', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-a15-attraction-definition',
    chunkId: 'bfs-ra-a-exclusion-to-attraction',
    label: 'Self-positing-in-a-one of the many ones is attraction',
    clauses: [
      'assert(selfPositingInOneOfMany(true))',
      'assert(attraction(true))'
    ],
    predicates: [{ name: 'RA_Attraction_Definition', args: [] }],
    relations: []
  }
]
