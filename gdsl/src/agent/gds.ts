import type { GraphArtifact } from '../schema/projection'

export type NeighborDir = 'out' | 'in' | 'both'

export type NeighborOpts = {
  dir?: NeighborDir
  type?: string
}

export interface GDSProvider {
  neighbors(id: string, opts?: NeighborOpts): string[]
  degree(id: string, opts?: NeighborOpts): number
  run<T = unknown>(algorithm: string, params: Record<string, any>): Promise<T>
}

/** Simple in-memory mock GDS over a GraphArtifact. */
export function createMockGDS(artifact: GraphArtifact): GDSProvider {
  const out = new Map<string, any[]>()
  const inn = new Map<string, any[]>()
  for (const e of artifact.edges ?? []) {
    const type = String(e.type ?? '')
    const from = String(e.from)
    const to = String(e.to)
    const oe = out.get(from) ?? []; oe.push({ type, to }); out.set(from, oe)
    const ie = inn.get(to) ?? [];  ie.push({ type, from }); inn.set(to, ie)
  }

  const neighbors = (id: string, opts?: NeighborOpts): string[] => {
    const dir = opts?.dir ?? 'out'
    const type = opts?.type
    const outNbrs = (out.get(id) ?? []).filter(e => !type || e.type === type).map(e => e.to)
    const inNbrs  = (inn.get(id) ?? []).filter(e => !type || e.type === type).map(e => e.from)
    const joined = dir === 'out' ? outNbrs : dir === 'in' ? inNbrs : outNbrs.concat(inNbrs)
    return Array.from(new Set(joined))
  }

  const degree = (id: string, opts?: NeighborOpts) => neighbors(id, opts).length

  const run = async <T = unknown>(algorithm: string, params: Record<string, any>): Promise<T> => {
    switch (algorithm) {
      case 'neighbors': {
        const { id, dir, type } = params
        return neighbors(String(id), { dir, type }) as unknown as T
      }
      case 'degree': {
        const { id, dir, type } = params
        return degree(String(id), { dir, type }) as unknown as T
      }
      default:
        throw new Error(`MockGDS: unknown algorithm ${algorithm}`)
    }
  }

  return { neighbors, degree, run }
}
