import type { DatasetPackageManifest } from './dataset-manifest'

export interface DatasetPkgExtValidator {
  ns: string                // e.g., '@organon/reality'
  validate: (manifest: DatasetPackageManifest, extValue: any) => void
}

// Apply validators to manifest.ext (if present)
export function validateDatasetPkgExtensions(
  manifest: DatasetPackageManifest,
  validators: DatasetPkgExtValidator[] = []
) {
  const ext = manifest.ext || {}
  if (!ext || !Object.keys(ext).length) return
  const byNs = new Map(validators.map(v => [v.ns, v]))
  for (const [ns, value] of Object.entries(ext)) {
    const v = byNs.get(ns)
    if (!v) continue // unknown ext is ignored by core; owner package should supply validator
    v.validate(manifest, value)
  }
}
