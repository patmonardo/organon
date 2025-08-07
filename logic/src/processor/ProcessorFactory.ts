/**
 * Processor Factory for creating RelationalFormsProcessor instances
 * This shows how to wire up the repository dependencies properly
 */

import { Neo4jConnection } from "../connection";
import { FormShapeRepository } from "../repository/form";
import { EntityShapeRepository } from "../repository/entity";
import { RelationShapeRepository } from "../repository/relation";
import { RelationalFormsProcessor } from "./RelationalFormsProcessor";

/**
 * Factory for creating properly configured RelationalFormsProcessor instances
 */
export class ProcessorFactory {

  /**
   * Create a RelationalFormsProcessor with all repository dependencies
   * This is the clean way to instantiate the processor using your CRUD interfaces
   */
  static async createProcessor(connection: Neo4jConnection): Promise<RelationalFormsProcessor> {
    // Initialize your repository CRUD interfaces
    const formRepository = new FormShapeRepository(connection);
    const entityRepository = new EntityShapeRepository(connection);
    const relationRepository = new RelationShapeRepository(connection);

    // Create the processor with repository dependencies
    return new RelationalFormsProcessor(
      formRepository,
      entityRepository,
      relationRepository
    );
  }

  /**
   * Create a processor with custom repository implementations
   * Useful for testing or when you want to use different repository strategies
   */
  static createProcessorWithRepositories(
    formRepository: FormShapeRepository,
    entityRepository: EntityShapeRepository,
    relationRepository: RelationShapeRepository
  ): RelationalFormsProcessor {
    return new RelationalFormsProcessor(
      formRepository,
      entityRepository,
      relationRepository
    );
  }
}

/**
 * Example usage:
 *
 * ```typescript
 * import { ProcessorFactory } from './ProcessorFactory';
 * import { Neo4jConnection } from '../connection';
 *
 * // Initialize connection (your existing pattern)
 * const connection = new Neo4jConnection({
 *   uri: process.env.NEO4J_URI || "neo4j://localhost:7687",
 *   username: process.env.NEO4J_USERNAME || "neo4j",
 *   password: process.env.NEO4J_PASSWORD || "neo4j",
 *   useDefaultDriver: false,
 * });
 *
 * await connection.initialize();
 *
 * // Create the processor - this is THE HEART
 * const processor = await ProcessorFactory.createProcessor(connection);
 *
 * // Now use the processor for Cosmic Intelligence Forms
 * const form = BECFormFactory.createCosmicIntelligenceForm("My Cosmic Form");
 * const processedForm = await processor.processBECForm(form);
 * ```
 */
