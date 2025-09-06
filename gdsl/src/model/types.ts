export type Id = string

export type Unit = { id: Id; name: string; symbol?: string; dimension?: string }
export type Variable = { id: Id; name: string; desc?: string; unit?: Id; domain?: 'number'|'string'|'boolean'|'json' }
export type Instrument = { id: Id; name: string; kind?: string; meta?: Record<string, unknown> }

export type Subject = { id: Id; kind?: string; props?: Record<string, unknown> }
export type Context = { id: Id; when?: string; where?: string; meta?: Record<string, unknown> }
export type Observation = { id: Id; subject: Id; context?: Id; at?: string; by?: Id } // by=Instrument

export type Measurement = {
  id: Id; variable: Id; observation: Id; value: unknown;
  unit?: Id; uncertainty?: number; method?: string
}

export type Dataset = { id: Id; title?: string; items: Id[]; meta?: Record<string, unknown> }

export type Hypothesis = { id: Id; title: string; desc?: string }
export type Claim = { id: Id; hypothesis: Id; statement: string }
export type Protocol = { id: Id; title: string; steps?: string[]; meta?: Record<string, unknown> }
export type Experiment = { id: Id; protocol: Id; inputs?: Record<string, unknown>; startedAt?: string; meta?: Record<string, unknown> }
export type Result = { id: Id; experiment: Id; outputs?: Record<string, unknown>; metrics?: Record<string, number>; finishedAt?: string }
export type EvidenceLink = { id: Id; claim: Id; supports: boolean; artifact: Id; note?: string }

// --- Projection / DSL artifact types (bootstrap for Projection tool) ---
export type NodeRow = {
  id: string
  labels?: string[]           // node labels (e.g., ['HLO'], ['Term'])
  props?: Record<string, unknown>
}

export type EdgeRow = {
  id?: string
  type: string                // edge type, prefer UPPER_SNAKE (e.g., 'CHUNK_HAS_HLO', 'WITNESS_EQUALS')
  from: string                // from node id
  to: string                  // to node id
  props?: Record<string, unknown>
}

export type ClauseRow = {
  id: string
  hloId: string
  raw: string
  kind: 'assert' | 'tag' | 'annotate' | 'unknown'
}

export type Token = {
  id: string                  // token:<symbol>
  token: string               // symbol name (camelCase)
  arity?: number
  doc?: string
}

export type Term = {
  id: string                  // canonical term id (e.g., 'Being')
  label?: string
  aliases?: string[]
  notes?: string
}

export type Chunk = {
  id: string
  title?: string
  text?: string
  summary?: string
  source?: string
}

export type HLO = {
  id: string
  chunkId: string
  label?: string
  digest?: string | string[]
  clauses?: string[]
  tokens?: string[]           // optional, derived or declared
  witnessEdges?: { type: string; from: string; to: string; props?: Record<string, unknown> }[]
}

export type TopicMap = {
  id: string
  title?: string
  description?: string
  terms: Term[]
  signatures: Record<string, { token: string; weight: number }[]> // termId -> signature
  provenance?: Record<string, unknown>
}

export type DatasetManifest = {
  id: string
  title?: string
  provenance?: Record<string, unknown>
  chunks?: Array<{ id: string; source?: string }>
  hlos?: Array<Partial<HLO>>
  terms?: Term[]
  signatureTokens?: string[]
}

// Artifact produced by ingest (JSON-first)
export type GraphArtifact = {
  dataset: string
  nodes: NodeRow[]
  edges: EdgeRow[]
  clauses?: ClauseRow[]
  tokens?: string[]
  terms?: Term[]
  counts?: { chunks?: number; hlos?: number; clauses?: number; tokens?: number }
}
