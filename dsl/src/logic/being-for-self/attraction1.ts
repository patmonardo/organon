import type { Chunk, LogicalOperation } from './index'

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'bfs-ra-b-one-one-of-attraction',
    title: 'C. Repulsion and Attraction — b) The one one of attraction (I)',
    text: `Repulsion is the fragmentation of the one,
first into the many of which
it is the negative relating,
since they presuppose each other as each existent;
it is only the ought of ideality;
this ideality will, however, be realized in attraction.
Repulsion passes over into attraction,
the many ones into one one.
Both, repulsion and attraction, are
at first distinguished from each other,
repulsion as the reality of the ones,
attraction as their posited ideality.
Attraction refers to repulsion
by having it for a presupposition.
Repulsion delivers the material for attraction.
If there were no ones,
there would be nothing to attract;
the representation of continuing attraction,
of the consumption of the ones,
presupposes an equally continuing generation of the ones;
the sense representation of spatial attraction
gives continuity to the flow of ones to be attracted;
to replace the atoms that vanish at the point of attraction,
another multitude comes forth from the void,
infinitely if one so wishes.
If attraction were represented as accomplished, that is,
the many as brought to the point of the one one,
the result would be just an inert one, no longer any attraction.
The ideality immediately present in attraction
still also has in it the determination of the negation of itself,
the many ones to which it refers;
attraction is inseparable from repulsion.`,
    concise:
      'Repulsion fragments the one into many (the ought of ideality), which is realized in attraction: many ones pass into one one. Repulsion = reality of ones; attraction = posited ideality and presupposes repulsion (its material). Continuous attraction presupposes continuous generation (a flow from the void). If fully accomplished, only an inert one remains (no attraction). Attraction bears its own negation (the many) and is inseparable from repulsion.'
  },
  {
    id: 'bfs-ra-b-one-one-of-attraction-ii',
    title: 'C. Repulsion and Attraction — b) The one one of attraction (II)',
    text: `To attract pertains at first in equal measure
to each of the many ones as immediately present;
none has advantage over an other;
what would result then is an equilibrium in the attraction,
or more precisely, an equilibrium in the attraction
and the repulsion themselves, and an inert state of rest
without any ideality present there.
But there can be no question here of
any such immediately present one taking precedence over another,
for this would presuppose a determinate distinction between them;
attraction is rather the positing of
the given lack of distinction among the ones.
Attraction is itself the positing in the first place
of a one distinct from other ones;
these are only the immediate ones
that are to preserve themselves through repulsion;
through their posited negation, however, what proceeds
is the one of attraction
which is therefore determined as the mediated one,
the one posited as one.
The first ones, as immediate, do not in their ideality
return into themselves,
but have this ideality each in another.`,
    concise:
      'Equal applicability of attraction to all immediate ones would yield equilibrium (attraction/repulsion) and inert rest (no ideality). No precedence is presupposable; attraction posits their lack of distinction. Attraction first posits a one distinct from others: the mediated “one of attraction,” arising through the posited negation of immediate ones that preserve themselves via repulsion. Immediate ones do not return to themselves in their ideality; each has it in another.'
  },
  {
    id: 'bfs-ra-b-one-one-of-attraction-iii',
    title: 'C. Repulsion and Attraction — b) The one one of attraction (III)',
    text: `The one one is, however,
ideality that has been realized,
posited in the one;
it attracts through the mediation of repulsion;
it contains in itself this mediation as its determination.
It thus does not swallow the attracted ones
within it as into one point, that is,
does not sublate them abstractly.
Since it contains repulsion in its determination,
the latter equally preserves the ones as many within it;
by its attracting, it musters, so to speak,
something before it, gains an area or a filling.
Thus there is in it the unity
of repulsion and attraction in general.`,
    concise:
      'The “one one” is realized ideality posited in the one; it attracts via repulsion and contains this mediation as its determination. It does not swallow/sublate abstractly; because it contains repulsion, it preserves the many within, mustering a filling/area. It is the unity of repulsion and attraction.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'bfs-op-ra-b1-fragmentation-and-ought',
    chunkId: 'bfs-ra-b-one-one-of-attraction',
    label: 'Repulsion fragments the one into many; it is only the ought of ideality, realized in attraction',
    clauses: [
      'assert(fragmentsOneIntoMany("repulsion",true))',
      'assert(negativeRelatingOfMany(true))',
      'assert(oughtOfIdeality("repulsion",true))',
      'assert(idealityRealizedIn("attraction"))'
    ],
    predicates: [{ name: 'RA_Fragmentation_Ought', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b2-transition-and-roles',
    chunkId: 'bfs-ra-b-one-one-of-attraction',
    label: 'Repulsion passes over into attraction; many ones into one one; roles: repulsion=reality, attraction=posited ideality',
    clauses: [
      'assert(passesOver("repulsion","attraction"))',
      'assert(transforms("many-ones","one-one"))',
      'assert(role("repulsion","reality-of-ones"))',
      'assert(role("attraction","posited-ideality"))'
    ],
    predicates: [{ name: 'RA_Transition_Roles', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b3-presupposition-material',
    chunkId: 'bfs-ra-b-one-one-of-attraction',
    label: 'Attraction presupposes repulsion; repulsion delivers the material (the ones)',
    clauses: [
      'assert(presupposes("attraction","repulsion"))',
      'assert(deliversMaterial("repulsion","ones"))',
      'assert(ifNo("ones"),"nothingToAttract")'
    ],
    predicates: [{ name: 'RA_Presupposition_Material', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b4-continuity-flow-from-void',
    chunkId: 'bfs-ra-b-one-one-of-attraction',
    label: 'Continuing attraction presupposes continuing generation; sense-image: spatial flow of ones from the void',
    clauses: [
      'assert(continuingAttractionRequiresGeneration(true))',
      'assert(flowOfOnesFromVoid(true))',
      'assert(replacesVanishedAtomsAtPointOfAttraction(true))'
    ],
    predicates: [{ name: 'RA_Continuity_Flow', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b5-accomplished-inert-one',
    chunkId: 'bfs-ra-b-one-one-of-attraction',
    label: 'If attraction were accomplished (many → one one), result is inert one; no attraction remains',
    clauses: [
      'assert(ifAccomplishedAttractionThen("inert-one"))',
      'assert(noAttractionRemains(true))'
    ],
    predicates: [{ name: 'RA_Accomplished_Inert', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b6-bears-negation-inseparable',
    chunkId: 'bfs-ra-b-one-one-of-attraction',
    label: 'Attraction bears its own negation (the many ones) and is inseparable from repulsion',
    clauses: [
      'assert(containsDeterminationOfItsNegation("attraction","many-ones"))',
      'assert(inseparableFrom("attraction","repulsion"))'
    ],
    predicates: [{ name: 'RA_Negation_Inseparable', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b7-equilibrium-no-ideality',
    chunkId: 'bfs-ra-b-one-one-of-attraction-ii',
    label: 'Equal attraction to all immediate ones ⇒ equilibrium of attraction/repulsion ⇒ inert rest (no ideality)',
    clauses: [
      'assert(equalAttractionToAllImmediateOnes(true))',
      'assert(equilibriumAttractionAndRepulsion(true))',
      'assert(inertRestNoIdeality(true))'
    ],
    predicates: [{ name: 'RA_Equilibrium_NoIdeality', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b8-no-precedence-lack-of-distinction',
    chunkId: 'bfs-ra-b-one-one-of-attraction-ii',
    label: 'No precedence without determinate distinction; attraction posits lack of distinction among ones',
    clauses: [
      'assert(noPrecedenceWithoutDistinction(true))',
      'assert(attractionPositsLackOfDistinction(true))'
    ],
    predicates: [{ name: 'RA_NoPrecedence_LackOfDist', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b9-positing-mediated-one',
    chunkId: 'bfs-ra-b-one-one-of-attraction-ii',
    label: 'Attraction posits a one distinct from others: the mediated one (the one posited as one)',
    clauses: [
      'assert(attractionPositsOneDistinct(true))',
      'assert(mediatedOnePositedAsOne(true))'
    ],
    predicates: [{ name: 'RA_Mediated_One', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b10-immediates-preserve-via-repulsion-negation-yields-one-of-attraction',
    chunkId: 'bfs-ra-b-one-one-of-attraction-ii',
    label: 'Immediate ones preserve via repulsion; through their posited negation proceeds the one of attraction',
    clauses: [
      'assert(immediateOnesPreserveThroughRepulsion(true))',
      'assert(positedNegationGeneratesOneOfAttraction(true))'
    ],
    predicates: [{ name: 'RA_Posited_Negation_Generates_One', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b11-ideality-in-another-not-self-return',
    chunkId: 'bfs-ra-b-one-one-of-attraction-ii',
    label: 'Immediate ones do not return to self in ideality; each has its ideality in another',
    clauses: [
      'assert(noSelfReturnOfIdealityForImmediates(true))',
      'assert(idealityEachInAnother(true))'
    ],
    predicates: [{ name: 'RA_Ideality_In_Another', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b12-realized-ideality-mediated-attraction',
    chunkId: 'bfs-ra-b-one-one-of-attraction-iii',
    label: 'The one one = realized ideality posited in the one; attracts through mediation of repulsion (mediation as determination)',
    clauses: [
      'assert(realizedIdealityPositedInOne(true))',
      'assert(attractsThroughMediationOfRepulsion(true))',
      'assert(containsMediationAsDetermination(true))'
    ],
    predicates: [{ name: 'RA_Realized_Ideality_Mediated', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b13-no-abstract-sublation',
    chunkId: 'bfs-ra-b-one-one-of-attraction-iii',
    label: 'Does not swallow into a point; does not sublate the attracted ones abstractly',
    clauses: [
      'assert(doesNotSwallowIntoPoint(true))',
      'assert(doesNotSublateAbstractly(true))'
    ],
    predicates: [{ name: 'RA_No_Abstract_Sublation', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b14-preserves-many-filling',
    chunkId: 'bfs-ra-b-one-one-of-attraction-iii',
    label: 'Contains repulsion in determination; preserves the many within; musters a filling/area',
    clauses: [
      'assert(containsRepulsionInDetermination(true))',
      'assert(preservesManyWithin(true))',
      'assert(mustersFillingArea(true))'
    ],
    predicates: [{ name: 'RA_Preserves_Many_Filling', args: [] }],
    relations: []
  },
  {
    id: 'bfs-op-ra-b15-unity-repulsion-attraction',
    chunkId: 'bfs-ra-b-one-one-of-attraction-iii',
    label: 'Unity of repulsion and attraction in general',
    clauses: [
      'assert(unityOfRepulsionAndAttraction(true))'
    ],
    predicates: [{ name: 'RA_Unity_Repulsion_Attraction', args: [] }],
    relations: []
  }
]
