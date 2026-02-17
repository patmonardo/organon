import { Neo4jConnection } from './neo4j-client';
import { DialecticalFormSchema } from '../schema/form';
import type { DialecticalForm } from '../schema/form';
import { ManagedTransaction, Session } from 'neo4j-driver';
import neo4j, { Integer } from 'neo4j-driver';
import { v4 as uuidv4 } from 'uuid';

/**
 * DialecticalFormRepository
 *
 * Manages the persistence of Dialectical Forms in Neo4j.
 * Forms are meta-objects that govern the dialectical process,
 * supporting raw form definitions and the Context system.
 */
export class DialecticalFormRepository {
  private connection: Neo4jConnection;

  constructor(connection: Neo4jConnection) {
    this.connection = connection;
  }

  /**
   * Saves a Dialectical Form meta-object to Neo4j.
   */
  async saveForm(form: Partial<DialecticalForm>): Promise<DialecticalForm> {
    const now = Date.now();
    const validatedForm: DialecticalForm = DialecticalFormSchema.parse({
      id: form.id ?? uuidv4(),
      phase: form.phase ?? 'quality',
      moments: form.moments ?? [],
      rawDefinition: form.rawDefinition ?? {},
      contextBindings: form.contextBindings ?? [],
      meta: form.meta ?? { catalogId: 'default' },
      createdAt: form.createdAt ?? now,
      updatedAt: now,
    });

    const props = {
      id: validatedForm.id,
      phase: validatedForm.phase,
      moments: JSON.stringify(validatedForm.moments),
      rawDefinition: JSON.stringify(validatedForm.rawDefinition ?? {}),
      contextBindings: JSON.stringify(validatedForm.contextBindings ?? []),
      catalogId: validatedForm.meta?.catalogId ?? 'default',
      version: validatedForm.meta?.version ?? null,
      dialect: validatedForm.meta?.dialect ?? null,
      invariants: validatedForm.meta?.invariants
        ? JSON.stringify(validatedForm.meta.invariants)
        : null,
      createdAt: validatedForm.createdAt,
      updatedAt: validatedForm.updatedAt,
    };

    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      await session.executeWrite(async (txc) => {
        await txc.run(
          `
          MERGE (f:DialecticalForm {id: $props.id})
          SET f += $props
        `,
          { props },
        );
      });

      return validatedForm;
    } catch (error) {
      console.error(`Error saving dialectical form: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Retrieves a Dialectical Form by its ID.
   */
  async getFormById(id: string): Promise<DialecticalForm | null> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const result = await session.executeRead(async (txc) => {
        return await txc.run(
          `
          MATCH (f:DialecticalForm {id: $id})
          RETURN f
        `,
          { id },
        );
      });

      if (result.records.length === 0) {
        return null;
      }

      const props = result.records[0].get('f').properties;

      const form: DialecticalForm = DialecticalFormSchema.parse({
        id: props.id,
        phase: props.phase,
        moments: props.moments ? JSON.parse(props.moments) : [],
        rawDefinition: props.rawDefinition
          ? JSON.parse(props.rawDefinition)
          : {},
        contextBindings: props.contextBindings
          ? JSON.parse(props.contextBindings)
          : [],
        meta: {
          catalogId: props.catalogId ?? 'default',
          version: props.version ?? undefined,
          dialect: props.dialect ?? undefined,
          invariants: props.invariants
            ? JSON.parse(props.invariants)
            : undefined,
        },
        createdAt: neo4j.isInt(props.createdAt)
          ? props.createdAt.toNumber()
          : props.createdAt,
        updatedAt: neo4j.isInt(props.updatedAt)
          ? props.updatedAt.toNumber()
          : props.updatedAt,
      });

      return form;
    } catch (error) {
      console.error(`Error getting dialectical form by ID (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Finds Dialectical Forms by criteria.
   */
  async findForms(
    criteria: {
      phase?: string;
      catalogId?: string;
      dialect?: string;
    } = {},
  ): Promise<DialecticalForm[]> {
    const session = this.connection.getSession({ defaultAccessMode: 'READ' });
    try {
      const params: Record<string, any> = {};
      let matchClause = `MATCH (f:DialecticalForm)`;
      const whereClauses: string[] = [];

      if (criteria.phase) {
        whereClauses.push(`f.phase = $phase`);
        params.phase = criteria.phase;
      }

      if (criteria.catalogId) {
        whereClauses.push(`f.catalogId = $catalogId`);
        params.catalogId = criteria.catalogId;
      }

      if (criteria.dialect) {
        whereClauses.push(`f.dialect = $dialect`);
        params.dialect = criteria.dialect;
      }

      let cypher = matchClause;
      if (whereClauses.length > 0) {
        cypher += `\nWHERE ${whereClauses.join(' AND ')}`;
      }
      cypher += `\nRETURN f`;

      const result = await session.executeRead(async (txc) => {
        return await txc.run(cypher, params);
      });

      const forms: DialecticalForm[] = [];
      for (const record of result.records) {
        const props = record.get('f').properties;
        forms.push(
          DialecticalFormSchema.parse({
            id: props.id,
            phase: props.phase,
            moments: props.moments ? JSON.parse(props.moments) : [],
            rawDefinition: props.rawDefinition
              ? JSON.parse(props.rawDefinition)
              : {},
            contextBindings: props.contextBindings
              ? JSON.parse(props.contextBindings)
              : [],
            meta: {
              catalogId: props.catalogId ?? 'default',
              version: props.version ?? undefined,
              dialect: props.dialect ?? undefined,
              invariants: props.invariants
                ? JSON.parse(props.invariants)
                : undefined,
            },
            createdAt: neo4j.isInt(props.createdAt)
              ? props.createdAt.toNumber()
              : props.createdAt,
            updatedAt: neo4j.isInt(props.updatedAt)
              ? props.updatedAt.toNumber()
              : props.updatedAt,
          }),
        );
      }

      return forms;
    } catch (error) {
      console.error(`Error finding dialectical forms: ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }

  /**
   * Deletes a Dialectical Form by ID.
   */
  async deleteForm(id: string): Promise<boolean> {
    const session = this.connection.getSession({ defaultAccessMode: 'WRITE' });
    try {
      const result = await session.executeWrite(async (txc) => {
        return await txc.run(
          `
          MATCH (f:DialecticalForm {id: $id})
          DELETE f
          RETURN count(f) as deleted
        `,
          { id },
        );
      });

      return result.records[0].get('deleted').toNumber() > 0;
    } catch (error) {
      console.error(`Error deleting dialectical form (${id}): ${error}`);
      throw error;
    } finally {
      await session.close();
    }
  }
}
