"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FormShapeSchema = exports.FormMetaSchema = exports.FormStateSchema = exports.FormLayoutSchema = exports.FormActionSchema = exports.FormHandlerSchema = exports.FormSectionSchema = exports.FormFieldSchema = exports.FormFieldMetaSchema = exports.FormFieldValidationSchema = exports.FormOptionSchema = exports.FormTagSchema = exports.FormContentSchema = exports.FormModeSchema = exports.FormDataSchema = void 0;
const zod_1 = require("zod");
/**
 * FormData - Defines the data access patterns for forms
 *
 * This represents the a priori context of how forms interact with data
 * rather than just being a generic record.
 */
exports.FormDataSchema = zod_1.z
    .object({
    // Data source configuration
    source: zod_1.z
        .object({
        type: zod_1.z.enum([
            "entity", // BEC entity
            "context", // BEC context
            "api", // External API
            "function", // Custom function
            "localStorage", // Browser localStorage
            "composite", // Multiple sources
        ]),
        // Entity reference if source.type is 'entity'
        entityRef: zod_1.z
            .object({
            entity: zod_1.z.string(),
            id: zod_1.z.string(),
        })
            .optional(),
        // Context reference if source.type is 'context'
        contextRef: zod_1.z
            .object({
            entityRef: zod_1.z.object({
                entity: zod_1.z.string(),
                id: zod_1.z.string(),
            }),
            type: zod_1.z.string(),
        })
            .optional(),
        // API configuration if source.type is 'api'
        apiConfig: zod_1.z
            .object({
            endpoint: zod_1.z.string(),
            method: zod_1.z.enum(["GET", "POST", "PUT", "DELETE", "PATCH"]),
            headers: zod_1.z.record(zod_1.z.string()).optional(),
            params: zod_1.z.record(zod_1.z.any()).optional(),
        })
            .optional(),
        // Function reference if source.type is 'function'
        functionRef: zod_1.z
            .object({
            name: zod_1.z.string(),
            args: zod_1.z.array(zod_1.z.any()).optional(),
        })
            .optional(),
        // LocalStorage configuration if source.type is 'localStorage'
        localStorageKey: zod_1.z.string().optional(),
        // Composite sources if source.type is 'composite'
        compositeSources: zod_1.z.array(zod_1.z.any()).optional(),
    })
        .optional(),
    // Data access patterns
    access: zod_1.z
        .object({
        // Read configuration
        read: zod_1.z
            .object({
            path: zod_1.z.string().optional(), // Path in the data structure to read from
            transform: zod_1.z.function().optional(), // Transform function for the read data
            default: zod_1.z.any().optional(), // Default value if read fails
            cache: zod_1.z.boolean().optional().default(false), // Whether to cache the read result
        })
            .optional(),
        // Write configuration
        write: zod_1.z
            .object({
            path: zod_1.z.string().optional(), // Path in the data structure to write to
            transform: zod_1.z.function().optional(), // Transform function for the write data
            merge: zod_1.z.boolean().optional().default(true), // Whether to merge with existing data
            validation: zod_1.z.function().optional(), // Validation function before writing
        })
            .optional(),
        // Subscribe to changes
        subscribe: zod_1.z
            .object({
            path: zod_1.z.string().optional(), // Path to subscribe to
            debounce: zod_1.z.number().optional(), // Debounce time in ms
            throttle: zod_1.z.number().optional(), // Throttle time in ms
        })
            .optional(),
    })
        .optional(),
    // Data schema
    schema: zod_1.z
        .object({
        type: zod_1.z.enum(["zod", "json-schema", "typescript", "custom"]).optional(),
        definition: zod_1.z.any().optional(), // The actual schema definition
    })
        .optional(),
    // Processing hooks for form data
    hooks: zod_1.z
        .object({
        beforeLoad: zod_1.z.function().optional(),
        afterLoad: zod_1.z.function().optional(),
        beforeSubmit: zod_1.z.function().optional(),
        afterSubmit: zod_1.z.function().optional(),
        onValidate: zod_1.z.function().optional(),
    })
        .optional(),
    // Metadata about the data
    meta: zod_1.z
        .object({
        createdAt: zod_1.z.date().optional(),
        updatedAt: zod_1.z.date().optional(),
        version: zod_1.z.string().optional(),
        owner: zod_1.z.string().optional(),
        permissions: zod_1.z.array(zod_1.z.string()).optional(),
    })
        .optional(),
})
    .optional();
// Define FormMode type
exports.FormModeSchema = zod_1.z
    .enum(["create", "edit", "view"])
    .default("create");
// Define FormContent type - Fixed the comment
exports.FormContentSchema = zod_1.z
    .enum(["jsx", "html", "json", "xml"])
    .default("jsx");
// Form tag
exports.FormTagSchema = zod_1.z.object({
    value: zod_1.z.any(),
    label: zod_1.z.string(),
});
// Form content
exports.FormOptionSchema = zod_1.z.object({
    value: zod_1.z.any(),
    label: zod_1.z.string(),
});
/**
 * Validation schema for form field validation rules
 */
exports.FormFieldValidationSchema = zod_1.z.object({
    required: zod_1.z.boolean().optional(),
    min: zod_1.z.number().optional(),
    max: zod_1.z.number().optional(),
    minLength: zod_1.z.number().optional(),
    maxLength: zod_1.z.number().optional(),
    pattern: zod_1.z.string().optional(),
    custom: zod_1.z.function().optional(),
    message: zod_1.z.string().optional(),
});
/**
 * Field metadata schema
 */
exports.FormFieldMetaSchema = zod_1.z.object({
    sectionHint: zod_1.z.string().optional(),
    validation: zod_1.z
        .object({
        performed: zod_1.z.boolean().optional(),
        timestamp: zod_1.z.number().optional(),
        level: zod_1.z.string().optional(),
    })
        .optional(),
    accessibility: zod_1.z
        .object({
        enhanced: zod_1.z.boolean().optional(),
        level: zod_1.z.string().optional(),
        guideline: zod_1.z.string().optional(),
    })
        .optional(),
    localization: zod_1.z
        .object({
        applied: zod_1.z.boolean().optional(),
        locale: zod_1.z.string().optional(),
    })
        .optional(),
});
exports.FormFieldSchema = zod_1.z.object({
    id: zod_1.z.string(),
    type: zod_1.z.string().optional(),
    name: zod_1.z.string().optional(),
    title: zod_1.z.string().optional(),
    description: zod_1.z.string().optional(),
    label: zod_1.z.string().optional(),
    placeholder: zod_1.z.string().optional(),
    required: zod_1.z.boolean().optional().default(false).optional(),
    disabled: zod_1.z.boolean().optional().default(false).optional(),
    createOnly: zod_1.z.boolean().optional(),
    editOnly: zod_1.z.boolean().optional(),
    readOnly: zod_1.z.boolean().optional().default(false).optional(),
    visible: zod_1.z.boolean().optional().default(true).optional(),
    validation: exports.FormFieldValidationSchema.optional(), // Using proper validation schema now
    options: zod_1.z.array(exports.FormOptionSchema).optional(),
    inputType: zod_1.z.string().optional(),
    format: zod_1.z.string().optional(),
    meta: exports.FormFieldMetaSchema.optional(),
    createdAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
    updatedAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
});
/**
 * Section schema for form layout
 * - A section groups fields together visually
 */
exports.FormSectionSchema = zod_1.z.object({
    id: zod_1.z.string(),
    name: zod_1.z.string().optional(),
    title: zod_1.z.string().optional(),
    description: zod_1.z.string().optional(),
    fields: zod_1.z.array(zod_1.z.string()).optional(), // Field IDs
    columns: zod_1.z.number().optional().default(1).optional(),
    priority: zod_1.z.number().optional().default(1).optional(),
    collapsible: zod_1.z.boolean().optional().default(false).optional(),
    collapsed: zod_1.z.boolean().optional().default(false).optional(),
    className: zod_1.z.string().optional(),
    createdAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
    updatedAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
});
exports.FormHandlerSchema = zod_1.z.object({
    submit: zod_1.z.function(),
    reset: zod_1.z.function().optional(),
    cancel: zod_1.z.function().optional(),
    delete: zod_1.z.function().optional(),
});
exports.FormActionSchema = zod_1.z.object({
    id: zod_1.z.string(),
    type: zod_1.z.enum(["submit", "reset", "button"]).optional(),
    label: zod_1.z.string().optional(),
    primary: zod_1.z.boolean().optional().default(false).optional(),
    disabled: zod_1.z.boolean().optional().default(false).optional(),
    position: zod_1.z
        .enum(["top", "bottom", "both"])
        .optional()
        .default("bottom")
        .optional(),
    createdAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
    updatedAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
});
exports.FormLayoutSchema = zod_1.z.object({
    id: zod_1.z.string(),
    name: zod_1.z.string().optional(),
    title: zod_1.z.string().optional(),
    description: zod_1.z.string().optional(),
    columns: zod_1.z.enum(["single", "double"]).optional(),
    sections: zod_1.z.array(exports.FormSectionSchema).optional(),
    actions: zod_1.z.array(exports.FormActionSchema).optional(),
    // Add responsive hints that Tailwind can use
    responsive: zod_1.z
        .object({
        sectionBreakpoints: zod_1.z
            .record(zod_1.z.enum(["stack", "grid", "tabs"]))
            .optional(),
        fieldArrangement: zod_1.z
            .enum(["natural", "importance", "groupRelated"])
            .optional(),
    })
        .optional(),
    createdAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
    updatedAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
});
/**
 * Form state schema for tracking form submission state
 */
exports.FormStateSchema = zod_1.z.object({
    status: zod_1.z.enum(["idle", "submitting", "success", "error"]),
    errors: zod_1.z.record(zod_1.z.array(zod_1.z.string())).optional(),
    message: zod_1.z.string().optional(),
});
/**
 * Form metadata schema
 * - Contains processing information, not structural properties
 */
exports.FormMetaSchema = zod_1.z.object({
    // Validation metadata
    validation: zod_1.z
        .object({
        performed: zod_1.z.boolean().optional(),
        timestamp: zod_1.z.number().optional(),
        fieldErrors: zod_1.z.number().optional(),
    })
        .optional(),
    // Layout processing metadata
    layout: zod_1.z
        .object({
        source: zod_1.z.string().optional(),
        timestamp: zod_1.z.number().optional(),
        generated: zod_1.z.boolean().optional(),
    })
        .optional(),
    // Accessibility metadata (fixed timestamps to be numbers)
    accessibility: zod_1.z
        .object({
        enhanced: zod_1.z.boolean().optional(),
        timestamp: zod_1.z.number().optional(),
        level: zod_1.z.string().optional(),
    })
        .optional(),
    // Localization metadata (fixed timestamps to be numbers)
    localization: zod_1.z
        .object({
        applied: zod_1.z.boolean().optional(),
        locale: zod_1.z.string().optional(),
        timestamp: zod_1.z.number().optional(),
    })
        .optional(),
});
/**
 * Complete Form Shape schema
 */
exports.FormShapeSchema = zod_1.z.object({
    // Core properties
    id: zod_1.z.string(),
    name: zod_1.z.string(),
    title: zod_1.z.string().optional(),
    description: zod_1.z.string().optional(),
    schemaId: zod_1.z.string().optional(),
    // Data layer
    data: exports.FormDataSchema.optional(),
    // Form structure
    fields: zod_1.z.array(exports.FormFieldSchema),
    options: zod_1.z.array(exports.FormOptionSchema).optional(),
    tags: zod_1.z.array(exports.FormTagSchema).optional(),
    isValid: zod_1.z.boolean().optional(),
    // Structural layout - top level property
    layout: exports.FormLayoutSchema.optional(),
    // Form submission state
    state: exports.FormStateSchema.optional(),
    // Processing metadata - not structural
    meta: exports.FormMetaSchema.optional(),
    createdAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
    updatedAt: zod_1.z
        .number()
        .optional()
        .default(() => Date.now())
        .optional(),
});
//# sourceMappingURL=shape.js.map