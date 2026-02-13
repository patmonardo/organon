import {
  AgentOmniscientContext,
  AgentOmniscientContextSchema,
  FactProjection,
  FactProjectionSchema,
  KnowledgeProjection,
  KnowledgeProjectionSchema,
  SpecificationRef,
} from '../schema/agent-omniscient';

/**
 * AgentOmniscientRepository
 *
 * In-memory projection assembler for unified Agent context.
 * Upstream adapters can hydrate Fact/Knowledge projections from Neo4j.
 */
export class AgentOmniscientRepository {
  private contexts: Map<string, AgentOmniscientContext> = new Map();

  composeContext(input: {
    id: string;
    graphId: string;
    factProjection: FactProjection;
    knowledgeProjection: KnowledgeProjection;
    specifications?: SpecificationRef[];
    metadata?: Record<string, unknown>;
  }): AgentOmniscientContext {
    const factProjection = FactProjectionSchema.parse(input.factProjection);
    const knowledgeProjection = KnowledgeProjectionSchema.parse(
      input.knowledgeProjection,
    );

    return AgentOmniscientContextSchema.parse({
      id: input.id,
      graphId: input.graphId,
      timestamp: new Date().toISOString(),
      factProjection,
      knowledgeProjection,
      specifications: input.specifications ?? [],
      metadata: input.metadata,
    });
  }

  async saveContext(
    context: AgentOmniscientContext,
  ): Promise<AgentOmniscientContext> {
    const parsed = AgentOmniscientContextSchema.parse(context);
    this.contexts.set(parsed.id, parsed);
    return parsed;
  }

  async getContextById(
    id: string,
  ): Promise<AgentOmniscientContext | undefined> {
    return this.contexts.get(id);
  }

  async deleteContext(id: string): Promise<boolean> {
    return this.contexts.delete(id);
  }
}
