import type { TopicMapEntry } from '@schema/topic';

/**
 * TopicMap for something-and-other.txt - B. FINITUDE (a) Something and other
 *
 * This TopicMap follows B. FINITUDE (a) Something and other in the source text.
 *
 * Structure:
 * - Outline: (a) Something and other → (b) Determination/constitution/limit → (c) Finitude
 * - Introduction: Negative determination (vs affirmative in first division)
 * - a. Something and other:
 *   - 1. Something and other are: both existents, both others, "this" as subjective, other as 'to heteron', nature as other-of-itself
 *   - 2. Being-for-other and being-in-itself: something preserves itself in non-being, two moments, their relationship
 *   - 3. Identity of being-in-itself and being-for-other: both moments of something, thing-in-itself critique, positedness, transition vs positing, determination as quality
 */
export const somethingAndOtherTopicMap: TopicMapEntry[] = [
  {
    id: 'something-and-other-1',
    title:
      'Outline: (a) Something and other → (b) Determination/constitution/limit → (c) Finitude',
    lineRange: { start: 4, end: 19 },
    description:
      '(a) Something and other: at first they are indifferent to one another;…',
    keyPoints: [
      '(a) Something and other: indifferent at first; negation falls outside both; determinateness belongs to in-itself',
      '(b) Determination passes over into constitution; limit of something',
      '(c) Limit is immanent determination; something is finite',
    
    ],
    status: 'pending',
    section: 'B. FINITUDE',
    order: 1,
    relatedChunks: ['something-and-other-2'],
  },
  {
    id: 'something-and-other-2',
    title:
      'Introduction: negative determination (vs affirmative in first division)',
    lineRange: { start: 21, end: 30 },
    description:
      'In the first division where existence in general was considered, this existence had, as at first taken up, the determination of an existent.',
    keyPoints: [
      'First division: existence had determination of existent (affirmative)',
      'Present division: develops negative determination present in existence',
      'Was first negation; now determined to being-in-itself, negation of negation',
    
    ],
    status: 'pending',
    section: 'B. FINITUDE',
    order: 2,
    relatedChunks: ['something-and-other-1', 'something-and-other-3'],
  },
  {
    id: 'something-and-other-3',
    title: 'Something and other I: both existents; both others; indifference',
    lineRange: { start: 34, end: 46 },
    description:
      'Something and other are, first, both existents or something. Second, each is equally an other.',
    keyPoints: [
      'First: both existents or something',
      'Second: each equally an other',
      'Indifferent which named first',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 3,
    relatedChunks: ['something-and-other-4'],
  },
  {
    id: 'something-and-other-4',
    title: '"This" as subjective designation; language expresses universal',
    lineRange: { start: 47, end: 63 },
    description:
      '"This" serves to fix the distinction and the something which is to be taken in the affirmative sense.',
    keyPoints: [
      '"This" fixes distinction, privileges one something',
      'Subjective designation falling outside something itself',
      'Determinateness falls on external pointing',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 4,
    relatedChunks: ['something-and-other-3', 'something-and-other-5'],
  },
  {
    id: 'something-and-other-5',
    title:
      'Otherness as external; comparison by Third; every existence is other',
    lineRange: { start: 65, end: 79 },
    description:
      'Otherness thus appears as a determination alien to the existence thus pointed at, or the other existence as outside this one existence, partly because the one existence is determined as other only by being compared by a Third, and partly b…',
    keyPoints: [
      'Otherness appears as determination alien to existence',
      'Determined as other by comparison by Third',
      'Determined on account of other outside it',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 5,
    relatedChunks: ['something-and-other-4', 'something-and-other-6'],
  },
  {
    id: 'something-and-other-6',
    title:
      'Sameness falls in external reflection; other for itself apart from something',
    lineRange: { start: 81, end: 89 },
    description:
      'Both are determined as something as well as other: thus they are the same and there is as yet no distinction present in them.',
    keyPoints: [
      'Both determined as something and other: same, no distinction',
      'Sameness falls only in external reflection (comparison)',
      'Other, as posited, is other with reference to something',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 6,
    relatedChunks: ['something-and-other-5', 'something-and-other-7'],
  },
  {
    id: 'something-and-other-7',
    title:
      "Other as 'to heteron' (Plato); other-of-itself; nature as other of spirit",
    lineRange: { start: 91, end: 108 },
    description:
      "Third, the other is therefore to be taken in isolation, with reference to itself, has to be taken abstractly as the other, the 'to heteron' of Plato who opposes it to the one as a moment of totality, and in this way ascribes to the other a nature of its own. Thus the other, taken solely as such, is not the other of something, but is the other within, that is, the other of itself. Such an other, which is the other by its own determination, is physical nature; nature is the other of spirit; this, its determination, is at first a mere relativity expressing not a quality of nature itself but only a reference external to it. But since spirit is the true something, and hence nature is what it is within only in contrast to spirit, taken for itself the quality of nature is just this, to be the other within, that which-exists-outside-itself (in the determinations of space, time, matter).",
    keyPoints: [
      'Other taken in isolation, abstractly as the other',
      'To heteron',
      'Other ascribed nature of its own',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 7,
    relatedChunks: ['something-and-other-6', 'something-and-other-8'],
  },
  {
    id: 'something-and-other-8',
    title:
      'Other-of-itself: absolutely unequal, negates/alters itself, yet identical',
    lineRange: { start: 110, end: 124 },
    description:
      'The other which is such for itself is the other within it, hence the other of itself and so the other of the other; therefore, the absolutely unequal in itself, that which negates itself, alters itself.',
    keyPoints: [
      'Other for itself is other within it, other of itself, other of other',
      'Absolutely unequal in itself',
      'Negates itself, alters itself',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 8,
    relatedChunks: ['something-and-other-7', 'something-and-other-9'],
  },
  {
    id: 'something-and-other-9',
    title:
      'Something preserves itself in non-being; being-for-other and being-in-itself',
    lineRange: { start: 126, end: 145 },
    description:
      'The something preserves itself in its non-being; it is essentially one with it, and essentially not one with it.',
    keyPoints: [
      'Something preserves itself in non-being',
      'Essentially one with it, essentially not one with it',
      'Stands in reference to otherness without being it',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 9,
    relatedChunks: ['something-and-other-8', 'something-and-other-10'],
  },
  {
    id: 'something-and-other-10',
    title: 'Two pairs of determinations; truth is connection',
    lineRange: { start: 147, end: 179 },
    description:
      'Being-for-other and being-in-itself constitute the two moments of something. There are here two pairs of determinations:…',
    keyPoints: [
      'Being-for-other and being-in-itself = two moments of something',
      'Two pairs: (1) something and other; (2) being-for-other and being-in-itself',
      'Former: non-connectedness; something and other fall apart',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 10,
    relatedChunks: ['something-and-other-9', 'something-and-other-11'],
  },
  {
    id: 'something-and-other-11',
    title: 'Being-in-itself: negative reference; has non-being in it',
    lineRange: { start: 181, end: 197 },
    description:
      'Hence being-in-itself is, first, negative reference to non-existence; it has otherness outside it and is opposed to it;…',
    keyPoints: [
      'Being-in-itself: first, negative reference to non-existence',
      'Has otherness outside, opposed to it',
      'Withdrawn from being-other and being-for-other',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 11,
    relatedChunks: ['something-and-other-10', 'something-and-other-12'],
  },
  {
    id: 'something-and-other-12',
    title:
      'Both moments of something; identity of being-in-itself and being-for-other',
    lineRange: { start: 199, end: 221 },
    description:
      'Both moments are determinations of one and the same, namely of something. Something is in-itself in so far as it has returned from the being-for-other back to itself.',
    keyPoints: [
      'Both moments determinations of one and same: something',
      'Something in-itself: returned from being-for-other to itself',
      'Something has determination/circumstance (in itself or in it)',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 12,
    relatedChunks: ['something-and-other-11', 'something-and-other-13'],
  },
  {
    id: 'something-and-other-13',
    title: 'In-itself as abstract; thing-in-itself critique I',
    lineRange: { start: 222, end: 232 },
    description:
      'Opinion has it that with the in-itself something lofty is being said, as with the inner; but what something is only in itself, is also only in it;…',
    keyPoints: [
      'Opinion: in-itself something lofty (like inner)',
      'But what is only in itself is also only in it',
      'In-itself = merely abstract, hence external determination',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 13,
    relatedChunks: ['something-and-other-12', 'something-and-other-14'],
  },
  {
    id: 'something-and-other-14',
    title: 'Thing-in-itself critique II: empty abstraction',
    lineRange: { start: 234, end: 255 },
    description:
      'It may be observed that here we have the meaning of the thing-in-itself.',
    keyPoints: [
      'Here meaning of thing-in-itself',
      'Very simple abstraction (was important, sophisticated)',
      '"We know nothing of things-in-themselves" = valued wisdom',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 14,
    relatedChunks: ['something-and-other-13', 'something-and-other-15'],
  },
  {
    id: 'something-and-other-15',
    title: 'Thing-in-itself in truth: concept (concrete, cognizable)',
    lineRange: { start: 256, end: 264 },
    description:
      'What, however, the thing-in-itself in truth is, what there basically is in it, of this the Logic is the exposition.',
    keyPoints: [
      'Thing-in-itself in truth: Logic is exposition',
      'In Logic, in-itself = what something is in its concept',
      'Concept is in itself concrete',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 15,
    relatedChunks: ['something-and-other-14', 'something-and-other-16'],
  },
  {
    id: 'something-and-other-16',
    title: 'Positedness; transition vs positing; in-itself vs posited',
    lineRange: { start: 266, end: 315 },
    description:
      'Being-in-itself has at first the being-for-other as a moment standing over against it.',
    keyPoints: [
      'Being-in-itself has being-for-other as moment over against it',
      'Positedness also positioned over against it',
      'Positedness: bending back of what is not in itself into being-in-itself',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 16,
    relatedChunks: ['something-and-other-15', 'something-and-other-17'],
  },
  {
    id: 'something-and-other-17',
    title: 'Identity in something; determination as quality',
    lineRange: { start: 324, end: 329 },
    description:
      'In the unity of the something with itself, being-for-other is identical with its in-itself; the being-for-other is thus in the something.',
    keyPoints: [
      'In unity of something with itself, being-for-other identical with in-itself',
      'Being-for-other thus in the something',
      'Determinateness reflected into itself = simple existent',
    
    ],
    status: 'pending',
    section: 'a. Something and other',
    order: 17,
    relatedChunks: ['something-and-other-16'],
  },
];
