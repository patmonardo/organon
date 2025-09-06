import fs from 'fs'
import path from 'path'

type Edge = { type: string; from: string; to: string; props?: any }
type Artifact = { edges: Edge[] }

function load(file: string): Artifact {
  return JSON.parse(fs.readFileSync(path.resolve(file), 'utf8'))
}

function collectTags(art: Artifact) {
  const tags = new Map<string, Record<string,string>>()
  for (const e of art.edges) {
    if (e.type === 'HLO_TAGS') {
      const key = e.props?.key
      const value = e.props?.value
      if (!key) continue
      const t = tags.get(e.from) ?? {}
      t[String(key)] = String(value)
      tags.set(e.from, t)
    }
    if (e.type === 'HLO_TAGS_TERM') {
      const key = e.props?.key
      const termId = String(e.to).startsWith('term:') ? String(e.to).slice(5) : String(e.to)
      if (!key) continue
      const t = tags.get(e.from) ?? {}
      t[String(key)] = termId
      tags.set(e.from, t)
    }
  }
  return tags
}

const LENS = new Map<string, number>([
  ['hegel', 0],
  ['vedanta', 1],
  ['fichte', 2],
])
const PLANE = new Map<string, number>([
  ['dyadic', 0],
  ['triadic', 1],
  ['notion', 2], // reserve for the Concept plane
])
function stageIndex(role?: string) {
  switch (role) {
    case 'original': return 0
    case 'judgment': return 1
    case 'reflection': return 1 // mid, same layer
    case 'reconstruction': return 2
    default: return NaN
  }
}

function main() {
  const file = process.argv[2]
  if (!file) {
    console.error('usage: tsx src/scripts/views/cube-view.ts dist/datasets/dataset_science-of-logic-being.json')
    process.exit(2)
  }
  const art = load(file)
  const tags = collectTags(art)

  const coords: any[] = []
  for (const [hloId, t] of tags) {
    const lens = LENS.has(t.lens) ? LENS.get(t.lens)! : 0
    const plane = PLANE.has(t.plane) ? PLANE.get(t.plane)! : 0
    const stage = stageIndex(t.role)
    if (Number.isNaN(stage)) continue
    coords.push({ hloId, lens, plane, stage, cycle: t.cycle ?? 'UNSPECIFIED', phase: t.phase ?? '', role: t.role })
  }

  // rollups
  const byCycle = new Map<string, number>()
  coords.forEach(c => byCycle.set(c.cycle, (byCycle.get(c.cycle) ?? 0) + 1))

  console.log(JSON.stringify({
    count: coords.length,
    byCycle: Array.from(byCycle.entries()).map(([cycle, n]) => ({ cycle, n })),
    coords
  }, null, 2))
}

main()
