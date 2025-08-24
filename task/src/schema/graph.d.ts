import { z } from 'zod';
export declare const KnowledgeNodeSchema: z.ZodObject<{
    id: z.ZodString;
    type: z.ZodEnum<["claim", "concept", "entity", "topic"]>;
    label: z.ZodString;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    type: "concept" | "claim" | "entity" | "topic";
    label: string;
    metadata?: Record<string, any> | undefined;
}, {
    id: string;
    type: "concept" | "claim" | "entity" | "topic";
    label: string;
    metadata?: Record<string, any> | undefined;
}>;
export declare const KnowledgeEdgeSchema: z.ZodObject<{
    id: z.ZodString;
    source: z.ZodString;
    target: z.ZodString;
    relation: z.ZodString;
    weight: z.ZodOptional<z.ZodNumber>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    target: string;
    source: string;
    relation: string;
    metadata?: Record<string, any> | undefined;
    weight?: number | undefined;
}, {
    id: string;
    target: string;
    source: string;
    relation: string;
    metadata?: Record<string, any> | undefined;
    weight?: number | undefined;
}>;
export declare const KnowledgeGraphSchema: z.ZodObject<{
    id: z.ZodString;
    nodes: z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        type: z.ZodEnum<["claim", "concept", "entity", "topic"]>;
        label: z.ZodString;
        metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        type: "concept" | "claim" | "entity" | "topic";
        label: string;
        metadata?: Record<string, any> | undefined;
    }, {
        id: string;
        type: "concept" | "claim" | "entity" | "topic";
        label: string;
        metadata?: Record<string, any> | undefined;
    }>, "many">;
    edges: z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        source: z.ZodString;
        target: z.ZodString;
        relation: z.ZodString;
        weight: z.ZodOptional<z.ZodNumber>;
        metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        target: string;
        source: string;
        relation: string;
        metadata?: Record<string, any> | undefined;
        weight?: number | undefined;
    }, {
        id: string;
        target: string;
        source: string;
        relation: string;
        metadata?: Record<string, any> | undefined;
        weight?: number | undefined;
    }>, "many">;
    createdAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    createdAt: Date;
    nodes: {
        id: string;
        type: "concept" | "claim" | "entity" | "topic";
        label: string;
        metadata?: Record<string, any> | undefined;
    }[];
    edges: {
        id: string;
        target: string;
        source: string;
        relation: string;
        metadata?: Record<string, any> | undefined;
        weight?: number | undefined;
    }[];
    metadata?: Record<string, any> | undefined;
}, {
    id: string;
    nodes: {
        id: string;
        type: "concept" | "claim" | "entity" | "topic";
        label: string;
        metadata?: Record<string, any> | undefined;
    }[];
    edges: {
        id: string;
        target: string;
        source: string;
        relation: string;
        metadata?: Record<string, any> | undefined;
        weight?: number | undefined;
    }[];
    createdAt?: Date | undefined;
    metadata?: Record<string, any> | undefined;
}>;
export type KnowledgeNode = z.infer<typeof KnowledgeNodeSchema>;
export type KnowledgeEdge = z.infer<typeof KnowledgeEdgeSchema>;
export type KnowledgeGraph = z.infer<typeof KnowledgeGraphSchema>;
//# sourceMappingURL=graph.d.ts.map