import type { Chunk, LogicalOperation } from './index'

/*
  Essence — C. REFLECTION — Part 1: Positing reflection
  Two-fold representation:
  - text: verbatim source segmented into readable chunks (preserve full passage)
  - concise: short IR summary to support HLO extraction (non-destructive)
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ess-ref-1-positing-reflection-intro',
    title: 'Positing reflection — from shine to absolute reflection',
    text: `1. Positing reflection

Shine is a nothingness or a lack of essence.
But a nothingness or that which is void of essence
does not have its being in an other in which it shines,
but its being is its own equality with itself;
this conversion of the negative with itself has been determined
as the absolute reflection of essence.`,
    concise: 'From shine (lack of essence) to equality-with-self: conversion of the negative with itself = absolute reflection of essence.'
  },
  {
    id: 'ess-ref-2-self-referring-negativity',
    title: 'Self-referring negativity: negation of itself; unity of being and non-being',
    text: `This self-referring negativity is
therefore the negating of itself.
It is thus just as much
sublated negativity as it is negativity.
Or again, it is itself the negative
and the simple equality with itself or immediacy.
It consists, therefore, in being itself
and not being itself,
and the two in one unity.`,
    concise: 'Self-referring negativity negates itself; it is both negativity and sublated negativity: unity of being-itself and not-being-itself.'
  },
  {
    id: 'ess-ref-3-nothing-to-nothing',
    title: 'Reflection as movement of nothing to nothing; coincidence of negation with itself',
    text: `Reflection is at first the movement of
the nothing to the nothing,
and thus negation coinciding with itself.
This self-coinciding is in general
simple equality with itself, immediacy.
But this falling together is not
the transition of negation into equality
as into a being other than it;
reflection is transition rather
as the sublating of transition,
for it is the immediate falling together
of the negative with itself.
And so this coinciding is, first,
self-equality or immediacy;
but, second, this immediacy is
the self-equality of the negative,
and hence self-negating equality,
immediacy which is in itself the negative,
the negative of itself:
its being is to be what it is not.`,
    concise: 'Reflection: nothing→nothing; negation coincides with itself. Immediacy = self-equality of the negative: being that is to be what it is not.'
  },
  {
    id: 'ess-ref-4-positedness',
    title: 'Immediacy as positedness (immediacy as determinateness/self-reflecting)',
    text: `The self-reference of the negative is
therefore its turning back into itself;
it is immediacy as the sublating of the negative,
but immediacy simply and solely as this reference
or as turning back from a one,
and hence as self-sublating immediacy.
This is positedness,
immediacy purely as determinateness
or as self-reflecting.`,
    concise: 'Turning-back of the negative gives immediacy as positedness: immediacy purely as determinateness/self-reflecting.'
  },
  {
    id: 'ess-ref-5-origin-in-reflection',
    title: 'Immediacy arises from reflection; reflection as turning (start/return)',
    text: `This immediacy, which is only as
the turning back of the negative into itself,
is the immediacy which constitutes the determinateness of shine,
and from which the previous reflective movement seemed to begin.
But, far from being able to begin with this immediacy,
the latter first is rather as the turning back
or as the reflection itself.
Reflection is therefore the movement which,
since it is the turning back,
only in this turning is that
which starts out or returns.`,
    concise: 'We cannot begin with immediacy; it first is as turning-back (reflection). Reflection is the movement where only in turning is the start/return.'
  },
  {
    id: 'ess-ref-6-positing-to-presupposing',
    title: 'From positing (turning-back immediacy) to presupposing',
    text: `It is a positing, inasmuch as it is
immediacy as a turning back;
that is to say, there is not an other beforehand,
one either from which or to which it would turn back;
it is, therefore, only as a turning back
or as the negative of itself.
But further, this immediacy is sublated negation
and sublated return into itself.
Reflection, as the sublating of the negative, is
the sublating of its other, of the immediacy.
Because it is thus immediacy as a turning back,
the coinciding of the negative with itself,
it is equally the negation of the negative as negative.
And so it is presupposing.
Or immediacy is as a turning back
only the negative of itself,
just this, not to be immediacy;
but reflection is the sublating
of the negative of itself,
coincidence with itself;
it therefore sublates its positing,
and inasmuch as it is in its positing
the sublating of positing, it is presupposing.`,
    concise: 'Immediacy-as-turning-back is “positing,” but reflection sublates this positing; thus reflection is presupposing (negation of the negative as negative).'
  },
  {
    id: 'ess-ref-7-structure-of-presupposing',
    title: 'Structure of presupposing: turning-back of the negative of itself',
    text: `In presupposing, reflection determines the turning back
into itself as the negative of itself,
as that of which essence is the sublating.
It is its relating to itself,
but to itself as to the negative of itself;
only so is it negativity which abides with itself,
self-referring negativity.
Immediacy comes on the scene simply and solely
as a turning back and is that negative
which is the semblance of a beginning,
the beginning which the return negates.
The turning back of essence is therefore its self-repulsion.
Or inner directed reflection is essentially
the presupposing of that from which
the reflection is the turning back.`,
    concise: 'Presupposing = relating to self as negative-of-self (abiding negativity). Immediacy is mere semblance of a beginning; turning-back is self-repulsion.'
  },
  {
    id: 'ess-ref-8-essence-presupposes-itself',
    title: 'Essence presupposes itself; sublating presupposition',
    text: `It is only by virtue of the sublating of
its equality with itself
that essence is equality with itself.
Essence presupposes itself,
and the sublating of this presupposing is essence itself;
contrariwise, this sublating of its presupposition is
the presupposition itself.
Reflection thus finds an immediate before it
which it transcends and from which it is the turning back.`,
    concise: 'Essence equals itself only by sublating its equality; it presupposes itself, and sublating this presupposition is itself presupposition. Reflection finds an immediate to transcend.'
  },
  {
    id: 'ess-ref-9-antecedent-by-being-left-behind',
    title: 'Antecedent arises by being left behind; arriving-at-itself',
    text: `But this turning back is only the presupposing of
what was antecedently found.
This antecedent comes to be only by being left behind;
its immediacy is sublated immediacy.
The sublated immediacy is, contrariwise, the turning
back into itself,
essence that arrives at itself,
simple being equal to itself.
This arriving at itself is thus
the sublating of itself
and self-repelling, presupposing reflection,
and its repelling of itself from itself is
the arriving at itself.`,
    concise: 'The “antecedent” exists only as left-behind; its immediacy is sublated. Repelling-itself-from-itself is arriving-at-itself (presupposing reflection).'
  },
  {
    id: 'ess-ref-10-absolute-internal-counter-repelling',
    title: 'Reflection as absolute internal counter-repelling; presupposition only in turning-back',
    text: `It follows from these considerations that
the movement of reflection is to be taken as
an absolute internal counter-repelling.
For the presupposition of
the turning back into itself
[that from which essence arises,
essence being only as this coming back]
is only in the turning back itself.
Transcending the immediate from which reflection begins
occurs rather only through this transcending;
and the transcending of the immediate is
the arriving at the immediate.
The movement, as forward movement, turns immediately
around into itself and so is only self-movement:
a movement which comes from itself in so far as
positing reflection is presupposing reflection, yet,
as presupposing reflection, is simply positing reflection.`,
    concise: 'Reflection = absolute internal counter-repelling; its presupposition exists only in turning-back. Transcending immediate = arriving at immediate. Positing ↔ presupposing reflection (self-movement).'
  },
  {
    id: 'ess-ref-11-reflection-self-and-nonbeing',
    title: 'Reflection is itself and its non-being; is itself by being negative of itself',
    text: `Thus is reflection itself and its non-being,
and only is itself by being the negative of itself,
for only in this way is the sublating of the negative
at the same time a coinciding with itself.`,
    concise: 'Reflection is itself-and-its-non-being: it is itself by being the negative of itself; sublating the negative coincides with itself.'
  },
  {
    id: 'ess-ref-12-external-reflection',
    title: 'Presupposed immediacy as positedness; determination as negative → external reflection',
    text: `The immediacy which reflection,
as a process of sublating,
presupposes for itself is
simply and solely a positedness,
something in itself sublated
which is not diverse from
reflection's turning back into itself
but is itself only this turning back.
But it is at the same time determined as a negative,
as immediately in opposition to something,
and hence to an other.
And so is reflection determined.
According to this determinateness,
because reflection has a presupposition
and takes its start from the immediate as its other,
it is external reflection.`,
    concise: 'The presupposed immediacy is positedness (already sublated, identical with turning-back), yet determined as negative against an other; thus reflection that starts from the immediate-as-other is external reflection.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ess-ref-op-1-define-positing-reflection',
    chunkId: 'ess-ref-1-positing-reflection-intro',
    label: 'Define positing reflection: conversion of the negative with itself',
    candidateSummary: 'From shine (lack) to equality-with-self; absolute reflection = negative converting with itself.',
    clauses: [
      'assert(Shine == lackOf(Essence))',
      'assert(AbsoluteReflection == convert(Negative, into: selfEquality))',
      'tag(Essence,"absolute-reflection")'
    ],
    predicates: [{ name: 'DefinesPositingReflection', args: [] }],
    relations: [
      { predicate: 'derivesFrom', from: 'Reflection', to: 'Shine' },
      { predicate: 'yields', from: 'Negative', to: 'SelfEquality' }
    ]
  },
  {
    id: 'ess-ref-op-2-self-negation-unity',
    chunkId: 'ess-ref-2-self-referring-negativity',
    label: 'Encode self-referring negativity: self-negation and unity of being/not-being',
    candidateSummary: 'Self-referring negativity negates itself; is both negativity and sublated negativity — being and not-being in one.',
    clauses: [
      'assert(selfRefers(Negativity) && negatesItself(Negativity))',
      'tag(Negativity,"sublated-and-as-such")',
      'assert(unity(BeingItself, NotBeingItself))'
    ],
    predicates: [{ name: 'EncodesSelfRefNegativity', args: [] }],
    relations: [{ predicate: 'unifies', from: 'Negativity', to: 'Being∧NotBeing' }]
  },
  {
    id: 'ess-ref-op-3-coinciding-negation',
    chunkId: 'ess-ref-3-nothing-to-nothing',
    label: 'Model reflection as nothing→nothing; immediacy as self-negating equality',
    candidateSummary: 'Reflection is negation coinciding with itself; immediacy is “being to be what it is not”.',
    clauses: [
      'assert(path(Reflection) == from(Nothing) -> to(Nothing))',
      'assert(coincidesWithItself(Negation) == true)',
      'tag(Immediacy,"self-equality-of-negative")',
      'annotate(Immediacy,{paradox:"its being is to be what it is not"})'
    ],
    predicates: [{ name: 'ModelsNothingToNothing', args: [] }],
    relations: [
      { predicate: 'coincides', from: 'Negation', to: 'Negation' },
      { predicate: 'constitutes', from: 'Reflection', to: 'Immediacy' }
    ]
  },
  {
    id: 'ess-ref-op-4-define-positedness',
    chunkId: 'ess-ref-4-positedness',
    label: 'Define positedness: immediacy as determinateness via turning-back',
    candidateSummary: 'Positedness = immediacy purely as determinateness (self-reflecting) produced by turning-back.',
    clauses: [
      'assert(Positedness == immediacy(as: Determinateness))',
      'assert(turnedBackFrom(Negative, into: Immediacy))',
      'tag(Immediacy,"self-reflecting-determinate")'
    ],
    predicates: [{ name: 'DefinesPositedness', args: [] }],
    relations: [
      { predicate: 'resultsFrom', from: 'Immediacy', to: 'TurningBack' },
      { predicate: 'determinesAs', from: 'Immediacy', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-ref-op-5-origin-in-reflection',
    chunkId: 'ess-ref-5-origin-in-reflection',
    label: 'Assert that immediacy cannot be the starting point; origin is reflection',
    candidateSummary: 'Immediacy first is as turning-back; reflection is the movement in which the start/return are only in the turning.',
    clauses: [
      'assert(cannotBeginWith(Immediacy) == true)',
      'assert(origin(Immediacy) == Reflection)',
      'annotate(Reflection,{structure:"turning-back", startEqualsReturn:true})'
    ],
    predicates: [{ name: 'AssertsOriginInReflection', args: [] }],
    relations: [
      { predicate: 'originates', from: 'Immediacy', to: 'Reflection' },
      { predicate: 'equates', from: 'Reflection', to: 'Start~Return' }
    ]
  },
  {
    id: 'ess-ref-op-6-positing-to-presupposing',
    chunkId: 'ess-ref-6-positing-to-presupposing',
    label: 'From positing (turning-back immediacy) to presupposing',
    candidateSummary: 'Reflection sublates its own positing (negation of negative as negative) and thereby is presupposing.',
    clauses: [
      'assert(Immediacy == turningBack(Negative, into: self))',
      'assert(Reflection == sublate(Negative))',
      'assert(Reflection == sublate(Immediacy))',
      'assert(negates(Negative, as:"negative") && hence(Reflection,"presupposing"))',
      'assert(sublates(Reflection,"its-positing"))'
    ],
    predicates: [{ name: 'DerivesPresupposingFromPositing', args: [] }],
    relations: [
      { predicate: 'sublates', from: 'Reflection', to: 'Positing' },
      { predicate: 'presupposes', from: 'Reflection', to: 'Immediacy' }
    ]
  },
  {
    id: 'ess-ref-op-7-define-presupposing-structure',
    chunkId: 'ess-ref-7-structure-of-presupposing',
    label: 'Define presupposing as self-relation to the negative-of-self',
    candidateSummary: 'Presupposing: relate-to-self as negative-of-self; immediacy is semblance of a beginning; turning-back = self-repulsion.',
    clauses: [
      'assert(presupposing(Reflection) == relatesTo(Self, as: negativeOf(Self)))',
      'tag(Negativity,"abides-with-itself")',
      'annotate(Immediacy,{role:"semblance-of-beginning"})',
      'assert(selfRepulsion(Essence) == turningBack(Essence))'
    ],
    predicates: [{ name: 'DefinesPresupposing', args: [] }],
    relations: [
      { predicate: 'relatesAs', from: 'Reflection', to: 'Self~NegativeOfSelf' },
      { predicate: 'negates', from: 'Return', to: 'Beginning(Semblance)' }
    ]
  },
  {
    id: 'ess-ref-op-8-essence-presupposes-itself',
    chunkId: 'ess-ref-8-essence-presupposes-itself',
    label: 'Essence presupposes itself; sublating its presupposition',
    candidateSummary: 'Essence equals itself by sublating its equality; it presupposes itself, and sublating this presupposition is itself presupposition.',
    clauses: [
      'assert(equalsItself(Essence) == sublate(equalsItself(Essence)))',
      'tag(Essence,"self-presupposing")',
      'assert(sublate(presupposition(Essence)) == presupposition(Essence))',
      'annotate(Reflection,{finds:"immediate", action:"transcend-and-turn-back"})'
    ],
    predicates: [{ name: 'EncodesSelfPresupposition', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'Essence', to: 'Essence' },
      { predicate: 'transcends', from: 'Reflection', to: 'Immediate' }
    ]
  },
  {
    id: 'ess-ref-op-9-antecedent-left-behind-arrival',
    chunkId: 'ess-ref-9-antecedent-by-being-left-behind',
    label: 'Antecedent by being-left-behind; repulsion equals arrival-at-itself',
    candidateSummary: 'Antecedent arises by being left behind; sublated immediacy = turning-back; repelling-from-itself equals arriving-at-itself.',
    clauses: [
      'assert(Antecedent == produceBy(leftBehind(Immediate)))',
      'assert(sublated(Immediate) == turningBackIntoItself(Essence))',
      'tag(Essence,"arrives-at-itself")',
      'assert(repelsFromItself(Essence) == arrivesAtItself(Essence))'
    ],
    predicates: [{ name: 'RelatesAntecedentAndArrival', args: [] }],
    relations: [
      { predicate: 'arisesAs', from: 'Antecedent', to: 'LeftBehind' },
      { predicate: 'arrivesAt', from: 'Essence', to: 'Essence' }
    ]
  },
  {
    id: 'ess-ref-op-10-absolute-counter-repelling',
    chunkId: 'ess-ref-10-absolute-internal-counter-repelling',
    label: 'Encode absolute internal counter-repelling; presupposition only in turning-back',
    candidateSummary: 'Reflection self-movement: presupposition exists only in turning-back; transcending immediate = arriving at immediate; positing ↔ presupposing.',
    clauses: [
      'assert(movement(Reflection) == "absolute-internal-counter-repelling")',
      'assert(presupposition(Reflection) == existsOnlyIn(turningBack(Reflection)))',
      'assert(transcend(Immediate) == arriveAt(Immediate))',
      'assert(forward(Reflection) == turnsIntoSelf(Reflection))',
      'annotate(Reflection,{duality:"positing↔presupposing", equivalence:true})'
    ],
    predicates: [{ name: 'EncodesCounterRepelling', args: [] }],
    relations: [
      { predicate: 'presupposesIn', from: 'Reflection', to: 'TurningBack' },
      { predicate: 'equates', from: 'TranscendImmediate', to: 'ArriveImmediate' }
    ]
  },
  {
    id: 'ess-ref-op-11-self-and-nonbeing',
    chunkId: 'ess-ref-11-reflection-self-and-nonbeing',
    label: 'Reflection is itself by being the negative of itself',
    candidateSummary: 'Only by being negative-of-itself does reflection coincide with itself (sublation as coincidence).',
    clauses: [
      'tag(Reflection,"itself-and-its-non-being")',
      'assert(isItselfByBeingNegativeOfItself(Reflection) == true)',
      'assert(coincidesWithItself(sublate(Negative)) == true)'
    ],
    predicates: [{ name: 'AssertsSelfViaNegativity', args: [] }],
    relations: [
      { predicate: 'coincides', from: 'Reflection', to: 'Reflection' },
      { predicate: 'sublates', from: 'Reflection', to: 'Negative' }
    ]
  },
  {
    id: 'ess-ref-op-12-define-external-reflection',
    chunkId: 'ess-ref-12-external-reflection',
    label: 'Define external reflection via positedness and opposition to other',
    candidateSummary: 'Presupposed immediacy = positedness (already sublated, identical with turning-back) but determined as negative vs other → external reflection.',
    clauses: [
      'assert(presupposedImmediacy(Reflection) == Positedness && sublated(Positedness) == true)',
      'assert(identical(Positedness, turningBack(Reflection)) == true)',
      'tag(Immediacy,"opposed-to-other")',
      'if startsFrom(Reflection, ImmediateAsOther) then tag(Reflection,"external")'
    ],
    predicates: [{ name: 'DefinesExternalReflection', args: [] }],
    relations: [
      { predicate: 'identicalTo', from: 'PresupposedImmediacy', to: 'TurningBack' },
      { predicate: 'startsFrom', from: 'Reflection', to: 'ImmediateAsOther' }
    ]
  }
]

/* accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}
export function getLogicalOpsForChunk(oneBasedIndex: number): LogicalOperation[] {
  const chunk = getChunk(oneBasedIndex)
  if (!chunk) return []
  return LOGICAL_OPERATIONS.filter(op => op.chunkId === chunk.id)
}
