import { randomUUID } from "node:crypto";
import { FormData, FormState } from "../../schema/shape";

/**
 * FormShape - runtime wrapper for a realized instance of a Form definition.
 * Kept minimal for ShapeEngine needs.
 */
export class FormShape {
  public readonly id: string;
  public readonly definitionId: string;
  public readonly definitionName: string;
  public data?: FormData;
  public state: FormState;

  constructor(config: {
    id?: string;
    definitionId: string;
    definitionName: string;
    initialData?: FormData;
    initialState?: FormState;
  }) {
    this.id = config.id ?? `form:${randomUUID()}`;
    this.definitionId = config.definitionId;
    this.definitionName = config.definitionName;
    this.data = config.initialData;
    this.state = config.initialState ?? { status: "idle" };
  }

  // Factory
  static create(params: {
    id?: string;
    definitionId: string;
    definitionName: string;
    data?: FormData;
    state?: FormState;
  }): FormShape {
    return new FormShape({
      id: params.id,
      definitionId: params.definitionId,
      definitionName: params.definitionName,
      initialData: params.data,
      initialState: params.state,
    });
  }

  // Data/state mutators (engine-friendly, schema-neutral)
  setData(data: FormData | undefined): this {
    this.data = data;
    return this;
  }

  mergeData(patch: Partial<FormData>): this {
    if (this.data && typeof this.data === "object") {
      this.data = {
        ...(this.data as Record<string, unknown>),
        ...(patch as Record<string, unknown>),
      } as FormData;
    } else {
      this.data = patch as FormData;
    }
    return this;
  }

  setState(state: FormState): this {
    this.state = state;
    return this;
  }

  patchState(patch: Partial<FormState>): this {
    this.state = { ...this.state, ...patch };
    return this;
  }

  // Introspection
  getInfo() {
    return {
      id: this.id,
      definitionId: this.definitionId,
      definitionName: this.definitionName,
      state: this.state,
    };
  }

  toJSON() {
    return {
      id: this.id,
      definitionId: this.definitionId,
      definitionName: this.definitionName,
      data: this.data,
      state: this.state,
    };
  }
}
