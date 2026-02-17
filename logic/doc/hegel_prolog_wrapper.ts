/**
 * Hegelian Prolog Wrapper
 * Wraps SWI Prolog with TS to interpret Hegelian Zod Schemas
 * Implements Conjunction Decoder and Disjunction Input Encoder
 * Translates Being (Kernel JSON) into Becoming Chains via Kantian Membership Protocol
 */

import { z } from 'zod';
import { spawn, ChildProcess } from 'child_process';
import { CpuGpuPhaseSchema, MomentSchema } from './schema/dialectic';

// Kantian Membership Protocol: Translate JSON to Prolog facts
function jsonToPrologFacts(json: any, prefix: string = 'fact'): string {
  const facts: string[] = [];
  for (const [key, value] of Object.entries(json)) {
    if (typeof value === 'object' && value !== null) {
      facts.push(`${prefix}('${key}', ${JSON.stringify(value)}).`);
    } else {
      facts.push(`${prefix}('${key}', '${value}').`);
    }
  }
  return facts.join('\n');
}

// Conjunction Decoder: AND logic for chains
function conjunctionDecoder(conditions: string[]): string {
  return conditions.map((c) => `satisfied('${c}')`).join(', ');
}

// Disjunction Input Encoder: OR logic for inputs
function disjunctionEncoder(inputs: string[]): string {
  return inputs.map((i) => `input('${i}')`).join('; ');
}

// Hegelian Becoming Chain: Immediate determinations as hypothetical chains
function buildBecomingChain(dialecticData: any): string {
  const phase = dialecticData.phase;
  const moments = dialecticData.moments || [];
  const transitions = dialecticData.transitions || [];

  let prolog = `
% Hegelian Becoming Chain for ${phase}
dialectic_phase('${phase}').

`;

  // Moments as facts
  for (const moment of moments) {
    prolog += `moment('${moment.name}', '${moment.definition}').\n`;
  }

  // Transitions as rules (hypothetical chains)
  for (const trans of transitions) {
    prolog += `transition('${trans.from}', '${trans.to}') :- ${conjunctionDecoder(trans.conditions || [])}.\n`;
  }

  // Dependent Origination for Becoming
  prolog += `
becomes(Effect, Causes) :-
    dependent_on(Effect, Causes),
    all_satisfied(Causes).

dependent_on('becoming_${phase}', [${moments.map((m: any) => `'${m.name}'`).join(', ')}]).
all_satisfied([]).
all_satisfied([H|T]) :- satisfied(H), all_satisfied(T).

satisfied(Moment) :- moment(Moment, _).
`;

  return prolog;
}

// Main wrapper class with persistent SWI Prolog process
export class HegelPrologWrapper {
  private prologProcess: ChildProcess | null = null;
  private prologProgram: string = '';

  constructor() {
    this.startPrologProcess();
  }

  private startPrologProcess(): void {
    this.prologProcess = spawn('swipl', ['--quiet', '--nosignals'], {
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    if (!this.prologProcess) {
      throw new Error('Failed to start SWI Prolog process');
    }

    // Handle process errors
    this.prologProcess.on('error', (err) => {
      console.error('SWI Prolog process error:', err);
    });

    this.prologProcess.on('exit', (code) => {
      console.log('SWI Prolog process exited with code:', code);
    });
  }

  // Load Hegelian schema and data (consult program)
  loadDialectic(schema: z.ZodSchema, data: any): Promise<void> {
    return new Promise((resolve, reject) => {
      // Validate with Zod
      const validated = schema.parse(data);

      // Translate to Prolog
      this.prologProgram =
        buildBecomingChain(validated) +
        '\n' +
        jsonToPrologFacts(validated, 'kernel_fact');

      if (!this.prologProcess || !this.prologProcess.stdin) {
        reject(new Error('Prolog process not available'));
        return;
      }

      // Send consult command
      const consultCommand = `consult(user).\n${this.prologProgram}\nend_of_file.\n`;

      this.prologProcess.stdin.write(consultCommand, (err) => {
        if (err) {
          reject(err);
        } else {
          // Wait a bit for consult to complete
          setTimeout(() => resolve(), 100);
        }
      });
    });
  }

  // Query Becoming Chain
  queryBecoming(phase: string): Promise<any> {
    return new Promise((resolve, reject) => {
      if (
        !this.prologProcess ||
        !this.prologProcess.stdin ||
        !this.prologProcess.stdout
      ) {
        reject(new Error('Prolog process not available'));
        return;
      }

      const query = `becomes('becoming_${phase}', Causes)`;
      const fullQuery = `${query}.\n`;

      let output = '';
      const onData = (data: Buffer) => {
        output += data.toString();
        if (output.includes('\n')) {
          this.prologProcess!.stdout!.off('data', onData);
          try {
            // Parse simple output (assuming list format)
            const match = output.match(/Causes = \[(.*?)\]/);
            if (match) {
              const causes = match[1]
                .split(',')
                .map((s: string) => s.trim().replace(/'/g, ''));
              resolve(causes);
            } else {
              resolve(null);
            }
          } catch (e) {
            reject(e);
          }
        }
      };

      this.prologProcess.stdout.on('data', onData);
      this.prologProcess.stdin.write(fullQuery, (err) => {
        if (err) reject(err);
      });
    });
  }

  // Conjunction Decode: Check if all conditions hold
  conjunctionDecode(conditions: string[]): Promise<boolean> {
    return new Promise((resolve, reject) => {
      if (
        !this.prologProcess ||
        !this.prologProcess.stdin ||
        !this.prologProcess.stdout
      ) {
        reject(new Error('Prolog process not available'));
        return;
      }

      const query = `${conjunctionDecoder(conditions)}`;
      const fullQuery = `${query}.\n`;

      let output = '';
      const onData = (data: Buffer) => {
        output += data.toString();
        if (output.includes('true') || output.includes('false')) {
          this.prologProcess!.stdout!.off('data', onData);
          resolve(output.includes('true'));
        }
      };

      this.prologProcess.stdout.on('data', onData);
      this.prologProcess.stdin.write(fullQuery, (err) => {
        if (err) reject(err);
      });
    });
  }

  // Disjunction Encode: Input alternatives
  disjunctionEncode(inputs: string[]): Promise<string[]> {
    return new Promise((resolve, reject) => {
      if (
        !this.prologProcess ||
        !this.prologProcess.stdin ||
        !this.prologProcess.stdout
      ) {
        reject(new Error('Prolog process not available'));
        return;
      }

      const query = `${disjunctionEncoder(inputs)}`;
      const fullQuery = `findall(I, ${query}, Inputs).\n`;

      let output = '';
      const onData = (data: Buffer) => {
        output += data.toString();
        if (output.includes('Inputs = ')) {
          this.prologProcess!.stdout!.off('data', onData);
          try {
            const match = output.match(/Inputs = \[(.*?)\]/);
            if (match) {
              const inputs = match[1]
                .split(',')
                .map((s: string) => s.trim().replace(/'/g, ''));
              resolve(inputs);
            } else {
              resolve([]);
            }
          } catch (e) {
            reject(e);
          }
        }
      };

      this.prologProcess.stdout.on('data', onData);
      this.prologProcess.stdin.write(fullQuery, (err) => {
        if (err) reject(err);
      });
    });
  }

  // Close the persistent process
  close(): void {
    if (this.prologProcess) {
      this.prologProcess.kill();
      this.prologProcess = null;
    }
  }
}

// Example usage
export async function exampleHegelWrapper() {
  const wrapper = new HegelPrologWrapper();

  // Sample dialectic data
  const sampleData = {
    phase: 'quality',
    moments: [
      {
        name: 'determination1',
        definition: 'Immediate quality',
        type: 'determination',
      },
      {
        name: 'determination2',
        definition: 'Something vs Nothing',
        type: 'determination',
      },
    ],
    transitions: [
      {
        from: 'quality',
        to: 'quantity',
        conditions: ['determination1', 'determination2'],
      },
    ],
  };

  wrapper.loadDialectic(
    z.object({
      phase: CpuGpuPhaseSchema,
      moments: z.array(MomentSchema),
      transitions: z.array(
        z.object({
          from: z.string(),
          to: z.string(),
          conditions: z.array(z.string()).optional(),
        }),
      ),
    }),
    sampleData,
  );

  const becoming = wrapper.queryBecoming('quality');
  console.log('Becoming chain:', becoming);

  const conj = wrapper.conjunctionDecode(['determination1', 'determination2']);
  console.log('Conjunction satisfied:', conj);

  wrapper.close();
}
