// import type {
//   ProcessorInputs,
//   ProcessorSnapshot,
//   ProcessorRunOptions,
// } from './contracts';

// // Skeletal Processor — no orchestration, no external contracts.
// // Provides a stable surface you can expand later.

// export class FormProcessor {
//   constructor(
//     readonly defaults: ProcessorRunOptions = {
//       projectContent: true,
//       contentIndexSource: 'inputs',
//       deriveSyllogistic: false,
//     },
//   ) {}

//   // Main entry: returns canonical snapshot (stable contract)
//   async compute(
//     input: ProcessorInputs,
//     opts?: ProcessorRunOptions,
//   ): Promise<ProcessorSnapshot> {
//     // In the skeletal runtime, compute == run(snapshot-only)
//     const result = await this.run(input, opts);
//     return {
//       world: result.world,
//       indexes: result.indexes,
//     };
//   }

//   // Orchestration entry (skeletal): returns same snapshot payload
//   async run(
//     input: ProcessorInputs,
//     _opts?: ProcessorRunOptions,
//   ): Promise<ProcessorSnapshot> {
//     const counts = countInputs(input);
//     const world = assembleWorld({ name: 'FormProcessor', stage: 'skeletal' });
//     return {
//       world,
//       indexes: { counts },
//     };
//   }

//   // Convenience: assemble only the World (no projections/derivations)
//   assemble(input: ProcessorInputs) {
//     // Name/world stage can be adapted from inputs later if desired
//     return assembleWorld({ name: 'FormProcessor', stage: 'skeletal' });
//   }
// }

// // Helpers

// function countInputs(input: ProcessorInputs) {
//   const entities = (input.entities ?? []).length;
//   const contexts = (input.contexts ?? []).length;
//   const properties = (input.properties ?? []).length;
//   const morphs = (input.morphs ?? []).length;
//   const aspects = (input.aspects ?? []).length;
//   const relations = (input.relations ?? []).length;
//   return { entities, contexts, properties, morphs, aspects, relations };
// }

// function assembleWorld(arg: { name: string; stage?: string }) {
//   return {
//     id: `world:${slug(arg.name)}`,
//     type: 'system.World',
//     name: arg.name,
//     horizon: { stage: arg.stage ?? 'default' },
//   };
// }

// function slug(s: string) {
//   return String(s)
//     .trim()
//     .toLowerCase()
//     .replace(/[^a-z0-9]+/g, '-')
//     .replace(/^-+|-+$/g, '');
// }
