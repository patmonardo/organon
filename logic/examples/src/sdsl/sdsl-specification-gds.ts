/**
 * Example: SDSL Specification -> Compiler Artifacts -> GDS TSJSON Dispatch
 *
 * Demonstrates the "other way" path:
 * - Author SDSL in TypeScript/Zod
 * - Compile to kernel application calls
 * - Execute Form Program via `form_eval.evaluate` over TSJSON (no Neo4j persistence layer)
 */

/// <reference types="node" />

import { tsjsonInvoke } from '../gds/_tsjson';
import { SdslSpecificationSchema } from '../../../src/schema/sdsl';
import {
  compileSdslSpecification,
  SdslCompilerInputSchema,
} from '../../../src/schema/sdsl-compiler';

export function runSdslSpecificationGdsDemo(): void {
  const timestamp = Date.now();
  const specificationId = `sdsl-gds-${timestamp}`;

  const specification = SdslSpecificationSchema.parse({
    id: specificationId,
    title: 'SDSL Ontological Given-Form Recognition Demo',
    description:
      'Compile ontology-first TS/Zod SDSL into Given-form recognition calls over GDS TSJSON.',
    classification: {
      genus: 'science-design',
      species: 'form-program',
    },
    gdslSource:
      'spec OntologicalForm { model Kernel; features [signal, evidence, synthesis]; }',
    ontologies: [
      {
        id: 'science-ontology',
        iri: 'https://organon.dev/ontology/science#',
        profile: 'owl',
        entities: [
          {
            id: 'Signal',
            iri: 'https://organon.dev/ontology/science#Signal',
            kind: 'class',
          },
          {
            id: 'hasEvidence',
            iri: 'https://organon.dev/ontology/science#hasEvidence',
            kind: 'property',
          },
          {
            id: 'DatasetAspect',
            iri: 'https://organon.dev/ontology/science#DatasetAspect',
            kind: 'aspect',
          },
        ],
        queries: [
          {
            id: 'q-signal-evidence',
            language: 'sparql',
            text: 'SELECT ?signal ?evidence WHERE { ?signal <https://organon.dev/ontology/science#hasEvidence> ?evidence . }',
          },
        ],
        constraints: [
          {
            id: 'c-signal-shape',
            language: 'shacl',
            text: 'ex:SignalShape a sh:NodeShape ; sh:targetClass ex:Signal ; sh:property [ sh:path ex:hasEvidence ; sh:minCount 1 ] .',
          },
        ],
      },
    ],
    engine: {
      logicalForm: 'relative-form',
      mvc: 'react-next',
    },
    models: [
      { id: 'kernel-model', label: 'Kernel Model', kind: 'domain-model' },
    ],
    features: [
      {
        id: 'signal-feature',
        label: 'Signal Feature',
        modelId: 'kernel-model',
        kind: 'analytic-feature',
      },
      {
        id: 'evidence-feature',
        label: 'Evidence Feature',
        modelId: 'kernel-model',
        kind: 'trace-feature',
      },
      {
        id: 'synthesis-feature',
        label: 'Synthesis Feature',
        modelId: 'kernel-model',
        kind: 'semantic-feature',
      },
    ],
    tags: ['sdsl', 'gds', 'tsjson', 'ontology', 'given-form'],
    meta: {
      note: 'Program features are ontological and dispatched as Given-form recognition payloads.',
    },
  });

  const compilerInput = SdslCompilerInputSchema.parse({
    user: { username: 'copilot', isAdmin: true },
    databaseId: 'db1',
    graphName: `sdsl-${specificationId}`,
    outputGraphName: `sdsl-${specificationId}-compiled`,
  });

  const compiled = compileSdslSpecification(specification, compilerInput);

  // Dataset-system boundary:
  // - Persist only dataset-derived graph structure
  // - Do not persist GDSL model/feature definitions as GraphStore nodes
  const datasetFrame = [
    { rowId: 0, signal: 0.92, evidence: 0.81, synthesis: 0.88 },
    { rowId: 1, signal: 0.72, evidence: 0.63, synthesis: 0.69 },
    { rowId: 2, signal: 0.85, evidence: 0.77, synthesis: 0.83 },
    { rowId: 3, signal: 0.61, evidence: 0.74, synthesis: 0.67 },
  ];

  const datasetSnapshot = {
    nodes: datasetFrame.map((row) => row.rowId),
    relationships: [
      { type: 'NEXT', source: 0, target: 1, properties: { weight: 1 } },
      { type: 'NEXT', source: 1, target: 2, properties: { weight: 1 } },
      { type: 'NEXT', source: 2, target: 3, properties: { weight: 1 } },
      { type: 'SIMILAR', source: 0, target: 2, properties: { weight: 0.4 } },
      { type: 'SIMILAR', source: 1, target: 3, properties: { weight: 0.5 } },
    ],
  };

  const batch = [
    {
      ...compiled.kernel.graphStorePut,
      snapshot: datasetSnapshot,
    },
    {
      ...compiled.kernel.formEvalEvaluate,
      program: {
        ...compiled.kernel.formEvalEvaluate.program,
        applicationForms: [
          {
            name: 'centrality',
            domain: 'ontology-runtime',
            features: ['feature.centrality.pagerank'],
            patterns: ['algo.pagerank'],
            specifications: {
              binding: 'spec.pagerank',
              ontologyMode: 'epistemic',
              ontologyIri: 'https://organon.dev/ontology/science#',
              recognition: 'pure-form-approves-given-form',
            },
          },
        ],
        selectedForms: ['centrality'],
      },
    },
  ];

  // eslint-disable-next-line no-console
  console.log('batch.request:', batch);
  const response = tsjsonInvoke(batch);
  // eslint-disable-next-line no-console
  console.log('batch.response:', response);

  // eslint-disable-next-line no-console
  console.log(
    JSON.stringify(
      {
        specificationId,
        graphName: compiled.kernel.graphStorePut.graphName,
        outputGraphName: compiled.kernel.formEvalEvaluate.outputGraphName,
        projectedToGdsOps: batch.map((call) => `${call.facade}.${call.op}`),
        ontologyCount: specification.ontologies.length,
        ontologyImage: {
          imageId: compiled.ontologyImage.imageId,
          modelRows: compiled.ontologyImage.tables.models.length,
          featureRows: compiled.ontologyImage.tables.features.length,
          queryRows: compiled.ontologyImage.tables.queries.length,
          constraintRows: compiled.ontologyImage.tables.constraints.length,
          runtimeMode:
            compiled.ontologyImage.tables.provenance[0]?.runtimeMode ??
            'direct-compute',
        },
        storeContract: {
          formCatalog: compiled.storeContract.formCatalog.listRef,
          factGraph: compiled.storeContract.factStore.graphRef,
          factGrounds: compiled.storeContract.factStore.groundsRef,
          knowledgeStoreTarget: compiled.storeContract.knowledgeStore.targetRef,
          compileMode: compiled.storeContract.knowledgeStore.compileMode,
        },
        datasetRows: datasetFrame.length,
        dataframeStepCount: compiled.dataframe.steps.length,
      },
      null,
      2,
    ),
  );
}

if (import.meta.url === `file://${process.argv[1]}`) {
  runSdslSpecificationGdsDemo();
}
