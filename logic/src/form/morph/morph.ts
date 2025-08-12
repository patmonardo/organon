import {
  type Morph,
  type MorphOptions,
  type MorphTransformer,
  type PostProcessor,
  createMorph,
  composeMorphs,
  IdentityMorph,
} from "./core";

/**
 * FormMorph - thin wrapper over core Morph for engine ergonomics.
 * No global registry; compose/identity helpers provided.
 */
export class FormMorph<T, U> {
  private readonly inner: Morph<T, U>;

  private constructor(inner: Morph<T, U>) {
    this.inner = inner;
  }

  // Factory: define a morph from name + transformer
  static define<T, U>(
    name: string,
    transformer: MorphTransformer<T, U>,
    options: Partial<MorphOptions> = {}
  ): FormMorph<T, U> {
    return new FormMorph(createMorph<T, U>(name, transformer, options));
  }

  // Factory: identity morph
  static identity<T>(name: string = "Identity"): FormMorph<T, T> {
    return new FormMorph<T, T>(new IdentityMorph<T>(name));
  }

  // Compose two morphs
  static compose<T, U, V>(
    a: FormMorph<T, U> | Morph<T, U>,
    b: FormMorph<U, V> | Morph<U, V>,
    name?: string
  ): FormMorph<T, V> {
    const aCore = a instanceof FormMorph ? a.toCore() : a;
    const bCore = b instanceof FormMorph ? b.toCore() : b;
    return new FormMorph<T, V>(composeMorphs<T, U, V>(aCore, bCore, name));
  }

  // Optional post-processing wrapper
  withPostProcessor<V>(
    post: PostProcessor<U, V>,
    name: string = `${this.name}➝post`
  ): FormMorph<T, V> {
    const base = this.inner;
    const wrapped = createMorph<T, V>(
      name,
      (input: T, context?: any) => {
        const u = base.transform(input, context);
        return post(u, context);
      },
      {
        // treat post as effectful by default
        pure: false,
        // step cost + post cost(=1)
        cost: (base.options.cost ?? 1) + 1,
      }
    );
    return new FormMorph<T, V>(wrapped);
  }

  // Execute
  run(input: T, context?: any): U {
    return this.inner.transform(input, context);
  }

  // Introspection
  get name(): string {
    return this.inner.name;
  }
  get options(): MorphOptions {
    return this.inner.options;
  }

  describe() {
    return { name: this.name, options: this.options };
  }

  // Access underlying core morph (for pipelines)
  toCore(): Morph<T, U> {
    return this.inner;
  }
}

// Convenience alias
export const defineMorph = FormMorph.define;

// Re-export pipeline API (no registry)
export {
  Pipeline,
  createPipeline,
  FormPipeline,
  createFormPipeline,
} from "./core";
