export type SankaraMode = 'markdown' | 'source' | 'literal' | 'technical';
export type SankaraQueue = 'normal' | 'review' | 'strict';
export type SankaraRecordKind =
  | 'sutra'
  | 'bhasya'
  | 'preamble'
  | 'transition'
  | 'unknown';
export type SankaraRootLanguage = 'gdsl' | 'sdsl';
export type SankaraPersistenceTarget =
  | 'prisma'
  | 'postgres-generator'
  | 'memory';
export type SankaraRuntimeSurface = 'local-static-viewer';
export type SankaraFrameworkTarget = 'none';
export type SankaraArtifactGrade = 'research-grade' | 'management-meta';
export type SankaraWorkbenchClass = 'ts-agential-programmers-workbench';
export type SankaraViewerStepKind =
  | 'select-work'
  | 'focus-section'
  | 'focus-record'
  | 'set-mode'
  | 'set-query'
  | 'filter-section'
  | 'anchor-records'
  | 'filter-kind'
  | 'filter-topic'
  | 'filter-transition'
  | 'filter-co-located';

export interface SankaraAnchor {
  work?: string;
  chapter?: number;
  section?: number;
  sutra?: number;
}

export interface SankaraSemanticQuery {
  anchor?: SankaraAnchor;
  includeKinds?: SankaraRecordKind[];
  topicTags?: string[];
  transitionTerms?: string[];
  coLocatedOnly?: boolean;
}

export type SankaraFrameAxis =
  | 'work'
  | 'anchor'
  | 'record'
  | 'kind'
  | 'topic'
  | 'transition'
  | 'query'
  | 'section';

export interface SankaraFrame {
  work: string;
  section?: string;
  record?: string;
  anchor?: SankaraAnchor;
  kinds: SankaraRecordKind[];
  topics: string[];
  transitions: string[];
  queryText?: string;
  sectionFilter?: string;
  coLocatedOnly: boolean;
}

export interface SankaraModelDSL {
  frame: SankaraFrame;
  manifest: 'records';
  selection: 'filtered-records';
}

export interface SankaraViewDSL {
  mode: SankaraMode;
  focus: 'record' | 'anchor-field' | 'section-field';
  showKinds: SankaraRecordKind[];
}

export interface SankaraControllerDSL {
  queue: SankaraQueue;
  threshold: number;
  canonicalizeUrl: boolean;
  preserveSelection: boolean;
}

export interface SankaraRootComponentModel {
  frame: SankaraFrame;
  model: SankaraModelDSL;
  view: SankaraViewDSL;
  controller: SankaraControllerDSL;
}

export interface SankaraPrototypeLanguageSpec {
  metaLanguage: SankaraRootLanguage;
  sourceLanguage: SankaraRootLanguage;
  sourceModelId: string;
  sourceModelKind: string;
}

export interface SankaraAgentialMVCSpec {
  kind: 'prototype-agential-mvc';
  logicalMvc: 'meta-sdsl';
  agentMvcFactory: 'AgentMVC';
  root: SankaraRootComponentModel;
  language: SankaraPrototypeLanguageSpec;
}

export interface SankaraViewerSurfaceSpec {
  runtimeSurface: SankaraRuntimeSurface;
  keepsLocalViewer: true;
  targetedFramework: SankaraFrameworkTarget;
  dataProcessor: 'manifest-markdown-browser';
}

export interface SankaraExampleAppSpec {
  kind: 'example-app';
  runtime: SankaraViewerSurfaceSpec;
  root: SankaraRootComponentModel;
  maturity: 'prototype';
}

export interface SankaraSchemaSpec {
  kind: 'prototype-schema';
  sourceLanguage: SankaraRootLanguage;
  metaLanguage: SankaraRootLanguage;
  relationModel: 'normalized-records';
  manifestShape: 'record-manifest';
  languageModel: SankaraManifestLanguageModelSpec;
}

export interface SankaraShellNamespaceSpec {
  id: string;
  role: 'corpus' | 'workbench' | 'planning';
}

export interface SankaraShellMacroSpec {
  name: string;
  arity: number;
  mapsSteps: SankaraViewerStepKind[];
}

export interface SankaraShellFunctionSpec {
  name: string;
  inputs: string[];
  output: string;
}

export interface SankaraShellExprSpec {
  forms: Array<'select' | 'filter' | 'anchor' | 'compose' | 'plan'>;
}

export interface SankaraShellLanguageInstanceSpec {
  kind: 'shell-language-instance';
  base: 'rust-internal-dsl';
  namespaces: SankaraShellNamespaceSpec[];
  macros: SankaraShellMacroSpec[];
  functions: SankaraShellFunctionSpec[];
  expr: SankaraShellExprSpec;
}

export interface SankaraManifestLanguageModelSpec {
  kind: 'proto-language-model';
  source: 'record-manifest';
  role: 'pseudo-external-dsl';
  planningLanguage: 'awpl-prototype';
  instance: SankaraShellLanguageInstanceSpec;
}

export interface SankaraCorpusArtifactSpec {
  kind: 'sankara-corpus-artifact';
  grade: 'research-grade';
  sourceLanguage: SankaraRootLanguage;
  translationStrata: Array<'sutra' | 'bhasya' | 'preamble' | 'transition'>;
  layout: 'chapter-section-record';
  preservationPolicy: 'append-and-supersede-never-delete';
  canonicalArtifacts: Array<
    | 'passage-records'
    | 'translation-jsonl'
    | 'record-manifest'
    | 'markdown-records'
  >;
  editorialRule: 'workbench-meta-must-not-overwrite-corpus';
}

export interface SankaraManagementMetaSpec {
  kind: 'sankara-management-meta';
  grade: 'management-meta';
  captures: Array<'viewer-steps' | 'macro-targets' | 'function-targets'>;
  analysisPolicy: SankaraAnalysisPolicySpec;
  logicalNotes: SankaraLogicalNotesSpec;
  curation: SankaraCurationSpec;
  agentKnowledge: SankaraAgentKnowledgeSpec;
  editorialRule: 'management-meta-may-describe-but-not-redefine-corpus';
}

export interface SankaraAnalysisPolicySpec {
  kind: 'analysis-policy';
  mode: 'technical-vocabulary-only';
  vocabularySources: Array<
    'record-manifest' | 'translation-notes' | 'domain-glossary'
  >;
  unknownTermHandling: 'flag-for-curation';
}

export interface SankaraLogicalNotesSpec {
  kind: 'logical-notes';
  enabled: true;
  semanticState: 'open-undefined';
  acceptedForms: Array<'gloss' | 'axiom' | 'bridge-note' | 'question'>;
  interpretationRule: 'human-curated-disambiguation';
}

export type SankaraAnnotationLabel =
  | 'mantra'
  | 'bhasya'
  | 'preamble'
  | 'transition'
  | 'technical-term'
  | 'logical-note';

export interface SankaraCorpusDocumentInput {
  kind: 'corpus-document-input';
  documentId: string;
  workCode: string;
  sectionCode: string;
  recordId?: string;
  content: string;
}

export interface SankaraAnnotationOutput {
  label: SankaraAnnotationLabel;
  start: number;
  end: number;
  text: string;
  vocabulary: 'technical' | 'logical-open';
}

export interface SankaraAnalyticalOutput {
  kind: 'analytical-output';
  mode: 'technical-vocabulary-only';
  annotations: SankaraAnnotationOutput[];
  summaryTerms: string[];
}

export interface SankaraCorpusAnnotationExample {
  kind: 'corpus-annotation-example';
  input: SankaraCorpusDocumentInput;
  output: SankaraAnalyticalOutput;
}

export interface SankaraCurationSpec {
  kind: 'curation-profile';
  workbenchClass: SankaraWorkbenchClass;
  requiresDomainCuration: true;
  minimumTranslationNoteLevel: 'lexical-notes';
  reviewExclusions: Array<'qa-uncertainty-notes' | 'provenance-review'>;
  sourceTrackPolicy: {
    mustRetainSourceTrack: true;
    preferredSourceTrack: 'sringeri.net';
  };
  protectedFields: Array<
    | 'translation.literal_translation'
    | 'translation.technical_translation'
    | 'translation.interpretive_note'
    | 'provenance'
    | 'source.track'
  >;
  structurableFields: Array<
    | 'record_kind'
    | 'analysis.tags'
    | 'analysis.lexical_notes'
    | 'analysis.transition_notes'
    | 'analysis.logical_notes'
  >;
  curationAxes: Array<
    | 'record-kind-boundary'
    | 'transition-signal-boundary'
    | 'chapter-section-layout'
    | 'translation-strata-preservation'
  >;
  reviewQueues: SankaraQueue[];
}

export interface SankaraAgentKnowledgeSpec {
  kind: 'agent-knowledge-profile';
  requiredDomains: Array<
    | 'sankara-corpus'
    | 'sanskrit-transition-signals'
    | 'kant-hegel-dialectics'
    | 'translation-editorial-policy'
  >;
  minimumGuidance: 'human-curated';
}

export interface SankaraUpgradeSpec {
  exampleApp: SankaraExampleAppSpec;
  schema: SankaraSchemaSpec;
  corpus: SankaraCorpusArtifactSpec;
  management: SankaraManagementMetaSpec;
  upgrades: Array<
    | 'workbench-recorder'
    | 'component-model'
    | 'manifest-language-model'
    | 'shell-language-instance'
    | 'awpl-notes'
    | 'corpus-annotation-examples'
    | 'logical-notes-taxonomy'
    | 'intent-dsl'
    | 'anchor-schema'
    | 'topic-schema'
    | 'transition-schema'
    | 'agent-projection'
  >;
}

export interface SankaraViewerStep {
  kind: SankaraViewerStepKind;
  value?: string;
}

export interface SankaraMacroSpec {
  name: string;
  recodes: SankaraViewerStepKind[];
  target: 'gdsl-macro';
}

export interface SankaraFunctionSpec {
  name: string;
  recodes: SankaraViewerStepKind[];
  target: 'dataset-function';
}

export interface SankaraWorkbenchRecordingSpec {
  kind: 'viewer-workbench-recording';
  runtime: SankaraViewerSurfaceSpec;
  steps: SankaraViewerStep[];
  macroTargets: SankaraMacroSpec[];
  functionTargets: SankaraFunctionSpec[];
}

export interface SankaraComponentModelSpec {
  kind: 'sankara-component-model';
  title: 'Sankaracaryay Component Model';
  root: SankaraRootComponentModel;
  language: SankaraPrototypeLanguageSpec;
  viewer: SankaraViewerSurfaceSpec;
  corpus: SankaraCorpusArtifactSpec;
  management: SankaraManagementMetaSpec;
  workbench: SankaraWorkbenchRecordingSpec;
}

export interface SankaraGeneratorSpec {
  target: SankaraPersistenceTarget;
  relationModel: 'normalized-records';
  appKind: 'prototype-meta-mvc';
  viewerSurface: SankaraViewerSurfaceSpec;
  agential: SankaraAgentialMVCSpec;
  componentModel: SankaraComponentModelSpec;
}

export interface SankaraRouteState {
  work: string;
  section?: string;
  record?: string;
  mode: SankaraMode;
  queue: SankaraQueue;
  threshold: number;
  q?: string;
  sectionFilter?: string;
  semantic?: SankaraSemanticQuery;
}

export interface SankaraPreferencesV1 {
  lastMode: SankaraMode;
  lastQueue: SankaraQueue;
  lastThreshold: number;
  lastSectionFilter?: string;
}

export interface SankaraManifestRecord {
  record_id: string;
  work_code: string;
  section_code: string;
}

export interface SankaraManifest {
  records: SankaraManifestRecord[];
}

export interface ParseRouteInput {
  path: string;
  search: string;
}

const DEFAULT_MODE: SankaraMode = 'markdown';
const DEFAULT_QUEUE: SankaraQueue = 'normal';
const DEFAULT_THRESHOLD = 0.9;

const VALID_MODES = new Set<SankaraMode>([
  'markdown',
  'source',
  'literal',
  'technical',
]);
const VALID_QUEUES = new Set<SankaraQueue>(['normal', 'review', 'strict']);
const VALID_KINDS = new Set<SankaraRecordKind>([
  'sutra',
  'bhasya',
  'preamble',
  'transition',
  'unknown',
]);

function clampThreshold(value: number): number {
  if (!Number.isFinite(value)) {
    return DEFAULT_THRESHOLD;
  }
  if (value < 0) {
    return 0;
  }
  if (value > 1) {
    return 1;
  }
  return Math.round(value * 100) / 100;
}

function parseMode(value: string | null): SankaraMode {
  if (!value) {
    return DEFAULT_MODE;
  }
  return VALID_MODES.has(value as SankaraMode)
    ? (value as SankaraMode)
    : DEFAULT_MODE;
}

function parseQueue(value: string | null): SankaraQueue {
  if (!value) {
    return DEFAULT_QUEUE;
  }
  return VALID_QUEUES.has(value as SankaraQueue)
    ? (value as SankaraQueue)
    : DEFAULT_QUEUE;
}

function parseThreshold(value: string | null): number {
  if (!value) {
    return DEFAULT_THRESHOLD;
  }
  return clampThreshold(Number.parseFloat(value));
}

function parseStringList(value: string | null): string[] | undefined {
  if (!value) {
    return undefined;
  }
  const items = value
    .split(',')
    .map((x) => x.trim())
    .filter((x) => x.length > 0);
  return items.length > 0 ? items : undefined;
}

function parseKinds(value: string | null): SankaraRecordKind[] | undefined {
  const values = parseStringList(value);
  if (!values) {
    return undefined;
  }
  const kinds = values
    .map((x) => x.toLowerCase())
    .filter((x): x is SankaraRecordKind =>
      VALID_KINDS.has(x as SankaraRecordKind),
    );
  return kinds.length > 0 ? kinds : undefined;
}

function parseAnchor(
  value: string | null,
  fallbackWork: string,
): SankaraAnchor | undefined {
  if (!value) {
    return undefined;
  }
  const parts = value
    .split('.')
    .map((x) => x.trim())
    .filter((x) => x.length > 0);
  if (parts.length === 0) {
    return undefined;
  }

  const asInt = (s: string): number | undefined => {
    const n = Number.parseInt(s, 10);
    return Number.isFinite(n) && n > 0 ? n : undefined;
  };

  if (parts.length === 1) {
    return {
      work: fallbackWork,
      sutra: asInt(parts[0]),
    };
  }

  const chapter = asInt(parts[0]);
  const section = asInt(parts[1]);
  const sutra = parts.length >= 3 ? asInt(parts[2]) : undefined;
  if (!chapter || !section) {
    return undefined;
  }
  return {
    work: fallbackWork,
    chapter,
    section,
    sutra,
  };
}

function parseSemanticQuery(
  params: URLSearchParams,
  fallbackWork: string,
): SankaraSemanticQuery | undefined {
  const includeKinds = parseKinds(params.get('kinds'));
  const topicTags = parseStringList(params.get('topics'));
  const transitionTerms = parseStringList(params.get('tx'));
  const anchor = parseAnchor(params.get('anchor'), fallbackWork);
  const coLocatedRaw = params.get('co_located');
  const coLocatedOnly =
    coLocatedRaw == null
      ? undefined
      : coLocatedRaw === '1' || coLocatedRaw.toLowerCase() === 'true';

  if (
    !includeKinds &&
    !topicTags &&
    !transitionTerms &&
    !anchor &&
    coLocatedOnly == null
  ) {
    return undefined;
  }

  return {
    anchor,
    includeKinds,
    topicTags,
    transitionTerms,
    coLocatedOnly,
  };
}

function isSectionCode(value: string): boolean {
  return /^\d+\.\d+$/.test(value);
}

function splitPath(path: string): string[] {
  return path
    .split('?')[0]
    .split('/')
    .map((x) => x.trim())
    .filter((x) => x.length > 0);
}

export function parseRoute(input: ParseRouteInput): SankaraRouteState {
  const parts = splitPath(input.path);
  const params = new URLSearchParams(
    input.search.startsWith('?') ? input.search.slice(1) : input.search,
  );

  const appIndex = parts.indexOf('sankara');
  const routeParts = appIndex >= 0 ? parts.slice(appIndex + 1) : [];

  const work = routeParts[0] || 'BS';
  const section = routeParts[1] || undefined;
  const record = routeParts[2] || undefined;
  const semantic = parseSemanticQuery(params, work);

  return {
    work,
    section: section && isSectionCode(section) ? section : undefined,
    record,
    mode: parseMode(params.get('mode')),
    queue: parseQueue(params.get('queue')),
    threshold: parseThreshold(params.get('threshold')),
    q: (params.get('q') || '').trim() || undefined,
    sectionFilter: (params.get('section') || '').trim() || undefined,
    semantic,
  };
}

export function mergePreferences(
  routeState: SankaraRouteState,
  preferences: SankaraPreferencesV1 | undefined,
): SankaraRouteState {
  if (!preferences) {
    return routeState;
  }

  return {
    ...routeState,
    mode: routeState.mode || preferences.lastMode,
    queue: routeState.queue || preferences.lastQueue,
    threshold: Number.isFinite(routeState.threshold)
      ? routeState.threshold
      : clampThreshold(preferences.lastThreshold),
    sectionFilter: routeState.sectionFilter || preferences.lastSectionFilter,
  };
}

export function toFrame(state: SankaraRouteState): SankaraFrame {
  return {
    work: state.work,
    section: state.section,
    record: state.record,
    anchor: state.semantic?.anchor,
    kinds: state.semantic?.includeKinds || [],
    topics: state.semantic?.topicTags || [],
    transitions: state.semantic?.transitionTerms || [],
    queryText: state.q,
    sectionFilter: state.sectionFilter,
    coLocatedOnly: state.semantic?.coLocatedOnly || false,
  };
}

export function toRootComponentModel(
  state: SankaraRouteState,
): SankaraRootComponentModel {
  const frame = toFrame(state);
  const showKinds: SankaraRecordKind[] =
    frame.kinds.length > 0 ? frame.kinds : ['sutra', 'bhasya'];
  const focus: SankaraViewDSL['focus'] = frame.record
    ? 'record'
    : frame.anchor
      ? 'anchor-field'
      : 'section-field';

  return {
    frame,
    model: {
      frame,
      manifest: 'records',
      selection: 'filtered-records',
    },
    view: {
      mode: state.mode,
      focus,
      showKinds,
    },
    controller: {
      queue: state.queue,
      threshold: clampThreshold(state.threshold),
      canonicalizeUrl: true,
      preserveSelection: true,
    },
  };
}

export function toPrototypeLanguageSpec(
  state: SankaraRouteState,
): SankaraPrototypeLanguageSpec {
  return {
    metaLanguage: 'sdsl',
    sourceLanguage: 'gdsl',
    sourceModelId: `sankara.${state.work.toLowerCase()}.workbench`,
    sourceModelKind: 'sankara-workbench-model',
  };
}

export function toAgentialMVCSpec(
  state: SankaraRouteState,
): SankaraAgentialMVCSpec {
  return {
    kind: 'prototype-agential-mvc',
    logicalMvc: 'meta-sdsl',
    agentMvcFactory: 'AgentMVC',
    root: toRootComponentModel(state),
    language: toPrototypeLanguageSpec(state),
  };
}

export function toViewerSurfaceSpec(): SankaraViewerSurfaceSpec {
  return {
    runtimeSurface: 'local-static-viewer',
    keepsLocalViewer: true,
    targetedFramework: 'none',
    dataProcessor: 'manifest-markdown-browser',
  };
}

export function toViewerSteps(state: SankaraRouteState): SankaraViewerStep[] {
  const steps: SankaraViewerStep[] = [
    {
      kind: 'select-work',
      value: state.work,
    },
  ];

  if (state.section) {
    steps.push({ kind: 'focus-section', value: state.section });
  }

  if (state.record) {
    steps.push({ kind: 'focus-record', value: state.record });
  }

  steps.push({ kind: 'set-mode', value: state.mode });

  if (state.q) {
    steps.push({ kind: 'set-query', value: state.q });
  }

  if (state.sectionFilter) {
    steps.push({ kind: 'filter-section', value: state.sectionFilter });
  }

  const anchor = state.semantic?.anchor;
  if (anchor) {
    const anchorValue = [anchor.chapter, anchor.section, anchor.sutra]
      .filter((value): value is number => Number.isFinite(value as number))
      .join('.');
    if (anchorValue) {
      steps.push({ kind: 'anchor-records', value: anchorValue });
    }
  }

  for (const kind of state.semantic?.includeKinds || []) {
    steps.push({ kind: 'filter-kind', value: kind });
  }

  for (const topic of state.semantic?.topicTags || []) {
    steps.push({ kind: 'filter-topic', value: topic });
  }

  for (const term of state.semantic?.transitionTerms || []) {
    steps.push({ kind: 'filter-transition', value: term });
  }

  if (state.semantic?.coLocatedOnly) {
    steps.push({ kind: 'filter-co-located', value: 'true' });
  }

  return steps;
}

export function toWorkbenchRecordingSpec(
  state: SankaraRouteState,
): SankaraWorkbenchRecordingSpec {
  return {
    kind: 'viewer-workbench-recording',
    runtime: toViewerSurfaceSpec(),
    steps: toViewerSteps(state),
    macroTargets: [
      {
        name: 'openSutraContext',
        recodes: ['select-work', 'anchor-records', 'filter-kind'],
        target: 'gdsl-macro',
      },
      {
        name: 'traceTransitionWindow',
        recodes: ['anchor-records', 'filter-transition', 'filter-co-located'],
        target: 'gdsl-macro',
      },
    ],
    functionTargets: [
      {
        name: 'selectWorkbenchRecords',
        recodes: ['select-work', 'focus-section', 'focus-record'],
        target: 'dataset-function',
      },
      {
        name: 'filterWorkbenchSemantics',
        recodes: [
          'filter-kind',
          'filter-topic',
          'filter-transition',
          'filter-co-located',
        ],
        target: 'dataset-function',
      },
    ],
  };
}

export function toComponentModelSpec(
  state: SankaraRouteState,
): SankaraComponentModelSpec {
  return {
    kind: 'sankara-component-model',
    title: 'Sankaracaryay Component Model',
    root: toRootComponentModel(state),
    language: toPrototypeLanguageSpec(state),
    viewer: toViewerSurfaceSpec(),
    corpus: toCorpusArtifactSpec(state),
    management: toManagementMetaSpec(),
    workbench: toWorkbenchRecordingSpec(state),
  };
}

export function toExampleAppSpec(
  state: SankaraRouteState,
): SankaraExampleAppSpec {
  return {
    kind: 'example-app',
    runtime: toViewerSurfaceSpec(),
    root: toRootComponentModel(state),
    maturity: 'prototype',
  };
}

export function toSchemaSpec(state: SankaraRouteState): SankaraSchemaSpec {
  const language = toPrototypeLanguageSpec(state);
  return {
    kind: 'prototype-schema',
    sourceLanguage: language.sourceLanguage,
    metaLanguage: language.metaLanguage,
    relationModel: 'normalized-records',
    manifestShape: 'record-manifest',
    languageModel: toManifestLanguageModelSpec(state),
  };
}

export function toShellLanguageInstanceSpec(
  state: SankaraRouteState,
): SankaraShellLanguageInstanceSpec {
  const work = state.work.toLowerCase();
  return {
    kind: 'shell-language-instance',
    base: 'rust-internal-dsl',
    namespaces: [
      { id: `sankara.${work}.corpus`, role: 'corpus' },
      { id: `sankara.${work}.workbench`, role: 'workbench' },
      { id: `sankara.${work}.planning`, role: 'planning' },
    ],
    macros: [
      {
        name: 'open_sutra_context',
        arity: 3,
        mapsSteps: ['select-work', 'anchor-records', 'filter-kind'],
      },
      {
        name: 'trace_transition_window',
        arity: 3,
        mapsSteps: ['anchor-records', 'filter-transition', 'filter-co-located'],
      },
    ],
    functions: [
      {
        name: 'select_workbench_records',
        inputs: ['work', 'section', 'record'],
        output: 'record-set',
      },
      {
        name: 'filter_workbench_semantics',
        inputs: ['record-set', 'kinds', 'topics', 'transitions', 'co_located'],
        output: 'record-set',
      },
      {
        name: 'emit_awpl_plan',
        inputs: ['viewer-steps'],
        output: 'awpl-plan',
      },
    ],
    expr: {
      forms: ['select', 'filter', 'anchor', 'compose', 'plan'],
    },
  };
}

export function toManifestLanguageModelSpec(
  state: SankaraRouteState,
): SankaraManifestLanguageModelSpec {
  return {
    kind: 'proto-language-model',
    source: 'record-manifest',
    role: 'pseudo-external-dsl',
    planningLanguage: 'awpl-prototype',
    instance: toShellLanguageInstanceSpec(state),
  };
}

export function toCorpusArtifactSpec(
  state: SankaraRouteState,
): SankaraCorpusArtifactSpec {
  const language = toPrototypeLanguageSpec(state);
  return {
    kind: 'sankara-corpus-artifact',
    grade: 'research-grade',
    sourceLanguage: language.sourceLanguage,
    translationStrata: ['sutra', 'bhasya', 'preamble', 'transition'],
    layout: 'chapter-section-record',
    preservationPolicy: 'append-and-supersede-never-delete',
    canonicalArtifacts: [
      'passage-records',
      'translation-jsonl',
      'record-manifest',
      'markdown-records',
    ],
    editorialRule: 'workbench-meta-must-not-overwrite-corpus',
  };
}

export function toManagementMetaSpec(): SankaraManagementMetaSpec {
  return {
    kind: 'sankara-management-meta',
    grade: 'management-meta',
    captures: ['viewer-steps', 'macro-targets', 'function-targets'],
    analysisPolicy: {
      kind: 'analysis-policy',
      mode: 'technical-vocabulary-only',
      vocabularySources: [
        'record-manifest',
        'translation-notes',
        'domain-glossary',
      ],
      unknownTermHandling: 'flag-for-curation',
    },
    logicalNotes: {
      kind: 'logical-notes',
      enabled: true,
      semanticState: 'open-undefined',
      acceptedForms: ['gloss', 'axiom', 'bridge-note', 'question'],
      interpretationRule: 'human-curated-disambiguation',
    },
    curation: {
      kind: 'curation-profile',
      workbenchClass: 'ts-agential-programmers-workbench',
      requiresDomainCuration: true,
      minimumTranslationNoteLevel: 'lexical-notes',
      reviewExclusions: ['qa-uncertainty-notes', 'provenance-review'],
      sourceTrackPolicy: {
        mustRetainSourceTrack: true,
        preferredSourceTrack: 'sringeri.net',
      },
      protectedFields: [
        'translation.literal_translation',
        'translation.technical_translation',
        'translation.interpretive_note',
        'provenance',
        'source.track',
      ],
      structurableFields: [
        'record_kind',
        'analysis.tags',
        'analysis.lexical_notes',
        'analysis.transition_notes',
        'analysis.logical_notes',
      ],
      curationAxes: [
        'record-kind-boundary',
        'transition-signal-boundary',
        'chapter-section-layout',
        'translation-strata-preservation',
      ],
      reviewQueues: ['normal', 'review', 'strict'],
    },
    agentKnowledge: {
      kind: 'agent-knowledge-profile',
      requiredDomains: [
        'sankara-corpus',
        'sanskrit-transition-signals',
        'kant-hegel-dialectics',
        'translation-editorial-policy',
      ],
      minimumGuidance: 'human-curated',
    },
    editorialRule: 'management-meta-may-describe-but-not-redefine-corpus',
  };
}

export function toUpgradeSpec(state: SankaraRouteState): SankaraUpgradeSpec {
  return {
    exampleApp: toExampleAppSpec(state),
    schema: toSchemaSpec(state),
    corpus: toCorpusArtifactSpec(state),
    management: toManagementMetaSpec(),
    upgrades: [
      'workbench-recorder',
      'component-model',
      'manifest-language-model',
      'shell-language-instance',
      'awpl-notes',
      'corpus-annotation-examples',
      'logical-notes-taxonomy',
      'intent-dsl',
      'anchor-schema',
      'topic-schema',
      'transition-schema',
      'agent-projection',
    ],
  };
}

export function toCorpusAnnotationExample(
  state: SankaraRouteState,
): SankaraCorpusAnnotationExample {
  const sectionCode = state.section || state.sectionFilter || '1.1';
  const recordId = state.record;
  const inputText = state.q || 'atha ato brahma jijñāsā';

  return {
    kind: 'corpus-annotation-example',
    input: {
      kind: 'corpus-document-input',
      documentId: `doc.${state.work}.${sectionCode}`,
      workCode: state.work,
      sectionCode,
      recordId,
      content: inputText,
    },
    output: {
      kind: 'analytical-output',
      mode: 'technical-vocabulary-only',
      annotations: [
        {
          label: 'technical-term',
          start: 0,
          end: inputText.length,
          text: inputText,
          vocabulary: 'technical',
        },
      ],
      summaryTerms: ['technical-vocabulary-only', 'flag-for-curation'],
    },
  };
}

export function toGeneratorSpec(
  state: SankaraRouteState,
  target: SankaraPersistenceTarget = 'postgres-generator',
): SankaraGeneratorSpec {
  return {
    target,
    relationModel: 'normalized-records',
    appKind: 'prototype-meta-mvc',
    viewerSurface: toViewerSurfaceSpec(),
    agential: toAgentialMVCSpec(state),
    componentModel: toComponentModelSpec(state),
  };
}

export function normalizeAgainstManifest(
  state: SankaraRouteState,
  manifest: SankaraManifest,
): SankaraRouteState {
  const byWork = manifest.records.filter((r) => r.work_code === state.work);
  const effectiveWork = byWork.length > 0 ? state.work : 'BS';
  const records =
    effectiveWork === state.work
      ? byWork
      : manifest.records.filter((r) => r.work_code === 'BS');

  const sections = new Set(records.map((r) => r.section_code));
  const effectiveSection =
    state.section && sections.has(state.section) ? state.section : undefined;

  const filteredBySection = effectiveSection
    ? records.filter((r) => r.section_code === effectiveSection)
    : records;

  const recordExists = state.record
    ? filteredBySection.some((r) => r.record_id === state.record)
    : false;

  const effectiveRecord = recordExists
    ? state.record
    : filteredBySection[0]?.record_id || undefined;

  const sectionFilterValid =
    state.sectionFilter && sections.has(state.sectionFilter)
      ? state.sectionFilter
      : undefined;

  return {
    ...state,
    work: effectiveWork,
    section: effectiveSection,
    record: effectiveRecord,
    sectionFilter: sectionFilterValid,
    threshold: clampThreshold(state.threshold),
    mode: parseMode(state.mode),
    queue: parseQueue(state.queue),
    semantic: state.semantic,
  };
}

export function toCanonicalPath(state: SankaraRouteState): string {
  const segments = ['sankara', state.work];
  if (state.section) {
    segments.push(state.section);
  }
  if (state.record) {
    segments.push(state.record);
  }
  return `/${segments.join('/')}`;
}

export function toCanonicalQuery(state: SankaraRouteState): string {
  const params = new URLSearchParams();

  if (state.mode !== DEFAULT_MODE) {
    params.set('mode', state.mode);
  }
  if (state.queue !== DEFAULT_QUEUE) {
    params.set('queue', state.queue);
  }

  const normalizedThreshold = clampThreshold(state.threshold);
  if (Math.abs(normalizedThreshold - DEFAULT_THRESHOLD) > 1e-9) {
    params.set('threshold', normalizedThreshold.toFixed(2));
  }

  if (state.q) {
    params.set('q', state.q);
  }
  if (state.sectionFilter) {
    params.set('section', state.sectionFilter);
  }

  if (state.semantic?.anchor) {
    const a = state.semantic.anchor;
    const parts = [a.chapter, a.section, a.sutra].filter((x): x is number =>
      Number.isFinite(x as number),
    );
    if (parts.length > 0) {
      params.set('anchor', parts.join('.'));
    }
  }

  if (state.semantic?.includeKinds && state.semantic.includeKinds.length > 0) {
    params.set('kinds', state.semantic.includeKinds.join(','));
  }

  if (state.semantic?.topicTags && state.semantic.topicTags.length > 0) {
    params.set('topics', state.semantic.topicTags.join(','));
  }

  if (
    state.semantic?.transitionTerms &&
    state.semantic.transitionTerms.length > 0
  ) {
    params.set('tx', state.semantic.transitionTerms.join(','));
  }

  if (state.semantic?.coLocatedOnly) {
    params.set('co_located', '1');
  }

  const encoded = params.toString();
  return encoded ? `?${encoded}` : '';
}

export function toCanonicalUrl(state: SankaraRouteState): string {
  return `${toCanonicalPath(state)}${toCanonicalQuery(state)}`;
}

export function toPreferences(state: SankaraRouteState): SankaraPreferencesV1 {
  return {
    lastMode: state.mode,
    lastQueue: state.queue,
    lastThreshold: clampThreshold(state.threshold),
    lastSectionFilter: state.sectionFilter,
  };
}
