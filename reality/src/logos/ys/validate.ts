import {
  YS_I_42_SUTRA_UNIT,
  YS_I_42_COMMENTARY_UNIT,
  YS_I_43_SUTRA_UNIT,
  YS_I_43_COMMENTARY_UNIT,
  YS_I_45_1_UNIT,
  YS_I_45_2_UNIT,
  YS_I_45_3_UNIT,
  YS_I_46_UNIT,
} from './index'

type Chunk = { id: string }
type Hlo = { id: string; chunkId: string }
type Unit = { id: string; chunks: Chunk[]; hlos: Hlo[] }

const UNITS: Unit[] = [
  YS_I_42_SUTRA_UNIT,
  YS_I_42_COMMENTARY_UNIT,
  YS_I_43_SUTRA_UNIT,
  YS_I_43_COMMENTARY_UNIT,
  YS_I_45_1_UNIT,
  YS_I_45_2_UNIT,
  YS_I_45_3_UNIT,
  YS_I_46_UNIT,
]

let hadErrors = false

for (const u of UNITS) {
  const ids = new Set(u.chunks.map(c => c.id))
  const missing = u.hlos.filter(h => !ids.has(h.chunkId))
  if (missing.length) {
    console.error(`[${u.id}] Missing chunkIds:`, missing.map(m => `${m.id} -> ${m.chunkId}`))
    hadErrors = true
  } else {
    console.log(`[${u.id}] OK`)
  }
}

export function assertChunkShape(unitId: string, chunks: any[]) {
  for (const c of chunks) {
    if ('text' in c) throw new Error(`[${unitId}] chunk ${c.id} uses "text"; use "source".`)
    for (const k of Object.keys(c)) {
      if (!['id','title','source','original','mode'].includes(k)) {
        throw new Error(`[${unitId}] chunk ${c.id} has unknown key "${k}".`)
      }
    }
    if (typeof c.source !== 'string' || !c.source.trim()) {
      throw new Error(`[${unitId}] chunk ${c.id} missing non-empty "source".`)
    }
    if (c.mode && !['summary','quote'].includes(c.mode)) {
      throw new Error(`[${unitId}] chunk ${c.id} has invalid mode "${c.mode}".`)
    }
  }
}

// call assertChunkShape(unit.id, unit.chunks) in your per-unit loop

if ((globalThis as any)?.process && hadErrors) (globalThis as any).process.exitCode = 1
