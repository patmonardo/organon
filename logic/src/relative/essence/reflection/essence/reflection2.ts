import type { Chunk, LogicalOperation } from './index'

/*
  Essence — C. REFLECTION — Part 3: Determining reflection (section 1 of 3)
  Two-fold representation:
  - text: verbatim source segmented into readable chunks (preserve lines)
  - summary: short IR summary to support HLO extraction (non-destructive)
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ess-ref2-0-determining-unity',
    title: 'Determining reflection — unity of positing and external reflection',
    text: `3. Determining reflection

Determining reflection is in general
the unity of positing and external reflection.
This is now to be examined more closely.`,
    summary: 'Determining reflection = unity of positing reflection and external reflection.'
  },
  {
    id: 'ess-ref2-1-origins-and-incompletion',
    title: 'Origins of the two reflections; incompletion and the merely posited',
    text: `1. External reflection begins from immediate being,
positing reflection from nothing.
In its determining, external reflection posits another in the
place of the sublated being, but this other is essence;
the positing does not posit its determination in the place of an other;
it has no presupposition.
But, precisely for this reason,
it is not complete as determining reflection;
the determination which it posits is consequently only a posited;
this is an immediate, not however as equal to itself
but as self-negating;
its connection with the turning back into itself is absolute;
it is only in the reflection-into-itself
but is not this reflection itself.

The posited is therefore an other,
but in such a manner that the self-equality
of reflection is retained;
for the posited is only as sublated,
as reference to the turning back into itself.`,
    summary: 'External begins from being; positing from nothing. External, in determining, posits “other” that is essence; positing has no presupposition, hence its determination is only a posited—immediate yet self-negating, only in reflection-into-itself.'
  },
  {
    id: 'ess-ref2-2-positedness-corresponds-existence',
    title: 'Positedness corresponds to existence; existence is positedness',
    text: `In the sphere of being, existence was the being
that had negation in it, and being was the immediate ground
and element of this negation which was,
therefore, itself immediate negation.
In the sphere of essence,
positedness is what corresponds to existence.
Positedness is equally an existence,
but its ground is being as essence
or as pure negativity;
it is a determinateness or a negation,
not as existent but immediately as sublated.
Existence is only positedness;
this is the principle of the essence of existence.`,
    summary: 'In essence, positedness corresponds to existence. Existence = positedness; its ground is essence (pure negativity); determinateness is immediately as sublated.'
  },
  {
    id: 'ess-ref2-3-positedness-mediator-and-superiority',
    title: 'Positedness mediates essence and existence; its “superiority” and scope',
    text: `Positedness stands on the one side over against existence,
and over against essence on the other:
it is to be regarded as the means which conjoins
existence with essence and essence with existence.
If it is said, a determination is only a positedness,
the claim can thus have a twofold meaning,
according to whether the determination is such
in opposition to existence or in opposition to essence.
In either meaning, existence is taken for
something superior to positedness,
which is attributed to external reflection, to the subjective.
In fact, however, positedness is the superior, because, as posited,
existence is what it is in itself something negative,
something that refers simply and solely to the turning back into itself.
For this reason positedness is only a positedness
with respect to essence:
it is the negation of this turning back
as achieved return into itself.`,
    summary: 'Positedness mediates between existence and essence. Though taken as “subjective,” it is superior: it shows existence as negative referring to return-into-itself. Yet it is “only positedness” with respect to essence: negation of achieved return.'
  },
  {
    id: 'ess-ref2-4-positedness-becomes-determination',
    title: 'Positedness united with external reflection → determination of reflection',
    text: `2. Positedness is not yet a determination of reflection;
it is only determinateness as negation in general.
But the positing is now united with external reflection;
in this unity, the latter is absolute presupposing, that is,
the repelling of reflection from itself
or the positing of determinateness as its own.
As posited, therefore, positedness is negation;
but as presupposed, it is reflected into itself.
And in this way positedness is a determination of reflection.`,
    summary: 'Uniting positing with external reflection (absolute presupposing) makes positedness a determination: as posited → negation; as presupposed → reflected-into-itself.'
  },
  {
    id: 'ess-ref2-5-determination-vs-quality',
    title: 'Determination of reflection vs determinateness of being (quality)',
    text: `The determination of reflection is distinct
from the determinateness of being, of quality;
the latter is immediate reference to other in general;
positedness also is reference to other,
but to immanently reflected being.
Negation as quality is existent negation;
being constitutes its ground and element.
The determination of reflection, on the contrary,
has for this ground immanent reflectedness.
Positedness gets fixed in determination precisely
because reflection is self-equality in its negatedness;
the latter is therefore itself reflection into itself.
Determination persists here, not by virtue of being
but because of its self-equality.
Since the being which sustains quality is
unequal to the negation, quality is
consequently unequal within itself,
and hence a transient moment which disappears in the other.
The determination of reflection is
on the contrary positedness as negation,
negation which has negatedness for its ground,
is therefore not unequal to itself within itself,
and hence essential rather than transient determinateness.
What gives subsistence to it is the self-equality of reflection
which has the negative only as negative,
as something sublated or posited.`,
    summary: 'Quality: existent negation grounded in being → transient and unequal. Determination of reflection: positedness-as-negation grounded in immanent reflectedness → persists by self-equality, essential, not unequal within itself.'
  },
  {
    id: 'ess-ref2-6-essential-shine-negation-predominates',
    title: 'Free essentialities; essential shine; negation predominates',
    text: `Because of this reflection into themselves,
the determinations of reflection appear as
free essentialities, sublated in the void
without reciprocal attraction or repulsion.
In them the determinateness has become entranced
and infinitely fixed by virtue of the reference to itself.
It is the determinate which has subjugated its transitoriness
and its mere positedness to itself, that is to say,
has deflected its reflection-into-other into reflection-into- itself.
These determinations hereby constitute the determinate shine
as it is in essence, the essential shine.
Determining reflection is for this reason
reflection that has exited from itself;
the equality of essence with itself is
lost in the negation, and negation predominates.`,
    summary: 'Determinations appear as free essentialities (in the void); reflection-into-other is deflected into reflection-into-itself: essential shine. Determining reflection exits from itself; negation predominates.'
  },
  {
    id: 'ess-ref2-7-two-sides-of-determination',
    title: 'Two sides: positedness (negation-as-negation) and immanent reflection',
    text: `Thus there are two distinct sides to the determination of reflection.
First, reflection is positedness, negation as such;
second, it is immanent reflection.
According to the side of positedness,
it is negation as negation,
and this already is its unity with itself.
But it is this unity at first only implicitly or in itself,
an immediate which sublates itself within, is the other of itself.
To this extent, reflection is a determining that abides in itself.
In it essence does not exit from itself;
the distinctions are solely posited,
taken back into essence.
But, from the other side, they are not posited
but are rather reflected into themselves;
negation as negation is equality with itself,
not in its other, not reflected into its non-being.`,
    summary: 'Side 1: positedness (negation-as-negation), unity implicit; determining abides in itself, distinctions taken back into essence. Side 2: immanent reflection—determinations reflected into themselves; equality not in an other.'
  },
  {
    id: 'ess-ref2-8-dual-nature-of-determination',
    title: 'Dual nature: positedness (negation vs essence) and immanent self-reference',
    text: `3. Now keeping in mind that the determination of reflection is
both immanently reflected reference and positedness as well,
its nature immediately becomes more transparent.
For, as positedness, the determination is negation as such,
a non-being as against another, namely,
as against the absolute immanent reflection or as against essence.
But as self-reference, it is reflected within itself.
This, the reflection of the determination,
and that positedness are distinct;
its positedness is rather the sublatedness of the determination
whereas its immanent reflectedness is its subsisting.`,
    summary: 'Determination of reflection = positedness (negation vs essence) + immanent self-reference. Positedness = sublatedness; immanent reflectedness = subsistence.'
  },
  {
    id: 'ess-ref2-9-reference-to-its-otherness',
    title: 'Reference-to-otherness inside the determination (non-quiescent reference)',
    text: `In so far as now the positedness is
at the same time immanent reflection,
the determinateness of the reflection is
the reference in it to its otherness.
It is not a determinateness that exists quiescent,
one which would be referred to an other
in such a way that the referred term
and its reference would be different,
each something existing in itself,
each a something that excludes its other
and its reference to this other from itself.
Rather, the determination of reflection is
within it the determinate side
and the reference of this determinate side as determinate,
that is, the reference to its negation.`,
    summary: 'Reference to otherness is internal: the determination includes both determinate side and its reference (to its negation) within itself.'
  },
  {
    id: 'ess-ref2-10-otherness-taken-back-essentiality',
    title: 'Otherness taken back; deflection into self; unity with its other (essentiality)',
    text: `Quality, through its reference, passes over into another;
its alteration begins in its reference.
The determination of reflection, on the contrary,
has taken its otherness back into itself.
It is positedness, negation which has however deflected
the reference to another into itself,
and negation which, equal to itself,
is the unity of itself and its other,
and only through this is an essentiality.`,
    summary: 'Unlike quality, determination of reflection internalizes otherness: deflects reference-to-another into itself; equals itself as unity of itself and its other → essentiality.'
  },
  {
    id: 'ess-ref2-11-infinite-reference-to-self',
    title: 'Positedness sublated in self-reflection → infinite reference to itself',
    text: `It is, therefore, positedness, negation,
but as reflection into itself it is at the same time
the sublatedness of this positedness,
infinite reference to itself.`,
    summary: 'As self-reflection, positedness is sublated; determination = infinite self-reference.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ess-ref2-op-0-unify-determining',
    chunkId: 'ess-ref2-0-determining-unity',
    label: 'Unify positing and external reflection as determining reflection',
    digest: 'Determining reflection is the unity/coordination of positing and external reflection.',
    clauses: [
      'assert(DeterminingReflection == unify(PositingReflection, ExternalReflection))'
    ],
    predicates: [{ name: 'UnifiesDeterminingReflection', args: [] }],
    relations: [{ predicate: 'unifies', from: 'DeterminingReflection', to: 'Positing|External' }]
  },
  {
    id: 'ess-ref2-op-1-origins-and-posited',
    chunkId: 'ess-ref2-1-origins-and-incompletion',
    label: 'Origins (being vs nothing); incomplete determining → merely posited (self-negating)',
    digest: 'External begins from being; positing from nothing; determination as merely posited—immediate yet self-negating, only in reflection-into-itself.',
    clauses: [
      'annotate(ExternalReflection,{origin:"immediate-being"})',
      'annotate(PositingReflection,{origin:"nothing"})',
      'assert(externalDetermining.posits(other:=Essence))',
      'assert(PositingReflection.hasPresupposition == false)',
      'tag(Determination,"posited")',
      'tag(Determination,"immediate-self-negating")',
      'assert(onlyIn(Determination, ReflectionIntoItself) && notEquals(Determination, ReflectionIntoItself))'
    ],
    predicates: [{ name: 'ClassifiesOriginsAndPosited', args: [] }],
    relations: [
      { predicate: 'posits', from: 'ExternalReflection', to: 'Essence' },
      { predicate: 'in', from: 'Determination', to: 'ReflectionIntoItself' }
    ]
  },
  {
    id: 'ess-ref2-op-2-correspondence-positedness-existence',
    chunkId: 'ess-ref2-2-positedness-corresponds-existence',
    label: 'Correspondence: positedness ↔ existence; existence is positedness',
    digest: 'In essence, positedness corresponds to existence; ground = pure negativity; determinateness is immediately sublated.',
    clauses: [
      'assert(corresponds(Positedness, Existence))',
      'assert(ground(Positedness) == Essence.asPureNegativity)',
      'tag(Determinateness,"immediately-sublated")',
      'assert(Existence == Positedness)',
      'annotate(Existence,{principle:"essence-of-existence-is-positedness"})'
    ],
    predicates: [{ name: 'RelatesPositednessAndExistence', args: [] }],
    relations: [
      { predicate: 'correspondsTo', from: 'Positedness', to: 'Existence' },
      { predicate: 'groundedIn', from: 'Positedness', to: 'Essence(PureNegativity)' }
    ]
  },
  {
    id: 'ess-ref2-op-3-mediator-and-superiority',
    chunkId: 'ess-ref2-3-positedness-mediator-and-superiority',
    label: 'Positedness mediates essence/existence; twofold sense; “superiority” scoped to essence',
    digest: 'Positedness is the means conjoining existence and essence; “only positedness” has two senses; positedness is superior since existence refers to return-into-itself; yet only with respect to essence.',
    clauses: [
      'assert(means(Positedness, between: [Existence, Essence]))',
      'annotate(Positedness,{twofoldSense:["opposed-to-existence","opposed-to-essence"]})',
      'annotate(Existence,{oftenTakenAs:"superior-to-positedness"})',
      'assert(superior(Positedness, Existence) == true because existenceRefersToReturnIntoItself)',
      'annotate(Positedness,{scope:"with-respect-to-essence", sense:"negation-of-achieved-return"})'
    ],
    predicates: [{ name: 'DefinesMediatorAndScope', args: [] }],
    relations: [
      { predicate: 'conjoins', from: 'Positedness', to: 'Essence|Existence' },
      { predicate: 'refersTo', from: 'Existence', to: 'TurningBackIntoItself' }
    ]
  },
  {
    id: 'ess-ref2-op-4-positedness-to-determination',
    chunkId: 'ess-ref2-4-positedness-becomes-determination',
    label: 'Uniting positing with external reflection makes positedness a determination',
    digest: 'Absolute presupposing (repelling from itself) yields: as posited → negation; as presupposed → reflected-into-itself.',
    clauses: [
      'assert(unite(PositingReflection, ExternalReflection) == AbsolutePresupposing)',
      'tag(Positedness,"as-posited:negation")',
      'tag(Positedness,"as-presupposed:reflected-into-itself")',
      'tag(Positedness,"determination-of-reflection")'
    ],
    predicates: [{ name: 'PromotesPositedness', args: [] }],
    relations: [
      { predicate: 'unites', from: 'PositingReflection', to: 'ExternalReflection' },
      { predicate: 'yields', from: 'AbsolutePresupposing', to: 'DeterminationOfReflection' }
    ]
  },
  {
    id: 'ess-ref2-op-5-contrast-with-quality',
    chunkId: 'ess-ref2-5-determination-vs-quality',
    label: 'Contrast determination of reflection with quality; essential persistence',
    digest: 'Quality = existent negation grounded in being (transient, unequal). Determination of reflection = positedness-as-negation grounded in immanent reflectedness (essential, self-equal).',
    clauses: [
      'annotate(Quality,{reference:"immediate-to-other", ground:"Being", status:"transient", unequalWithin:true})',
      'annotate(DeterminationOfReflection,{reference:"to-immanently-reflected-being", ground:"ImmanentReflectedness", persistsBy:"self-equality", unequalWithin:false, essential:true})',
      'tag(Negative,"only-as-negative (sublated/posited)")'
    ],
    predicates: [{ name: 'ContrastsQualityVsDetermination', args: [] }],
    relations: [
      { predicate: 'groundedIn', from: 'Quality', to: 'Being' },
      { predicate: 'groundedIn', from: 'DeterminationOfReflection', to: 'ImmanentReflectedness' }
    ]
  },
  {
    id: 'ess-ref2-op-6-essential-shine',
    chunkId: 'ess-ref2-6-essential-shine-negation-predominates',
    label: 'Free essentialities; deflect into reflection-into-itself; essential shine; negation predominates',
    digest: 'Determinations-as-free-essentialities (void, no attraction/repulsion); reflection-into-other deflected into reflection-into-itself; essential shine; determining reflection exits from itself, negation predominates.',
    clauses: [
      'tag(DeterminationsOfReflection,"free-essentialities")',
      'annotate(Context,{void:true, reciprocalAttraction:false, reciprocalRepulsion:false})',
      'assert(deflect(ReflectionIntoOther) == ReflectionIntoItself)',
      'tag(Shine,"essential")',
      'assert(exitsFromItself(DeterminingReflection) == true)',
      'annotate(Essence,{equalityWithItself:"lost-in-negation", negation:"predominates"})'
    ],
    predicates: [{ name: 'EstablishesEssentialShine', args: [] }],
    relations: [
      { predicate: 'deflectsTo', from: 'ReflectionIntoOther', to: 'ReflectionIntoItself' },
      { predicate: 'constitutes', from: 'Determinations', to: 'EssentialShine' }
    ]
  },
  {
    id: 'ess-ref2-op-7-two-sides',
    chunkId: 'ess-ref2-7-two-sides-of-determination',
    label: 'Two sides: positedness (implicit unity) and immanent reflection',
    digest: 'Side 1: positedness as negation-as-negation (implicit unity), immediate that self-sublates; determining abides in itself; distinctions taken back into essence. Side 2: reflected-into-themselves; equality not in an other.',
    clauses: [
      'tag(Side1,"positedness/negation-as-negation")',
      'annotate(Side1,{unity:"implicit(in-itself)", immediateSelfSublates:true, determiningAbides:true})',
      'assert(exitsFromItself(Essence) == false)',
      'annotate(Distinctions,{status:"posited-and-taken-back-into-essence"})',
      'tag(Side2,"immanent-reflection")',
      'assert(reflectedIntoItself(Determinations) == true)',
      'assert(equalityWithItself(NegationAsNegation) && not reflectedInto(NegationAsNegation,"non-being"))'
    ],
    predicates: [{ name: 'EncodesTwoSidedStructure', args: [] }],
    relations: [
      { predicate: 'takesBackInto', from: 'Essence', to: 'Distinctions' },
      { predicate: 'reflectsInto', from: 'Determinations', to: 'Themselves' }
    ]
  },
  {
    id: 'ess-ref2-op-8-dual-nature',
    chunkId: 'ess-ref2-8-dual-nature-of-determination',
    label: 'Dual nature: positedness vs essence, and immanent self-reference',
    digest: 'Determination = positedness (negation vs essence) + immanent reflectedness (subsistence).',
    clauses: [
      'tag(DeterminationOfReflection,"positedness:negation-vs-essence")',
      'tag(DeterminationOfReflection,"immanent-reflection:subsistence")',
      'assert(positedness(DeterminationOfReflection) == sublated(DeterminationOfReflection))'
    ],
    predicates: [{ name: 'EncodesDualNature', args: [] }],
    relations: [
      { predicate: 'opposes', from: 'Positedness', to: 'Essence' },
      { predicate: 'subsistsAs', from: 'Determination', to: 'ImmanentReflectedness' }
    ]
  },
  {
    id: 'ess-ref2-op-9-internal-reference-to-negation',
    chunkId: 'ess-ref2-9-reference-to-its-otherness',
    label: 'Reference to otherness internal to the determination (reference to its negation)',
    digest: 'Determination contains both determinate side and its reference, i.e., reference to its negation, not two externals.',
    clauses: [
      'assert(internalizeReference(Determination, to: negation(Determination)))',
      'annotate(Determination,{quiescent:false, excludesExternalDualism:true})'
    ],
    predicates: [{ name: 'InternalizesOtherness', args: [] }],
    relations: [
      { predicate: 'refersTo', from: 'Determination', to: 'ItsNegation' }
    ]
  },
  {
    id: 'ess-ref2-op-10-essential-unity',
    chunkId: 'ess-ref2-10-otherness-taken-back-essentiality',
    label: 'Deflect reference-to-another into itself; unity with its other → essentiality',
    digest: 'Otherness taken back; negation equal-to-itself unifies itself and its other; thereby becomes essential.',
    clauses: [
      'assert(deflectsToSelf(Determination, reference:"to-another"))',
      'assert(equalsToItself(NegationOf(Determination)))',
      'assert(unity(Determination, otherOf(Determination)) && tag(Determination,"essential"))'
    ],
    predicates: [{ name: 'EstablishesEssentiality', args: [] }],
    relations: [
      { predicate: 'unifiesWith', from: 'Determination', to: 'ItsOther' }
    ]
  },
  {
    id: 'ess-ref2-op-11-infinite-self-reference',
    chunkId: 'ess-ref2-11-infinite-reference-to-self',
    label: 'Positedness sublated in self-reflection → infinite self-reference',
    digest: 'As reflection-into-itself, positedness is sublated, yielding infinite reference to itself.',
    clauses: [
      'assert(sublated(Positedness) && via(ImmanentReflection))',
      'tag(DeterminationOfReflection,"infinite-self-reference")'
    ],
    predicates: [{ name: 'EncodesInfiniteSelfReference', args: [] }],
    relations: [
      { predicate: 'returnsInto', from: 'Determination', to: 'Itself(Infinite)' }
    ]
  },
  {
    id: 'ess-ref2-op-12-sublation-of-becoming',
    chunkId: 'ess-ref2-11-infinite-reference-to-self',
    label: 'Sublation of Becoming: sublated positedness + internal otherness → infinite self-reference',
    digest: 'Truth of Becoming sublated: positedness as sublated; reference-to-otherness internalized; essentiality as infinite self-reference.',
    clauses: [
      'tag(Theme,"SublationOfBecoming")',
      'assert(sublated(Positedness) == true)',
      'assert(internalReference(Determination, to:"its-otherness"))',
      'assert(infiniteSelfReference(DeterminationOfReflection) == true)'
    ],
    predicates: [{ name: 'SublationOfBecoming', args: [] }],
    relations: [
      { predicate: 'internalizes', from: 'Determination', to: 'Otherness' },
      { predicate: 'yields', from: 'SublatedPositedness', to: 'InfiniteSelfReference' }
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
