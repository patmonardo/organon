import { pathToFileURL } from 'node:url'
import fg from 'fast-glob'
import { writeFile, mkdir } from 'node:fs/promises'
import { resolve, dirname } from 'node:path'

type Unit = { id: string; title: string; chunks: any[]; hlos: any[] }

async function main() {
  const files = await fg(['reality/src/logos/ys/**/*.ts'])
  const units: Unit[] = []

  for (const f of files) {
    const mod = await import(pathToFileURL(f).toString())
    for (const [k, v] of Object.entries(mod)) {
      if (k.endsWith('_UNIT') && v && typeof v === 'object' && 'chunks' in v && 'hlos' in v) {
        const u = v as Unit
        units.push({ id: u.id, title: u.title, chunks: u.chunks, hlos: u.hlos })
      }
    }
  }

  const outPath = resolve('dist/kg.json')
  await mkdir(dirname(outPath), { recursive: true })
  await writeFile(outPath, JSON.stringify({ units }, null, 2), 'utf8')
  console.log(`KG written: ${outPath} (units: ${units.length})`)
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
