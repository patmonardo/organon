import type { TopicMapEntry } from '@schema/topic';

/**
 * TopicMap for existence.txt - A. EXISTENCE AS SUCH
 *
 * This TopicMap follows A. EXISTENCE AS SUCH in the source text.
 *
 * Structure:
 * - Outline: determinateness → quality (reality|negation) → something
 * - a. Existence in general: From becoming, da-sein, reflection vs posited
 * - b. Quality: Immediate unity, reality and negation, limit/restriction
 * - c. Something: Negation of negation, beginning of subject, mediation-with-itself, alteration
 */
export const existenceTopicMap: TopicMapEntry[] = [
  {
    id: 'existence-1',
    title:
      'Outline: determinateness → quality (reality|negation) → something (existent)',
    lineRange: { start: 4, end: 13 },
    description:
      'In existence (a) as such, its determinateness is first (b) to be distinguished as quality. The latter, however, is to be taken in both the two determinations of existence as reality and negation.',
    keyPoints: [
      'Existence as such: determinateness first',
      'Determinateness distinguished as quality',
      'Quality taken in both determinations: reality and negation',
    
    ],
    status: 'pending',
    section: 'A. EXISTENCE AS SUCH',
    order: 1,
    relatedChunks: ['existence-2', 'existence-7', 'existence-11'],
  },
  {
    id: 'existence-2',
    title:
      'Existence from becoming: simple oneness; immediate (becoming behind it)',
    lineRange: { start: 17, end: 28 },
    description:
      'Existence proceeds from becoming. It is the simple oneness of being and nothing.',
    keyPoints: [
      'Existence proceeds from becoming',
      'Simple oneness of being and nothing',
      'Form of an immediate',
    
    ],
    status: 'pending',
    section: 'a. Existence in general',
    order: 2,
    relatedChunks: ['existence-3', 'existence-1'],
  },
  {
    id: 'existence-3',
    title:
      '"Da"-sein: being with non-being; determinateness as such (in form of being)',
    lineRange: { start: 30, end: 43 },
    description:
      'It is not mere being but existence, or Dasein [in German]; according to its [German] etymology, it is being (Sein) in a certain place (da).',
    keyPoints: [
      'Not mere being but existence, or Dasein',
      'Etymology: being (Sein) in a certain place (da) — but space representation excluded',
      'Being with non-being',
    
    ],
    status: 'pending',
    section: 'a. Existence in general',
    order: 3,
    relatedChunks: ['existence-2', 'existence-4'],
  },
  {
    id: 'existence-4',
    title:
      'Reflection vs posited I: being-as-moment (for us) vs determinateness-posited (in it)',
    lineRange: { start: 45, end: 55 },
    description:
      'The whole is likewise in the form or determinateness of being, since in becoming being has likewise shown itself to be only a moment: something sublated, negatively determined.',
    keyPoints: [
      'Whole in form of being (since in becoming, being shown as only a moment)',
      'Being as moment: sublated, negatively determined',
      'For us, in our reflection — not yet posited in it',
    
    ],
    status: 'pending',
    section: 'a. Existence in general',
    order: 4,
    relatedChunks: ['existence-3', 'existence-5'],
  },
  {
    id: 'existence-5',
    title:
      'Reflection vs posited II: scope of commentary vs moments of the fact itself',
    lineRange: { start: 56, end: 92 },
    description:
      'Only that which is posited in a concept belongs in the course of the elaboration of the latter to its content.',
    keyPoints: [
      'Only what is posited in concept belongs to its content',
      'Unposited determinateness belongs to our reflection (ancillary)',
      'Reflection: for clarification or anticipation',
    
    ],
    status: 'pending',
    section: 'a. Existence in general',
    order: 5,
    relatedChunks: ['existence-4', 'existence-6'],
  },
  {
    id: 'existence-6',
    title:
      'Correspondence: being (indeterminate) vs existence (determinate being, concrete)',
    lineRange: { start: 94, end: 100 },
    description:
      'Existence corresponds to being in the preceding sphere. But being is the indeterminate;…',
    keyPoints: [
      'Existence corresponds to being in preceding sphere',
      'Being is indeterminate; no determinations transpire in it',
      'Existence is determinate being, something concrete',
    
    ],
    status: 'pending',
    section: 'a. Existence in general',
    order: 6,
    relatedChunks: ['existence-5', 'existence-7'],
  },
  {
    id: 'existence-7',
    title:
      'Quality I: immediacy of unity (existent ↔ non-being); no posited differentiation',
    lineRange: { start: 104, end: 119 },
    description:
      'On account of the immediacy with which being and nothing are one in existence, neither oversteps the other; to the extent that existence is existent, to that extent it is non-being;…',
    keyPoints: [
      'Immediacy: being and nothing one in existence',
      'Neither oversteps the other',
      'To the extent existent, to that extent non-being',
    
    ],
    status: 'pending',
    section: 'b. Quality',
    order: 7,
    relatedChunks: ['existence-6', 'existence-8', 'existence-1'],
  },
  {
    id: 'existence-8',
    title:
      'Quality II: definition (existent determinateness; simple, immediate)',
    lineRange: { start: 121, end: 129 },
    description:
      'Determinateness thus isolated by itself, as existent determinateness, is quality: something totally simple, immediate.',
    keyPoints: [
      'Determinateness isolated by itself, as existent determinateness, is quality',
      'Totally simple, immediate',
      'Determinateness in general more universal (can further determine as quantity)',
    
    ],
    status: 'pending',
    section: 'b. Quality',
    order: 8,
    relatedChunks: ['existence-7', 'existence-9'],
  },
  {
    id: 'existence-9',
    title: 'Quality III: reality and negation; reflection; limit/restriction',
    lineRange: { start: 131, end: 143 },
    description:
      'Existence, however, in which nothing and being are equally contained, is itself the measure of the one-sidedness of quality as an only immediate or existent determinateness.',
    keyPoints: [
      'Existence measures one-sidedness of quality as mere immediacy',
      'Quality equally posited in determination of nothing',
      'Result: immediate determinateness posited as distinct, reflected',
    
    ],
    status: 'pending',
    section: 'b. Quality',
    order: 9,
    relatedChunks: ['existence-8', 'existence-10'],
  },
  {
    id: 'existence-10',
    title: 'Quality IV: valuation of reality vs negation (both are existence)',
    lineRange: { start: 145, end: 155 },
    description:
      'Both are an existence, but in reality, as quality with the accent on being an existent, that it is determinateness and hence also negation is concealed;…',
    keyPoints: [
      'Both are existence',
      'Reality: accent on being an existent — conceals its determinateness/negation',
      'Reality appears as pure positive (negating, restriction, lack excluded)',
    
    ],
    status: 'pending',
    section: 'b. Quality',
    order: 10,
    relatedChunks: ['existence-9', 'existence-11'],
  },
  {
    id: 'existence-11',
    title:
      'Something I: reality ∧ negation as existences; quality unseparated from existence',
    lineRange: { start: 159, end: 171 },
    description:
      'In existence its determinateness has been distinguished as quality; in this quality as something existing, the distinction exists:…',
    keyPoints: [
      'Determinateness distinguished as quality',
      'Distinction exists: reality and negation',
      'Distinctions present in existence, yet null and sublated',
    
    ],
    status: 'pending',
    section: 'c. Something',
    order: 11,
    relatedChunks: ['existence-10', 'existence-12', 'existence-1'],
  },
  {
    id: 'existence-12',
    title:
      'Something II: sublation (not omission) → simplicity mediated → being-in-itself',
    lineRange: { start: 173, end: 187 },
    description:
      "This sublating of the distinction is more than the mere retraction and external re-omission of it, or a simple return to the simple beginning, to existence as such. The distinction cannot be left out, for it is. Therefore, what de facto is at hand is this: existence in general, distinction in it, and the sublation of this distinction; the existence, not void of distinctions as at the beginning, but as again self-equal through the sublation of the distinction; the simplicity of existence mediated through this sublation. This state of sublation of the distinction is existence's own determinateness; existence is thus being-in-itself; it is existent, something.",
    keyPoints: [
      'Sublation more than retraction/external re-omission',
      'Not simple return to beginning',
      'Distinction cannot be left out, for it is',
    
    ],
    status: 'pending',
    section: 'c. Something',
    order: 12,
    relatedChunks: ['existence-11', 'existence-13'],
  },
  {
    id: 'existence-13',
    title: 'Something III: negation of negation; beginning of the subject',
    lineRange: { start: 189, end: 222 },
    description:
      'Something is the first negation of negation, as simple existent self-reference. Existence, life, thought, and so forth, essentially take on the determination of an existent being, a living thing, a thinking mind ("I"), and so forth.',
    keyPoints: [
      'Something = first negation of negation (simple existent self-reference)',
      'Existence, life, thought take on determination of existent being, living thing, thinking mind ("I")',
      'Important: not halt at generalities (Godhood vs God)',
    
    ],
    status: 'pending',
    section: 'c. Something',
    order: 13,
    relatedChunks: ['existence-12', 'existence-14'],
  },
  {
    id: 'existence-14',
    title: 'Something IV: mediation-with-itself (vs alleged bare immediacy)',
    lineRange: { start: 224, end: 244 },
    description:
      'Something is an existent as the negation of negation, for such a negation is the restoration of the simple reference to itself; but the something is thereby equally the mediation of itself with itself.',
    keyPoints: [
      'Something as negation of negation = restoration of self-reference',
      'But also mediation of itself with itself',
      'Present in simplicity of something, then in being-for-itself, subject, etc.',
    
    ],
    status: 'pending',
    section: 'c. Something',
    order: 14,
    relatedChunks: ['existence-13', 'existence-15'],
  },
  {
    id: 'existence-15',
    title:
      'Something V: mediation-only collapses to unity; becoming with moments as existents (other)',
    lineRange: { start: 246, end: 257 },
    description:
      'This mediation with itself which something is in itself, when taken only as the negation of negation, has no concrete determinations for its sides; thus it collapses into the simple unity which is being.',
    keyPoints: [
      'Mediation-with-itself, taken only as negation of negation, has no concrete determinations for sides',
      'Collapses into simple unity which is being',
      'Something is, and is therefore also an existent',
    
    ],
    status: 'pending',
    section: 'c. Something',
    order: 15,
    relatedChunks: ['existence-14', 'existence-16'],
  },
  {
    id: 'existence-16',
    title:
      'Something VI: alteration; initially only in concept; other as qualitative',
    lineRange: { start: 258, end: 267 },
    description:
      'As becoming, something is a transition, the moments of which are themselves something, and for that reason it is an alteration, a becoming that has already become concrete. At first, however, something alters only in its concept;…',
    keyPoints: [
      'As becoming, something is transition',
      'Moments are themselves something',
      'Therefore it is alteration (concrete becoming)',
    
    ],
    status: 'pending',
    section: 'c. Something',
    order: 16,
    relatedChunks: ['existence-15'],
  },
];
