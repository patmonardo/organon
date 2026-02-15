/**
 * Example: Pure GDSL Form Program over TSJSON
 *
 * - Builds dataset-derived snapshot separately
 * - Sends pure Form Program payload to `form_eval.evaluate`
 * - Avoids SDSL model/feature specification as persisted graph structure
 */

/// <reference types="node" />

import { tsjsonInvoke } from '../gds/_tsjson';

export function runGdslFormProgramDemo(): void {
  const user = { username: 'copilot', isAdmin: true };
  const databaseId = 'db1';
  const graphName = `gdsl-${Date.now()}`;

  const datasetSnapshot = {
    nodes: [0, 1, 2, 3],
    relationships: [
      { type: 'NEXT', source: 0, target: 1, properties: { weight: 1 } },
      { type: 'NEXT', source: 1, target: 2, properties: { weight: 1 } },
      { type: 'NEXT', source: 2, target: 3, properties: { weight: 1 } },
      { type: 'SIMILAR', source: 0, target: 2, properties: { weight: 0.4 } },
    ],
  };

  const formProgramCall = {
    kind: 'ApplicationForm',
    facade: 'form_eval',
    op: 'evaluate',
    user,
    databaseId,
    graphName,
    outputGraphName: `${graphName}-compiled`,
    program: {
      shape: {
        required_fields: ['graphName'],
      },
      context: {
        runtime_strategy: 'relative-form',
        conditions: ['gdsl-pure', 'dataset-projected'],
      },
      morph: {
        patterns: ['spec.validate', 'algo.pagerank'],
        steps: [
          { kind: 'form', op: 'spec.validate' },
          { kind: 'form', op: 'algo.pagerank' },
          { kind: 'judge', moment: 'reflection' },
        ],
      },
      applicationForms: [
        {
          name: 'centrality',
          domain: 'graph-ml',
          features: ['feature.centrality.pagerank'],
          patterns: ['algo.pagerank'],
          specifications: { binding: 'spec.pagerank' },
        },
      ],
      selectedForms: ['centrality'],
    },
    artifacts: {
      source: 'gdsl',
      ontology: 'active',
    },
  };

  const batch = [
    {
      kind: 'ApplicationForm',
      facade: 'graph_store',
      op: 'put',
      user,
      databaseId,
      graphName,
      snapshot: datasetSnapshot,
    },
    formProgramCall,
  ];

  // eslint-disable-next-line no-console
  console.log('batch.request:', batch);
  const response = tsjsonInvoke(batch);
  // eslint-disable-next-line no-console
  console.log('batch.response:', response);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  runGdslFormProgramDemo();
}
