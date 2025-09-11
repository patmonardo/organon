import { DatasetUnit } from '@organon/gdsl/registry/canon'

type Report = {
  unitId: string
  duplicateChunkIds: string[]
  danglingHLOChunkRefs: string[]
}

export function validateUnit(unit: DatasetUnit): Report {
  const ids = unit.chunks.map((c: any) => c.id)
  const seen = new Set<string>()
  const dups: string[] = []
  for (const id of ids) (seen.has(id) ? dups.push(id) : seen.add(id))

  const chunkIdSet = new Set(ids)
  const dangling: string[] = []
  for (const h of unit.hlos as any[]) {
    if (!chunkIdSet.has(h.chunkId)) dangling.push(h.chunkId)
  }

  return {
    unitId: unit.id,
    duplicateChunkIds: [...new Set(dups)],
    danglingHLOChunkRefs: [...new Set(dangling)],
  }
}

// Example: validate IV.11 (adjust import if paths differ)
async function main() {
  const { YS_IV_11_UNIT } = await import('../reality/src/logos/ys/ys_iv_11')
  const report = validateUnit(YS_IV_11_UNIT as unknown as DatasetUnit)
  console.log(JSON.stringify(report, null, 2))
}

if (require.main === module) {
  // Run with: npx ts-node scripts/validate_dataset.ts
  main().catch((e) => {
    console.error(e)
    process.exit(1)
  })
}
