import { Orientation } from "@/projection";
import { RelationshipType } from "@/projection";
import { RelationshipProjection } from "@/projection";
import { GraphCharacteristics } from "@/api";
import { Aggregation } from "@/core";
import { IdMap } from "@/api";
import { PartialIdMap } from "@/api";
import { DefaultValue } from "@/api";
import { PropertyState } from "@/api";
import { Direction } from "@/api/schema";
import { GraphSchema } from "@/api/schema";
import { MutableGraphSchema } from "@/api/schema";
import { NodeSchema } from "@/api/schema";
import { MutableNodeSchema } from "@/api/schema";
import { MutableRelationshipSchema } from "@/api/schema";
import { NodePropertyValues } from "@/api/properties";
import { IdMapBehaviorServiceProvider } from "@/core";
import { Concurrency } from "@/concurrency";
import { DefaultPool } from "@/concurrency";
import { HugeGraph } from "@/core/huge";
import { NodesBuilder } from "@/core/loading";
import { IdMapBuilder } from "@/core/loading";
import { HighLimitIdMap } from "@/core/loading";
import { NodesBuilderContext } from "@/core/loading/";
import { RelationshipsBuilder } from "@/core/loading";
import { SingleTypeRelationships } from "@/core/loading";
import { SingleTypeRelationshipImporterBuilder } from "@/core/loading";
import { SingleTypeRelationshipsBuilderBuilder } from "@/core/loading";
import { LocalRelationshipsBuilderProvider } from "@/core/loading";
import { ImmutableImportMetaData } from "@/core/loading";
import { ImportSizing } from "@/core/loading";
import { RecordsBatchBuffer } from "@/core/loading";

const NO_SUCH_RELATIONSHIP_TYPE = -1;

export class GraphFactory {
  private constructor() {}

  // Method overloads (declare the signatures)
  static initNodesBuilder(): NodesBuilderBuilder;
  static initNodesBuilder(nodeSchema: NodeSchema): NodesBuilderBuilder;

  static initNodesBuilder(nodeSchema?: NodeSchema): NodesBuilderBuilder {
    const builder = new NodesBuilderBuilder();

    if (nodeSchema) {
      return builder.nodeSchema(nodeSchema);
    }

    return builder;
  }

  static nodesBuilder(
    maxOriginalId?: number,
    nodeCount?: number,
    nodeSchema?: NodeSchema,
    hasLabelInformation?: boolean,
    hasProperties?: boolean,
    deduplicateIds?: boolean,
    concurrency?: Concurrency,
    propertyState?: PropertyState,
    idMapBuilderType?: string,
    usePooledBuilderProvider?: boolean
  ): NodesBuilder {
    const labelInformation = nodeSchema
      ? !(
          nodeSchema.availableLabels().isEmpty() ||
          nodeSchema.containsOnlyAllNodesLabel()
        )
      : hasLabelInformation ?? false;

    const threadCount = concurrency ?? new Concurrency(1);

    // Make sure that we do not pass on negative values to the id map builders.
    const filteredMaxOriginalId =
      maxOriginalId !== undefined && maxOriginalId > 0
        ? maxOriginalId
        : undefined;

    const idMapBehavior = IdMapBehaviorServiceProvider.idMapBehavior();

    const idMapType = idMapBuilderType ?? IdMap.NO_TYPE;
    const idMapBuilder = idMapBehavior.create(
      idMapType,
      threadCount,
      filteredMaxOriginalId,
      nodeCount
    );

    const maxOriginalNodeId =
      filteredMaxOriginalId ?? NodesBuilder.UNKNOWN_MAX_ID;
    const deduplicate = deduplicateIds ?? true;
    const usePooled = usePooledBuilderProvider ?? false;
    let maxIntermediateId = maxOriginalNodeId;

    if (HighLimitIdMap.isHighLimitIdMap(idMapType)) {
      // If the requested id map is high limit, we need to make sure that
      // internal data structures are sized accordingly. Using the highest
      // original id will potentially fail due to size limitations.
      // If the node count is not given, we fall back to an unknown max id,
      // which is fine since label building relies on growing bitsets.
      maxIntermediateId = nodeCount
        ? nodeCount - 1
        : NodesBuilder.UNKNOWN_MAX_ID;

      if (deduplicate) {
        // We internally use HABS for deduplication, which is being initialized
        // with max original id. This is fine for all id maps except high limit,
        // where original ids can exceed the supported HABS range.
        throw new Error("Cannot use high limit id map with deduplication.");
      }
    }

    return nodeSchema
      ? GraphFactory.fromSchema(
          maxOriginalNodeId,
          maxIntermediateId,
          idMapBuilder,
          threadCount,
          nodeSchema,
          labelInformation,
          deduplicate,
          usePooled
        )
      : new NodesBuilder(
          maxOriginalNodeId,
          maxIntermediateId,
          threadCount,
          NodesBuilderContext.lazy(threadCount),
          idMapBuilder,
          labelInformation,
          hasProperties ?? false,
          deduplicate,
          usePooled,
          (__) => propertyState ?? PropertyState.PERSISTENT
        );
  }

  private static fromSchema(
    maxOriginalId: number,
    maxIntermediateId: number,
    idMapBuilder: IdMapBuilder,
    concurrency: Concurrency,
    nodeSchema: NodeSchema,
    hasLabelInformation: boolean,
    deduplicateIds: boolean,
    usePooledBuilderProvider: boolean
  ): NodesBuilder {
    return new NodesBuilder(
      maxOriginalId,
      maxIntermediateId,
      concurrency,
      NodesBuilderContext.fixed(nodeSchema, concurrency),
      idMapBuilder,
      hasLabelInformation,
      nodeSchema.hasProperties(),
      deduplicateIds,
      usePooledBuilderProvider,
      (propertyKey) => nodeSchema.unionProperties().get(propertyKey).state()
    );
  }

  static initRelationshipsBuilder(): RelationshipsBuilderBuilder {
    return new RelationshipsBuilderBuilder();
  }

  static relationshipsBuilder(
    nodes: PartialIdMap,
    relationshipType: RelationshipType,
    orientation?: Orientation,
    propertyConfigs: PropertyConfig[] = [],
    aggregation?: Aggregation,
    skipDanglingRelationships?: boolean,
    concurrency?: Concurrency,
    indexInverse?: boolean,
    executorService?: any,
    usePooledBuilderProvider?: boolean
  ): RelationshipsBuilder {
    const loadRelationshipProperties = propertyConfigs.length > 0;

    const aggregations =
      propertyConfigs.length === 0
        ? [aggregation ?? Aggregation.DEFAULT]
        : propertyConfigs.map((config) =>
            Aggregation.resolve(config.aggregation())
          );

    const isMultiGraph = aggregations.every((agg) => agg.equivalentToNone());

    const actualOrientation = orientation ?? Orientation.NATURAL;
    const projectionBuilder = RelationshipProjection.builder()
      .type(relationshipType.name())
      .orientation(actualOrientation)
      .indexInverse(indexInverse ?? false);

    propertyConfigs.forEach((propertyConfig) =>
      projectionBuilder.addProperty(
        propertyConfig.propertyKey(),
        propertyConfig.propertyKey(),
        DefaultValue.of(propertyConfig.defaultValue()),
        propertyConfig.aggregation()
      )
    );

    const projection = projectionBuilder.build();

    const propertyKeyIds = Array.from(
      { length: propertyConfigs.length },
      (_, i) => i
    );
    const defaultValues = propertyConfigs.map((c) =>
      c.defaultValue().doubleValue()
    );

    const finalConcurrency = concurrency ?? new Concurrency(1);
    const maybeRootNodeCount = nodes.rootNodeCount();
    const importSizing =
      maybeRootNodeCount !== undefined
        ? ImportSizing.of(finalConcurrency, maybeRootNodeCount)
        : ImportSizing.of(finalConcurrency);

    let bufferSize = RecordsBatchBuffer.DEFAULT_BUFFER_SIZE;
    if (maybeRootNodeCount !== undefined) {
      const rootNodeCount = maybeRootNodeCount;
      if (
        rootNodeCount > 0 &&
        rootNodeCount < RecordsBatchBuffer.DEFAULT_BUFFER_SIZE
      ) {
        bufferSize = Math.floor(rootNodeCount);
      }
    }

    const skipDangling = skipDanglingRelationships ?? true;

    const importMetaData = ImmutableImportMetaData.builder()
      .projection(projection)
      .aggregations(aggregations)
      .propertyKeyIds(propertyKeyIds)
      .defaultValues(defaultValues)
      .typeTokenId(NO_SUCH_RELATIONSHIP_TYPE)
      .skipDanglingRelationships(skipDangling)
      .build();

    const singleTypeRelationshipImporter =
      new SingleTypeRelationshipImporterBuilder()
        .importMetaData(importMetaData)
        .nodeCountSupplier(() => nodes.rootNodeCount() ?? 0)
        .importSizing(importSizing)
        .build();

    const singleTypeRelationshipsBuilderBuilder =
      new SingleTypeRelationshipsBuilderBuilder()
        .idMap(nodes)
        .importer(singleTypeRelationshipImporter)
        .bufferSize(bufferSize)
        .relationshipType(relationshipType)
        .propertyConfigs(propertyConfigs)
        .isMultiGraph(isMultiGraph)
        .loadRelationshipProperty(loadRelationshipProperties)
        .direction(Direction.fromOrientation(actualOrientation))
        .executorService(executorService ?? DefaultPool.INSTANCE)
        .concurrency(finalConcurrency);

    if (indexInverse ?? false) {
      const inverseProjection = RelationshipProjection.builder()
        .from(projection)
        .orientation(projection.orientation().inverse())
        .build();

      const inverseImportMetaData = ImmutableImportMetaData.builder()
        .from(importMetaData)
        .projection(inverseProjection)
        .skipDanglingRelationships(skipDangling)
        .build();

      const inverseImporter = new SingleTypeRelationshipImporterBuilder()
        .importMetaData(inverseImportMetaData)
        .nodeCountSupplier(() => nodes.rootNodeCount() ?? 0)
        .importSizing(importSizing)
        .build();

      singleTypeRelationshipsBuilderBuilder.inverseImporter(inverseImporter);
    }

    const singleTypeRelationshipsBuilder =
      singleTypeRelationshipsBuilderBuilder.build();

    const localBuilderProvider =
      usePooledBuilderProvider ?? false
        ? LocalRelationshipsBuilderProvider.pooled(
            () =>
              singleTypeRelationshipsBuilder.threadLocalRelationshipsBuilder(),
            finalConcurrency
          )
        : LocalRelationshipsBuilderProvider.threadLocal(() =>
            singleTypeRelationshipsBuilder.threadLocalRelationshipsBuilder()
          );

    return new RelationshipsBuilder(
      singleTypeRelationshipsBuilder,
      localBuilderProvider,
      skipDangling
    );
  }

  /**
   * Creates a {@link HugeGraph} from the given node and relationship data.
   *
   * The node schema will be inferred from the available node labels.
   * The relationship schema will use default relationship type {@code "REL"}.
   * If a relationship property is present, the default relationship property key {@code "property"}
   * will be used.
   */
  static create(
    idMap: IdMap,
    relationships: SingleTypeRelationships
  ): HugeGraph {
    const nodeSchema = MutableNodeSchema.empty();
    idMap
      .availableNodeLabels()
      .forEach((label) => nodeSchema.getOrCreateLabel(label));

    const relationshipProperties = relationships.properties();
    if (relationshipProperties) {
      if (relationshipProperties.values().size() !== 1) {
        throw new Error(
          "Cannot instantiate graph with more than one relationship property."
        );
      }
    }

    const relationshipSchema = MutableRelationshipSchema.empty();
    relationshipSchema.set(relationships.relationshipSchemaEntry());

    return GraphFactory.create(
      MutableGraphSchema.of(nodeSchema, relationshipSchema, new Map()),
      idMap,
      new Map(),
      relationships
    );
  }

  static create(
    graphSchema: GraphSchema,
    idMap: IdMap,
    nodeProperties: Map<string, NodePropertyValues>,
    relationships: SingleTypeRelationships
  ): HugeGraph {
    const topology = relationships.topology();
    const inverseTopology = relationships.inverseTopology();

    const properties = relationships.properties()
      ? relationships.properties()!.values().size() === 1
        ? relationships.properties()!.values().values().next().value.values()
        : undefined
      : undefined;

    const inverseProperties = relationships.inverseProperties()
      ? relationships.inverseProperties()!.values().size() === 1
        ? relationships
            .inverseProperties()!
            .values()
            .values()
            .next()
            .value.values()
        : undefined
      : undefined;

    const characteristicsBuilder = GraphCharacteristics.builder().withDirection(
      graphSchema.direction()
    );
    if (relationships.inverseTopology()) {
      characteristicsBuilder.inverseIndexed();
    }

    return HugeGraph.create(
      idMap,
      graphSchema,
      characteristicsBuilder.build(),
      nodeProperties,
      topology,
      inverseTopology,
      properties,
      inverseProperties
    );
  }
}
