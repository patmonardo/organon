import type { Chunk, LogicalOperation } from './index'

/*
  Essence — C. REFLECTION — Part 2: External reflection
  Two-fold representation:
  - text: verbatim source segmented into readable chunks (preserve full passage/lines)
  - summary: short IR summary to support HLO extraction (non-destructive)
*/

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'ess-ref1-1-definition-and-doubling',
    title: 'External reflection — definition and doubling',
    text: `2. External reflection

Reflection, as absolute reflection,
is essence shining within,
essence that posits only shine,
only positedness, for its presupposition;
and as presupposing reflection,
it is immediately only positing reflection.
But external or real reflection
presupposes itself as sublated,
as the negative of itself.
In this determination, it is doubled.
At one time it is as what is presupposed,
or the reflection into itself which is the immediate.
At another time, it is as the reflection
negatively referring to itself;
it refers itself to itself as
to that its non-being.`,
    summary: 'External reflection presupposes itself as sublated (negative of itself) and is doubled: (a) immediate-as-presupposed, (b) negative self-reference (to its non-being).'
  },
  {
    id: 'ess-ref1-2-presupposes-a-being',
    title: 'Presupposing a being; immediacy/self-reference; moment-only',
    text: `External reflection thus presupposes a being,
at first not in the sense that
its immediacy is only positedness or moment,
but in the sense rather that
this immediacy refers to itself
and the determinateness is only as moment.`,
    summary: 'It presupposes a being where immediacy self-refers; determinateness only as moment.'
  },
  {
    id: 'ess-ref1-3-posits-and-finds-presupposition',
    title: 'Posits then sublates its positing; finds an immediate presupposition',
    text: `Reflection refers to its presupposition in such a way
that the latter is its negative,
but this negative is thereby sublated as negative.
Reflection, in positing, immediately sublates its positing,
and so it has an immediate presupposition.
It therefore finds this presupposition before it
as something from which it starts,
and from which it only makes its way back into itself,
negating it as its negative.
But that this presupposition is a negative
or a positedness is not its concern;
this determinateness belongs only to positing reflection,
whereas in the presupposing positedness
it is only as sublated.`,
    summary: 'Reflection posits and instantly sublates (thus “finds” an immediate presupposition) and starts from it back to itself; whether it is negative/positedness is not its concern here (only as sublated).'
  },
  {
    id: 'ess-ref1-4-external-determinations-and-infinite',
    title: 'External determinations; finite/infinite pattern from being',
    text: `What external reflection determines and posits in the immediate
are determinations which to that extent are external to it.
In the sphere of being, external reflection was the infinite;
the finite stands as the first,
as the real from which the beginning is made
as from a foundation that abides,
whereas the infinite is the reflection into itself
standing over against it.`,
    summary: 'External reflection’s posited determinations are external to it. In being: begin from finite as abiding foundation; infinite = reflection-into-itself over against it.'
  },
  {
    id: 'ess-ref1-5-syllogism-form',
    title: 'Syllogism of external reflection',
    text: `This external reflection is the syllogism
in which the two extremes are
the immediate and the reflection into itself;
the middle term is the reference connecting the two,
the determinate immediate, so that one part of this connecting reference,
the immediate, falls to one extreme alone, and the other,
the determinateness or the negation, only to the other extreme.`,
    summary: 'Extremes: immediate vs reflection-into-itself; middle: determinate immediate linking them (immediacy to one extreme, determinateness/negation to the other).'
  },
  {
    id: 'ess-ref1-6-positing-and-negating-its-negating',
    title: 'Positing the immediate → negative; negating its negating; beginning-only-in-beginning',
    text: `But if one takes a closer look at what the external reflection does,
it turns out that it is, secondly, the positing of the immediate,
an immediate which thus becomes the negative or the determined;
but it is immediately also the sublating of this positing,
for it presupposes the immediate;
in negating, it is the negating of its negating.
But thereby it immediately is equally a positing,
the sublating of the immediate which is its negative;
and this negative, from which it seemed to begin
as from something alien,
only is in this its beginning.`,
    summary: 'It posits the immediate (as negative/determined) and sublates that positing; negating-of-its-negating. The “alien” negative exists only in and as this beginning.'
  },
  {
    id: 'ess-ref1-7-externality-sublated-determining',
    title: 'Externality sublated; coincidence with immediate; determining reflection',
    text: `In this way, the immediate is not only implicitly in itself
(that is, for us or in external reflection)
the same as what reflection is,
but is posited as being the same.
For the immediate is determined by reflection as
the negative of the latter or as the other of it,
but it is reflection itself which negates this determining.
The externality of reflection vis-à-vis
the immediate is consequently sublated;
its self-negating positing is its coinciding
with its negative, with the immediate,
and this coinciding is the immediacy of essence itself.
It thus transpires that external reflection is not external
but is just as much the immanent reflection of immediacy itself;
or that the result of positing reflection is
essence existing in and for itself.
External reflection is thus determining reflection.`,
    summary: 'Immediate is posited as same as reflection; reflection negates its own determining. Externality is sublated; coincidence with immediate = immediacy of essence. Hence external reflection = immanent/determining reflection; result: essence in-and-for-itself.'
  }
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'ess-ref1-op-1-define-external-reflection',
    chunkId: 'ess-ref1-1-definition-and-doubling',
    label: 'Define external reflection: presupposed-as-sublated and doubled',
    digest: 'External reflection presupposes itself as sublated and splits into immediate-as-presupposed and negative self-reference.',
    clauses: [
      'assert(presupposed(Reflection) == sublated(Reflection))',
      'tag(Reflection,"doubled")',
      'annotate(Reflection,{modes:["immediate-as-presupposed","negative-self-reference"]})',
      'relate(Reflection,"refers-to-non-being-of-self")'
    ],
    predicates: [{ name: 'DefinesExternalReflection', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'Reflection', to: 'Reflection' },
      { predicate: 'splitsInto', from: 'Reflection', to: 'Immediate|NegativeSelfRef' }
    ]
  },
  {
    id: 'ess-ref1-op-2-presupposes-being-moment',
    chunkId: 'ess-ref1-2-presupposes-a-being',
    label: 'Presuppose a being; immediacy self-refers; determinateness as moment',
    digest: 'External reflection presupposes being with self-referring immediacy; determinateness only as moment.',
    clauses: [
      'assert(presupposes(Reflection, Being) == true)',
      'assert(selfRefers(Immediacy) == true)',
      'tag(Determinateness,"moment-only")'
    ],
    predicates: [{ name: 'PresupposesBeingWithMoment', args: [] }],
    relations: [
      { predicate: 'presupposes', from: 'Reflection', to: 'Being' },
      { predicate: 'hasMoment', from: 'Being', to: 'Determinateness' }
    ]
  },
  {
    id: 'ess-ref1-op-3-posits-then-finds-presupposition',
    chunkId: 'ess-ref1-3-posits-and-finds-presupposition',
    label: 'Posit → sublate → find presupposition and return',
    digest: 'Reflection posits and immediately sublates; hence finds an immediate presupposition to start from and returns, without concern for its “positedness.”',
    clauses: [
      'assert(posit(Reflection, Presupposition))',
      'assert(sublate(Presupposition) == true)',
      'tag(Presupposition,"immediate-before-it")',
      'assert(startsFrom(Reflection, Presupposition) && returnsToSelf(Reflection))',
      'annotate(Presupposition,{status:"sublated", note:"not Reflection’s concern here"})'
    ],
    predicates: [{ name: 'PositSublateFind', args: [] }],
    relations: [
      { predicate: 'startsFrom', from: 'Reflection', to: 'Presupposition' },
      { predicate: 'returnsTo', from: 'Reflection', to: 'Reflection' }
    ]
  },
  {
    id: 'ess-ref1-op-4-external-determinations-infinite',
    chunkId: 'ess-ref1-4-external-determinations-and-infinite',
    label: 'Mark determinations as external; finite/infinite schema',
    digest: 'What is posited in the immediate is external; in being: begin from finite (abiding), infinite stands as reflection-into-itself against it.',
    clauses: [
      'tag(Determinations,"external-to-reflection")',
      'annotate(Being,{finite:"abiding-foundation", infinite:"reflection-into-itself-over-against"})'
    ],
    predicates: [{ name: 'MarksExternalityAndSchema', args: [] }],
    relations: [
      { predicate: 'overAgainst', from: 'Infinite', to: 'Finite' },
      { predicate: 'beginsFrom', from: 'Analysis', to: 'Finite' }
    ]
  },
  {
    id: 'ess-ref1-op-5-syllogism-structure',
    chunkId: 'ess-ref1-5-syllogism-form',
    label: 'Encode syllogism of external reflection',
    digest: 'Extremes: Immediate vs Reflection-into-itself; Middle: Determinate-immediate connecting them.',
    clauses: [
      'assert(syllogism(ExternalReflection).extremes == ["Immediate","ReflectionIntoItself"])',
      'assert(syllogism(ExternalReflection).middle == "DeterminateImmediate")',
      'partition(MiddleReference, {immediacy:"toImmediate", determinateness:"toReflection"})'
    ],
    predicates: [{ name: 'EncodesSyllogism', args: [] }],
    relations: [
      { predicate: 'connects', from: 'DeterminateImmediate', to: 'Immediate|ReflectionIntoItself' }
    ]
  },
  {
    id: 'ess-ref1-op-6-negating-its-negating',
    chunkId: 'ess-ref1-6-positing-and-negating-its-negating',
    label: 'Posit immediate → negative; negate its negating; alien negative only-in-beginning',
    digest: 'Posits immediate as negative/determined; sublates this positing; negates its negating; the “alien” negative exists only as this beginning.',
    clauses: [
      'assert(posit(Immediate) && determineAs(Immediate,"negative"))',
      'assert(sublate(positing(Immediate)) == true)',
      'assert(negate(negate(Immediate)) == positing(Immediate))',
      'annotate(Negative,{onlyIn:"its-beginning"})'
    ],
    predicates: [{ name: 'DoubleNegationPattern', args: [] }],
    relations: [
      { predicate: 'determinesAs', from: 'Reflection', to: 'Immediate~Negative' }
    ]
  },
  {
    id: 'ess-ref1-op-7-externality-sublated-determining',
    chunkId: 'ess-ref1-7-externality-sublated-determining',
    label: 'Externality sublated → coincidence with immediate → determining reflection',
    digest: 'Immediate is posited as same as reflection; reflection negates its own determining; externality sublated; coincidence with immediate = essence’s immediacy; hence determining reflection (essence in-and-for-itself).',
    clauses: [
      'assert(positedSame(Immediate, Reflection) == true)',
      'assert(negatesOwnDetermining(Reflection) == true)',
      'tag(Reflection,"externality-sublated")',
      'assert(coincides(Reflection, Immediate) == true)',
      'tag(Essence,"in-and-for-itself")',
      'tag(Reflection,"determining")'
    ],
    predicates: [{ name: 'SublatesExternality', args: [] }],
    relations: [
      { predicate: 'coincides', from: 'Reflection', to: 'Immediate' },
      { predicate: 'yields', from: 'DeterminingReflection', to: 'EssenceInAndForItself' }
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
