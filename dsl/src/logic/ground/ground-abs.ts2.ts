import type { Chunk, LogicalOperation } from './index';

// Canonical chunks for "A. ABSOLUTE GROUND — c. Form and content"
export const CANONICAL_CHUNKS: Chunk[] = [
  {
    id: 'agc-chunk-1-form-stance-over-essence',
    title: 'Form stands over against essence — ground-connection in general',
    text: `Form initially stands over against essence: it is the ground-connection in general and its determinations articulate ground and grounded. As such form also stands over against matter and finally against content, taking each formerly self-identical term under its dominion as one of its determinations.`,
  },
  {
    id: 'agc-chunk-2-content-as-unity-of-form-and-matter',
    title:
      'Content = unity of form and matter; content stands over against form',
    text: `Content is the unity of a form and a matter that belong to it essentially. Because this unity is determinate (posited), content stands over against form, making form appear unessential; content therefore contains both a form and matter as its substrate while regarding them as mere positedness.`,
  },
  {
    id: 'agc-chunk-3-content-identity-and-ground-connection',
    title:
      'Content identical in form & matter; ground-connection returns in content',
    text: `Content is what is identical in form and matter: their returned unity. This identity is both indifferent to form and, simultaneously, the identity of ground. The ground disappears into content while content is the negative reflection of form-determinations into themselves so that content bears the ground-connection as its essential form.`,
  },
  {
    id: 'agc-chunk-4-content-of-ground-and-informed-identity',
    title: 'Content of ground = informed identity; content informs matter',
    text: `The content of the ground is the ground returned into unity with itself: informed identity. As such the formerly indeterminate matter becomes informed; the determinations of form acquire a material, indifferent subsistence within content. Content is thus both the self-identity of ground and a posited identity against the ground-connection.`,
  },
  {
    id: 'agc-chunk-5-positedness-forms-and-total-positedness',
    title:
      'Positedness and twofold form: total positedness vs immediate positedness',
    text: `Within content positedness appears as a determination of form that stands over against the form-as-total-connection. One form is the total positedness returning into itself; the other is mere immediate positedness or determinateness as such. Content thereby distinguishes forms while remaining their substrate.`,
  },
  {
    id: 'agc-chunk-6-twofold-determinateness-and-conclusion',
    title:
      'Twofold determinateness: form external to content and content’s determinateness',
    text: `The ground has become a determinate ground in general and this determinateness is twofold: (1) form's determinateness as external to content (content remains indifferent), and (2) the determinateness of the content itself that the ground possesses — closing the synthesis of form, matter and content.`,
  },
];

// HLO — Logical operations derived from "Form and content"
export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  {
    id: 'agc-op-1-form-stance',
    chunkId: 'agc-chunk-1-form-stance-over-essence',
    label: 'Form as ground-connection; determinations = ground + grounded',
    clauses: [
      'form.standsOver(essence)',
      'form.isGroundConnection()',
      'form.determinations = {ground, grounded}',
    ],
    predicates: [
      { name: 'StandsOver', args: ['form', 'essence'] },
      { name: 'IsGroundConnection', args: ['form'] },
    ],
    relations: [
      { predicate: 'determines', from: 'form', to: 'ground+grounded' },
    ],
  },

  {
    id: 'agc-op-2-content-unity',
    chunkId: 'agc-chunk-2-content-as-unity-of-form-and-matter',
    label: 'Content = unity(form, matter) ; content stands over against form',
    clauses: [
      'content = unity(form, matter)',
      'content.isDeterminate => content.standsOver(form)',
      'form = unessentialAgainst(content)',
    ],
    predicates: [
      { name: 'IsUnityOf', args: ['content', 'form+matter'] },
      { name: 'StandsOver', args: ['content', 'form'] },
    ],
    relations: [
      { predicate: 'composedOf', from: 'content', to: 'form+matter' },
      { predicate: 'contrasts', from: 'content', to: 'form' },
    ],
  },

  {
    id: 'agc-op-3-content-identity-ground-connection',
    chunkId: 'agc-chunk-3-content-identity-and-ground-connection',
    label:
      'Content identity = indifferent to form and identity of ground; ground-connection in content',
    clauses: [
      'content.isIdentityOf(form, matter)',
      'content.indifferentTo(form) && content.isGroundIdentity',
      'ground.disappearsInto(content) && content.reflects(form.determinations)',
    ],
    predicates: [
      { name: 'IsIdentityOf', args: ['content', 'form+matter'] },
      { name: 'IsGroundIdentity', args: ['content'] },
    ],
    relations: [
      { predicate: 'reflects', from: 'content', to: 'form.determinations' },
      { predicate: 'absorbs', from: 'content', to: 'ground' },
    ],
  },

  {
    id: 'agc-op-4-informed-content',
    chunkId: 'agc-chunk-4-content-of-ground-and-informed-identity',
    label:
      'Content as informed matter; form determinations obtain material subsistence',
    clauses: [
      'content.informs(matter)',
      'form.determinations.obtainSubsistenceWithin(content)',
      'content = positedIdentityAgainst(groundConnection)',
    ],
    predicates: [
      { name: 'Informs', args: ['content', 'matter'] },
      {
        name: 'HasMaterialSubsistence',
        args: ['content', 'form.determinations'],
      },
    ],
    relations: [
      { predicate: 'informs', from: 'content', to: 'matter' },
      { predicate: 'sustains', from: 'content', to: 'form.determinations' },
    ],
  },

  {
    id: 'agc-op-5-positedness-forms',
    chunkId: 'agc-chunk-5-positedness-forms-and-total-positedness',
    label: 'Total positedness vs immediate positedness; two forms in content',
    clauses: [
      'form_total = totalPositednessReturningIntoItself',
      'form_immediate = immediatePositedness (determinateness)',
      'content.distinguishes(form_total, form_immediate)',
    ],
    predicates: [
      { name: 'IsTotalPositedness', args: ['form_total'] },
      { name: 'IsImmediatePositedness', args: ['form_immediate'] },
    ],
    relations: [
      {
        predicate: 'distinguishes',
        from: 'content',
        to: 'form_total+form_immediate',
      },
    ],
  },

  {
    id: 'agc-op-6-twofold-determinateness',
    chunkId: 'agc-chunk-6-twofold-determinateness-and-conclusion',
    label:
      'Twofold determinateness: externality of form / determinateness of content',
    clauses: [
      'ground.becomesDeterminateGround()',
      'determinateness = {formExternalToContent, contentDeterminateness}',
      'synthesis.closes(form, matter, content)',
    ],
    predicates: [
      { name: 'IsDeterminateGround', args: ['ground'] },
      { name: 'HasTwofoldDeterminateness', args: ['determinateness'] },
    ],
    relations: [
      { predicate: 'yields', from: 'ground', to: 'determinateness' },
      { predicate: 'synthesizes', from: 'form+matter', to: 'content' },
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
