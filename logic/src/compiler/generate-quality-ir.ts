import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { becomingIR } from '@relative/being/quality/being/becoming-ir';
import { beingIR } from '@relative/being/quality/being/being-ir';
import { nothingIR } from '@relative/being/quality/being/nothing-ir';
import { attractionIR } from '@relative/being/quality/being-for-self/attraction-ir';
import { beingForSelfIR } from '@relative/being/quality/being-for-self/being-for-self-ir';
import { oneManyIR } from '@relative/being/quality/being-for-self/one-many-ir';
import { existenceChapterIR } from '@relative/being/quality/existence/existence-chapter-ir';
import {
  DialecticIRSchema,
  type DialecticIR,
  type DialecticState,
} from '@schema/dialectic';

function cloneState(state: DialecticState): DialecticState {
  return JSON.parse(JSON.stringify(state)) as DialecticState;
}

function collectOutgoingTargets(state: DialecticState): string[] {
  const targets = new Set<string>();

  for (const next of state.nextStates ?? []) {
    targets.add(next);
  }

  for (const transition of state.transitions ?? []) {
    targets.add(transition.to);
  }

  for (const force of state.forces ?? []) {
    targets.add(force.targetState);
  }

  return [...targets];
}

function assertQualityGraphIntegrity(states: DialecticState[]): void {
  const errors: string[] = [];
  const byId = new Map<string, DialecticState>();
  const externalStatePatterns = [/^method-advance-/, /^quantity-/];

  const isKnownState = (id: string): boolean => {
    if (byId.has(id)) {
      return true;
    }

    return externalStatePatterns.some((pattern) => pattern.test(id));
  };

  for (const state of states) {
    if (byId.has(state.id)) {
      errors.push(`Duplicate state id detected: ${state.id}`);
      continue;
    }
    byId.set(state.id, state);
  }

  for (const state of states) {
    for (const target of state.nextStates ?? []) {
      if (!isKnownState(target)) {
        errors.push(
          `State ${state.id} has nextState to unknown target ${target}`,
        );
      }
    }

    for (const source of state.previousStates ?? []) {
      if (!isKnownState(source)) {
        errors.push(
          `State ${state.id} has previousState from unknown source ${source}`,
        );
      }
    }

    for (const transition of state.transitions ?? []) {
      if (!isKnownState(transition.from)) {
        errors.push(
          `Transition ${transition.id} has unknown from state ${transition.from}`,
        );
      }
      if (!isKnownState(transition.to)) {
        errors.push(
          `Transition ${transition.id} has unknown to state ${transition.to}`,
        );
      }
    }

    for (const force of state.forces ?? []) {
      if (!isKnownState(force.targetState)) {
        errors.push(
          `Force ${force.id} in ${state.id} targets unknown state ${force.targetState}`,
        );
      }
    }
  }

  const requiredBoundaries: Array<{ from: string; to: string; label: string }> =
    [
      {
        from: 'becoming-7',
        to: 'existence-1',
        label: 'Being/Becoming -> Existence boundary',
      },
      {
        from: 'affirmative-infinity-16',
        to: 'being-for-self-a',
        label: 'Existence -> Being-for-self boundary',
      },
    ];

  for (const boundary of requiredBoundaries) {
    const fromState = byId.get(boundary.from);
    const toState = byId.get(boundary.to);

    if (!fromState) {
      errors.push(`${boundary.label}: missing from-state ${boundary.from}`);
      continue;
    }

    if (!toState) {
      errors.push(`${boundary.label}: missing to-state ${boundary.to}`);
      continue;
    }

    const outgoingTargets = collectOutgoingTargets(fromState);
    if (!outgoingTargets.includes(boundary.to)) {
      errors.push(
        `${boundary.label}: ${boundary.from} does not link to ${boundary.to} via nextStates/transitions/forces`,
      );
    }

    if (!(toState.previousStates ?? []).includes(boundary.from)) {
      errors.push(
        `${boundary.label}: ${boundary.to} is missing previousStates reference to ${boundary.from}`,
      );
    }
  }

  const startId = 'being-1';
  const finalGateId = 'being-for-self-a';
  if (byId.has(startId) && byId.has(finalGateId)) {
    const visited = new Set<string>();
    const queue: string[] = [startId];

    while (queue.length > 0) {
      const current = queue.shift();
      if (!current || visited.has(current)) {
        continue;
      }

      visited.add(current);
      const currentState = byId.get(current);
      if (!currentState) {
        continue;
      }

      for (const target of collectOutgoingTargets(currentState)) {
        if (!visited.has(target)) {
          queue.push(target);
        }
      }
    }

    if (!visited.has(finalGateId)) {
      errors.push(
        `Reachability failed: ${finalGateId} is not reachable from ${startId}`,
      );
    }
  }

  if (errors.length > 0) {
    throw new Error(
      `Quality graph integrity checks failed:\n${errors.join('\n')}`,
    );
  }
}

function mergeQualityStates(): DialecticState[] {
  return [
    ...beingIR.states.map(cloneState),
    ...nothingIR.states.map(cloneState),
    ...becomingIR.states.map(cloneState),
    ...existenceChapterIR.states.map(cloneState),
    ...beingForSelfIR.states.map(cloneState),
    ...oneManyIR.states.map(cloneState),
    ...attractionIR.states.map(cloneState),
  ];
}

async function main() {
  const here = path.dirname(fileURLToPath(import.meta.url));
  const packageRoot = path.resolve(here, '..', '..', '..', '..');

  const states = mergeQualityStates();
  assertQualityGraphIntegrity(states);
  const cpuGpuMapping = Object.fromEntries(
    states.map((state) => [state.id, state.phase]),
  );

  const qualityIR: DialecticIR = {
    id: 'quality-ir',
    title: 'Quality IR: Being, Existence, and Being-for-itself',
    section: 'BEING - QUALITY',
    states,
    metadata: {
      sourceFile: 'quality',
      totalStates: states.length,
      cpuGpuMapping,
      updatedAt: new Date().toISOString(),
    },
  };

  const parsed = DialecticIRSchema.safeParse(qualityIR);
  if (!parsed.success) {
    const issues = parsed.error.issues
      .map((issue) => `${issue.path.join('.')}: ${issue.message}`)
      .join('\n');
    throw new Error(`Quality DialecticIR validation failed:\n${issues}`);
  }

  const outputPath = path.join(
    packageRoot,
    'src',
    'relative',
    'being',
    'quality',
    'quality-ir.ts',
  );

  const output = [
    "import type { DialecticIR } from '@schema/dialectic';",
    '',
    '// Generated by src/relative/core/compiler/generate-quality-ir.ts',
    'export const qualityIR: DialecticIR = ',
    `${JSON.stringify(parsed.data, null, 2)};`,
    '',
    'export const qualityStateMap = Object.fromEntries(',
    '  qualityIR.states.map((state) => [state.id, state]),',
    ');',
    '',
  ].join('\n');

  await fs.writeFile(outputPath, output, 'utf8');

  console.log(`Generated ${path.relative(packageRoot, outputPath)}.`);
}

await main();
