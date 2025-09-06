import type { Chunk, LogicalOperation } from './index';

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'existence-1-outline-as-such',
    title:
      'Existence as such: outline (determinateness → quality → reality/negation → something)',
    text: `In existence
(a) as such, its determinateness is first
(b) to be distinguished as quality.
The latter, however, is to be taken in
both the two determinations of
existence as reality and negation.
In these determinacies, however,
existence is equally reflected into itself,
and, as so reflected, it is posited as
(c) something, an existent.`,
    summary:
      'Outline: determinateness first; becomes quality as reality|negation; in these, existence reflects-into-itself and is posited as “something” (an existent).'
  },
  {
    id: 'existence-2-from-becoming-immediacy',
    title:
      'Existence from becoming: simple oneness; immediate (becoming behind it)',
    text: `Existence proceeds from becoming.
It is the simple oneness of being and nothing.
On account of this simplicity,
it has the form of an immediate.
Its mediation, the becoming, lies behind it;
it has sublated itself,
and existence therefore appears as a first
from which the forward move is made.
It is at first in the one-sided determination of being;
the other determination which it contains, nothing,
will likewise come up in it,
in contrast to the first.`,
    summary:
      'From Becoming: existence = simple oneness of being|nothing; now as immediacy (mediation behind it). Initially one-sided as being; its nothing will appear in contrast.'
  },
  {
    id: 'existence-3-da-being-with-non-being',
    title:
      '“Da”-sein: being with non-being; determinateness as such (in form of being)',
    text: `It is not mere being but existence,
or Dasein [in German];
according to its [German] etymology,
it is being (Sein) in a certain place (da).
But the representation of space does not belong here.
As it follows upon becoming,
existence is in general
being with a non-being,
so that this non-being is taken up
into simple unity with being.
Non-being thus taken up into being
with the result that the concrete whole is
in the form of being, of immediacy,
constitutes determinateness as such.`,
    summary:
      'Da‑sein: not mere being but being‑with‑non‑being; non‑being is taken up into unity, so the whole stands in the form of being (immediacy). This is determinateness as such.'
  },
  {
    id: 'existence-4-reflection-vs-posited-I',
    title:
      'Reflection vs posited I: being-as-moment (for us) vs determinateness-posited (in it)',
    text: `The whole is likewise in the form
or determinateness of being,
since in becoming being has likewise
shown itself to be only a moment:
something sublated, negatively determined.
It is such, however, for us, in our reflection;
not yet as posited in it.
What is posited, however, is
the determinateness as such of existence,
as is also expressed by the da (or “there”) of the Dasein.
The two are always to be clearly distinguished.`,
    summary:
      'Method: being is only-a-moment (for us, by reflection); what is posited in existence itself is determinateness‑as‑such (“da”). Keep reflection vs posited strictly distinct.'
  },
  {
    id: 'existence-5-reflection-vs-posited-II',
    title:
      'Reflection vs posited II: scope of commentary vs moments of the fact itself',
    text: `Only that which is posited in a concept
belongs in the course of the elaboration
of the latter to its content.
Any determinateness not yet posited
in the concept itself
belongs instead to our reflection,
whether this reflection is directed to
the nature of the concept itself
or is a matter of external comparison.
To remark on a determinateness
of this last kind can only be
for the clarification or anticipation of the whole
that will transpire in the course of the development itself.
That the whole, the unity of being and nothing,
is in the one-sided determinateness of being
is an external reflection;
but in negation, in something and other,
and so forth, it will become posited.
It was necessary here to call attention to
the distinction just given;
but to comment on all
that reflection can allow itself,
to give an account of it,
would lead to a long-winded anticipation
of what must transpire in the fact itself.
Although such reflections may
serve to facilitate a general overview
and thus facilitate understanding,
they also bring the disadvantage of
being seen as unjustified assertions,
unjustified grounds and foundations,
of what is to follow.
They should be taken for no more than
what they are supposed to be
and should be distinguished from
what constitutes a moment in
the advance of the fact itself.`,
    summary:
      'Discipline: only what is posited belongs to the concept’s content; unposited belongs to reflection (ancillary, at most clarifying). Negation/something/other will be posited in due course.'
  },
  {
    id: 'existence-6-correspondence-to-being',
    title:
      'Correspondence: being (indeterminate) vs existence (determinate being, concrete)',
    text: `Existence corresponds to being in the preceding sphere.
But being is the indeterminate;
there are no determinations that therefore transpire in it.
But existence is determinate being, something concrete;
consequently, several determinations,
several distinct relations of its moments,
immediately emerge in it.`,
    summary:
      'Correspondence: being = indeterminate (no inner articulation); existence = determinate being (concrete), where multiple determinations and relations emerge at once.'
  }
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // Outline
  {
    id: 'existence-op-1-outline-as-such',
    chunkId: 'existence-1-outline-as-such',
    label:
      'Outline: determinateness → quality (reality|negation) → something (existent)',
    clauses: [
      'tag(Existence,"as-such")',
      'assert(distinguished(Determinateness,Quality))',
      'assert(determinationsOf(Existence,["reality","negation"]))',
      'assert(reflectedIntoItself(Existence))',
      'tag(Something,"existent")',
    ],
    predicates: [{ name: 'ExistenceOutline', args: [] }],
    relations: [
      { predicate: 'hasDetermination', from: 'Existence', to: 'Reality' },
      { predicate: 'hasDetermination', from: 'Existence', to: 'Negation' },
      { predicate: 'yields', from: 'Existence', to: 'Something' },
    ],
  },

  // From becoming → immediacy
  {
    id: 'existence-op-2-from-becoming-immediate',
    chunkId: 'existence-2-from-becoming-immediacy',
    label: 'From becoming: simple oneness; immediate (mediation behind it)',
    clauses: [
      'assert(proceedsFrom(Existence,Becoming))',
      'tag(Existence,"simple-oneness-of-being-and-nothing")',
      'tag(Existence,"immediate")',
      'assert(mediationBehind(Existence,Becoming))',
      'tag(Existence,"appears-as-first")',
      'assert(contains(Existence,Being))',
      'assert(contains(Existence,Nothing))',
    ],
    predicates: [{ name: 'ExistenceFromBecoming', args: [] }],
    relations: [
      { predicate: 'proceedsFrom', from: 'Existence', to: 'Becoming' },
    ],
  },

  // Da-sein: being-with-non-being
  {
    id: 'existence-op-3-being-with-non-being',
    chunkId: 'existence-3-da-being-with-non-being',
    label: 'Being with non-being: determinateness in form of being (immediacy)',
    clauses: [
      'tag(Existence,"being-with-non-being")',
      'assert(takenUpIntoUnity(NonBeing,Being))',
      'assert(inFormOf(Whole,Being))',
      'tag(Determinateness,"as-such")',
      'annotate(Existence,{etymology:"da-sein",spatialRepresentation:"excluded"})',
    ],
    predicates: [{ name: 'BeingWithNonBeing', args: [] }],
    relations: [
      { predicate: 'includes', from: 'Existence', to: 'NonBeing' },
      { predicate: 'includes', from: 'Existence', to: 'Being' },
    ],
  },

  // Reflection vs posited (I)
  {
    id: 'existence-op-4-reflection-vs-posited-I',
    chunkId: 'existence-4-reflection-vs-posited-I',
    label: 'Being-as-moment (for us) vs determinateness-posited (in it)',
    clauses: [
      'assert(momentOnly(Being,Becoming))',
      'tag(Being,"sublated")',
      'assert(forUs(Reflection,BeingAsMoment))',
      'assert(positedIn(Existence,Determinateness))',
      'tag(Existence,"there-ness-da")',
    ],
    predicates: [{ name: 'ReflectionVsPositedI', args: [] }],
    relations: [
      { predicate: 'posits', from: 'Existence', to: 'Determinateness' },
    ],
  },

  // Reflection vs posited (II)
  {
    id: 'existence-op-5-reflection-vs-posited-II',
    chunkId: 'existence-5-reflection-vs-posited-II',
    label: 'Scope: only posited belongs to concept; reflection is ancillary',
    clauses: [
      'assert(onlyPositedBelongsToConcept(Content))',
      'assert(unpositedBelongsTo(Reflection))',
      'tag(Reflection,"ancillary")',
      'assert(willBePositedLater(["Negation","Something","Other"]))',
    ],
    predicates: [{ name: 'ReflectionDiscipline', args: [] }],
    relations: [
      {
        predicate: 'belongsTo',
        from: 'UnpositedDeterminateness',
        to: 'Reflection',
      },
    ],
  },

  // Correspondence to being
  {
    id: 'existence-op-6-correspondence-to-being',
    chunkId: 'existence-6-correspondence-to-being',
    label:
      'Existence vs being: indeterminate vs determinate (multiple relations emerge)',
    clauses: [
      'assert(correspondsTo(Existence,Being))',
      'tag(Being,"indeterminate")',
      'tag(Existence,"determinate-being")',
      'tag(Existence,"concrete")',
      'assert(multipleDeterminationsEmerge(Existence))',
    ],
    predicates: [{ name: 'ExistenceVsBeing', args: [] }],
    relations: [{ predicate: 'correspondsTo', from: 'Existence', to: 'Being' }],
  },
];
