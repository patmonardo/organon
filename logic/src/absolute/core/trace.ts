import { randomUUID } from 'node:crypto';
/**
 * Trace vocabulary (structural)
 *
 * Canonical meta types live in sdk/terminology.
 */

import type { EventMeta } from './invariants';
export type { FactStoreInfo, FactStoreOp, EventMeta } from './invariants';

export type TraceMeta = {
  traceId: string;
  spanId: string;
  parentSpanId?: string;
  correlationId?: string;
  engine?: string;
  timestamp: string;
  [k: string]: unknown;
};

export function startTrace(
  engine: string,
  correlationId?: string,
  extra?: Record<string, unknown>,
) {
  return {
    traceId: randomUUID(),
    spanId: randomUUID(),
    correlationId,
    engine,
    timestamp: new Date().toISOString(),
    ...(extra ?? {}),
  } as TraceMeta;
}

export function childSpan(parent: TraceMeta, extra?: Record<string, unknown>) {
  return {
    traceId: parent.traceId,
    spanId: randomUUID(),
    parentSpanId: parent.spanId,
    correlationId: parent.correlationId,
    engine: parent.engine,
    timestamp: new Date().toISOString(),
    ...(extra ?? {}),
  } as TraceMeta;
}

export type TraceEvent<Payload = unknown> = {
  kind: string;
  payload?: Payload;
  meta?: EventMeta;
};
