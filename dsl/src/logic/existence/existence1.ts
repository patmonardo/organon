import type { Chunk, LogicalOperation } from './index';

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'existence-7-quality-immediate-unity',
    title:
      'Quality I: immediacy of unity (existent = non-being to that extent)',
    text: `b. Quality

On account of the immediacy
with which being and nothing are
one in existence, neither oversteps the other;
to the extent that existence is existent,
to that extent it is non-being;
it is determined.
Being is not the universal,
determinateness not the particular.
Determinateness has yet to detach itself from being;
nor will it ever detach itself from it,
since the now underlying truth is
the unity of non-being with being;
all further determinations will transpire on this basis.
But the connection which determinateness now has
with being is one of the immediate unity of the two,
so that as yet no differentiation between the two is posited.`,
    concise:
      'Immediacy: being≡nothing in existence; “to the extent existent, to that extent non‑being.” Determinateness inseparable from being (unity basis); no posited differentiation yet.',
  },
  {
    id: 'existence-8-quality-as-existent-determinateness',
    title:
      'Quality II: definition (existent determinateness; simple, immediate)',
    text: `Determinateness thus isolated by itself,
as existent determinateness, is quality:
something totally simple, immediate.
Determinateness in general is the more universal
which, further determined, can be
something quantitative as well.
On account of this simplicity,
there is nothing further to say
about quality as such.`,
    concise:
      'Definition: “quality” = existent determinateness—simple, immediate. Determinateness (more universal) can further determine as quantity; qua quality, nothing more to add.',
  },
  {
    id: 'existence-9-quality-reality-negation-limit',
    title: 'Quality III: reality and negation; reflection; limit/restriction',
    text: `Existence, however, in which
nothing and being are equally contained,
is itself the measure of the one-sidedness of
quality as an only immediate or existent determinateness.
Quality is equally to be posited in the determination of nothing,
and the result is that the immediate or existent determinateness is
posited as distinct, reflected, and the nothing,
as thus the determinate element of determinateness,
will equally be something reflected, a negation.
Quality, in the distinct value of existent, is reality;
when affected by a negating, it is negation in general,
still a quality but one that counts as a lack
and is further determined as limit, restriction.`,
    concise:
      'Measure: existence measures the one‑sidedness of “quality as mere immediacy.” Posit quality also as nothing→ reflection: existent‑determinateness becomes distinct/for‑it, and “nothing” appears as negation. Thus: reality = quality accented as existent; negation = quality under negating (lack), further as limit/restriction.',
  },
  {
    id: 'existence-10-reality-vs-negation-valuation',
    title: 'Quality IV: valuation of reality vs negation (both are existence)',
    text: `Both are an existence, but in reality,
as quality with the accent on being an existent,
that it is determinateness
and hence also negation is concealed;
reality only has, therefore,
the value of something positive
from which negating, restriction, lack, are excluded.
Negation, for its part, taken as mere lack,
would be what nothing is;
but it is an existence, a quality,
only determined with a non-being.`,
    concise:
      'Both are existences. “Reality” (accent on being) conceals its own negation, thus appears as pure positive. “Negation,” if taken as mere lack, collapses to nothing; yet as quality it is an existence—determined with non‑being.',
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'existence-op-7-immediate-unity-no-differentiation',
    chunkId: 'existence-7-quality-immediate-unity',
    label: 'Immediate unity: existent ↔ non-being; no posited differentiation',
    clauses: [
      'tag(Existence,"immediate-unity-of-being-and-nothing")',
      'assert(toTheExtent(Existence,"existent","non-being"))',
      'annotate(Determinateness,{inseparableFrom:"Being"})',
      'assert(basisIsUnity(Being,NonBeing))',
      'tag(Existence,"no-differentiation-posited")',
    ],
    predicates: [{ name: 'ImmediateUnity', args: [] }],
    relations: [
      { predicate: 'inseparableFrom', from: 'Determinateness', to: 'Being' },
    ],
  },
  {
    id: 'existence-op-8-quality-definition',
    chunkId: 'existence-8-quality-as-existent-determinateness',
    label: 'Quality = existent determinateness (simple, immediate)',
    clauses: [
      'tag(Quality,"existent-determinateness")',
      'tag(Quality,"simple")',
      'tag(Quality,"immediate")',
      'assert(canFurtherDetermine(Determinateness,"quantity"))',
    ],
    predicates: [{ name: 'QualityDefinition', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-9-reality-and-negation',
    chunkId: 'existence-9-quality-reality-negation-limit',
    label: 'Reality vs negation; reflection; limit/restriction',
    clauses: [
      'assert(measuresOneSidedness(Existence,Quality))',
      'assert(positIn(Quality,"nothing"))',
      'assert(reflected(ExistentDeterminateness))',
      'tag(Negation,"reflected-nothing")',
      'assert(realityIs(Quality,"accent-on-existent"))',
      'assert(negationIs(Quality,"under-negating"))',
      'tag(Negation,"lack")',
      'tag(Negation,"limit")',
      'tag(Negation,"restriction")',
    ],
    predicates: [{ name: 'RealityNegationLimit', args: [] }],
    relations: [
      { predicate: 'determinesAs', from: 'Quality', to: 'Reality' },
      { predicate: 'determinesAs', from: 'Quality', to: 'Negation' },
    ],
  },
  {
    id: 'existence-op-10-valuation-reality-vs-negation',
    chunkId: 'existence-10-reality-vs-negation-valuation',
    label:
      'Both are existence; reality as positive; negation as existent quality',
    clauses: [
      'assert(bothAre(Reality,Negation,"existence"))',
      'tag(Reality,"positive-appearance")',
      'assert(conceals(Reality,["determinateness","negation"]))',
      'assert(wouldBeNothingIfMereLack(Negation))',
      'assert(isExistenceAsQuality(Negation))',
      'annotate(Negation,{determinedWith:"non-being"})',
    ],
    predicates: [{ name: 'RealityNegationValuation', args: [] }],
    relations: [],
  },
];
