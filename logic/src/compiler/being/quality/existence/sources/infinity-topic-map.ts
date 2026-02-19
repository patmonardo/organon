import type { TopicMapEntry } from '@schema/topic';

/**
 * TopicMap for infinity.txt - C. INFINITY (a) The infinite in general
 *
 * This TopicMap follows C. INFINITY (a) The infinite in general in the source text.
 *
 * Structure:
 * - Introduction: Infinite as definition of absolute; true vs bad infinity; outline
 * - a. The infinite in general:
 *   - Infinite as negation of negation; spirit at home
 *   - Finite transcends itself to become infinite; not external elevation
 *   - Finite vanishes into infinite
 */
export const infinityTopicMap: TopicMapEntry[] = [
  {
    id: 'infinity-1',
    title:
      'Introduction I: infinite as definition of absolute; not free from restrictedness',
    lineRange: { start: 4, end: 24 },
    description:
      'The infinite in its simple concept can be regarded, first of all, as a fresh definition of the absolute; as self-reference devoid of determination, it is posited as being and becoming.',
    keyPoints: [
      'Infinite in simple concept = fresh definition of absolute',
      'Self-reference devoid of determination; posited as being and becoming',
      'Forms of existence have no place in definitions of absolute (only determinacies, finite)',
    
    ],
    status: 'pending',
    section: 'C. INFINITY',
    order: 1,
    relatedChunks: ['infinity-2'],
  },
  {
    id: 'infinity-2',
    title: 'Introduction II: true vs bad infinity; outline',
    lineRange: { start: 25, end: 44 },
    description:
      'It is essential to distinguish the true concept of infinity from bad infinity, the infinite of reason from the infinite of the understanding.',
    keyPoints: [
      'Essential to distinguish true concept of infinity from bad infinity',
      'Infinite of reason vs infinite of understanding',
      'Bad infinite = finitized infinite',
    
    ],
    status: 'pending',
    section: 'C. INFINITY',
    order: 2,
    relatedChunks: ['infinity-1', 'infinity-3'],
  },
  {
    id: 'infinity-3',
    title: 'Infinite in general I: negation of negation; spirit at home',
    lineRange: { start: 48, end: 58 },
    description:
      'The infinite is the negation of negation, the affirmative, being that has reinstated itself out of restrictedness. The infinite is, in a more intense sense than the first immediate being;…',
    keyPoints: [
      'Infinite = negation of negation, affirmative',
      'Being that has reinstated itself out of restrictedness',
      'In more intense sense than first immediate being',
    
    ],
    status: 'pending',
    section: 'a. The infinite in general',
    order: 3,
    relatedChunks: ['infinity-2', 'infinity-4'],
  },
  {
    id: 'infinity-4',
    title:
      'Infinite in general II: finite transcends itself to become infinite',
    lineRange: { start: 59, end: 65 },
    description:
      'What is first given with the concept of the infinite is this, that in its being-in-itself existence is determined as finite and transcends restriction.',
    keyPoints: [
      'First given with concept of infinite: in being-in-itself existence determined as finite and transcends restriction',
      'Very nature of finite: transcend itself, negate its negation, become infinite',
    
    ],
    status: 'pending',
    section: 'a. The infinite in general',
    order: 4,
    relatedChunks: ['infinity-3', 'infinity-5'],
  },
  {
    id: 'infinity-5',
    title:
      "Infinite in general III: not external elevation; finite's own nature",
    lineRange: { start: 66, end: 84 },
    description:
      'Consequently, the infinite does not stand above the finite as something ready-made by itself, as if the finite stood fixed outside or below it.',
    keyPoints: [
      'Infinite does not stand above finite as ready-made (finite fixed outside/below)',
      'Not we only (subjective reason) who transcend finite into infinite',
      't affect it)",',
    
    ],
    status: 'pending',
    section: 'a. The infinite in general',
    order: 5,
    relatedChunks: ['infinity-4', 'infinity-6'],
  },
  {
    id: 'infinity-6',
    title: 'Infinite in general IV: finite vanishes into infinite',
    lineRange: { start: 85, end: 92 },
    description:
      'The finite has thus vanished into the infinite and what is, is only the infinite.',
    keyPoints: [
      'Finite has vanished into infinite',
      'What is, is only the infinite',
    
    ],
    status: 'pending',
    section: 'a. The infinite in general',
    order: 6,
    relatedChunks: ['infinity-5'],
  },
];
