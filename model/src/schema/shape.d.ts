import { z } from "zod";
/**
 * FormData - Defines the data access patterns for forms
 *
 * This represents the a priori context of how forms interact with data
 * rather than just being a generic record.
 */
export declare const FormDataSchema: any;
export declare const FormModeSchema: any;
export declare const FormContentSchema: any;
export declare const FormTagSchema: any;
export declare const FormOptionSchema: any;
/**
 * Validation schema for form field validation rules
 */
export declare const FormFieldValidationSchema: any;
/**
 * Field metadata schema
 */
export declare const FormFieldMetaSchema: any;
export declare const FormFieldSchema: any;
/**
 * Section schema for form layout
 * - A section groups fields together visually
 */
export declare const FormSectionSchema: any;
export declare const FormHandlerSchema: any;
export declare const FormActionSchema: any;
export declare const FormLayoutSchema: any;
/**
 * Form state schema for tracking form submission state
 */
export declare const FormStateSchema: any;
/**
 * Form metadata schema
 * - Contains processing information, not structural properties
 */
export declare const FormMetaSchema: any;
/**
 * Complete Form Shape schema
 */
export declare const FormShapeSchema: any;
export type FormData = z.infer<typeof FormDataSchema>;
export type FormMode = z.infer<typeof FormModeSchema>;
export type FormContent = z.infer<typeof FormContentSchema>;
export type FormFieldValidation = z.infer<typeof FormFieldValidationSchema>;
export type FormOption = z.infer<typeof FormOptionSchema>;
export type FormTag = z.infer<typeof FormTagSchema>;
export type FormField = z.infer<typeof FormFieldSchema>;
export type FormHandler = z.infer<typeof FormHandlerSchema>;
export type FormAction = z.infer<typeof FormActionSchema>;
export type FormFieldMeta = z.infer<typeof FormFieldMetaSchema>;
export type FormSection = z.infer<typeof FormSectionSchema>;
export type FormLayout = z.infer<typeof FormLayoutSchema>;
export type FormState = z.infer<typeof FormStateSchema>;
export type FormMeta = z.infer<typeof FormMetaSchema>;
export type FormShape = z.infer<typeof FormShapeSchema>;
export type ShapeData = z.infer<typeof FormDataSchema>;
export type ShapeMode = z.infer<typeof FormModeSchema>;
export type ShapeContent = z.infer<typeof FormContentSchema>;
export type ShapeFieldValidation = z.infer<typeof FormFieldValidationSchema>;
export type ShapeOption = z.infer<typeof FormOptionSchema>;
export type ShapeTag = z.infer<typeof FormTagSchema>;
export type ShapeField = z.infer<typeof FormFieldSchema>;
export type ShapeHandler = z.infer<typeof FormHandlerSchema>;
export type ShapeAction = z.infer<typeof FormActionSchema>;
export type ShapeFieldMeta = z.infer<typeof FormFieldMetaSchema>;
export type ShapeSection = z.infer<typeof FormSectionSchema>;
export type ShapeLayout = z.infer<typeof FormLayoutSchema>;
export type ShapeState = z.infer<typeof FormStateSchema>;
//# sourceMappingURL=shape.d.ts.map