import type { TopicMapEntry } from '@schema/topic';

/**
 * TopicMap for constitution.txt - B. FINITUDE (b) Determination, constitution, and limit
 *
 * This TopicMap follows B. FINITUDE (b) Determination, constitution, and limit in the source text.
 *
 * Structure:
 * - Introduction: In-itself mediated through being-for-other; determinateness existent-in-itself
 * - 1. Determination: Affirmative determinateness; example (human being); still only in itself as ought
 * - 2. Constitution: External existence; alteration; determination and constitution pass over into each other; alteration posited
 * - Transition: Sublation → two somethings; negation immanent; limit emerges
 * - 3. Limit: (a) Non-being of other and of something; mediation
 *   - (b) Existence outside/inside limit; limit as middle point
 *   - (c) Something outside limit = existence in general; limit as principle; unrest/contradiction
 * - Conclusion: Something with immanent limit = finite
 */
export const constitutionTopicMap: TopicMapEntry[] = [
  {
    id: 'constitution-1',
    title:
      'Introduction: in-itself mediated through being-for-other; determinateness existent-in-itself',
    lineRange: { start: 4, end: 25 },
    description:
      'The in-itself, in which the something is reflected into itself from its being-for-other, no longer is an abstract in-itself but, as the negation of its being-for-other, is mediated through this latter, which is thus its moment.',
    keyPoints: [
      'In-itself no longer abstract but mediated through being-for-other',
      'Being-for-other is its moment',
      'Identity by virtue of which something has present in it what it is in itself',
    
    ],
    status: 'pending',
    section: 'b. Determination, constitution, and limit',
    order: 1,
    relatedChunks: ['constitution-2'],
  },
  {
    id: 'constitution-2',
    title:
      'Determination I: definition (affirmative determinateness; in-itself by which something abides)',
    lineRange: { start: 27, end: 52 },
    description:
      "The quality which in the simple something is an in-itself essentially in unity with the something's other moment, its being-in-it, can be named its determination, provided that this word is distinguished, in a more precise signification, from determinateness in general. Determination is affirmative determinateness; it is the in-itself by which a something abides in its existence while involved with an other that would determine it, by which it preserves itself in its self-equality, holding on to it in its being-for-other. Something fulfills its determination to the extent that the further determinateness, which variously accrues to it in the measure of its being-in-itself as it relates to an other, becomes its filling. Determination implies that what something is in itself is also present in it.",
    keyPoints: [
      'Quality in simple something is in-itself in unity with being-in-it',
      'Can be named determination (distinguished from determinateness in general)',
      'Determination = affirmative determinateness',
    
    ],
    status: 'pending',
    section: '1. Determination',
    order: 2,
    relatedChunks: ['constitution-1', 'constitution-3'],
  },
  {
    id: 'constitution-3',
    title:
      'Determination II: example (human being; rational thought; still only in itself as ought)',
    lineRange: { start: 54, end: 78 },
    description:
      'The determination of the human being, its vocation, is rational thought: thinking in general is his simple determinateness;…',
    keyPoints: [
      's determination: rational thought (vocation)",',
      ',',
      ',',
    
    ],
    status: 'pending',
    section: '1. Determination',
    order: 3,
    relatedChunks: ['constitution-2', 'constitution-4'],
  },
  {
    id: 'constitution-4',
    title:
      'Constitution I: external existence (not belonging to being-in-itself)',
    lineRange: { start: 80, end: 91 },
    description:
      'The filling of the being-in-itself with determinateness is also distinct from the determinateness which is only being-for-other and remains outside the determination.',
    keyPoints: [
      'Filling of being-in-itself with determinateness distinct from determinateness only being-for-other',
      'In qualitative sphere, distinguished terms left with immediate qualitative being contrasting',
      'What something has in it separates itself',
    
    ],
    status: 'pending',
    section: '2. Constitution',
    order: 4,
    relatedChunks: ['constitution-3', 'constitution-5'],
  },
  {
    id: 'constitution-5',
    title:
      'Constitution II: external influences; alteration falls on constitution',
    lineRange: { start: 93, end: 108 },
    description:
      "Constituted in this or that way, the something is caught up in external influences and in external relationships. This external connection on which the constitution depends, and the being determined through an other, appear as something accidental. But it is the quality of the something to be given over to this externality and to have a constitution. In so far as something alters, the alteration falls on the side of its constitution; the latter is that in the something which becomes an other. The something itself preserves itself in the alteration; the latter affects only this unstable surface of the something's otherness, not its determination.",
    keyPoints: [
      'Constituted: caught up in external influences, relationships',
      'External connection appears accidental',
      'But quality of something to be given over to externality, have constitution',
    
    ],
    status: 'pending',
    section: '2. Constitution',
    order: 5,
    relatedChunks: ['constitution-4', 'constitution-6'],
  },
  {
    id: 'constitution-6',
    title:
      'Determination and constitution: distinct but connected; pass over into each other',
    lineRange: { start: 110, end: 137 },
    description:
      'Determination and constitution are thus distinct from each other; something, according to its determination, is indifferent to its constitution.',
    keyPoints: [
      'Determination and constitution distinct; something indifferent to constitution according to determination',
      'What something has in it = middle term connecting determination and constitution',
      'Being-in-something falls apart into two extremes',
    
    ],
    status: 'pending',
    section: '2. Constitution',
    order: 6,
    relatedChunks: ['constitution-5', 'constitution-7'],
  },
  {
    id: 'constitution-7',
    title:
      'Constitution passes over into determination; constitution depends on determination',
    lineRange: { start: 138, end: 151 },
    description:
      "Conversely, the being-for-other, isolated as constitution and posited on its own, is in it the same as what the other as such is, the other in it, that is, the other of itself; but it consequently is self-referring existence, thus being-in-itself with a determinateness, therefore determination. Consequently, inasmuch as the two are also to be held apart, constitution, which appears to be grounded in something external, in an other in general, also depends on determination, and the determining from outside is at the same time determined by the something's own immanent determination. And further, constitution belongs to that which something is in itself: something alters along with its constitution.",
    keyPoints: [
      'Being-for-other isolated as constitution: same as other in it, other of itself',
      'Consequently self-referring existence = being-in-itself with determinateness = determination',
      'Constitution appears grounded in external, other, but also depends on determination',
    
    ],
    status: 'pending',
    section: '2. Constitution',
    order: 7,
    relatedChunks: ['constitution-6', 'constitution-8'],
  },
  {
    id: 'constitution-8',
    title: 'Alteration posited in something; negation immanent',
    lineRange: { start: 153, end: 161 },
    description:
      'This altering of something is no longer the first alteration of something merely in accordance with its being-for-other. The first was an alteration only implicitly present, one that belonged to the inner concept;…',
    keyPoints: [
      'Alteration no longer first alteration (merely in accordance with being-for-other)',
      'First was only implicitly present (inner concept)',
      'Now alteration also posited in something',
    
    ],
    status: 'pending',
    section: '2. Constitution',
    order: 8,
    relatedChunks: ['constitution-7', 'constitution-9'],
  },
  {
    id: 'constitution-9',
    title: 'Transition I: sublation → two somethings; negation immanent',
    lineRange: { start: 163, end: 179 },
    description:
      'The transition of determination and constitution into each other is at first the sublation of their distinction, and existence or something in general is thereby posited;…',
    keyPoints: [
      'Transition of determination and constitution: sublation of distinction',
      'Existence or something in general posited',
      'Since distinction includes qualitative otherness: two somethings',
    
    ],
    status: 'pending',
    section: 'Transition',
    order: 9,
    relatedChunks: ['constitution-8', 'constitution-10'],
  },
  {
    id: 'constitution-10',
    title:
      'Transition II: something behaves to other through itself; negation of other = quality; limit',
    lineRange: { start: 181, end: 227 },
    description:
      "Something behaves in this way in relation to the other through itself; since otherness is posited in it as its own moment, its in-itselfness holds negation in itself, and it now has its affirmative existence through its intermediary alone. But the other is also qualitatively distinguished from this affirmative existence and is thus posited outside the something. The negation of its other is only the quality of the something, for it is in this sublation of its other that it is something. The other, for its part, truly confronts an existence only with this sublation; it confronts the first something only externally, or, since the two are in fact inherently joined together, that is, according to their concept, their connectedness consists in this, that existence has passed over into otherness, something into other; that something is just as much an other as the other is. Now in so far as the in-itselfness is the non-being of the otherness that is contained in it but is at the same time also distinct as existent, something is itself negation, the ceasing to be of an other in it; it is posited as behaving negatively in relation to the other and in so doing preserving itself. This other, the in-itselfness of the something as negation of the negation, is the something's being-in-itself, and this sublation is as simple negation at the same time in it, namely, as its negation of the other something external to it. It is one determinateness of the two somethings that, on the one hand, as negation of the negation, is identical with the in-itselfness of the somethings, and also, on the other hand, since these negations are to each other as other somethings, joins them together of their own accord and, since each negation negates the other, equally separates them. This determinateness is limit.",
    keyPoints: [
      'Something behaves to other through itself',
      'Otherness posited in it as own moment',
      'In-itselfness holds negation in itself',
    
    ],
    status: 'pending',
    section: 'Transition',
    order: 10,
    relatedChunks: ['constitution-9', 'constitution-11'],
  },
  {
    id: 'constitution-11',
    title: 'Limit I: non-being of other and of something; mediation',
    lineRange: { start: 245, end: 260 },
    description:
      'Something is therefore immediate, self-referring existence and at first it has a limit with respect to an other; limit is the non-being of the other, not of the something itself;…',
    keyPoints: [
      'Something immediate, self-referring existence',
      'At first has limit with respect to other',
      'Limit = non-being of other, not of something itself',
    
    ],
    status: 'pending',
    section: '3. Limit (a)',
    order: 11,
    relatedChunks: ['constitution-10', 'constitution-12'],
  },
  {
    id: 'constitution-12',
    title: 'Limit II: through limit something is; limit = mediation',
    lineRange: { start: 262, end: 285 },
    description:
      'But the limit is equally, essentially, the non-being of the other; thus, through its limit, something at the same time is.',
    keyPoints: [
      'Limit essentially non-being of other',
      'Through limit, something at same time is',
      'In limiting, something reduced to being limited',
    
    ],
    status: 'pending',
    section: '3. Limit (a)',
    order: 12,
    relatedChunks: ['constitution-11', 'constitution-13'],
  },
  {
    id: 'constitution-13',
    title: 'Limit III: existence outside/inside limit; limit as middle point',
    lineRange: { start: 287, end: 312 },
    description:
      'Now in so far as something in its limit both is and is not, and these moments are an immediate, qualitative distinction, the non-existence and the existence of the something fall outside each other.',
    keyPoints: [
      'Something in limit both is and is not',
      'Moments immediate, qualitative distinction',
      'Non-existence and existence fall outside each other',
    
    ],
    status: 'pending',
    section: '3. Limit (b)',
    order: 13,
    relatedChunks: ['constitution-12', 'constitution-14'],
  },
  {
    id: 'constitution-14',
    title:
      'Limit IV: something outside limit = existence in general; limit as principle',
    lineRange: { start: 314, end: 359 },
    description:
      'But further, something as it is outside the limit, as the unlimited something, is only existence in general. As such, it is not distinguished from its other;…',
    keyPoints: [
      'Something outside limit = unlimited something = only existence in general',
      'Not distinguished from other; same determination',
      'Both same',
    
    ],
    status: 'pending',
    section: '3. Limit (c)',
    order: 14,
    relatedChunks: ['constitution-13', 'constitution-15'],
  },
  {
    id: 'constitution-15',
    title: 'Limit V: unrest/contradiction; dialectic of point/line/plane',
    lineRange: { start: 361, end: 405 },
    description:
      'The other determination is the unrest of the something in its limit in which it is immanent, the contradiction that propels it beyond itself. Thus the point is this dialectic of itself becoming line;…',
    keyPoints: [
      'Other determination: unrest of something in limit (immanent)',
      'Contradiction propels it beyond itself',
      'Point = dialectic of itself becoming line',
    
    ],
    status: 'pending',
    section: '3. Limit (c)',
    order: 15,
    relatedChunks: ['constitution-14', 'constitution-16'],
  },
  {
    id: 'constitution-16',
    title: 'Conclusion: something with immanent limit = finite',
    lineRange: { start: 407, end: 410 },
    description:
      'The something, posited with its immanent limit as the contradiction of itself by virtue of which it is directed and driven out and beyond itself, the finite.',
    keyPoints: [
      'Something posited with immanent limit',
      'As contradiction of itself',
      'By virtue of which directed and driven out and beyond itself',
    
    ],
    status: 'pending',
    section: 'Conclusion',
    order: 16,
    relatedChunks: ['constitution-15'],
  },
];
