import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'
import { canonicalizeManifest, upsertArtifactToStore, MemoryGraphStore } from '../../../dsl/src/scripts/projection/projection'

function ensureDir(d: string) { fs.mkdirSync(d, { recursive: true }) }

async function main() {
  const manifestArg = process.argv[2]
  if (!manifestArg) {
    console.error('usage: tsx reality/src/scripts/project.ts reality/src/data/datasets/logos-lens.ts')
    process.exit(2)
  }
  const manifestAbs = path.resolve(process.cwd(), manifestArg)
  const artifact = await canonicalizeManifest(manifestAbs)

  const outDir = path.resolve(process.cwd(), 'reality', 'dist', 'datasets')
  ensureDir(outDir)
  const outFile = path.join(outDir, `${artifact.dataset.replace(/[:\/]/g, '_')}.json`)
  fs.writeFileSync(outFile, JSON.stringify(artifact, null, 2), 'utf8')
  console.log('[reality:projection] wrote', outFile, 'counts=', artifact.counts)

  const store = new MemoryGraphStore()
  await upsertArtifactToStore(store, artifact)
  const dump = store.dump()
  console.log('[reality:projection] memory store nodes=', dump.nodes.length, 'edges=', dump.edges.length)
}

if (fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch(err => { console.error(err); process.exit(1) })
}
