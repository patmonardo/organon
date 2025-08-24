/**
 * Agent Schema: Capacity–Awareness (Active Essence in Agency)
 * ===========================================================
 *
 * In the TAW dialectical system, Agent represents the transformation of
 * Essence into active, self-aware agency:
 *
 *   - Capacity: The agent's powers, skills, and capabilities (Active Essence as potential)
 *   - Awareness: The agent's consciousness, perspective, and understanding (Active Essence as actuality)
 *
 * Agent = Capacity : Awareness
 *
 * Philosophical Note:
 * Agent is Essence in agency - not static mediation but dynamic self-aware action.
 * It represents the movement of Essence toward self-consciousness and autonomous activity.
 * The Capacity is Essence as power, the Awareness is Essence as consciousness.
 * This enables the dialectical bridge between mediated Logic and conscious Agency.
 */
import { z } from 'zod';
export declare const CapacitySchema: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    skills: z.ZodArray<z.ZodString, "many">;
    tools: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    potentialLevel: z.ZodDefault<z.ZodEnum<["reactive", "deliberative", "intuitive", "absolute"]>>;
    actualizationProgress: z.ZodDefault<z.ZodNumber>;
}, "strip", z.ZodTypeAny, {
    id: string;
    name: string;
    skills: string[];
    potentialLevel: "absolute" | "reactive" | "deliberative" | "intuitive";
    actualizationProgress: number;
    resources?: Record<string, any> | undefined;
    constraints?: string[] | undefined;
    tools?: string[] | undefined;
}, {
    id: string;
    name: string;
    skills: string[];
    resources?: Record<string, any> | undefined;
    constraints?: string[] | undefined;
    tools?: string[] | undefined;
    potentialLevel?: "absolute" | "reactive" | "deliberative" | "intuitive" | undefined;
    actualizationProgress?: number | undefined;
}>;
export declare const AwarenessSchema: z.ZodObject<{
    id: z.ZodString;
    viewpoint: z.ZodString;
    context: z.ZodRecord<z.ZodString, z.ZodAny>;
    filters: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    history: z.ZodOptional<z.ZodArray<z.ZodRecord<z.ZodString, z.ZodAny>, "many">>;
    consciousnessLevel: z.ZodDefault<z.ZodEnum<["surface", "depth", "integral", "non-dual"]>>;
    selfAwareness: z.ZodDefault<z.ZodBoolean>;
    reflectionCapacity: z.ZodDefault<z.ZodEnum<["none", "basic", "dialectical", "absolute"]>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    viewpoint: string;
    context: Record<string, any>;
    consciousnessLevel: "surface" | "depth" | "integral" | "non-dual";
    selfAwareness: boolean;
    reflectionCapacity: "absolute" | "dialectical" | "none" | "basic";
    filters?: Record<string, any> | undefined;
    history?: Record<string, any>[] | undefined;
}, {
    id: string;
    viewpoint: string;
    context: Record<string, any>;
    filters?: Record<string, any> | undefined;
    history?: Record<string, any>[] | undefined;
    consciousnessLevel?: "surface" | "depth" | "integral" | "non-dual" | undefined;
    selfAwareness?: boolean | undefined;
    reflectionCapacity?: "absolute" | "dialectical" | "none" | "basic" | undefined;
}>;
export declare const AgentSchema: z.ZodObject<{
    id: z.ZodString;
    capacity: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        skills: z.ZodArray<z.ZodString, "many">;
        tools: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        potentialLevel: z.ZodDefault<z.ZodEnum<["reactive", "deliberative", "intuitive", "absolute"]>>;
        actualizationProgress: z.ZodDefault<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        skills: string[];
        potentialLevel: "absolute" | "reactive" | "deliberative" | "intuitive";
        actualizationProgress: number;
        resources?: Record<string, any> | undefined;
        constraints?: string[] | undefined;
        tools?: string[] | undefined;
    }, {
        id: string;
        name: string;
        skills: string[];
        resources?: Record<string, any> | undefined;
        constraints?: string[] | undefined;
        tools?: string[] | undefined;
        potentialLevel?: "absolute" | "reactive" | "deliberative" | "intuitive" | undefined;
        actualizationProgress?: number | undefined;
    }>;
    awareness: z.ZodObject<{
        id: z.ZodString;
        viewpoint: z.ZodString;
        context: z.ZodRecord<z.ZodString, z.ZodAny>;
        filters: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        history: z.ZodOptional<z.ZodArray<z.ZodRecord<z.ZodString, z.ZodAny>, "many">>;
        consciousnessLevel: z.ZodDefault<z.ZodEnum<["surface", "depth", "integral", "non-dual"]>>;
        selfAwareness: z.ZodDefault<z.ZodBoolean>;
        reflectionCapacity: z.ZodDefault<z.ZodEnum<["none", "basic", "dialectical", "absolute"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        viewpoint: string;
        context: Record<string, any>;
        consciousnessLevel: "surface" | "depth" | "integral" | "non-dual";
        selfAwareness: boolean;
        reflectionCapacity: "absolute" | "dialectical" | "none" | "basic";
        filters?: Record<string, any> | undefined;
        history?: Record<string, any>[] | undefined;
    }, {
        id: string;
        viewpoint: string;
        context: Record<string, any>;
        filters?: Record<string, any> | undefined;
        history?: Record<string, any>[] | undefined;
        consciousnessLevel?: "surface" | "depth" | "integral" | "non-dual" | undefined;
        selfAwareness?: boolean | undefined;
        reflectionCapacity?: "absolute" | "dialectical" | "none" | "basic" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["idle", "active", "learning", "reflecting", "transcendent"]>>;
    energy: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        autonomyLevel: z.ZodEnum<["reactive", "deliberative", "dialectical", "absolute"]>;
        learningEnabled: z.ZodDefault<z.ZodBoolean>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        autonomyLevel: "absolute" | "dialectical" | "reactive" | "deliberative";
        learningEnabled: boolean;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        autonomyLevel: "absolute" | "dialectical" | "reactive" | "deliberative";
        learningEnabled?: boolean | undefined;
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    lastActive: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodAny, z.objectOutputType<{
    id: z.ZodString;
    capacity: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        skills: z.ZodArray<z.ZodString, "many">;
        tools: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        potentialLevel: z.ZodDefault<z.ZodEnum<["reactive", "deliberative", "intuitive", "absolute"]>>;
        actualizationProgress: z.ZodDefault<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        skills: string[];
        potentialLevel: "absolute" | "reactive" | "deliberative" | "intuitive";
        actualizationProgress: number;
        resources?: Record<string, any> | undefined;
        constraints?: string[] | undefined;
        tools?: string[] | undefined;
    }, {
        id: string;
        name: string;
        skills: string[];
        resources?: Record<string, any> | undefined;
        constraints?: string[] | undefined;
        tools?: string[] | undefined;
        potentialLevel?: "absolute" | "reactive" | "deliberative" | "intuitive" | undefined;
        actualizationProgress?: number | undefined;
    }>;
    awareness: z.ZodObject<{
        id: z.ZodString;
        viewpoint: z.ZodString;
        context: z.ZodRecord<z.ZodString, z.ZodAny>;
        filters: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        history: z.ZodOptional<z.ZodArray<z.ZodRecord<z.ZodString, z.ZodAny>, "many">>;
        consciousnessLevel: z.ZodDefault<z.ZodEnum<["surface", "depth", "integral", "non-dual"]>>;
        selfAwareness: z.ZodDefault<z.ZodBoolean>;
        reflectionCapacity: z.ZodDefault<z.ZodEnum<["none", "basic", "dialectical", "absolute"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        viewpoint: string;
        context: Record<string, any>;
        consciousnessLevel: "surface" | "depth" | "integral" | "non-dual";
        selfAwareness: boolean;
        reflectionCapacity: "absolute" | "dialectical" | "none" | "basic";
        filters?: Record<string, any> | undefined;
        history?: Record<string, any>[] | undefined;
    }, {
        id: string;
        viewpoint: string;
        context: Record<string, any>;
        filters?: Record<string, any> | undefined;
        history?: Record<string, any>[] | undefined;
        consciousnessLevel?: "surface" | "depth" | "integral" | "non-dual" | undefined;
        selfAwareness?: boolean | undefined;
        reflectionCapacity?: "absolute" | "dialectical" | "none" | "basic" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["idle", "active", "learning", "reflecting", "transcendent"]>>;
    energy: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        autonomyLevel: z.ZodEnum<["reactive", "deliberative", "dialectical", "absolute"]>;
        learningEnabled: z.ZodDefault<z.ZodBoolean>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        autonomyLevel: "absolute" | "dialectical" | "reactive" | "deliberative";
        learningEnabled: boolean;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        autonomyLevel: "absolute" | "dialectical" | "reactive" | "deliberative";
        learningEnabled?: boolean | undefined;
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    lastActive: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, z.ZodAny, "strip">, z.objectInputType<{
    id: z.ZodString;
    capacity: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        skills: z.ZodArray<z.ZodString, "many">;
        tools: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        potentialLevel: z.ZodDefault<z.ZodEnum<["reactive", "deliberative", "intuitive", "absolute"]>>;
        actualizationProgress: z.ZodDefault<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        skills: string[];
        potentialLevel: "absolute" | "reactive" | "deliberative" | "intuitive";
        actualizationProgress: number;
        resources?: Record<string, any> | undefined;
        constraints?: string[] | undefined;
        tools?: string[] | undefined;
    }, {
        id: string;
        name: string;
        skills: string[];
        resources?: Record<string, any> | undefined;
        constraints?: string[] | undefined;
        tools?: string[] | undefined;
        potentialLevel?: "absolute" | "reactive" | "deliberative" | "intuitive" | undefined;
        actualizationProgress?: number | undefined;
    }>;
    awareness: z.ZodObject<{
        id: z.ZodString;
        viewpoint: z.ZodString;
        context: z.ZodRecord<z.ZodString, z.ZodAny>;
        filters: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        history: z.ZodOptional<z.ZodArray<z.ZodRecord<z.ZodString, z.ZodAny>, "many">>;
        consciousnessLevel: z.ZodDefault<z.ZodEnum<["surface", "depth", "integral", "non-dual"]>>;
        selfAwareness: z.ZodDefault<z.ZodBoolean>;
        reflectionCapacity: z.ZodDefault<z.ZodEnum<["none", "basic", "dialectical", "absolute"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        viewpoint: string;
        context: Record<string, any>;
        consciousnessLevel: "surface" | "depth" | "integral" | "non-dual";
        selfAwareness: boolean;
        reflectionCapacity: "absolute" | "dialectical" | "none" | "basic";
        filters?: Record<string, any> | undefined;
        history?: Record<string, any>[] | undefined;
    }, {
        id: string;
        viewpoint: string;
        context: Record<string, any>;
        filters?: Record<string, any> | undefined;
        history?: Record<string, any>[] | undefined;
        consciousnessLevel?: "surface" | "depth" | "integral" | "non-dual" | undefined;
        selfAwareness?: boolean | undefined;
        reflectionCapacity?: "absolute" | "dialectical" | "none" | "basic" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["idle", "active", "learning", "reflecting", "transcendent"]>>;
    energy: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        autonomyLevel: z.ZodEnum<["reactive", "deliberative", "dialectical", "absolute"]>;
        learningEnabled: z.ZodDefault<z.ZodBoolean>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        autonomyLevel: "absolute" | "dialectical" | "reactive" | "deliberative";
        learningEnabled: boolean;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        autonomyLevel: "absolute" | "dialectical" | "reactive" | "deliberative";
        learningEnabled?: boolean | undefined;
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    lastActive: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, z.ZodAny, "strip">>;
export type Capacity = z.infer<typeof CapacitySchema>;
export type Awareness = z.infer<typeof AwarenessSchema>;
export type Agent = z.infer<typeof AgentSchema>;
/**
 * Agent Factory: Creates agents with dialectical consciousness levels
 */
export declare class AgentFactory {
    /**
     * Create a Being-level agent (reactive, immediate responses)
     */
    static createBeingAgent(name: string, skills: string[]): Agent;
    /**
     * Create an Essence-level agent (deliberative, reflective capabilities)
     */
    static createEssenceAgent(name: string, skills: string[], reflectionCapacity?: 'basic' | 'dialectical'): Agent;
    /**
     * Create a Concept-level agent (intuitive, self-determining)
     */
    static createConceptAgent(name: string, skills: string[]): Agent;
    /**
     * Create an Absolute Idea agent (transcendent, absolute autonomy)
     */
    static createAbsoluteAgent(name: string, systemContext: Record<string, any>): Agent;
}
export default AgentSchema;
//# sourceMappingURL=agent.d.ts.map