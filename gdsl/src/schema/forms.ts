export interface Shape { id: string; name: string; meta?: Record<string, unknown> }
export interface Context { id: string; name: string; meta?: Record<string, unknown> }
export interface Morph { id: string; name: string; shapeId: string; contextId: string; meta?: Record<string, unknown> }
export interface Entity { id: string; shapeId: string; clauses?: unknown; meta?: Record<string, unknown> }
export interface Property { id: string; contextId: string; law?: unknown; value?: unknown; meta?: Record<string, unknown> }

export interface Judgment {
  type: 'Affirmative' | 'Negative' | string
  subject: string
  object: string
  relation?: string
  predicate?: string
  source?: string
  [k: string]: unknown
}

export interface Aspect {
  id: string
  morphId: string
  entityId: string
  propertyId: string
  stage?: string | number
  meta?: Record<string, unknown>
  judgments?: Judgment[]
}

export interface FormsDoc {
  shapes: Shape[]
  contexts: Context[]
  morphs: Morph[]
  entities: Entity[]
  properties: Property[]
  aspects: Aspect[]
}
