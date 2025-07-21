// model/src/model/form.ts
import { z } from 'zod';
import type { FormShape, FormState } from '@/logic/schema/morph';

export class FormModel <FormShape> {
  constructor(schema: z.ZodType<FormShape>, shape: FormShape) {
    super(schema, shape);
  }

  // Access form-specific state
  get formState(): FormState {
    return this.shape.state;
  }

  // Validate the form shape
  validateForm(): FormShape {
    return this.validate();
  }

  // Transform form shape (e.g., for workflow steps)
  mapForm(fn: (shape: FormShape) => FormShape): FormShape {
    return this.map(fn);
  }

  // Add more form-specific logic as needed
}
