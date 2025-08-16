import {
  ProcessorInputs as InputsSchema,
  ProcessorSnapshot as SnapshotSchema,
} from './contracts';
import type { ProcessorInputs, ProcessorSnapshot } from './contracts';
import { assembleWorld } from '../essence/world';
import { runKriya, type KriyaResult } from './orchestrator';
import type { KriyaOptions } from './kriya';

export type ProcessorRunOptions = KriyaOptions;

/**
 * FormProcessor — integrates schemas and Form wrappers via Kriya orchestration.
 * - compute: validated inputs -> ProcessorSnapshot (world + indexes)
 * - run: validated inputs -> KriyaResult (projectedContent, derivedEdges, indexes)
 * - assemble: quick world assembly without orchestration
 */
export class FormProcessor {
  constructor(
    private readonly defaults: ProcessorRunOptions = {
      projectContent: true,
      contentIndexSource: 'inputs',
      deriveSyllogistic: false,
    },
  ) {}

  // Main entry: returns canonical snapshot (stable contract)
  async compute(
    input: ProcessorInputs,
    opts?: ProcessorRunOptions,
  ): Promise<ProcessorSnapshot> {
    const parsed = InputsSchema.parse(input);
    const result = await this.run(parsed, opts);
    return SnapshotSchema.parse({
      world: result.world,
      indexes: result.indexes,
    });
  }

  // Orchestration entry: returns full KriyaResult for advanced callers
  async run(
    input: ProcessorInputs,
    opts?: ProcessorRunOptions,
  ): Promise<KriyaResult> {
    const effective: ProcessorRunOptions = {
      ...this.defaults,
      ...(opts ?? {}),
    };
    return runKriya(input, effective);
  }

  // Convenience: assemble only the World (no projections/derivations)
  assemble(input: ProcessorInputs) {
    const parsed = InputsSchema.parse(input);
    return assembleWorld(parsed);
  }
}
