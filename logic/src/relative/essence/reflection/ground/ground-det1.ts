import type { Chunk, LogicalOperation } from './index';

// Canonical chunks for "B. DETERMINATE GROUND — b. Real ground"
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'dg1-chunk-1-intro-real-ground',
    title: 'Real ground — diversity of content and realization',
    text: `When the ground and the grounded possess diverse contents the ground-connection ceases to be merely formal: asking for a ground now demands a different content-determination. This diversity realizes the ground — the turning-back is no longer tautological.`,
  },
  {
    id: 'dg1-chunk-2-two-immediates-and-indifference',
    title: 'Two immediates, mutual indifference, and empty unity',
    text: `As the two sides acquire different contents they become immediate and self-identical, indifferent to one another. Their unity is a negative unity that, being an empty external reference, holds them together without being a genuine mediation.`,
  },
  {
    id: 'dg1-chunk-3-continuous-extension-and-essential-compactness',
    title: 'Ground-content extends into positedness — essential compactness',
    text: `In one respect the ground-content extends into the positedness so fully that the grounded contains the ground within itself — a compact essential identity. Anything beyond this essential element in the grounded is unessential, an external manifold not of the ground.`,
  },
  {
    id: 'dg1-chunk-4-unessential-manifold-and-substrate',
    title: 'Unessential manifold as indifferent substrate',
    text: `The unessential manifold present in the grounded is a positive, indifferent substrate. It resides within the grounded but does not posit itself as grounded content; it is external to the ground-connection and therefore not grounded by it.`,
  },
  {
    id: 'dg1-chunk-5-connection-as-external-tie',
    title: 'Connection becomes external tie not true mediation',
    text: `The tie that links distinct contents is not a form-reference holding positedness in place; rather it is an external bond that fails to make the unessential manifold truly posited. The real ground thus breaks into different substrates and external determinations.`,
  },
  {
    id: 'dg1-chunk-6-loss-of-self-identity-of-ground-connection',
    title: 'Self-identity of ground vanishes; ground-connection externalized',
    text: `Because of content diversity the self-identical form of the ground (one thing alternating essential and posited) disappears. The ground-connection becomes external to itself and no longer contains its mediation immanently.`,
  },
  {
    id: 'dg1-chunk-7-conclusion-real-ground-as-reference-to-another',
    title: 'Conclusion: real ground = reference to another content',
    text: `The real ground is an external reference: a content-determination refers to another content, and the form (ground-connection) itself refers to something immediate not posited by it. Real ground is thus grounded by a further, distinct content.`,
  },
];

// HLO — Logical operations derived from "Real ground"
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'dg1-op-1-intro-real-ground',
    chunkId: 'dg1-chunk-1-intro-real-ground',
    label:
      'Diverse contents → ground realized; demand another content as ground',
    clauses: [
      'if ground.content != grounded.content then ground.isRealized',
      'askingForGround => demand(differentContent)',
    ],
    predicates: [
      { name: 'IsRealized', args: ['ground'] },
      { name: 'DemandsDifferentContent', args: ['query'] },
    ],
    relations: [
      {
        predicate: 'requires',
        from: 'requestForGround',
        to: 'differentContent',
      },
    ],
  },

  {
    id: 'dg1-op-2-indifferent-immediates-empty-unity',
    chunkId: 'dg1-chunk-2-two-immediates-and-indifference',
    label: 'Different contents → immediates indifferent; unity empty reference',
    clauses: [
      'sideA.isImmediate && sideB.isImmediate',
      'sideA.indifferentTo(sideB)',
      'unity = emptyReference(sideA, sideB)',
    ],
    predicates: [
      { name: 'IsImmediate', args: ['side'] },
      { name: 'IsEmptyUnity', args: ['unity'] },
    ],
    relations: [{ predicate: 'holdsExternally', from: 'unity', to: 'sides' }],
  },

  {
    id: 'dg1-op-3-compactness-and-unessential',
    chunkId: 'dg1-chunk-3-continuous-extension-and-essential-compactness',
    label:
      'Ground-content extends into positedness; unessential manifold external',
    clauses: [
      'ground.content.extendsInto(positedness)',
      'grounded.contains(groundContentFully)',
      'otherElementsInGrounded = unessentialManifold (external)',
    ],
    predicates: [
      { name: 'ExtendsInto', args: ['groundContent', 'positedness'] },
      { name: 'IsUnessentialManifold', args: ['elements'] },
    ],
    relations: [
      { predicate: 'containsFully', from: 'grounded', to: 'groundContent' },
      {
        predicate: 'isExternalTo',
        from: 'unessentialManifold',
        to: 'groundConnection',
      },
    ],
  },

  {
    id: 'dg1-op-4-unessential-substrate',
    chunkId: 'dg1-chunk-4-unessential-manifold-and-substrate',
    label:
      'Unessential manifold = indifferent substrate, not posited by ground',
    clauses: [
      'unessential.isPositiveIndifferentElement',
      'unessential.doesNotPositItselfAsGrounded',
      'unessential.remaining => substrateOnly',
    ],
    predicates: [
      { name: 'IsIndifferentSubstrate', args: ['unessential'] },
      { name: 'NotPositedByGround', args: ['unessential'] },
    ],
    relations: [
      { predicate: 'residesIn', from: 'unessential', to: 'grounded' },
      { predicate: 'notPositedBy', from: 'unessential', to: 'ground' },
    ],
  },

  {
    id: 'dg1-op-5-connection-external-tie',
    chunkId: 'dg1-chunk-5-connection-as-external-tie',
    label:
      'Connection becomes external tie; real ground breaks into substrates',
    clauses: [
      'connection = externalTie(distinctContents)',
      'externalTie.doesNotPosit(unessentialManifold)',
      'result => differentSubstratesExist',
    ],
    predicates: [
      { name: 'IsExternalTie', args: ['connection'] },
      { name: 'BreaksIntoSubstrates', args: ['ground'] },
    ],
    relations: [
      {
        predicate: 'bindsExternally',
        from: 'connection',
        to: 'distinctContents',
      },
      { predicate: 'yields', from: 'externalTie', to: 'multipleSubstrates' },
    ],
  },

  {
    id: 'dg1-op-6-loss-self-identity',
    chunkId: 'dg1-chunk-6-loss-of-self-identity-of-ground-connection',
    label:
      'Diverse content → loss of self-identity; ground-connection externalized',
    clauses: [
      'if contents.diverse then selfIdenticalForm.vanishes',
      'groundConnection.externalized = true',
    ],
    predicates: [
      { name: 'Vanishes', args: ['selfIdenticalForm'] },
      { name: 'IsExternalized', args: ['groundConnection'] },
    ],
    relations: [
      { predicate: 'becomes', from: 'selfIdenticalForm', to: 'vanishedState' },
      { predicate: 'externalizes', from: 'groundConnection', to: 'itself' },
    ],
  },

  {
    id: 'dg1-op-7-real-ground-is-reference',
    chunkId: 'dg1-chunk-7-conclusion-real-ground-as-reference-to-another',
    label: 'Real ground = reference to another content or immediate',
    clauses: [
      'realGround = reference(contentA -> contentB)',
      'formRefersTo(immediateNotPositedByIt) => groundIsExternalReference',
    ],
    predicates: [
      { name: 'IsReferenceToAnother', args: ['realGround'] },
      { name: 'RefersToImmediate', args: ['form'] },
    ],
    relations: [
      { predicate: 'refers', from: 'realGround', to: 'otherContent' },
      { predicate: 'groundsByReference', from: 'form', to: 'immediate' },
    ],
  },
];
