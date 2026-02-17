// import { Neo4jConnection } from './neo4j-client';
// import { EntityShapeRepository } from './entity';
// import { PropertyShapeRepository } from './property';
// import { AspectShapeRepository } from './aspect';

// import {
//   SdslSpecificationSchema,
//   type SdslSpecification,
//   specificationToDesignSurface,
// } from '@schema/sdsl';
// import {
//   compileSdslSpecification,
//   SdslCompilerInputSchema,
//   type SdslCompilerArtifacts,
//   type SdslCompilerInput,
// } from '@relative/core/compiler/sdsl-compiler';

// import type { EntityShapeRepo } from '@schema/entity';
// import type { PropertyShapeRepo } from '@schema/property';
// import type { AspectShapeRepo } from '@schema/aspect';

// export type PersistedSdslDesignSurface = {
//   specification: SdslSpecification;
//   entity: EntityShapeRepo;
//   properties: PropertyShapeRepo[];
//   aspects: AspectShapeRepo[];
// };

// export type PersistedCompiledSdsl = {
//   persisted: PersistedSdslDesignSurface;
//   compiled: SdslCompilerArtifacts;
// };

// export class SdslSpecificationRepository {
//   private readonly entityRepo: EntityShapeRepository;
//   private readonly propertyRepo: PropertyShapeRepository;
//   private readonly aspectRepo: AspectShapeRepository;

//   constructor(private readonly connection: Neo4jConnection) {
//     this.entityRepo = new EntityShapeRepository(connection);
//     this.propertyRepo = new PropertyShapeRepository(connection);
//     this.aspectRepo = new AspectShapeRepository(connection);
//   }

//   async saveSpecification(
//     specificationInput: SdslSpecification,
//   ): Promise<PersistedSdslDesignSurface> {
//     const specification = SdslSpecificationSchema.parse(specificationInput);
//     const payload = specificationToDesignSurface(specification);

//     const entity = await this.entityRepo.saveEntity(payload.entity);

//     const properties: PropertyShapeRepo[] = [];
//     for (const property of payload.properties) {
//       properties.push(await this.propertyRepo.saveProperty(property));
//     }

//     const aspects: AspectShapeRepo[] = [];
//     for (const aspect of payload.aspects) {
//       aspects.push(await this.aspectRepo.saveAspect(aspect));
//     }

//     return {
//       specification,
//       entity,
//       properties,
//       aspects,
//     };
//   }

//   async loadSpecificationDesignSurface(
//     specificationId: string,
//   ): Promise<PersistedSdslDesignSurface | null> {
//     const specEntityId = `sdsl:spec:${specificationId}`;
//     const entity = await this.entityRepo.getEntityById(specEntityId);
//     if (!entity) return null;

//     const specTag = `sdsl.spec:${specificationId}`;
//     const [properties, aspects] = await Promise.all([
//       this.propertyRepo.findProperties({ tags: [specTag] }),
//       this.aspectRepo.findAspects({ tags: [specTag] }),
//     ]);

//     const rawSpec = (entity.facets as any)?.specification;
//     const specification = SdslSpecificationSchema.parse(rawSpec);

//     return {
//       specification,
//       entity,
//       properties,
//       aspects,
//     };
//   }

//   async saveAndCompileSpecification(
//     specificationInput: SdslSpecification,
//     compilerInput: SdslCompilerInput,
//   ): Promise<PersistedCompiledSdsl> {
//     const persisted = await this.saveSpecification(specificationInput);
//     const compileInput = SdslCompilerInputSchema.parse(compilerInput);
//     const compiled = compileSdslSpecification(
//       persisted.specification,
//       compileInput,
//     );

//     return {
//       persisted,
//       compiled,
//     };
//   }
// }
