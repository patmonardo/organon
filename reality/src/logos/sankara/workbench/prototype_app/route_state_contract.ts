export type SankaraMode = 'markdown' | 'source' | 'literal' | 'technical';
export type SankaraQueue = 'normal' | 'review' | 'strict';
export type SankaraRecordKind = 'sutra' | 'bhasya' | 'preamble' | 'unknown';

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
