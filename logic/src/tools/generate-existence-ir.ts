import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import type { DialecticIR, DialecticState } from '@schema/dialectic';
import { DialecticIRSchema } from '@schema/dialectic';
import { affirmativeInfinityIR } from '@relative/being/quality/existence/affirmative-infinity-ir';
import { alternatingInfinityIR } from '@relative/being/quality/existence/alternating-infinity-ir';
import { constitutionIR } from '@relative/being/quality/existence/constitution-ir';
import { existenceIR } from '@relative/being/quality/existence/existence-ir';
import { finitudeIR } from '@relative/being/quality/existence/finitude-ir';
import { infinityIR } from '@relative/being/quality/existence/infinity-ir';
import { somethingAndOtherIR } from '@relative/being/quality/existence/something-and-other-ir';
import { affirmativeInfinityTopicMap } from '@relative/being/quality/existence/sources/affirmative-infinity-topic-map';
import { alternatingInfinityTopicMap } from '@relative/being/quality/existence/sources/alternating-infinity-topic-map';
import { constitutionTopicMap } from '@relative/being/quality/existence/sources/constitution-topic-map';
import { existenceTopicMap } from '@relative/being/quality/existence/sources/existence-topic-map';
import { finitudeTopicMap } from '@relative/being/quality/existence/sources/finitude-topic-map';
import { infinityTopicMap } from '@relative/being/quality/existence/sources/infinity-topic-map';
import { somethingAndOtherTopicMap } from '@relative/being/quality/existence/sources/something-and-other-topic-map';
import type { TopicMapEntry } from '@schema/topic';

type SourceSpec = {
  id: string;
  title: string;
  sourceFile: string;
  topicMap: TopicMapEntry[];
};

type TraceType = 'NEXT' | 'NEGATES' | 'SUBLATES' | 'REFLECTS' | 'MEDIATES';

type IntegratedChunk = {
  id: string;
  title: string;
  sourceId: string;
  sourceFile: string;
  lineRange: { start: number; end: number };
  description: string;
  keyPoints: string[];
  orderInSource: number;
  globalOrder: number;
  sourceText: string;
  tags: string[];
};

type IntegratedSourceDocument = {
  id: string;
  title: string;
  sourceFile: string;
  totalLines: number;
  chunks: IntegratedChunk[];
};

type IntegratedTrace = {
  fromChunkId: string;
  toChunkId: string;
  type: TraceType;
  reason: string;
};

type IntegratedIR = {
  id: string;
  mode: 'debug';
  title: string;
  section: string;
  sourceDocuments: IntegratedSourceDocument[];
  traces: IntegratedTrace[];
  metadata: {
    totalSources: number;
    totalChunks: number;
    generatedAt: string;
  };
};

type SectionCompatibilitySpec = {
  fileName: string;
  exportName: string;
  statesExportName: string;
  irId: string;
  title: string;
  section: string;
  sourceFile: string;
  stateIds: string[];
};

const sectionCompatibilitySpecs: SectionCompatibilitySpec[] = [
  {
    fileName: 'existence-ir.ts',
    exportName: 'existenceIR',
    statesExportName: 'existenceStates',
    irId: 'existence-ir',
    title: 'Existence IR: Determinate Existence as Such',
    section: 'BEING - QUALITY - A. Existence as Such',
    sourceFile: 'existence.txt',
    stateIds: ['existence-1', 'existence-9', 'existence-16'],
  },
  {
    fileName: 'something-and-other-ir.ts',
    exportName: 'somethingAndOtherIR',
    statesExportName: 'somethingAndOtherStates',
    irId: 'something-and-other-ir',
    title: 'Something and Other IR: Relational Finitude',
    section: 'BEING - QUALITY - B. Finitude - a. Something and Other',
    sourceFile: 'something-and-other.txt',
    stateIds: [
      'something-and-other-1',
      'something-and-other-9',
      'something-and-other-17',
    ],
  },
  {
    fileName: 'constitution-ir.ts',
    exportName: 'constitutionIR',
    statesExportName: 'constitutionStates',
    irId: 'constitution-ir',
    title: 'Constitution IR: Determination, Constitution, and Limit',
    section:
      'BEING - QUALITY - B. Finitude - b. Determination, Constitution, and Limit',
    sourceFile: 'constitution.txt',
    stateIds: ['constitution-1', 'constitution-10', 'constitution-16'],
  },
  {
    fileName: 'finitude-ir.ts',
    exportName: 'finitudeIR',
    statesExportName: 'finitudeStates',
    irId: 'finitude-ir',
    title: 'Finitude IR: Restriction, Ought, and Transition to Infinite',
    section: 'BEING - QUALITY - B. Finitude - c. Finitude',
    sourceFile: 'finitude.txt',
    stateIds: ['finitude-1', 'finitude-8', 'finitude-13'],
  },
  {
    fileName: 'infinity-ir.ts',
    exportName: 'infinityIR',
    statesExportName: 'infinityStates',
    irId: 'infinity-ir',
    title: 'Infinity IR: Infinite in General',
    section: 'BEING - QUALITY - C. Infinity - a. The Infinite in General',
    sourceFile: 'infinity.txt',
    stateIds: ['infinity-1', 'infinity-4', 'infinity-6'],
  },
  {
    fileName: 'alternating-infinity-ir.ts',
    exportName: 'alternatingInfinityIR',
    statesExportName: 'alternatingInfinityStates',
    irId: 'alternating-infinity-ir',
    title:
      'Alternating Infinity IR: Bad Infinite, Contradiction, Progress to Infinity',
    section:
      'I. BEING - A. QUALITY - C. Infinity - B. Alternating Determination',
    sourceFile: 'alternating-infinity.txt',
    stateIds: [
      'alternating-infinity-1',
      'alternating-infinity-5',
      'alternating-infinity-12',
    ],
  },
  {
    fileName: 'affirmative-infinity-ir.ts',
    exportName: 'affirmativeInfinityIR',
    statesExportName: 'affirmativeInfinityStates',
    irId: 'affirmative-infinity-ir',
    title:
      'Affirmative Infinity IR: Unity, True Infinite as Becoming, True Infinite as Being',
    section: 'I. BEING - A. QUALITY - C. Infinity - C. Affirmative Infinity',
    sourceFile: 'affirmative-infinity.txt',
    stateIds: [
      'affirmative-infinity-7',
      'affirmative-infinity-15',
      'affirmative-infinity-16',
    ],
  },
];

function cypherEscape(value: string): string {
  return value
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\r/g, '\\r')
    .replace(/\n/g, '\\n');
}

function toCypherString(value: string): string {
  return `'${cypherEscape(value)}'`;
}

function toCypherArray(values: string[]): string {
  return `[${values.map(toCypherString).join(', ')}]`;
}

function extractChunkText(
  sourceText: string,
  lineRange: { start: number; end: number },
): string {
  const lines = sourceText.split(/\r?\n/);
  return lines
    .slice(lineRange.start - 1, lineRange.end)
    .join('\n')
    .trim();
}

function inferTraceType(nextChunk: IntegratedChunk): TraceType {
  const body =
    `${nextChunk.title} ${nextChunk.description} ${nextChunk.keyPoints.join(' ')}`.toLowerCase();

  if (body.includes('negation') || body.includes('negative')) return 'NEGATES';
  if (body.includes('sublat')) return 'SUBLATES';
  if (body.includes('mediate')) return 'MEDIATES';
  if (body.includes('reflect')) return 'REFLECTS';
  return 'NEXT';
}

function deriveTags(chunk: IntegratedChunk): string[] {
  const body =
    `${chunk.title} ${chunk.description} ${chunk.keyPoints.join(' ')}`.toLowerCase();
  const tags = new Set<string>();

  if (body.includes('negation') || body.includes('negative')) {
    tags.add('negation');
  }
  if (body.includes('sublat')) {
    tags.add('sublation');
  }
  if (body.includes('reflect')) {
    tags.add('reflection');
  }
  if (body.includes('mediate')) {
    tags.add('mediation');
  }
  if (body.includes('finite')) {
    tags.add('finite');
  }
  if (body.includes('infinite')) {
    tags.add('infinite');
  }
  if (body.includes('existence')) {
    tags.add('existence');
  }
  if (body.includes('limit')) {
    tags.add('limit');
  }

  return [...tags];
}

function buildCypher(ir: IntegratedIR): string {
  const statements: string[] = [];

  statements.push('// Integrated TopicMap Cypher IR (debug mode)');
  statements.push('// Generated by src/tools/generate-existence-ir.ts');
  statements.push('');
  statements.push(
    'CREATE CONSTRAINT integrated_source_id IF NOT EXISTS FOR (s:SourceText) REQUIRE s.id IS UNIQUE;',
  );
  statements.push(
    'CREATE CONSTRAINT integrated_chunk_id IF NOT EXISTS FOR (c:IntegratedChunk) REQUIRE c.id IS UNIQUE;',
  );
  statements.push(
    'CREATE CONSTRAINT integrated_kp_id IF NOT EXISTS FOR (k:KeyPoint) REQUIRE k.id IS UNIQUE;',
  );
  statements.push('');

  statements.push(
    `MERGE (ir:IntegratedIR {id: ${toCypherString(ir.id)}}) SET ir.mode = ${toCypherString(ir.mode)}, ir.title = ${toCypherString(ir.title)}, ir.section = ${toCypherString(ir.section)}, ir.generatedAt = ${toCypherString(ir.metadata.generatedAt)}, ir.totalSources = ${ir.metadata.totalSources}, ir.totalChunks = ${ir.metadata.totalChunks};`,
  );
  statements.push('');

  for (const source of ir.sourceDocuments) {
    statements.push(`MERGE (s:SourceText {id: ${toCypherString(source.id)}})`);
    statements.push('SET s.title = ' + toCypherString(source.title));
    statements.push('SET s.sourceFile = ' + toCypherString(source.sourceFile));
    statements.push('SET s.totalLines = ' + String(source.totalLines) + ';');
    statements.push(`MATCH (ir:IntegratedIR {id: ${toCypherString(ir.id)}})`);
    statements.push(`MATCH (s:SourceText {id: ${toCypherString(source.id)}})`);
    statements.push('MERGE (ir)-[:HAS_SOURCE]->(s);');

    for (const chunk of source.chunks) {
      statements.push(
        `MERGE (c:IntegratedChunk {id: ${toCypherString(chunk.id)}})`,
      );
      statements.push('SET c.title = ' + toCypherString(chunk.title));
      statements.push('SET c.sourceId = ' + toCypherString(chunk.sourceId));
      statements.push('SET c.sourceFile = ' + toCypherString(chunk.sourceFile));
      statements.push('SET c.lineStart = ' + String(chunk.lineRange.start));
      statements.push('SET c.lineEnd = ' + String(chunk.lineRange.end));
      statements.push(
        'SET c.description = ' + toCypherString(chunk.description),
      );
      statements.push('SET c.keyPoints = ' + toCypherArray(chunk.keyPoints));
      statements.push('SET c.tags = ' + toCypherArray(chunk.tags));
      statements.push('SET c.orderInSource = ' + String(chunk.orderInSource));
      statements.push('SET c.globalOrder = ' + String(chunk.globalOrder));
      statements.push(
        'SET c.sourceText = ' + toCypherString(chunk.sourceText) + ';',
      );

      statements.push(
        `MATCH (s:SourceText {id: ${toCypherString(source.id)}})`,
      );
      statements.push(
        `MATCH (c:IntegratedChunk {id: ${toCypherString(chunk.id)}})`,
      );
      statements.push('MERGE (s)-[:HAS_CHUNK]->(c);');

      for (let index = 0; index < chunk.keyPoints.length; index += 1) {
        const keyPoint = chunk.keyPoints[index] ?? '';
        const keyPointId = `${chunk.id}:kp:${index + 1}`;

        statements.push(
          `MERGE (kp:KeyPoint {id: ${toCypherString(keyPointId)}})`,
        );
        statements.push('SET kp.chunkId = ' + toCypherString(chunk.id));
        statements.push('SET kp.ordinal = ' + String(index + 1));
        statements.push('SET kp.text = ' + toCypherString(keyPoint) + ';');
        statements.push(
          `MATCH (c:IntegratedChunk {id: ${toCypherString(chunk.id)}})`,
        );
        statements.push(
          `MATCH (kp:KeyPoint {id: ${toCypherString(keyPointId)}})`,
        );
        statements.push('MERGE (c)-[:HAS_KEY_POINT]->(kp);');
      }
    }
  }

  for (const trace of ir.traces) {
    statements.push(
      `MATCH (a:IntegratedChunk {id: ${toCypherString(trace.fromChunkId)}})`,
    );
    statements.push(
      `MATCH (b:IntegratedChunk {id: ${toCypherString(trace.toChunkId)}})`,
    );
    statements.push(`MERGE (a)-[r:${trace.type}]->(b)`);
    statements.push('SET r.reason = ' + toCypherString(trace.reason) + ';');
  }

  return statements.join('\n');
}

function buildQueryPack(ir: IntegratedIR): string {
  return [
    '// Neo4j Query Pack — Integrated Existence IR',
    `// IR: ${ir.id}`,
    `// Generated for section: ${ir.section}`,
    '',
    '// Q1: Graph inventory by labels',
    'MATCH (n)',
    'UNWIND labels(n) AS label',
    'RETURN label, count(*) AS count',
    'ORDER BY count DESC;',
    '',
    '// Q2: Source -> Chunk chain',
    'MATCH (s:SourceText)-[:HAS_CHUNK]->(c:IntegratedChunk)',
    'RETURN s.id AS sourceId, c.id AS chunkId, c.globalOrder AS globalOrder, c.lineStart AS lineStart, c.lineEnd AS lineEnd',
    'ORDER BY c.globalOrder;',
    '',
    '// Q3: Cross-source trace edges',
    'MATCH (a:IntegratedChunk)-[r:NEXT|NEGATES|SUBLATES|REFLECTS|MEDIATES]->(b:IntegratedChunk)',
    'WHERE a.sourceId <> b.sourceId',
    'RETURN a.id AS fromChunk, a.sourceId AS fromSource, type(r) AS rel, b.id AS toChunk, b.sourceId AS toSource, r.reason AS reason',
    'ORDER BY a.globalOrder;',
    '',
    '// Q4: Key points for one chunk (replace chunk id)',
    "MATCH (c:IntegratedChunk {id: 'affirmative-infinity-7'})-[:HAS_KEY_POINT]->(k:KeyPoint)",
    'RETURN c.id AS chunkId, k.ordinal AS ordinal, k.text AS keyPoint',
    'ORDER BY k.ordinal;',
  ].join('\n');
}

function toPascalIdentifier(value: string): string {
  const normalized = value
    .replace(/[^a-zA-Z0-9]+/g, ' ')
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .map((part) => part[0]?.toUpperCase() + part.slice(1))
    .join('');

  if (!normalized) {
    return 'Item';
  }

  if (/^[0-9]/.test(normalized)) {
    return `N${normalized}`;
  }

  return normalized;
}

function renderIntegratedTopicMapFile(
  generatedBy: string,
  constName: string,
  typeName: string,
  sourcePrefix: string,
  ir: IntegratedIR,
): string {
  const sourceLines = ir.sourceDocuments.map((source) => {
    const sourceConst = `${sourcePrefix}Source${toPascalIdentifier(source.id)}`;
    return {
      sourceConst,
      line: `const ${sourceConst} = ${JSON.stringify(source, null, 2)} as const;`,
    };
  });

  const sourceDocList = sourceLines
    .map((source) => source.sourceConst)
    .join(',\n  ');

  return [
    `// Generated by ${generatedBy}`,
    '',
    ...sourceLines.map((source) => source.line),
    '',
    `const ${sourcePrefix}Sources = [`,
    `  ${sourceDocList},`,
    '] as const;',
    '',
    `const ${sourcePrefix}Traces = ${JSON.stringify(ir.traces, null, 2)} as const;`,
    '',
    `const ${sourcePrefix}Metadata = ${JSON.stringify(ir.metadata, null, 2)} as const;`,
    '',
    `export const ${constName} = {`,
    `  id: ${JSON.stringify(ir.id)},`,
    `  mode: ${JSON.stringify(ir.mode)},`,
    `  title: ${JSON.stringify(ir.title)},`,
    `  section: ${JSON.stringify(ir.section)},`,
    `  sourceDocuments: ${sourcePrefix}Sources,`,
    `  traces: ${sourcePrefix}Traces,`,
    `  metadata: ${sourcePrefix}Metadata,`,
    '} as const;',
    '',
    `export type ${typeName} = typeof ${constName};`,
    '',
  ].join('\n');
}

function renderSectionDialecticIR(
  generatedBy: string,
  exportName: string,
  statesExportName: string,
  ir: DialecticIR,
): string {
  const stateEntries = ir.states.map((state, index) => {
    const baseName = toPascalIdentifier(state.id);
    const constName = `state${baseName || `Index${index + 1}`}`;
    return {
      id: state.id,
      constName,
      content: `const ${constName}: DialecticState = ${JSON.stringify(state, null, 2)};`,
    };
  });

  const stateList = stateEntries
    .map((entry) => entry.constName)
    .join(',\n    ');

  const stateMapEntries = stateEntries
    .map((entry) => `  ${JSON.stringify(entry.id)}: ${entry.constName},`)
    .join('\n');

  return [
    "import type { DialecticIR, DialecticState } from '@schema/dialectic';",
    '',
    `// Generated by ${generatedBy}`,
    ...stateEntries.map((entry) => entry.content),
    '',
    `export const ${exportName}: DialecticIR = {`,
    `  id: ${JSON.stringify(ir.id)},`,
    `  title: ${JSON.stringify(ir.title)},`,
    `  section: ${JSON.stringify(ir.section)},`,
    '  states: [',
    `    ${stateList},`,
    '  ],',
    `  metadata: ${JSON.stringify(ir.metadata, null, 2)},`,
    '};',
    '',
    `export const ${statesExportName} = {`,
    stateMapEntries,
    '};',
    '',
  ].join('\n');
}

function cloneState(state: DialecticState): DialecticState {
  return JSON.parse(JSON.stringify(state)) as DialecticState;
}

function buildChapterDialecticIR(): DialecticIR {
  const states: DialecticState[] = [
    ...existenceIR.states.map(cloneState),
    ...somethingAndOtherIR.states.map(cloneState),
    ...constitutionIR.states.map(cloneState),
    ...finitudeIR.states.map(cloneState),
    ...infinityIR.states.map(cloneState),
    ...alternatingInfinityIR.states.map(cloneState),
    ...affirmativeInfinityIR.states.map(cloneState),
  ];

  const cpuGpuMapping = Object.fromEntries(
    states.map((state) => [state.id, state.phase]),
  );

  return {
    id: 'existence-chapter-ir',
    title: 'Existence Chapter IR: Existence, Finitude, and Infinity',
    section: 'BEING - QUALITY - EXISTENCE CHAPTER',
    states,
    metadata: {
      sourceFile: 'existence-chapter',
      totalStates: states.length,
      cpuGpuMapping,
      updatedAt: new Date().toISOString(),
    },
  };
}

function buildSectionCompatibilityIR(
  chapterStates: DialecticState[],
  spec: SectionCompatibilitySpec,
): DialecticIR {
  const stateById = new Map(chapterStates.map((state) => [state.id, state]));

  const states = spec.stateIds.map((id) => {
    const found = stateById.get(id);
    if (!found) {
      throw new Error(
        `Missing chapter state ${id} while building ${spec.fileName}`,
      );
    }
    return cloneState(found);
  });

  const cpuGpuMapping = Object.fromEntries(
    states.map((state) => [state.id, state.phase]),
  );

  return {
    id: spec.irId,
    title: spec.title,
    section: spec.section,
    states,
    metadata: {
      sourceFile: spec.sourceFile,
      totalStates: states.length,
      cpuGpuMapping,
      updatedAt: new Date().toISOString(),
    },
  };
}

async function main() {
  const here = path.dirname(fileURLToPath(import.meta.url));
  const packageRoot = path.resolve(here, '..', '..');

  const sourceSpecs: SourceSpec[] = [
    {
      id: 'source-existence',
      title: 'A. EXISTENCE AS SUCH',
      sourceFile: 'relative/being/quality/existence/sources/existence.txt',
      topicMap: existenceTopicMap,
    },
    {
      id: 'source-something-and-other',
      title: 'B. FINITUDE (a) Something and other',
      sourceFile:
        'relative/being/quality/existence/sources/something-and-other.txt',
      topicMap: somethingAndOtherTopicMap,
    },
    {
      id: 'source-constitution',
      title: 'B. FINITUDE (b) Determination, constitution, and limit',
      sourceFile: 'relative/being/quality/existence/sources/constitution.txt',
      topicMap: constitutionTopicMap,
    },
    {
      id: 'source-finitude',
      title: 'B. FINITUDE (c) Finitude',
      sourceFile: 'relative/being/quality/existence/sources/finitude.txt',
      topicMap: finitudeTopicMap,
    },
    {
      id: 'source-infinity',
      title: 'C. INFINITY (a) The infinite in general',
      sourceFile: 'relative/being/quality/existence/sources/infinity.txt',
      topicMap: infinityTopicMap,
    },
    {
      id: 'source-alternating-infinity',
      title: 'C. INFINITY (b) Alternating determination of finite and infinite',
      sourceFile:
        'relative/being/quality/existence/sources/alternating-infinity.txt',
      topicMap: alternatingInfinityTopicMap,
    },
    {
      id: 'source-affirmative-infinity',
      title: 'C. INFINITY (c) Affirmative infinity',
      sourceFile:
        'relative/being/quality/existence/sources/affirmative-infinity.txt',
      topicMap: affirmativeInfinityTopicMap,
    },
  ];

  let globalOrder = 1;
  const traces: IntegratedTrace[] = [];
  const sourceDocuments: IntegratedSourceDocument[] = [];

  for (const sourceSpec of sourceSpecs) {
    const sourceAbsPath = path.join(packageRoot, 'src', sourceSpec.sourceFile);
    const fullSource = await fs.readFile(sourceAbsPath, 'utf8');
    const totalLines = fullSource.split(/\r?\n/).length;

    const chunks: IntegratedChunk[] = sourceSpec.topicMap.map(
      (entry, index) => {
        const sourceText = extractChunkText(fullSource, entry.lineRange);
        const chunk: IntegratedChunk = {
          id: entry.id,
          title: entry.title,
          sourceId: sourceSpec.id,
          sourceFile: sourceSpec.sourceFile,
          lineRange: entry.lineRange,
          description: entry.description,
          keyPoints: entry.keyPoints,
          orderInSource: index + 1,
          globalOrder,
          sourceText,
          tags: [],
        };

        chunk.tags = deriveTags(chunk);
        globalOrder += 1;
        return chunk;
      },
    );

    for (let index = 0; index < chunks.length - 1; index += 1) {
      const current = chunks[index];
      const next = chunks[index + 1];
      if (!current || !next) continue;

      traces.push({
        fromChunkId: current.id,
        toChunkId: next.id,
        type: 'NEXT',
        reason: 'Sequential order in source text.',
      });

      const inferredType = inferTraceType(next);
      if (inferredType !== 'NEXT') {
        traces.push({
          fromChunkId: current.id,
          toChunkId: next.id,
          type: inferredType,
          reason: `Dialectical transition inferred from ${next.id} semantics.`,
        });
      }
    }

    sourceDocuments.push({
      id: sourceSpec.id,
      title: sourceSpec.title,
      sourceFile: sourceSpec.sourceFile,
      totalLines,
      chunks,
    });
  }

  traces.push({
    fromChunkId: 'existence-16',
    toChunkId: 'something-and-other-1',
    type: 'SUBLATES',
    reason: 'Something passes into explicit something-and-other relation.',
  });

  traces.push({
    fromChunkId: 'something-and-other-17',
    toChunkId: 'constitution-1',
    type: 'SUBLATES',
    reason: 'Reflected determinateness passes into determination/constitution.',
  });

  traces.push({
    fromChunkId: 'constitution-16',
    toChunkId: 'finitude-1',
    type: 'SUBLATES',
    reason: 'Immanent limit as contradiction yields finitude proper.',
  });

  traces.push({
    fromChunkId: 'finitude-13',
    toChunkId: 'infinity-1',
    type: 'SUBLATES',
    reason: 'Finite rejoining itself yields the infinite.',
  });

  traces.push({
    fromChunkId: 'infinity-6',
    toChunkId: 'alternating-infinity-1',
    type: 'NEGATES',
    reason: 'Infinite result unfolds as the bad infinite progression.',
  });

  traces.push({
    fromChunkId: 'alternating-infinity-12',
    toChunkId: 'affirmative-infinity-7',
    type: 'SUBLATES',
    reason: 'Unresolved progress is sublated into true/affirmative infinity.',
  });

  const topicMapIR: IntegratedIR = {
    id: 'integrated-existence-topicmap-ir',
    mode: 'debug',
    title: 'Integrated Existence TopicMap IR',
    section: 'Doctrine of Being / Quality / Existence',
    sourceDocuments,
    traces,
    metadata: {
      totalSources: sourceDocuments.length,
      totalChunks: sourceDocuments.reduce(
        (sum, source) => sum + source.chunks.length,
        0,
      ),
      generatedAt: new Date().toISOString(),
    },
  };

  const chapterIR = buildChapterDialecticIR();
  const parsedChapter = DialecticIRSchema.safeParse(chapterIR);
  if (!parsedChapter.success) {
    const issues = parsedChapter.error.issues
      .map((issue) => `${issue.path.join('.')}: ${issue.message}`)
      .join('\n');
    throw new Error(
      `Existence chapter DialecticIR validation failed:\n${issues}`,
    );
  }

  const topicOutputDir = path.join(
    packageRoot,
    'src',
    'relative',
    'being',
    'quality',
    'existence',
    'sources',
    'generated',
  );

  await fs.mkdir(topicOutputDir, { recursive: true });

  const topicCypherPath = path.join(
    topicOutputDir,
    'integrated-topicmap-ir.cypher',
  );
  const topicQueryPath = path.join(
    topicOutputDir,
    'integrated-topicmap-query-pack.cypher',
  );
  const topicDebugPath = path.join(
    topicOutputDir,
    'integrated-topicmap-ir.debug.ts',
  );

  const chapterOutPath = path.join(
    packageRoot,
    'src',
    'relative',
    'being',
    'quality',
    'existence',
    'existence-chapter-ir.ts',
  );

  const topLevelTopicMapOutPath = path.join(
    packageRoot,
    'src',
    'relative',
    'being',
    'quality',
    'existence',
    'integrated-topicmap-ir.ts',
  );

  const topicDebugTs = renderIntegratedTopicMapFile(
    'src/tools/generate-existence-ir.ts',
    'integratedTopicMapDebug',
    'IntegratedTopicMapDebug',
    'integratedTopicMapDebug',
    topicMapIR,
  );

  const topLevelTopicMapTs = renderIntegratedTopicMapFile(
    'src/tools/generate-existence-ir.ts',
    'integratedExistenceTopicMapIR',
    'IntegratedExistenceTopicMapIR',
    'integratedExistenceTopicMap',
    topicMapIR,
  );

  const chapterTs = renderSectionDialecticIR(
    'src/tools/generate-existence-ir.ts',
    'existenceChapterIR',
    'existenceChapterStateMap',
    parsedChapter.data,
  );

  const sectionOutDir = path.join(
    packageRoot,
    'src',
    'relative',
    'being',
    'quality',
    'existence',
  );

  const sectionOutputs = sectionCompatibilitySpecs.map((spec) => {
    const sectionIR = buildSectionCompatibilityIR(
      parsedChapter.data.states,
      spec,
    );
    const parsedSection = DialecticIRSchema.safeParse(sectionIR);
    if (!parsedSection.success) {
      const issues = parsedSection.error.issues
        .map((issue) => `${issue.path.join('.')}: ${issue.message}`)
        .join('\n');
      throw new Error(
        `Section compatibility IR validation failed (${spec.fileName}):\n${issues}`,
      );
    }

    const sectionTs = renderSectionDialecticIR(
      'src/tools/generate-existence-ir.ts',
      spec.exportName,
      spec.statesExportName,
      parsedSection.data,
    );

    return {
      path: path.join(sectionOutDir, spec.fileName),
      content: sectionTs,
    };
  });

  await fs.writeFile(topicCypherPath, buildCypher(topicMapIR), 'utf8');
  await fs.writeFile(topicQueryPath, buildQueryPack(topicMapIR), 'utf8');
  await fs.writeFile(topicDebugPath, topicDebugTs, 'utf8');
  await fs.writeFile(topLevelTopicMapOutPath, topLevelTopicMapTs, 'utf8');
  await fs.writeFile(chapterOutPath, chapterTs, 'utf8');
  await Promise.all(
    sectionOutputs.map((output) =>
      fs.writeFile(output.path, output.content, 'utf8'),
    ),
  );

  console.log(
    `Generated ${path.relative(packageRoot, topicCypherPath)}, ${path.relative(packageRoot, topicQueryPath)}, ${path.relative(packageRoot, topicDebugPath)}, ${path.relative(packageRoot, topLevelTopicMapOutPath)}, ${path.relative(packageRoot, chapterOutPath)}, and ${sectionOutputs.length} section compatibility IR files.`,
  );
}

await main();
