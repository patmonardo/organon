// import type { DatasetPkgExtValidator } from '@organon/gdsl/packaging/extensions'
import type { DatasetPackageManifest } from '@organon/gdsl/packaging/dataset-manifest'

type RealityExt = {
  provenance?: { source?: string; doi?: string; note?: string }
  citations?: Array<{ id: string; title: string }>
}

export const VALIDATOR: DatasetPkgExtValidator = {
  ns: '@organon/reality',
  validate: (m: DatasetPackageManifest, ext: RealityExt) => {
    if (!ext) return
    if (ext.provenance && !ext.provenance.source && !ext.provenance.doi) {
      throw new Error(`[${m.id}] @organon/reality.ext.provenance needs source or doi`)
    }
    if (ext.citations) {
      const ids = new Set<string>()
      for (const c of ext.citations) {
        if (!c.id || !c.title) throw new Error(`[${m.id}] citation id/title required`)
        if (ids.has(c.id)) throw new Error(`[${m.id}] duplicate citation id ${c.id}`)
        ids.add(c.id)
      }
    }
  }
}

export default VALIDATOR
