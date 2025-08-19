import type { ProcessorInputs } from './contracts';
import type { World } from '../../schema/world';

/**
 * Proposal / Commit shapes used by drivers.
 */
export interface Proposal {
  id: string;
  actor: string;
  baseRevision?: number;
  patch?: unknown;
  meta?: Record<string, unknown>;
}

export interface CommitResult {
  success: boolean;
  revision?: number;
  conflicts?: unknown[];
  errors?: string[];
}

export type BackpropagationHook = (summary: unknown) => void | Promise<void>;

/**
 * Base abstract Driver class. Essence drivers should extend this and
 * implement the APIs used by engines.
 */
export abstract class BaseDriver {
  readonly name: string;
  private hooks: Map<string, BackpropagationHook[]> = new Map();

  constructor(name = 'BaseDriver') {
    this.name = name;
  }

  // Primary assemble hook used by world-like drivers
  abstract assemble(input: ProcessorInputs): World;

  // Optional content indexing
  indexContent?(input: ProcessorInputs): { subtleWorldTotal: number; grossByThing: Record<string, number> };

  // Proposal/commit lifecycle (optional for simple drivers)
  propose?(p: Proposal): Promise<CommitResult> | CommitResult;
  commit?(patch: unknown, meta?: unknown): Promise<CommitResult> | CommitResult;

  registerHook(name: string, fn: BackpropagationHook) {
    const prev = this.hooks.get(name) ?? [];
    prev.push(fn);
    this.hooks.set(name, prev);
  }

  async invokeHooks(name: string, payload: unknown) {
    const hs = this.hooks.get(name) ?? [];
    for (const h of hs) {
      try {
        // allow hooks to be async
        // eslint-disable-next-line no-await-in-loop
        await h(payload);
      } catch (err) {
        // swallow hook errors; drivers may override to surface
        // but avoid breaking the core flow
        // eslint-disable-next-line no-console
        console.warn(`${this.name} hook ${name} failed:`, err);
      }
    }
  }

  emitTrace?(event: string, payload?: unknown): void;

  // Conversion helpers may be overridden by drivers
  toActive?(input: unknown): unknown;
  fromActive?(input: unknown): unknown;
}

export default BaseDriver;
