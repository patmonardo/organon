import { NodeLabel } from "@/projection";
import { RelationshipType } from "@/projection";
import { ValueType } from "@/api/";
import { Capabilities } from "@/core/loading";
import { DeletionResult } from "@/core/loading";
import { SingleTypeRelationships } from "@/core/loading";
import { GraphSchema } from "./schema";
import { GraphProperty } from "./properties";
import { GraphPropertyValues } from "./properties";
import { NodeProperty } from "./properties";
import { NodePropertyValues } from "./properties";
import { RelationshipProperty } from "./properties";
import { RelationshipPropertyStore } from "./properties";
import { Graph } from "./Graph";
import { GraphStore } from "./GraphStore";
import { DatabaseInfo } from "./DatabaseInfo";
import { IdMap } from "./IdMap";
import { Topology } from "./Topology";
import { CompositeRelationshipIterator } from "./CompositeRelationshipIterator";

export abstract class GraphStoreAdapter implements GraphStore {
  protected readonly graphStore: GraphStore;

  protected constructor(graphStore: GraphStore) {
    this.graphStore = graphStore;
  }

  // --- Graph Properties ---

  databaseInfo(): DatabaseInfo {
    return this.graphStore.databaseInfo();
  }

  capabilities(): Capabilities {
    return this.graphStore.capabilities();
  }

  schema(): GraphSchema {
    return this.graphStore.schema();
  }

  creationTime(): Date {
    return this.graphStore.creationTime();
  }

  modificationTime(): Date {
    return this.graphStore.modificationTime();
  }

  graphPropertyKeys(): Set<string> {
    return this.graphStore.graphPropertyKeys();
  }

  hasGraphProperty(propertyKey: string): boolean {
    return this.graphStore.hasGraphProperty(propertyKey);
  }

  graphProperty(propertyKey: string): GraphProperty {
    return this.graphStore.graphProperty(propertyKey);
  }

  graphPropertyValues(propertyKey: string): GraphPropertyValues {
    return this.graphStore.graphPropertyValues(propertyKey);
  }

  addGraphProperty(
    propertyKey: string,
    propertyValues: GraphPropertyValues
  ): void {
    this.graphStore.addGraphProperty(propertyKey, propertyValues);
  }

  removeGraphProperty(propertyKey: string): void {
    this.graphStore.removeGraphProperty(propertyKey);
  }

  // --- Nodes ---

  nodeCount(): number {
    return this.graphStore.nodeCount();
  }

  nodes(): IdMap {
    return this.graphStore.nodes();
  }

  nodeLabels(): Set<NodeLabel> {
    return this.graphStore.nodeLabels();
  }

  addNodeLabel(nodeLabel: NodeLabel): void {
    this.graphStore.addNodeLabel(nodeLabel);
  }

  // --- Node Properties ---

  // Overloads
  nodePropertyKeys(): Set<string>;
  nodePropertyKeys(label: NodeLabel): Set<string>;
  nodePropertyKeys(labels: Set<NodeLabel>): string[];

  // Implementation
  nodePropertyKeys(
    labelOrLabels?: NodeLabel | Set<NodeLabel>
  ): Set<string> | string[] {
    if (labelOrLabels === undefined) {
      return this.graphStore.nodePropertyKeys();
    }
    // Single label
    return this.graphStore.nodePropertyKeys(labelOrLabels as NodeLabel);
  }

  hasNodeProperty(propertyKey: string): boolean;
  hasNodeProperty(label: NodeLabel, propertyKey: string): boolean;
  hasNodeProperty(labels: Set<NodeLabel>, propertyKey: string): boolean;
  hasNodeProperty(
    labelOrLabelsOrKey: NodeLabel | Set<NodeLabel> | string,
    propertyKey?: string
  ): boolean {
    if (propertyKey === undefined) {
      // Only propertyKey provided
      return this.graphStore.hasNodeProperty(labelOrLabelsOrKey as string);
    }
    return this.graphStore.hasNodeProperty(
      labelOrLabelsOrKey as NodeLabel,
      propertyKey
    );
  }

  nodeProperty(propertyKey: string): NodeProperty {
    return this.graphStore.nodeProperty(propertyKey);
  }

  addNodeProperty(
    nodeLabels: Set<NodeLabel>,
    propertyKey: string,
    propertyValues: NodePropertyValues
  ): void {
    this.graphStore.addNodeProperty(nodeLabels, propertyKey, propertyValues);
  }

  removeNodeProperty(propertyKey: string): void {
    this.graphStore.removeNodeProperty(propertyKey);
  }

  // --- Relationships ---

  relationshipCount(): number;
  relationshipCount(relationshipType: RelationshipType): number;
  relationshipCount(relationshipType?: RelationshipType): number {
    if (relationshipType !== undefined) {
      return this.graphStore.relationshipCount(relationshipType);
    }
    return this.graphStore.relationshipCount();
  }

  relationshipTypes(): Set<RelationshipType> {
    return this.graphStore.relationshipTypes();
  }

  hasRelationshipType(relationshipType: RelationshipType): boolean {
    return this.graphStore.hasRelationshipType(relationshipType);
  }

  inverseIndexedRelationshipTypes(): Set<RelationshipType> {
    return this.graphStore.inverseIndexedRelationshipTypes();
  }

  hasRelationshipProperty(
    relType: RelationshipType,
    propertyKey: string
  ): boolean {
    return this.graphStore.hasRelationshipProperty(relType, propertyKey);
  }

  relationshipPropertyKeys(): Set<string>;
  relationshipPropertyKeys(relationshipType: RelationshipType): Set<string>;
  relationshipPropertyKeys(relTypes: Set<RelationshipType>): Set<string>;

  relationshipPropertyKeys(
    relTypesOrType?: RelationshipType | Set<RelationshipType>
  ): Set<string> | string[] {
    if (relTypesOrType === undefined) {
      return this.graphStore.relationshipPropertyKeys();
    }
    return this.graphStore.relationshipPropertyKeys(
      relTypesOrType as RelationshipType
    );
  }

  relationshipPropertyType(propertyKey: string): ValueType {
    return this.graphStore.relationshipPropertyType(propertyKey);
  }
  relationshipPropertyValues(
    relationshipType: RelationshipType,
    propertyKey: string
  ): RelationshipProperty {
    return this.graphStore.relationshipPropertyValues(
      relationshipType,
      propertyKey
    );
  }

  addRelationshipType(relationships: SingleTypeRelationships): void {
    this.graphStore.addRelationshipType(relationships);
  }

  addInverseIndex(
    relationshipType: RelationshipType,
    topology: Topology,
    properties?: RelationshipPropertyStore
  ): void {
    this.graphStore.addInverseIndex(relationshipType, topology, properties);
  }
  deleteRelationships(relationshipType: RelationshipType): DeletionResult {
    return this.graphStore.deleteRelationships(relationshipType);
  }

  // --- Graph Retrieval (getGraph) ---

  getGraph(nodeLabel: NodeLabel): Graph;
  getGraph(nodeLabels: Set<NodeLabel>): Graph;
  getGraph(relationshipTypes: Set<RelationshipType>): Graph;
  getGraph(relationshipProperty: string): Graph;
  getGraph(
    relationshipType: RelationshipType,
    relationshipProperty?: string
  ): Graph;
  getGraph(
    relationshipTypes: Set<RelationshipType>,
    relationshipProperty?: string
  ): Graph;
  getGraph(
    nodeLabel: string,
    relationshipType: string,
    relationshipProperty?: string
  ): Graph;
  getGraph(
    nodeLabel: NodeLabel,
    relationshipType: RelationshipType,
    relationshipProperty?: string
  ): Graph;
  getGraph(
    nodeLabels: Set<NodeLabel>,
    relationshipTypes: Array<RelationshipType>,
    relationshipProperty?: string
  ): Graph;
  getGraph(param1: any, param2?: any, param3?: any): Graph {
    return (this.graphStore.getGraph as any)(param1, param2, param3);
  }

  getUnion(): Graph {
    return this.graphStore.getUnion();
  }

  getCompositeRelationshipIterator(
    relationshipType: RelationshipType,
    propertyKeys: Set<string>
  ): CompositeRelationshipIterator {
    return this.graphStore.getCompositeRelationshipIterator(
      relationshipType,
      propertyKeys
    );
  }
}
