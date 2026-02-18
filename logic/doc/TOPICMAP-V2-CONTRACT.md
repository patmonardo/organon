# TopicMap V2 Contract (Source-First, KG-Ready)

Status: draft for working use
Scope: analysis artifacts under logic/src/relative/\*\*/sources
Primary objective: improve semantic quality of source analysis before any generation

## Why V2 exists

V1 is good at chunk planning and generation bootstrap, but weak at claim semantics and evidence traceability.

V2 makes source analysis explicit and reviewable by separating:

- Text segmentation (what span is being analyzed)
- Analytical claims (what is being asserted)
- Evidence anchors (why the claim is justified)
- Inter-entry relations (how chunks connect dialectically)

## Design principles

- Source-first: source correctness is the product; generation is an adapter.
- Minimal ontology lock-in: analysis should survive emitter rewrites.
- Explicit evidence: every non-trivial claim should point to text anchors.
- Typed relations: avoid free-form relation strings where possible.
- Non-breaking migration: V1 entries can be normalized into V2.

## Core V2 data model

```ts
export type TopicMapStatus =
  | 'draft'
  | 'review_pending'
  | 'reviewed'
  | 'approved'
  | 'rejected';

export type EpistemicModality =
  | 'asserted'
  | 'inferred'
  | 'interpretive'
  | 'questionable';

export type RelationType =
  | 'supports'
  | 'contrasts'
  | 'negates'
  | 'sublates'
  | 'presupposes'
  | 'refines'
  | 'transitions_to';

export type EvidenceAnchor = {
  sourceFile: string;
  lineStart: number;
  lineEnd: number;
  quote?: string;
  rationale?: string;
};

export type TopicClaim = {
  id: string;
  subject: string;
  predicate: string;
  object: string;
  polarity?: 'positive' | 'negative' | 'mixed';
  modality?: EpistemicModality;
  confidence?: number; // 0..1
  evidence: EvidenceAnchor[];
};

export type TopicRelation = {
  type: RelationType;
  targetEntryId: string;
  note?: string;
  confidence?: number; // 0..1
};

export type TopicMapEntryV2 = {
  id: string;
  title: string;
  span: {
    sourceFile: string;
    lineStart: number;
    lineEnd: number;
  };
  summary: string; // one-sentence analytic thesis
  keyPoints: string[]; // editor notes, concise, non-duplicative
  claims: TopicClaim[];
  relations: TopicRelation[];
  tags?: string[];
  section?: string;
  order?: number;
  status?: TopicMapStatus;
  review?: {
    reviewer?: string;
    reviewedAt?: string;
    notes?: string;
    decision?: 'accept' | 'revise' | 'reject';
  };
};
```

## Required invariants

Each V2 entry must satisfy:

1. span.lineStart <= span.lineEnd and both >= 1.
2. summary is a single sentence and non-empty.
3. claims.length >= 1 for non-trivial entries.
4. each claim has at least one evidence anchor.
5. relation targets resolve to existing entry ids in the same map.
6. keyPoints are unique within the entry (case-insensitive normalization).
7. confidence values are in [0,1] when present.

## What changed from V1

V1 fields retained conceptually:

- id
- title
- keyPoints
- section
- order
- status

V1 fields upgraded:

- lineRange -> span with sourceFile included
- description -> summary (thesis sentence)
- relatedChunks -> relations[] with explicit type

V2 additions:

- claims[] for proposition-level analysis
- evidence[] under each claim
- optional review metadata

## Migration strategy

Phase 1: parallel schemas

- Keep existing TopicMapEntry (V1).
- Introduce TopicMapEntryV2 and normalizer.

Phase 2: normalization at boundary

- Generators consume normalized V2 internal representation.
- Legacy V1 files are auto-upgraded through adapter defaults.

Phase 3: strict mode

- Enable strict validation for Essence first.
- Block generation when required V2 invariants fail.

## Essence-first workflow

For each section (one file or one subpart at a time):

1. Confirm span boundaries against source text.
2. Write one summary sentence.
3. Extract 3-8 non-duplicative key points.
4. Encode 1-3 explicit claims.
5. Attach evidence anchors for each claim.
6. Add only necessary typed relations.
7. Mark status as review_pending.
8. Review together and decide accept/revise.

## Definition of close enough

A TopicMapEntryV2 is close enough to proceed if:

- no structural invariant fails,
- claims are text-grounded,
- key points are not redundant,
- relation types are explicit,
- reviewer can explain the entry in under one minute.

## Non-goals

- Perfect philosophical consensus in one pass.
- Full ontology completion before source cleanup.
- Immediate refactor of all generators.

## Next implementation targets

1. Add V2 type and validator in schema.
2. Build normalizeTopicMapEntry(v1|v2) adapter.
3. Pilot on Essence Reflection only.
4. Compare generated outputs before broad migration.
