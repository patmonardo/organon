/**
 * Workflow Schema: Process–Coordination (Active Concept in Organization)
 * ======================================================================
 *
 * In the TAW dialectical system, Workflow represents the transformation of
 * Concept into active, organized coordination:
 *
 *   - Process: The active sequence of transformations (Active Concept as temporal unfolding)
 *   - Coordination: The organizational principle that governs the process (Active Concept as systematic unity)
 *
 * Workflow = Process : Coordination
 *
 * Philosophical Note:
 * Workflow is Concept in organization - not static logical structure but dynamic systematic coordination.
 * It represents the movement of Concept toward complete systematic organization and temporal actualization.
 * The Process is Concept as temporal unfolding, the Coordination is Concept as systematic unity.
 * This enables the dialectical bridge between conceptual Logic and organized Agency.
 */
import { z } from 'zod';
export declare const ProcessSchema: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    steps: z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        type: z.ZodEnum<["task", "decision", "transformation", "synthesis"]>;
        dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        duration: z.ZodOptional<z.ZodNumber>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        dialecticalRole: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        type: "synthesis" | "task" | "decision" | "transformation";
        dialecticalRole: "thesis" | "antithesis" | "synthesis";
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
        duration?: number | undefined;
    }, {
        id: string;
        name: string;
        type: "synthesis" | "task" | "decision" | "transformation";
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
        duration?: number | undefined;
        dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
    }>, "many">;
    approach: z.ZodDefault<z.ZodEnum<["linear", "parallel", "dialectical", "organic"]>>;
    adaptability: z.ZodDefault<z.ZodEnum<["fixed", "flexible", "self-organizing", "transcendent"]>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    name: string;
    approach: "parallel" | "dialectical" | "organic" | "linear";
    steps: {
        id: string;
        name: string;
        type: "synthesis" | "task" | "decision" | "transformation";
        dialecticalRole: "thesis" | "antithesis" | "synthesis";
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
        duration?: number | undefined;
    }[];
    adaptability: "transcendent" | "fixed" | "flexible" | "self-organizing";
}, {
    id: string;
    name: string;
    steps: {
        id: string;
        name: string;
        type: "synthesis" | "task" | "decision" | "transformation";
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
        duration?: number | undefined;
        dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
    }[];
    approach?: "parallel" | "dialectical" | "organic" | "linear" | undefined;
    adaptability?: "transcendent" | "fixed" | "flexible" | "self-organizing" | undefined;
}>;
export declare const CoordinationSchema: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    rules: z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        condition: z.ZodString;
        action: z.ZodString;
        priority: z.ZodDefault<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        priority: number;
        action: string;
        condition: string;
    }, {
        id: string;
        action: string;
        condition: string;
        priority?: number | undefined;
    }>, "many">;
    synchronization: z.ZodDefault<z.ZodEnum<["loose", "tight", "dialectical", "organic"]>>;
    conflictResolution: z.ZodDefault<z.ZodEnum<["hierarchy", "consensus", "dialectical", "transcendent"]>>;
    organizationalPrinciple: z.ZodDefault<z.ZodEnum<["centralized", "distributed", "dialectical", "absolute"]>>;
    emergenceLevel: z.ZodDefault<z.ZodEnum<["none", "basic", "complex", "transcendent"]>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    name: string;
    rules: {
        id: string;
        priority: number;
        action: string;
        condition: string;
    }[];
    synchronization: "dialectical" | "organic" | "loose" | "tight";
    conflictResolution: "dialectical" | "transcendent" | "hierarchy" | "consensus";
    organizationalPrinciple: "absolute" | "dialectical" | "centralized" | "distributed";
    emergenceLevel: "none" | "basic" | "transcendent" | "complex";
}, {
    id: string;
    name: string;
    rules: {
        id: string;
        action: string;
        condition: string;
        priority?: number | undefined;
    }[];
    synchronization?: "dialectical" | "organic" | "loose" | "tight" | undefined;
    conflictResolution?: "dialectical" | "transcendent" | "hierarchy" | "consensus" | undefined;
    organizationalPrinciple?: "absolute" | "dialectical" | "centralized" | "distributed" | undefined;
    emergenceLevel?: "none" | "basic" | "transcendent" | "complex" | undefined;
}>;
export declare const WorkflowSchema: z.ZodObject<{
    id: z.ZodString;
    process: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        steps: z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            name: z.ZodString;
            type: z.ZodEnum<["task", "decision", "transformation", "synthesis"]>;
            dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            duration: z.ZodOptional<z.ZodNumber>;
            resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
            dialecticalRole: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dialecticalRole: "thesis" | "antithesis" | "synthesis";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
        }, {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
            dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
        }>, "many">;
        approach: z.ZodDefault<z.ZodEnum<["linear", "parallel", "dialectical", "organic"]>>;
        adaptability: z.ZodDefault<z.ZodEnum<["fixed", "flexible", "self-organizing", "transcendent"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        approach: "parallel" | "dialectical" | "organic" | "linear";
        steps: {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dialecticalRole: "thesis" | "antithesis" | "synthesis";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
        }[];
        adaptability: "transcendent" | "fixed" | "flexible" | "self-organizing";
    }, {
        id: string;
        name: string;
        steps: {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
            dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
        }[];
        approach?: "parallel" | "dialectical" | "organic" | "linear" | undefined;
        adaptability?: "transcendent" | "fixed" | "flexible" | "self-organizing" | undefined;
    }>;
    coordination: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        rules: z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            condition: z.ZodString;
            action: z.ZodString;
            priority: z.ZodDefault<z.ZodNumber>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            priority: number;
            action: string;
            condition: string;
        }, {
            id: string;
            action: string;
            condition: string;
            priority?: number | undefined;
        }>, "many">;
        synchronization: z.ZodDefault<z.ZodEnum<["loose", "tight", "dialectical", "organic"]>>;
        conflictResolution: z.ZodDefault<z.ZodEnum<["hierarchy", "consensus", "dialectical", "transcendent"]>>;
        organizationalPrinciple: z.ZodDefault<z.ZodEnum<["centralized", "distributed", "dialectical", "absolute"]>>;
        emergenceLevel: z.ZodDefault<z.ZodEnum<["none", "basic", "complex", "transcendent"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        rules: {
            id: string;
            priority: number;
            action: string;
            condition: string;
        }[];
        synchronization: "dialectical" | "organic" | "loose" | "tight";
        conflictResolution: "dialectical" | "transcendent" | "hierarchy" | "consensus";
        organizationalPrinciple: "absolute" | "dialectical" | "centralized" | "distributed";
        emergenceLevel: "none" | "basic" | "transcendent" | "complex";
    }, {
        id: string;
        name: string;
        rules: {
            id: string;
            action: string;
            condition: string;
            priority?: number | undefined;
        }[];
        synchronization?: "dialectical" | "organic" | "loose" | "tight" | undefined;
        conflictResolution?: "dialectical" | "transcendent" | "hierarchy" | "consensus" | undefined;
        organizationalPrinciple?: "absolute" | "dialectical" | "centralized" | "distributed" | undefined;
        emergenceLevel?: "none" | "basic" | "transcendent" | "complex" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["initialized", "running", "paused", "completed", "failed", "transcended"]>>;
    progress: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        systemicLevel: z.ZodEnum<["simple", "complex", "dialectical", "absolute"]>;
        emergentProperties: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        systemicLevel: "absolute" | "dialectical" | "complex" | "simple";
        emergentProperties?: string[] | undefined;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        systemicLevel: "absolute" | "dialectical" | "complex" | "simple";
        emergentProperties?: string[] | undefined;
    }>>;
    metrics: z.ZodOptional<z.ZodObject<{
        efficiency: z.ZodOptional<z.ZodNumber>;
        coherence: z.ZodOptional<z.ZodNumber>;
        adaptability: z.ZodOptional<z.ZodNumber>;
        emergence: z.ZodOptional<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        adaptability?: number | undefined;
        efficiency?: number | undefined;
        coherence?: number | undefined;
        emergence?: number | undefined;
    }, {
        adaptability?: number | undefined;
        efficiency?: number | undefined;
        coherence?: number | undefined;
        emergence?: number | undefined;
    }>>;
    startedAt: z.ZodOptional<z.ZodDate>;
    completedAt: z.ZodOptional<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodAny, z.objectOutputType<{
    id: z.ZodString;
    process: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        steps: z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            name: z.ZodString;
            type: z.ZodEnum<["task", "decision", "transformation", "synthesis"]>;
            dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            duration: z.ZodOptional<z.ZodNumber>;
            resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
            dialecticalRole: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dialecticalRole: "thesis" | "antithesis" | "synthesis";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
        }, {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
            dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
        }>, "many">;
        approach: z.ZodDefault<z.ZodEnum<["linear", "parallel", "dialectical", "organic"]>>;
        adaptability: z.ZodDefault<z.ZodEnum<["fixed", "flexible", "self-organizing", "transcendent"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        approach: "parallel" | "dialectical" | "organic" | "linear";
        steps: {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dialecticalRole: "thesis" | "antithesis" | "synthesis";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
        }[];
        adaptability: "transcendent" | "fixed" | "flexible" | "self-organizing";
    }, {
        id: string;
        name: string;
        steps: {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
            dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
        }[];
        approach?: "parallel" | "dialectical" | "organic" | "linear" | undefined;
        adaptability?: "transcendent" | "fixed" | "flexible" | "self-organizing" | undefined;
    }>;
    coordination: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        rules: z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            condition: z.ZodString;
            action: z.ZodString;
            priority: z.ZodDefault<z.ZodNumber>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            priority: number;
            action: string;
            condition: string;
        }, {
            id: string;
            action: string;
            condition: string;
            priority?: number | undefined;
        }>, "many">;
        synchronization: z.ZodDefault<z.ZodEnum<["loose", "tight", "dialectical", "organic"]>>;
        conflictResolution: z.ZodDefault<z.ZodEnum<["hierarchy", "consensus", "dialectical", "transcendent"]>>;
        organizationalPrinciple: z.ZodDefault<z.ZodEnum<["centralized", "distributed", "dialectical", "absolute"]>>;
        emergenceLevel: z.ZodDefault<z.ZodEnum<["none", "basic", "complex", "transcendent"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        rules: {
            id: string;
            priority: number;
            action: string;
            condition: string;
        }[];
        synchronization: "dialectical" | "organic" | "loose" | "tight";
        conflictResolution: "dialectical" | "transcendent" | "hierarchy" | "consensus";
        organizationalPrinciple: "absolute" | "dialectical" | "centralized" | "distributed";
        emergenceLevel: "none" | "basic" | "transcendent" | "complex";
    }, {
        id: string;
        name: string;
        rules: {
            id: string;
            action: string;
            condition: string;
            priority?: number | undefined;
        }[];
        synchronization?: "dialectical" | "organic" | "loose" | "tight" | undefined;
        conflictResolution?: "dialectical" | "transcendent" | "hierarchy" | "consensus" | undefined;
        organizationalPrinciple?: "absolute" | "dialectical" | "centralized" | "distributed" | undefined;
        emergenceLevel?: "none" | "basic" | "transcendent" | "complex" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["initialized", "running", "paused", "completed", "failed", "transcended"]>>;
    progress: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        systemicLevel: z.ZodEnum<["simple", "complex", "dialectical", "absolute"]>;
        emergentProperties: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        systemicLevel: "absolute" | "dialectical" | "complex" | "simple";
        emergentProperties?: string[] | undefined;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        systemicLevel: "absolute" | "dialectical" | "complex" | "simple";
        emergentProperties?: string[] | undefined;
    }>>;
    metrics: z.ZodOptional<z.ZodObject<{
        efficiency: z.ZodOptional<z.ZodNumber>;
        coherence: z.ZodOptional<z.ZodNumber>;
        adaptability: z.ZodOptional<z.ZodNumber>;
        emergence: z.ZodOptional<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        adaptability?: number | undefined;
        efficiency?: number | undefined;
        coherence?: number | undefined;
        emergence?: number | undefined;
    }, {
        adaptability?: number | undefined;
        efficiency?: number | undefined;
        coherence?: number | undefined;
        emergence?: number | undefined;
    }>>;
    startedAt: z.ZodOptional<z.ZodDate>;
    completedAt: z.ZodOptional<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, z.ZodAny, "strip">, z.objectInputType<{
    id: z.ZodString;
    process: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        steps: z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            name: z.ZodString;
            type: z.ZodEnum<["task", "decision", "transformation", "synthesis"]>;
            dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            duration: z.ZodOptional<z.ZodNumber>;
            resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
            dialecticalRole: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dialecticalRole: "thesis" | "antithesis" | "synthesis";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
        }, {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
            dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
        }>, "many">;
        approach: z.ZodDefault<z.ZodEnum<["linear", "parallel", "dialectical", "organic"]>>;
        adaptability: z.ZodDefault<z.ZodEnum<["fixed", "flexible", "self-organizing", "transcendent"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        approach: "parallel" | "dialectical" | "organic" | "linear";
        steps: {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dialecticalRole: "thesis" | "antithesis" | "synthesis";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
        }[];
        adaptability: "transcendent" | "fixed" | "flexible" | "self-organizing";
    }, {
        id: string;
        name: string;
        steps: {
            id: string;
            name: string;
            type: "synthesis" | "task" | "decision" | "transformation";
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
            duration?: number | undefined;
            dialecticalRole?: "thesis" | "antithesis" | "synthesis" | undefined;
        }[];
        approach?: "parallel" | "dialectical" | "organic" | "linear" | undefined;
        adaptability?: "transcendent" | "fixed" | "flexible" | "self-organizing" | undefined;
    }>;
    coordination: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        rules: z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            condition: z.ZodString;
            action: z.ZodString;
            priority: z.ZodDefault<z.ZodNumber>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            priority: number;
            action: string;
            condition: string;
        }, {
            id: string;
            action: string;
            condition: string;
            priority?: number | undefined;
        }>, "many">;
        synchronization: z.ZodDefault<z.ZodEnum<["loose", "tight", "dialectical", "organic"]>>;
        conflictResolution: z.ZodDefault<z.ZodEnum<["hierarchy", "consensus", "dialectical", "transcendent"]>>;
        organizationalPrinciple: z.ZodDefault<z.ZodEnum<["centralized", "distributed", "dialectical", "absolute"]>>;
        emergenceLevel: z.ZodDefault<z.ZodEnum<["none", "basic", "complex", "transcendent"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        rules: {
            id: string;
            priority: number;
            action: string;
            condition: string;
        }[];
        synchronization: "dialectical" | "organic" | "loose" | "tight";
        conflictResolution: "dialectical" | "transcendent" | "hierarchy" | "consensus";
        organizationalPrinciple: "absolute" | "dialectical" | "centralized" | "distributed";
        emergenceLevel: "none" | "basic" | "transcendent" | "complex";
    }, {
        id: string;
        name: string;
        rules: {
            id: string;
            action: string;
            condition: string;
            priority?: number | undefined;
        }[];
        synchronization?: "dialectical" | "organic" | "loose" | "tight" | undefined;
        conflictResolution?: "dialectical" | "transcendent" | "hierarchy" | "consensus" | undefined;
        organizationalPrinciple?: "absolute" | "dialectical" | "centralized" | "distributed" | undefined;
        emergenceLevel?: "none" | "basic" | "transcendent" | "complex" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["initialized", "running", "paused", "completed", "failed", "transcended"]>>;
    progress: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        systemicLevel: z.ZodEnum<["simple", "complex", "dialectical", "absolute"]>;
        emergentProperties: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        systemicLevel: "absolute" | "dialectical" | "complex" | "simple";
        emergentProperties?: string[] | undefined;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        systemicLevel: "absolute" | "dialectical" | "complex" | "simple";
        emergentProperties?: string[] | undefined;
    }>>;
    metrics: z.ZodOptional<z.ZodObject<{
        efficiency: z.ZodOptional<z.ZodNumber>;
        coherence: z.ZodOptional<z.ZodNumber>;
        adaptability: z.ZodOptional<z.ZodNumber>;
        emergence: z.ZodOptional<z.ZodNumber>;
    }, "strip", z.ZodTypeAny, {
        adaptability?: number | undefined;
        efficiency?: number | undefined;
        coherence?: number | undefined;
        emergence?: number | undefined;
    }, {
        adaptability?: number | undefined;
        efficiency?: number | undefined;
        coherence?: number | undefined;
        emergence?: number | undefined;
    }>>;
    startedAt: z.ZodOptional<z.ZodDate>;
    completedAt: z.ZodOptional<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, z.ZodAny, "strip">>;
export type Process = z.infer<typeof ProcessSchema>;
export type Coordination = z.infer<typeof CoordinationSchema>;
export type Workflow = z.infer<typeof WorkflowSchema>;
/**
 * Workflow Factory: Creates workflows with dialectical organization principles
 */
export declare class WorkflowFactory {
    /**
     * Create a Being-level workflow (linear, simple coordination)
     */
    static createBeingWorkflow(name: string, steps: string[]): Workflow;
    /**
     * Create an Essence-level workflow (parallel, mediated coordination)
     */
    static createEssenceWorkflow(name: string, processes: Array<{
        name: string;
        steps: string[];
    }>): Workflow;
    /**
     * Create a Concept-level workflow (organic, self-organizing coordination)
     */
    static createConceptWorkflow(name: string, conceptStructure: Record<string, any>): Workflow;
    /**
     * Create an Absolute Idea workflow (transcendent, absolute coordination)
     */
    static createAbsoluteWorkflow(name: string, systemContext: Record<string, any>): Workflow;
}
export default WorkflowSchema;
//# sourceMappingURL=workflow.d.ts.map