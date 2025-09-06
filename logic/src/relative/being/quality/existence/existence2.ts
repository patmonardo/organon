import type { Chunk, LogicalOperation } from './index';

export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'existence-11-something-quality-unseparated',
    title:
      'Something I: reality ∧ negation as existences; quality unseparated from existence',
    text: `c. Something

In existence its determinateness has been distinguished as quality;
in this quality as something existing, the distinction exists:
the distinction of reality and negation.
Now though these distinctions are present in existence,
they are just as much null and sublated.
Reality itself contains negation;
it is existence, not indeterminate or abstract being.
Negation is for its part equally existence,
not the supposed abstract nothing
but posited here as it is in itself,
as existent, as belonging to existence.
Thus quality is in general unseparated from existence,
and the latter is only determinate, qualitative being.`,
    summary:
      'Reality and negation are present as existences in quality, yet sublated. Reality contains negation; negation is existent (not abstract nothing). Hence quality is unseparated from existence; existence = determinate, qualitative being.',
  },
  {
    id: 'existence-12-sublation-yields-being-in-itself',
    title:
      'Something II: sublation (not omission) → simplicity mediated → being-in-itself',
    text: `This sublating of the distinction is more than
the mere retraction and external re-omission of it,
or a simple return to the simple beginning,
to existence as such.
The distinction cannot be left out, for it is.
Therefore, what de facto is at hand is this:
existence in general, distinction in it,
and the sublation of this distinction;
the existence, not void of distinctions as at the beginning,
but as again self-equal through the sublation of the distinction;
the simplicity of existence mediated through this sublation.
This state of sublation of the distinction is
existence's own determinateness;
existence is thus being-in-itself;
it is existent, something.`,
    summary:
      'Sublation is not erasure: existence + distinction + its sublation. Existence regains self-equality as simplicity mediated by sublation; this is its own determinateness—being-in-itself, i.e., something.',
  },
  {
    id: 'existence-13-something-negation-of-negation-beginning-of-subject',
    title: 'Something III: negation of negation; beginning of the subject',
    text: `Something is the first negation of negation,
as simple existent self-reference.
Existence, life, thought, and so forth,
essentially take on the determination of an existent being,
a living thing, a thinking mind (“I”), and so forth.
This determination is of the highest importance
if we do not wish to halt at existence, life, thought,
and so forth, as generalities, also not at Godhood (instead of God).
In common representation, something rightly carries
the connotation of a real thing.
Yet it still is a very superficial determination,
just as reality and negation, existence and its determinateness,
though no longer the empty being and nothing,
still are quite abstract determinations.
For this reason they also are the most common expressions,
and a reflection that is still philosophically
unschooled uses them the most;
it casts its distinctions in them,
fancying that in them it has something
really well and firmly determined.
As something, the negative of the negative is
only the beginning of the subject;
its in-itselfness is still quite indeterminate.
It determines itself further on,
at first as existent-for-itself and so on,
until it finally obtains in the concept
the intensity of the subject.
At the base of all these determinations
there lies the negative unity with itself.
In all this, however, care must be taken
to distinguish the first negation, negation as negation in general,
from the second negation, the negation of negation
which is concrete, absolute negativity,
just as the first is on the contrary only abstract negativity.`,
    summary:
      '“Something” = first negation of negation: simple existent self-reference. It is the beginning of the subject (still indeterminate), later developing to for-itself and concept. Distinguish abstract negation from concrete negation-of-negation (absolute negativity).',
  },
  {
    id: 'existence-14-something-mediation-with-itself',
    title: 'Something IV: mediation-with-itself (vs alleged bare immediacy)',
    text: `Something is an existent as the negation of negation,
for such a negation is the restoration of
the simple reference to itself;
but the something is thereby equally
the mediation of itself with itself.
Present in the simplicity of something,
and then with greater determinateness
in being-for-itself,
in the subject, and so forth,
this mediation of itself with itself is
also already present in becoming,
but only as totally abstract mediation;
mediation with itself is posited in the something
in so far as the latter is determined as a simple identity.
Attention can be drawn to the presence of mediation in general,
as against the principle of the alleged bare immediacy of a knowledge
from which mediation should be excluded.
But there is no further need to draw
particular attention to the moment of mediation,
since it is to be found everywhere and on all sides,
in every concept.`,
    summary:
      'As negation-of-negation, something restores self-reference and is mediation-with-itself (already implicit in becoming, there abstract; here posited). This rebuts doctrines of “bare immediacy”; mediation pervades concepts.',
  },
  {
    id: 'existence-15-something-becoming-other',
    title:
      'Something V: mediation-only collapses to unity; becoming with moments as existents (other)',
    text: `This mediation with itself which something is in itself,
when taken only as the negation of negation,
has no concrete determinations for its sides;
thus it collapses into the simple unity which is being.
Something is, and is therefore also an existent.
Further, it is in itself also becoming,
but a becoming that no longer has only
being and nothing for its moments.
One of these moments, being, is now
existence and further an existent.
The other moment is equally an existent,
but determined as the negative of something; an other.`,
    summary:
      'If grasped only as negation-of-negation, mediation lacks sides and collapses to being. Yet something is an existent and in itself becoming whose moments are now existents: itself and its negative, the other.',
  },
  {
    id: 'existence-16-something-alteration-and-other-in-general',
    title:
      'Something VI: alteration; initially only in concept; other as qualitative',
    text: `As becoming, something is a transition,
the moments of which are themselves something,
and for that reason it is an alteration,
a becoming that has already become concrete.
At first, however, something alters only in its concept;
it is not yet posited in this way, as mediated and mediating,
but at first only as maintaining itself
simply in its reference to itself;
and its negative is posited as equally qualitative,
as only an other in general.`,
    summary:
      'Something’s becoming is alteration (moments are something). Initially, alteration is only conceptual; something maintains simple self-reference, and its negative is posited merely as qualitative “other in general.”',
  },
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'existence-op-11-something-unity-of-reality-negation',
    chunkId: 'existence-11-something-quality-unseparated',
    label:
      'Reality ∧ negation present and sublated; quality unseparated; existence qualitative',
    clauses: [
      'assert(distinctionPresent(Quality,["reality","negation"]))',
      'assert(sublatedIn(Existence,["reality","negation"]))',
      'assert(contains(Reality,Negation))',
      'assert(isExistence(Negation))',
      'tag(Quality,"unseparated-from-existence")',
      'tag(Existence,"determinate-qualitative-being")',
    ],
    predicates: [{ name: 'RealityNegationInExistence', args: [] }],
    relations: [{ predicate: 'contains', from: 'Reality', to: 'Negation' }],
  },
  {
    id: 'existence-op-12-sublation-being-in-itself',
    chunkId: 'existence-12-sublation-yields-being-in-itself',
    label:
      'Sublation (not omission) → simplicity mediated → being-in-itself (something)',
    clauses: [
      'assert(notMereOmission(Sublation))',
      'assert(triad(Existence,Distinction,Sublation))',
      'tag(Existence,"self-equal-through-sublation")',
      'tag(Existence,"simplicity-mediated")',
      'tag(Existence,"being-in-itself")',
      'tag(Something,"posited")',
    ],
    predicates: [{ name: 'SublationYieldsBeingInItself', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-13-something-negation-of-negation',
    chunkId: 'existence-13-something-negation-of-negation-beginning-of-subject',
    label:
      'Something = negation of negation; beginning of subject; abstract vs concrete negation',
    clauses: [
      'tag(Something,"negation-of-negation")',
      'tag(Something,"self-reference")',
      'tag(Something,"existent")',
      'assert(takesOnDetermination(["Existence","Life","Thought"],"existent"))',
      'assert(distinguish(AbstractNegation,NegationOfNegation))',
      'tag(NegationOfNegation,"concrete-absolute-negativity")',
      'tag(AbstractNegation,"abstract-negativity")',
    ],
    predicates: [{ name: 'SomethingAsBeginning', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-14-mediation-with-itself',
    chunkId: 'existence-14-something-mediation-with-itself',
    label: 'Mediation-with-itself posited (against “bare immediacy”)',
    clauses: [
      'assert(mediatesWithItself(Something))',
      'annotate(Becoming,{mediation:"abstract"})',
      'assert(positsMediationAsIdentity(Something))',
      'tag(Mediation,"pervasive-in-concepts")',
      'assert(againstBareImmediacy(Knowledge))',
    ],
    predicates: [{ name: 'MediationPosited', args: [] }],
    relations: [],
  },
  {
    id: 'existence-op-15-becoming-with-other',
    chunkId: 'existence-15-something-becoming-other',
    label: 'Something as becoming: moments are existents—self and other',
    clauses: [
      'assert(collapsesTo(Something,Being))',
      'tag(Something,"existent")',
      'assert(inItselfBecoming(Something))',
      'assert(momentIs(Being,Existence))',
      'tag(Other,"negative-of-something")',
      'tag(Other,"existent")',
    ],
    predicates: [{ name: 'SomethingBecomingOther', args: [] }],
    relations: [{ predicate: 'otherOf', from: 'Other', to: 'Something' }],
  },
  {
    id: 'existence-op-16-alteration-and-other-general',
    chunkId: 'existence-16-something-alteration-and-other-in-general',
    label:
      'Alteration: at first only conceptual; negative as qualitative other-in-general',
    clauses: [
      'assert(transition(Something,Alteration))',
      'tag(Alteration,"concrete-becoming")',
      'assert(maintainsSelfReference(Something))',
      'assert(negativePositedAs(Other,"qualitative-other-in-general"))',
    ],
    predicates: [{ name: 'AlterationInitial', args: [] }],
    relations: [],
  },
];
