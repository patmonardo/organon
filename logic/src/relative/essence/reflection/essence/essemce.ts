import type { Chunk, LogicalOperation } from './index'

/*
  Essence — A. THE ESSENTIAL AND THE UNESSENTIAL
  - Two-fold representation:
    - text: full source chunk (verbatim segments, preserving the whole passage across chunks)
    - summary: short IR summary for HLO extraction (non-destructive)
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ess-1-sublated-being',
    title: 'Essence is sublated being',
    text: `Essence is sublated being.`,
    summary: 'Define essence as being that has been sublated (negated-and-preserved).'
  },
  {
    id: 'ess-2-determined-negation-and-immediacy',
    title: 'Determined negation; immediacy preserved; equal value as immediacies',
    text: `It is simple equality with itself
but is such as the negation of
the sphere of being in general.
And so it has immediacy over against it,
as something from which it has come to be
but which has preserved and maintained itself in this sublating.
Essence itself is in this determination
an existent immediate essence,
and with reference to it
being is only something negative,
nothing in and for itself:
essence, therefore, is a determined negation.
Being and essence relate to each other in this fashion
as against others in general which are mutually indifferent,
for each has a being, an immediacy,
and according to this being they stand in equal value.`,
    summary: 'Essence = equality-with-self as negation of being; both retain immediacy, yet being is only negative; as immediacies they stand in equal value (indifference).'
  },
  {
    id: 'ess-3-being-unessential-essential-problem',
    title: 'Being as unessential; “essential” as mere other if opposed',
    text: `But as contrasted with essence,
being is at the same time the unessential;
as against essence, it has the determination of something sublated.
And in so far as it thus relates to essence
as an other only in general,
essence itself is not essence proper
but is just another existence, the essential.`,
    summary: 'Opposed to essence, being is unessential (sublated). If essence is only an “other” to being, it lapses into mere “essential” existence, not essence proper.'
  },
  {
    id: 'ess-4-external-positing-and-standpoint',
    title: 'Relapse into existence; external positing of essential vs unessential',
    text: `The distinction of essential and unessential has
made essence relapse into the sphere of existence,
for as essence is at first,
it is determined with respect to being
as an existent and therefore as an other.
The sphere of existence is thus laid out as foundation,
and that in this sphere being is being-in-and-for-itself,
is a further determination external to existence,
just as, contrariwise, essence is indeed being-in-and-for-itself,
but only over against an other, in a determinate respect.
Consequently, inasmuch as essential and unessential aspects are
distinguished in an existence from each other,
this distinguishing is an external positing,
a taking apart that leaves the existence itself untouched;
it is a separation which falls on the side of
a third and leaves undetermined
what belongs to the essential
and what belongs to the unessential.
It is dependent on some external standpoint or consideration
and the same content can therefore sometimes be considered
as essential, sometimes as unessential.`,
    summary: 'Treating essential/unessential as a mere separation in an existent is an external positing (by a third standpoint), leaving the content underdetermined and contingent.'
  },
  {
    id: 'ess-5-absolute-negativity-and-schein',
    title: 'From first negation to absolute negativity; the immediate as Schein (shine)',
    text: `On closer consideration, essence becomes something
only essential as contrasted with an unessential
because essence is only taken,
is as sublated being or existence.
In this fashion, essence is only the first negation,
or the negation, which is determinateness,
through which being becomes only existence,
or existence only an other.
But essence is the absolute negativity of being;
it is being itself, but not being determined only as an other:
it is being rather that has sublated itself
both as immediate being
and as immediate negation,
as the negation which is affected by an otherness.
Being or existence, therefore, does not persist
except as what essence is,
and the immediate which still differs from essence is not just
an unessential existence but an immediate
which is null in and for itself;
it only is a non-essence, shine.`,
    summary: 'Essence is not mere first negation but absolute negativity: being sublating itself as being and as negation. What remains as mere immediacy is null—non-essence (Schein).'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ess-op-1-define-essence-as-sublated-being',
    chunkId: 'ess-1-sublated-being',
    label: 'Define essence as sublated being',
    digest: 'Tag Essence as the result of sublation (being negated and preserved).',
    clauses: [
      'if x.kind == "Essence" then assert(x = sublate(Being))',
      'tag(x,"sublated-being")'
    ],
    predicates: [{ name: 'DefinesSublatedBeing', args: [] }],
    relations: [{ predicate: 'derivesFrom', from: 'Essence', to: 'Being' }]
  },
  {
    id: 'ess-op-2-characterize-determined-negation',
    chunkId: 'ess-2-determined-negation-and-immediacy',
    label: 'Characterize essence as determined negation with preserved immediacy; mark indifference',
    digest: 'Essence equals self-equality as negation of being; both retain immediacy; as immediacies, being and essence are indifferent (equal value).',
    clauses: [
      'tag(Essence,"determined-negation")',
      'assert(Essence.relationTo(Being) == "negates-and-preserves")',
      'tag(Essence,"immediate-as-essence"); tag(Being,"immediate-as-being")',
      'annotate(pair:{Essence,Being}, "equal-value-as-immediacies")',
      'record(meta:{indifference:true})'
    ],
    predicates: [{ name: 'CharacterizesDeterminedNegation', args: [] }],
    relations: [
      { predicate: 'negates', from: 'Essence', to: 'Being' },
      { predicate: 'preserves', from: 'Essence', to: 'Being' }
    ]
  },
  {
    id: 'ess-op-3-classify-unessential-vs-essential',
    chunkId: 'ess-3-being-unessential-essential-problem',
    label: 'Classify being as unessential; caution: “essential” vs essence proper',
    digest: 'Opposed to essence, being is unessential (sublated); if essence is only an other to being, it is merely “essential” (another existent), not essence proper.',
    clauses: [
      'if opposes(Being,Essence) then tag(Being,"unessential")',
      'if Essence.role == "mere-other-to-being" then tag(Essence,"essential-not-proper")'
    ],
    predicates: [{ name: 'ClassifiesUnessential', args: [] }],
    relations: [{ predicate: 'opposes', from: 'Being', to: 'Essence' }]
  },
  {
    id: 'ess-op-4-detect-external-positing',
    chunkId: 'ess-4-external-positing-and-standpoint',
    label: 'Detect external positing of essential/unessential within an existent',
    digest: 'Separation of essential vs unessential by a third standpoint is external positing; leaves content underdetermined.',
    clauses: [
      'if partition(existent, ["essential","unessential"]).by == "external-standpoint" then tag(existent,"externally-posited")',
      'annotate(existent,{underdetermined:true, standpoint:"third"})'
    ],
    predicates: [{ name: 'DetectsExternalPositing', args: [] }],
    relations: [{ predicate: 'partitionedBy', from: 'existent', to: 'third-standpoint' }]
  },
  {
    id: 'ess-op-5-assert-absolute-negativity-and-schein',
    chunkId: 'ess-5-absolute-negativity-and-schein',
    label: 'Assert essence as absolute negativity; classify mere immediacy as Schein',
    digest: 'Essence is absolute negativity (self-sublating being and negation). Pure immediacy remaining outside essence is non-essence (Schein).',
    clauses: [
      'tag(Essence,"absolute-negativity")',
      'assert(Essence == selfSublation(Being, Negation))',
      'if immediate(x) && differsFrom(x,Essence) then tag(x,"schein")'
    ],
    predicates: [{ name: 'AssertsAbsoluteNegativity', args: [] }],
    relations: [
      { predicate: 'selfSublates', from: 'Being', to: 'Essence' },
      { predicate: 'classifies', from: 'Essence', to: 'Schein' }
    ]
  }
]
