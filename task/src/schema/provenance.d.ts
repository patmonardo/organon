import { z } from 'zod';
export declare const KnowledgeOriginSchema: z.ZodEnum<["empirical", "transcendental", "synthetic", "reflective"]>;
export declare const EvidenceSchema: z.ZodObject<{
    id: z.ZodString;
    kind: z.ZodDefault<z.ZodEnum<["observation", "experiment", "derivation", "citation", "model", "testimonial", "other"]>>;
    description: z.ZodOptional<z.ZodString>;
    dataRef: z.ZodOptional<z.ZodString>;
    strength: z.ZodOptional<z.ZodNumber>;
    timestamp: z.ZodOptional<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    kind: "observation" | "experiment" | "derivation" | "citation" | "model" | "testimonial" | "other";
    description?: string | undefined;
    metadata?: Record<string, any> | undefined;
    dataRef?: string | undefined;
    strength?: number | undefined;
    timestamp?: Date | undefined;
}, {
    id: string;
    description?: string | undefined;
    metadata?: Record<string, any> | undefined;
    kind?: "observation" | "experiment" | "derivation" | "citation" | "model" | "testimonial" | "other" | undefined;
    dataRef?: string | undefined;
    strength?: number | undefined;
    timestamp?: Date | undefined;
}>;
export declare const ProvenanceSchema: z.ZodObject<{
    id: z.ZodString;
    origin: z.ZodEnum<["empirical", "transcendental", "synthetic", "reflective"]>;
    sources: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    evidence: z.ZodOptional<z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        kind: z.ZodDefault<z.ZodEnum<["observation", "experiment", "derivation", "citation", "model", "testimonial", "other"]>>;
        description: z.ZodOptional<z.ZodString>;
        dataRef: z.ZodOptional<z.ZodString>;
        strength: z.ZodOptional<z.ZodNumber>;
        timestamp: z.ZodOptional<z.ZodDate>;
        metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        kind: "observation" | "experiment" | "derivation" | "citation" | "model" | "testimonial" | "other";
        description?: string | undefined;
        metadata?: Record<string, any> | undefined;
        dataRef?: string | undefined;
        strength?: number | undefined;
        timestamp?: Date | undefined;
    }, {
        id: string;
        description?: string | undefined;
        metadata?: Record<string, any> | undefined;
        kind?: "observation" | "experiment" | "derivation" | "citation" | "model" | "testimonial" | "other" | undefined;
        dataRef?: string | undefined;
        strength?: number | undefined;
        timestamp?: Date | undefined;
    }>, "many">>;
    agentId: z.ZodOptional<z.ZodString>;
    createdAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    createdAt: Date;
    origin: "empirical" | "transcendental" | "synthetic" | "reflective";
    metadata?: Record<string, any> | undefined;
    sources?: string[] | undefined;
    evidence?: {
        id: string;
        kind: "observation" | "experiment" | "derivation" | "citation" | "model" | "testimonial" | "other";
        description?: string | undefined;
        metadata?: Record<string, any> | undefined;
        dataRef?: string | undefined;
        strength?: number | undefined;
        timestamp?: Date | undefined;
    }[] | undefined;
    agentId?: string | undefined;
}, {
    id: string;
    origin: "empirical" | "transcendental" | "synthetic" | "reflective";
    createdAt?: Date | undefined;
    metadata?: Record<string, any> | undefined;
    sources?: string[] | undefined;
    evidence?: {
        id: string;
        description?: string | undefined;
        metadata?: Record<string, any> | undefined;
        kind?: "observation" | "experiment" | "derivation" | "citation" | "model" | "testimonial" | "other" | undefined;
        dataRef?: string | undefined;
        strength?: number | undefined;
        timestamp?: Date | undefined;
    }[] | undefined;
    agentId?: string | undefined;
}>;
export type KnowledgeOrigin = z.infer<typeof KnowledgeOriginSchema>;
export type Evidence = z.infer<typeof EvidenceSchema>;
export type Provenance = z.infer<typeof ProvenanceSchema>;
//# sourceMappingURL=provenance.d.ts.map