export type Scope = 'being-only' | 'essence' | 'idea';
export type LogosMode = 'prajna' | 'samskara' | 'nirodha' | 'jnana';
export type Synthesis = 'pre-factum' | 'post-factum';
export type Clause = string;

// Authoring/validation stance (per agent/process)
export type Policy = 'speculative' | 'curative';

export type CurationStatus = 'seed' | 'first-fruit' | 'heirloom';

// Provenance stamp (who/when/how a change/view was produced)
export interface Provenance {
  agentId: string;
  policy: Policy;
  at: string;        // ISO datetime
  tool?: string;     // e.g., 'dsl-compiler@0.1'
  notes?: string;
}

export interface Meta {
  status: CurationStatus; // lifecycle
  confidence?: number; // 0..1 rough curator confidence
  sources?: string[]; // bibliographic refs/links
  editor?: string; // curator id
  reviewedAt?: string; // ISO date
  notes?: string; // brief editorial note
  provenance?: Provenance[]; // append-only provenance trail
}

export interface Chunk {
  id: string;
  title: string;
  source: string; // close-text summary/key
  original?: string; // optional verbatim
  mode?: 'summary' | 'quote';
  meta?: Meta;
}

export interface Hlo {
  id: string;
  chunkId: string;
  label: string;
  clauses: Clause[];
  meta?: Meta;
}

export interface DatasetUnit {
  id: string;
  title: string;
  scope: string;
  logosMode: string;
  synthesis: string;
  faculty: string;
  lens: string;
  chunks: Chunk[];
  hlos: Hlo[];
  meta?: Meta;
}

// Add a dataset container over units
export interface Dataset {
  id: string;
  title: string;
  scope: Scope;
  logosMode: LogosMode;
  synthesis: Synthesis;
  faculty: string;
  lens: string;
  units: DatasetUnit[];
  meta?: Meta;
}

export const NS = 'reality:logos';

export function makeUnitId(sutra: string) {
  // sutra like 'i.42' or 'i.43'
  return `${NS}:ys:${sutra}`;
}

export function makeDatasetId(name: string) {
  return `${NS}:${name}`;
}

// Basic validator (now typed) + include all LogosMode variants
export function validateDataset(ds: Dataset) {
  if (!ds.id.startsWith(NS)) throw new Error('invalid namespace');
  const scopes: Scope[] = ['being-only', 'essence', 'idea'];
  const modes: LogosMode[] = ['prajna', 'samskara', 'nirodha', 'jnana'];
  const synths: Synthesis[] = ['pre-factum', 'post-factum'];
  if (!scopes.includes(ds.scope)) throw new Error('invalid scope');
  if (!modes.includes(ds.logosMode)) throw new Error('invalid logosMode');
  if (!synths.includes(ds.synthesis)) throw new Error('invalid synthesis');
  if (!Array.isArray(ds.units) || ds.units.length === 0) {
    throw new Error('dataset has no units');
  }
  // light sanity on units
  for (const u of ds.units) {
    if (!u.id.startsWith(NS)) throw new Error(`unit ${u.id} invalid namespace`);
    if (!u.chunks?.length) throw new Error(`unit ${u.id} missing chunks`);
    if (!u.hlos?.length) throw new Error(`unit ${u.id} missing hlos`);
  }
  return true;
}

export interface Term {
  id: string;
  label: string;
  aliases?: string[];
}

export interface WitnessEdge {
  type: string;
  from: string;
  to: string;
}

export interface View {
  id: string;
  title: string;
  terms: Term[];
  edges: WitnessEdge[];
  refs?: string[]; // optional: unit/chunk ids you’re bridging
  meta?: Meta;
}

export function validateView(v: View) {
  if (!v.id) throw new Error('view.id required');
  if (!v.title) throw new Error(`[${v.id}] title required`);
  if (!Array.isArray(v.terms)) throw new Error(`[${v.id}] terms must be array`);
  if (!Array.isArray(v.edges)) throw new Error(`[${v.id}] edges must be array`);
  return true;
}

// -------- Multi-Agent registry additions --------
export type AgentRole = 'seeder' | 'analyst' | 'curator' | 'compiler' | 'reviewer';
export interface Agent {
  id: string;       // e.g., 'agent:pat' or 'agent:copilot'
  name: string;
  role: AgentRole;
  policy: Policy;   // default stance for outputs
  capabilities?: string[]; // e.g., ['seed.chunks','append.hlos','compile.ir']
  contact?: string; // optional
  meta?: Meta;
}

// A Representation is an agent-authored perspective over resources
export interface Representation {
  id: string;                 // e.g., 'repr:ys:i.50:analysis@copilot'
  title: string;
  agentId: string;
  policy: Policy;             // speculative|curative for this artifact
  about: string[];            // ids referenced (units/chunks/hlos)
  kind: 'seed' | 'analysis' | 'curation' | 'bridge';
  refs?: string[];            // optional bibliographic ids
  meta?: Meta;
}

// Essential relation set (agent-declared, curated)
export type Necessity = 'essential' | 'accidental';
export interface EssentialRelation {
  id: string;                 // e.g., 'essrel:ys:i.50#unity-in-relation'
  from: string;               // id (unit/chunk/hlo/term)
  to: string;                 // id (unit/chunk/hlo/term)
  rel: string;                // e.g., 'unity-in-relation', 'grounds', 'inhibits'
  necessity: Necessity;
  agentId: string;
  meta?: Meta;
}

export function validateAgent(a: Agent) {
  if (!a.id || !a.name) throw new Error('agent id/name required');
  if (!['seeder','analyst','curator','compiler','reviewer'].includes(a.role)) {
    throw new Error(`[${a.id}] invalid role`);
  }
  if (!['speculative','curative'].includes(a.policy)) {
    throw new Error(`[${a.id}] invalid policy`);
  }
  return true;
}

export function validateRepresentation(r: Representation) {
  if (!r.id || !r.title) throw new Error('representation id/title required');
  if (!r.agentId) throw new Error(`[${r.id}] agentId required`);
  if (!['speculative','curative'].includes(r.policy)) throw new Error(`[${r.id}] invalid policy`);
  if (!Array.isArray(r.about) || r.about.length === 0) throw new Error(`[${r.id}] about[] required`);
  if (!['seed','analysis','curation','bridge'].includes(r.kind)) throw new Error(`[${r.id}] invalid kind`);
  return true;
}

export function validateEssentialRelation(e: EssentialRelation) {
  if (!e.id) throw new Error('essential relation id required');
  if (!e.from || !e.to || !e.rel) throw new Error(`[${e.id}] from/to/rel required`);
  if (!['essential','accidental'].includes(e.necessity)) throw new Error(`[${e.id}] invalid necessity`);
  if (!e.agentId) throw new Error(`[${e.id}] agentId required`);
  return true;
}
