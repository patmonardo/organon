import { z } from 'zod';
export declare const TopicIdentitySchema: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    description: z.ZodOptional<z.ZodString>;
}, "strip", z.ZodTypeAny, {
    id: string;
    name: string;
    description?: string | undefined;
}, {
    id: string;
    name: string;
    description?: string | undefined;
}>;
export declare const TopicModelSchema: z.ZodObject<{
    identity: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        description: z.ZodOptional<z.ZodString>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        description?: string | undefined;
    }, {
        id: string;
        name: string;
        description?: string | undefined;
    }>;
    corpusRef: z.ZodOptional<z.ZodString>;
    vocabulary: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    topics: z.ZodOptional<z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        label: z.ZodString;
        terms: z.ZodOptional<z.ZodArray<z.ZodObject<{
            term: z.ZodString;
            weight: z.ZodNumber;
        }, "strip", z.ZodTypeAny, {
            term: string;
            weight: number;
        }, {
            term: string;
            weight: number;
        }>, "many">>;
        knowledgeUnits: z.ZodOptional<z.ZodArray<z.ZodObject<{
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
        }>, "many">>;
        metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        label: string;
        metadata?: Record<string, any> | undefined;
        terms?: {
            term: string;
            weight: number;
        }[] | undefined;
        knowledgeUnits?: {
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
        }[] | undefined;
    }, {
        id: string;
        label: string;
        metadata?: Record<string, any> | undefined;
        terms?: {
            term: string;
            weight: number;
        }[] | undefined;
        knowledgeUnits?: {
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
        }[] | undefined;
    }>, "many">>;
    graph: z.ZodOptional<z.ZodObject<{
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
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    updatedAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodTypeAny, {
    createdAt: Date;
    updatedAt: Date;
    identity: {
        id: string;
        name: string;
        description?: string | undefined;
    };
    metadata?: Record<string, any> | undefined;
    corpusRef?: string | undefined;
    vocabulary?: string[] | undefined;
    topics?: {
        id: string;
        label: string;
        metadata?: Record<string, any> | undefined;
        terms?: {
            term: string;
            weight: number;
        }[] | undefined;
        knowledgeUnits?: {
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
        }[] | undefined;
    }[] | undefined;
    graph?: {
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
    } | undefined;
}, {
    identity: {
        id: string;
        name: string;
        description?: string | undefined;
    };
    createdAt?: Date | undefined;
    updatedAt?: Date | undefined;
    metadata?: Record<string, any> | undefined;
    corpusRef?: string | undefined;
    vocabulary?: string[] | undefined;
    topics?: {
        id: string;
        label: string;
        metadata?: Record<string, any> | undefined;
        terms?: {
            term: string;
            weight: number;
        }[] | undefined;
        knowledgeUnits?: {
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
        }[] | undefined;
    }[] | undefined;
    graph?: {
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
    } | undefined;
}>;
export type TopicIdentity = z.infer<typeof TopicIdentitySchema>;
export type TopicModel = z.infer<typeof TopicModelSchema>;
//# sourceMappingURL=topic.d.ts.map