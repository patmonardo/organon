/**
 * Example: SDSL Specification -> Neo4j Design Surface -> Compiler Artifacts
 *
 * Demonstrates a TS-first, specification-driven flow:
 * 1) Author SDSL specification (Zod schema)
 * 2) Persist as Entity/Property/Aspect records in Neo4j (FormDB)
 * 3) Compile into DataFrame/Kernel/MVC artifacts from the same spec
 */

/// <reference types="node" />

import { defaultConnection } from '../../../src/repository/neo4j-client';
import { SdslSpecificationRepository } from '../../../src/repository/sdsl-specification';
import { SdslSpecificationSchema } from '../../../src/schema/sdsl';
import { SdslCompilerInputSchema } from '../../../src/schema/sdsl-compiler';

export async function runSdslSpecificationNeo4jDemo(): Promise<void> {
  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    // eslint-disable-next-line no-console
    console.error('Neo4j is not reachable. Skipping SDSL demo run.');
    return;
  }

  const timestamp = Date.now();
  const specificationId = `research-assistant-${timestamp}`;
  const specification = SdslSpecificationSchema.parse({
    id: specificationId,
    title: 'Research Assistant Science Design Surface',
    description:
      'Specification-driven ToolChain for science design using TS-first SDSL.',
    classification: {
      genus: 'science-design',
      species: 'toolchain-specification',
    },
    gdslSource:
      'spec ScienceDesign { model Research; features [hypothesis, signal, evidence]; }',
    engine: {
      logicalForm: 'relative-form',
      mvc: 'react-next',
    },
    models: [
      { id: 'research-model', label: 'Research Model', kind: 'domain-model' },
      { id: 'evidence-model', label: 'Evidence Model', kind: 'domain-model' },
    ],
    features: [
      {
        id: 'hypothesis-feature',
        label: 'Hypothesis Feature',
        modelId: 'research-model',
        kind: 'semantic-feature',
      },
      {
        id: 'signal-feature',
        label: 'Signal Feature',
        modelId: 'research-model',
        kind: 'analytic-feature',
      },
      {
        id: 'evidence-feature',
        label: 'Evidence Feature',
        modelId: 'evidence-model',
        kind: 'trace-feature',
      },
    ],
    tags: ['sdsl-demo', 'toolchain-first', 'neo4j-community'],
    meta: {
      note: 'FormDB records can be adapted into React/Next dashboard UX.',
    },
  });

  const compilerInput = SdslCompilerInputSchema.parse({
    user: { username: 'copilot', isAdmin: true },
    databaseId: 'formdb',
    graphName: `sdsl-spec-${specificationId}`,
    outputGraphName: `sdsl-spec-${specificationId}-compiled`,
  });

  const repository = new SdslSpecificationRepository(defaultConnection);
  const result = await repository.saveAndCompileSpecification(
    specification,
    compilerInput,
  );

  const specTag = `sdsl.spec:${specificationId}`;
  const session = defaultConnection.getSession({ defaultAccessMode: 'READ' });

  try {
    const countsResult = await session.run(
      `
      MATCH (n)-[:HAS_TAG]->(:Tag {name: $specTag})
      RETURN labels(n)[0] as label, count(n) as count
      ORDER BY label ASC
      `,
      { specTag },
    );

    const counts = countsResult.records.map((record) => ({
      label: String(record.get('label') ?? 'Unknown'),
      count: Number(record.get('count') ?? 0),
    }));

    // eslint-disable-next-line no-console
    console.log('\nSDSL Specification persisted to Neo4j:');
    // eslint-disable-next-line no-console
    console.log(
      JSON.stringify(
        {
          specificationId,
          entityId: result.persisted.entity.id,
          propertyCount: result.persisted.properties.length,
          aspectCount: result.persisted.aspects.length,
          counts,
        },
        null,
        2,
      ),
    );

    // eslint-disable-next-line no-console
    console.log('\nCompiled Artifact Summary:');
    // eslint-disable-next-line no-console
    console.log(
      JSON.stringify(
        {
          dataframe: {
            engine: result.compiled.dataframe.engine,
            stepCount: result.compiled.dataframe.steps.length,
            firstStep: result.compiled.dataframe.steps[0],
          },
          kernel: {
            graphStoreOp: result.compiled.kernel.graphStorePut.op,
            formEvalOp: result.compiled.kernel.formEvalEvaluate.op,
            morphPatterns:
              result.compiled.kernel.formEvalEvaluate.op === 'evaluate'
                ? result.compiled.kernel.formEvalEvaluate.program.morph.patterns
                : [],
          },
          mvc: result.compiled.mvc,
        },
        null,
        2,
      ),
    );
  } finally {
    await session.close();
    await defaultConnection.close();
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  runSdslSpecificationNeo4jDemo().catch((error) => {
    // eslint-disable-next-line no-console
    console.error('SDSL Neo4j demo failed:', error);
    process.exitCode = 1;
  });
}
