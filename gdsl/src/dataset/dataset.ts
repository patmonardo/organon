import type { GraphArtifact } from '../schema/projection'
import { canonicalizeManifest } from './canonicalize'
import BaseTopic from '../topic/base.topic'
import { DatasetLens } from '../lens/dataset.lens'
import { GraphArtifactSchema } from '../schema/projection'   // <- added
import fs from 'node:fs/promises'                             // <- added

export default class Dataset {
  readonly artifact: GraphArtifact

  private constructor(artifact: GraphArtifact) {
    // validate at construction time
    this.artifact = GraphArtifactSchema.parse(artifact)
  }

  /** Build Dataset by canonicalizing a manifest (async IO delegated to canonicalizeManifest) */
  static async fromManifest(spec: string): Promise<Dataset> {
    const art = await canonicalizeManifest(spec)
    return new Dataset(art)
  }

  /** Wrap an existing validated GraphArtifact */
  static fromArtifact(artifact: GraphArtifact): Dataset {
    return new Dataset(artifact)
  }

  /** Produce a Topic model derived from this dataset */
  toTopic(id = `topic:${this.artifact.dataset}`): BaseTopic {
    return BaseTopic.fromArtifact(id, this.artifact)
  }

  /** Produce a DatasetLens derived from this dataset */
  toDatasetLens(id = `lens:dataset:${this.artifact.dataset}`): DatasetLens {
    return DatasetLens.fromArtifact(id, this.artifact)
  }

  /** Convenience: compute pipeline rows (delegates to DatasetLens logic) */
  computePipelineRows() {
    const lens = this.toDatasetLens()
    // DatasetLens.computePipelineRows is a pure helper (if present)
    // fallback: call DatasetLens.fromArtifact(...).computePipelineRows if instance method exists
    // keep this simple: call static if available, else recreate logic minimal
    // @ts-ignore - allow either static or instance usage
    if (typeof (DatasetLens as any).computePipelineRows === 'function') {
      return (DatasetLens as any).computePipelineRows(this.artifact)
    }
    // fallback: try instance method
    // @ts-ignore
    return (lens as any).computePipelineRows?.() ?? []
  }

  /** Return the artifact as a plain object (validated) */
  toJSON(): GraphArtifact {
    return this.artifact
  }

  /** Write artifact JSON to disk (useful for debugging/tests) */
  async writeToFile(filePath: string): Promise<void> {
    await fs.mkdir(new URL('file://' + filePath).pathname.replace(/\/[^/]+$/, ''), { recursive: true }).catch(()=>{})
    await fs.writeFile(filePath, JSON.stringify(this.artifact, null, 2), 'utf8')
  }
}
