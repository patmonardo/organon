import { z } from 'zod';
export declare const EpistemicStatusSchema: z.ZodEnum<["hypothesis", "supported", "contested", "established", "axiomatic"]>;
export declare const ClaimSchema: z.ZodObject<{
    id: z.ZodString;
    statement: z.ZodString;
    language: z.ZodDefault<z.ZodString>;
    context: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    statement: string;
    language: string;
    context?: Record<string, any> | undefined;
}, {
    id: string;
    statement: string;
    context?: Record<string, any> | undefined;
    language?: string | undefined;
}>;
export declare const JustificationSchema: z.ZodObject<{
    id: z.ZodString;
    rationale: z.ZodOptional<z.ZodString>;
    arguments: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    modelRefs: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
}, "strip", z.ZodTypeAny, {
    id: string;
    rationale?: string | undefined;
    arguments?: string[] | undefined;
    modelRefs?: string[] | undefined;
}, {
    id: string;
    rationale?: string | undefined;
    arguments?: string[] | undefined;
    modelRefs?: string[] | undefined;
}>;
export declare const KnowledgeUnitSchema: z.ZodObject<{
    id: z.ZodString;
    claim: z.ZodObject<{
        id: z.ZodString;
        statement: z.ZodString;
        language: z.ZodDefault<z.ZodString>;
        context: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        statement: string;
        language: string;
        context?: Record<string, any> | undefined;
    }, {
        id: string;
        statement: string;
        context?: Record<string, any> | undefined;
        language?: string | undefined;
    }>;
    justification: z.ZodOptional<z.ZodObject<{
        id: z.ZodString;
        rationale: z.ZodOptional<z.ZodString>;
        arguments: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        modelRefs: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        rationale?: string | undefined;
        arguments?: string[] | undefined;
        modelRefs?: string[] | undefined;
    }, {
        id: string;
        rationale?: string | undefined;
        arguments?: string[] | undefined;
        modelRefs?: string[] | undefined;
    }>>;
    provenance: z.ZodObject<{
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
    status: z.ZodDefault<z.ZodEnum<["hypothesis", "supported", "contested", "established", "axiomatic"]>>;
    confidence: z.ZodDefault<z.ZodNumber>;
    tags: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    createdAt: z.ZodDefault<z.ZodDate>;
    updatedAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    status: "hypothesis" | "supported" | "contested" | "established" | "axiomatic";
    createdAt: Date;
    updatedAt: Date;
    claim: {
        id: string;
        statement: string;
        language: string;
        context?: Record<string, any> | undefined;
    };
    provenance: {
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
    };
    confidence: number;
    metadata?: Record<string, any> | undefined;
    justification?: {
        id: string;
        rationale?: string | undefined;
        arguments?: string[] | undefined;
        modelRefs?: string[] | undefined;
    } | undefined;
    tags?: string[] | undefined;
}, {
    id: string;
    claim: {
        id: string;
        statement: string;
        context?: Record<string, any> | undefined;
        language?: string | undefined;
    };
    provenance: {
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
    };
    status?: "hypothesis" | "supported" | "contested" | "established" | "axiomatic" | undefined;
    createdAt?: Date | undefined;
    updatedAt?: Date | undefined;
    metadata?: Record<string, any> | undefined;
    justification?: {
        id: string;
        rationale?: string | undefined;
        arguments?: string[] | undefined;
        modelRefs?: string[] | undefined;
    } | undefined;
    confidence?: number | undefined;
    tags?: string[] | undefined;
}>;
export type EpistemicStatus = z.infer<typeof EpistemicStatusSchema>;
export type Claim = z.infer<typeof ClaimSchema>;
export type Justification = z.infer<typeof JustificationSchema>;
export type KnowledgeUnit = z.infer<typeof KnowledgeUnitSchema>;
//# sourceMappingURL=knowledge.d.ts.map