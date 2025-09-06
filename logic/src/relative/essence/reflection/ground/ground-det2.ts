import type { Chunk, LogicalOperation, Predicate, Relation } from './index';

// Canonical chunks for "B. DETERMINATE GROUND — c. Complete ground"
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'dg2-chunk-1-complete-ground-intro',
    title: 'Complete ground — introduction',
    text: `Real ground returns to its own ground: the previously posited, external link is sublated into a new ground that contains both formal and real moments. The complete ground unifies the formal and the real, mediating content determinations that formerly confronted each other immediately.`,
  },
  {
    id: 'dg2-chunk-2-two-connections-and-new-ground',
    title: 'Two connections; new ground contains same contents and linkages',
    text: `When a posited link is grounded, the new ground is determined so that it is identical with the ground it grounds: both sides share the same content determinations and their linkage is present in the new ground as the immanent reflection of that link.`,
  },
  {
    id: 'dg2-chunk-3-formality-reasserts-and-completeness',
    title: 'Formality reasserts; complete ground contains formal + real',
    text: `Because real ground returns to its ground, the formality of ground reasserts itself; the newly arisen ground-connection is complete, containing within itself both the formal and the real ground and mediating the previously immediate confrontations.`,
  },
  {
    id: 'dg2-chunk-4-something-has-a-ground-two-determinations',
    title:
      'Something has a ground — contains ground-determination and posited determination',
    text: `A something that has a ground contains the content-determination that is ground and a second determination posited by that ground. Due to content indifference, neither determination alone constitutes ground in itself; the connection is posited and has its ground in another connection.`,
  },
  {
    id: 'dg2-chunk-5-general-linking-and-relative-ground',
    title: 'General linking; relative ground vs true absolute connection',
    text: `The second connection is distinguished only by form and shares the same contents; the linking is general and produces content diversification indifferent to each other. This linking is not the true absolute connection but a relative ground supported by a something that connects them immediately.`,
  },
  {
    id: 'dg2-chunk-6-two-somethings-andKindsOfConnection',
    title:
      'Two somethings: distinct connections of content with identical whole',
    text: `There are two distinct somethings — two connections of content — that form one whole content (two determinations + their connection). They differ solely by the kind of connection (immediate vs posited); thus ground and grounded are distinguished only according to form.`,
  },
  {
    id: 'dg2-chunk-7-formal-to-real-transition-and-self-subsistence',
    title:
      'Formal → Real transition; one content becomes self-subsistent and grounded',
    text: `Formal ground passes into real ground as the moments reflect into themselves and become self-subsistent content. One content determination becomes the essential, identical substrate and ground of the other, which is posited within the second something.`,
  },
  {
    id: 'dg2-chunk-8-mediation-through-first-something',
    title: 'Mediation through the first something; ground-of-ground inference',
    text: `Because B is implicitly linked with A in a first something, in a second something where only A is immediate, B is nonetheless linked to it via the original connection. Thus the connection in the first something mediates and becomes the ground of the ground A in the second something.`,
  },
  {
    id: 'dg2-chunk-9-complete-ground-as-self-external-reflection',
    title:
      'Complete ground = self-external reflection; identity restored via negation',
    text: `Real ground is the self-external reflection of ground; its complete mediation restores identity with itself. The ground-connection thus mediates itself through negation: it is simultaneously self-positing and self-sublating, connecting immediate determinations with their negations.`,
  },
  {
    id: 'dg2-chunk-10-conditioning-mediation-and-conclusion',
    title: 'Ground as conditioning mediation; conclusion',
    text: `The total ground-connection presupposes reflection: formal ground presupposes immediate content and that content presupposes form as real ground. Ground is a linkage that refers to immediacy while also referring itself as to another — in short, the complete ground is conditioning mediation.`,
  },
];

// HLO — Logical operations derived from "Complete ground"
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'dg2-op-1-complete-intro',
    chunkId: 'dg2-chunk-1-complete-ground-intro',
    label:
      'CompleteGround := formal + real moments united; mediates confronted contents',
    clauses: [
      'completeGround.contains(formalGround, realGround)',
      'completeGround.mediates(contentDeterminations)',
    ],
    predicates: [
      { name: 'ContainsMoments', args: ['completeGround', 'formal+real'] },
      { name: 'Mediates', args: ['completeGround', 'contents'] },
    ],
    relations: [
      { predicate: 'contains', from: 'completeGround', to: 'formal+real' },
      {
        predicate: 'mediates',
        from: 'completeGround',
        to: 'contentDeterminations',
      },
    ],
  },

  {
    id: 'dg2-op-2-two-connections-new-ground',
    chunkId: 'dg2-chunk-2-two-connections-and-new-ground',
    label: 'New ground mirrors contents and linkage; immanent reflection',
    clauses: [
      'newGround.identicalWith(previousGround).inContentAndLinkage',
      'newGround.linkage = immanentReflection(previousLink)',
    ],
    predicates: [
      { name: 'IsIdenticalWith', args: ['newGround', 'previousGround'] },
      { name: 'IsImmanentReflection', args: ['linkage'] },
    ],
    relations: [
      { predicate: 'mirrors', from: 'newGround', to: 'previousContents+link' },
    ],
  },

  {
    id: 'dg2-op-3-formality-reasserts',
    chunkId: 'dg2-chunk-3-formality-reasserts-and-completeness',
    label:
      'Return to ground => formality reasserts; complete ground contains both forms',
    clauses: [
      'if realGround.returnsToGround then formality.reasserts',
      'completeGround.includes(formal, real)',
    ],
    predicates: [
      { name: 'ReassertsFormality', args: ['process'] },
      { name: 'Includes', args: ['completeGround', 'moments'] },
    ],
    relations: [
      { predicate: 'includes', from: 'completeGround', to: 'formal+real' },
    ],
  },

  {
    id: 'dg2-op-4-something-has-ground',
    chunkId: 'dg2-chunk-4-something-has-a-ground-two-determinations',
    label:
      'Something.hasGround => contains groundDetermination + positedDetermination; ground in another connection',
    clauses: [
      'something.contains(groundDetermination, positedDetermination)',
      'connectionOfThese.presupposes(anotherConnection)',
    ],
    predicates: [
      { name: 'ContainsDeterminations', args: ['something'] },
      { name: 'PresupposesAnotherConnection', args: ['connection'] },
    ],
    relations: [
      { predicate: 'contains', from: 'something', to: 'determinations' },
      { predicate: 'presupposes', from: 'connection', to: 'anotherConnection' },
    ],
  },

  {
    id: 'dg2-op-5-general-linking-relative-ground',
    chunkId: 'dg2-chunk-5-general-linking-and-relative-ground',
    label:
      'General linking yields relative ground; contents remain indifferent',
    clauses: [
      'linking.isGeneral && contents.indifferent => ground.isRelative',
      'relativeGround.supportedBy(something) not trueAbsoluteConnection',
    ],
    predicates: [
      { name: 'IsRelativeGround', args: ['link'] },
      { name: 'IsGeneralLinking', args: ['link'] },
    ],
    relations: [
      { predicate: 'supports', from: 'relativeGround', to: 'contents' },
    ],
  },

  {
    id: 'dg2-op-6-two-somethings',
    chunkId: 'dg2-chunk-6-two-somethings-andKindsOfConnection',
    label:
      'Two somethings: same whole content, different connection kinds (immediate vs posited)',
    clauses: [
      'twoSomethings.share(wholeContent)',
      'distinction := kindOfConnection (immediate | posited)',
    ],
    predicates: [
      { name: 'SharesWholeContent', args: ['somethings'] },
      { name: 'KindOfConnection', args: ['connection'] },
    ],
    relations: [
      {
        predicate: 'distinguishedBy',
        from: 'somethings',
        to: 'connectionKind',
      },
    ],
  },

  {
    id: 'dg2-op-7-formal-to-real-transition',
    chunkId: 'dg2-chunk-7-formal-to-real-transition-and-self-subsistence',
    label:
      'Formal → Real: moments reflect into self-subsistence; one content becomes essential ground',
    clauses: [
      'formalGround.transformsInto(realGround) via reflectionIntoSelf',
      'oneContent.becomes(essentialGround) for the other',
    ],
    predicates: [
      { name: 'TransformsInto', args: ['formal', 'real'] },
      { name: 'BecomesEssentialGround', args: ['content'] },
    ],
    relations: [
      { predicate: 'grounds', from: 'essentialContent', to: 'otherContent' },
    ],
  },

  {
    id: 'dg2-op-8-mediation-through-first',
    chunkId: 'dg2-chunk-8-mediation-through-first-something',
    label:
      'Mediation: original connection in first something grounds the second',
    clauses: [
      'if B.linkedWith(A) in firstSomething then B.linkedWith(A) in secondSomething',
      'firstConnection.mediates(secondConnection) => groundOfGroundInference',
    ],
    predicates: [
      { name: 'IsMediatedBy', args: ['second', 'first'] },
      { name: 'GroundOfGround', args: ['inference'] },
    ],
    relations: [
      {
        predicate: 'mediates',
        from: 'firstConnection',
        to: 'secondConnection',
      },
    ],
  },

  {
    id: 'dg2-op-9-complete-as-self-external',
    chunkId: 'dg2-chunk-9-complete-ground-as-self-external-reflection',
    label:
      'CompleteGround = self-external reflection; self-positing & self-sublating',
    clauses: [
      'realGround = selfExternalReflection(ground)',
      'completeConnection.selfPosits && completeConnection.selfSublates',
    ],
    predicates: [
      { name: 'IsSelfExternalReflection', args: ['realGround'] },
      { name: 'SelfPositsAndSublates', args: ['connection'] },
    ],
    relations: [
      { predicate: 'reflects', from: 'completeGround', to: 'itself' },
    ],
  },

  {
    id: 'dg2-op-10-conditioning-mediation',
    chunkId: 'dg2-chunk-10-conditioning-mediation-and-conclusion',
    label:
      'Ground as conditioning mediation: formal presupposes immediate and vice versa',
    clauses: [
      'formalGround.presupposes(immediateContent)',
      'immediateContent.presupposes(form.asRealGround)',
      'groundConnection := conditioningMediation',
    ],
    predicates: [
      { name: 'PresupposesImmediate', args: ['formalGround'] },
      { name: 'IsConditioningMediation', args: ['ground'] },
    ],
    relations: [
      {
        predicate: 'presupposes',
        from: 'formalGround',
        to: 'immediateContent',
      },
      {
        predicate: 'conditions',
        from: 'groundConnection',
        to: 'contentDeterminations',
      },
    ],
  },
];

// Accessors
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null;
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS;
}

export function getLogicalOpsForChunk(
  oneBasedIndex: number,
): LogicalOperation | null {
  const chunk = getChunk(oneBasedIndex);
  if (!chunk) return null;
  return LOGICAL_OPERATIONS.find((op) => op.chunkId === chunk.id) ?? null;
}
