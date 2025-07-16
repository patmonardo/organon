import { RelationshipType } from "@/projection";
import { ConfigLoader } from "./ConfigLoader";
import { ConfigValidation } from "./ConfigValidation";
import { GraphStoreFileExporterConfig } from "./GraphStoreConfigs";
import { GraphStoreDatabaseExporterConfig } from "./GraphStoreConfigs";
import { GraphStoreFileImporterConfig } from "./GraphStoreConfigs";
import { GraphStoreDatabaseImporterConfig } from "./GraphStoreConfigs";
import { GraphStoreMemoryConfig } from "./GraphStoreConfigs";
import { GraphStoreCacheConfig } from "./GraphStoreConfigs";
import { GraphStoreComputeConfig } from "./GraphStoreConfigs";

/**
 * Factory functions for GraphStore configuration objects.
 */
export class GraphStoreConfigFactory {
  static fileExporter(
    params: Partial<GraphStoreFileExporterConfig> = {}
  ): GraphStoreFileExporterConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreFileExporterConfig>("graphstore.export.file");
    const builtInDefaults: GraphStoreFileExporterConfig = {
      exportPath: "/tmp/graphstore-export",
      writeConcurrency: 4,
      batchSize: 10000,
      defaultRelationshipType: RelationshipType.of("REL"),
      compressionEnabled: true,
      compressionLevel: 6,
      includeMetadata: true,
    };

    const config: GraphStoreFileExporterConfig = {
      ...builtInDefaults,
      ...fileDefaults,
      ...params,
    };

    ConfigValidation.validateRequired(config.exportPath, "exportPath");
    ConfigValidation.validatePath(config.exportPath);
    ConfigValidation.validatePositive(
      config.writeConcurrency,
      "writeConcurrency"
    );
    ConfigValidation.validateRange(
      config.writeConcurrency,
      1,
      100,
      "writeConcurrency"
    );
    ConfigValidation.validatePositive(config.batchSize, "batchSize");
    ConfigValidation.validateRange(
      config.compressionLevel,
      1,
      9,
      "compressionLevel"
    );

    return config;
  }

  static databaseExporter(
    params: Partial<GraphStoreDatabaseExporterConfig> = {}
  ): GraphStoreDatabaseExporterConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreDatabaseExporterConfig>("graphstore.export.database");
    const builtInDefaults: GraphStoreDatabaseExporterConfig = {
      databaseName: "graphstore-export",
      writeConcurrency: 4,
      batchSize: 10000,
      defaultRelationshipType: RelationshipType.of("REL"),
      enableDebugLog: false,
      databaseFormat: "standard",
      highIO: false,
      force: false,
      transactional: true,
      indexOptimization: true,
    };

    const config: GraphStoreDatabaseExporterConfig = {
      ...builtInDefaults,
      ...fileDefaults,
      ...params,
    };

    ConfigValidation.validateRequired(config.databaseName, "databaseName");
    ConfigValidation.validateDatabaseName(config.databaseName);
    ConfigValidation.validatePositive(
      config.writeConcurrency,
      "writeConcurrency"
    );
    ConfigValidation.validateRange(
      config.writeConcurrency,
      1,
      100,
      "writeConcurrency"
    );
    ConfigValidation.validatePositive(config.batchSize, "batchSize");

    return config;
  }

  static fileImporter(
    params: Partial<GraphStoreFileImporterConfig> = {}
  ): GraphStoreFileImporterConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreFileImporterConfig>("graphstore.import.file");
    const builtInDefaults: GraphStoreFileImporterConfig = {
      importPath: "/tmp/graphstore-import",
      readConcurrency: 4,
      batchSize: 10000,
      skipInvalidLines: false,
      delimiter: ",",
      quotationCharacter: '"',
      escapeCharacter: "\\",
      autoDetectFormat: true,
      validateSchema: true,
      progressReporting: true,
    };

    const config = { ...builtInDefaults, ...fileDefaults, ...params };

    ConfigValidation.validateRequired(config.importPath, "importPath");
    ConfigValidation.validatePath(config.importPath);
    ConfigValidation.validatePositive(
      config.readConcurrency,
      "readConcurrency"
    );
    ConfigValidation.validatePositive(config.batchSize, "batchSize");

    return config;
  }

  static databaseImporter(
    params: Partial<GraphStoreDatabaseImporterConfig> = {}
  ): GraphStoreDatabaseImporterConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreDatabaseImporterConfig>("graphstore.import.database");
    const builtInDefaults: GraphStoreDatabaseImporterConfig = {
      databaseName: "graphstore-source",
      readConcurrency: 4,
      batchSize: 10000,
      connectionPool: 10,
      queryTimeout: 300000,
      retryAttempts: 3,
    };

    const config = { ...builtInDefaults, ...fileDefaults, ...params };

    ConfigValidation.validateRequired(config.databaseName, "databaseName");
    ConfigValidation.validateDatabaseName(config.databaseName);
    ConfigValidation.validatePositive(
      config.readConcurrency,
      "readConcurrency"
    );
    ConfigValidation.validatePositive(config.batchSize, "batchSize");
    ConfigValidation.validatePositive(config.connectionPool, "connectionPool");
    ConfigValidation.validatePositive(config.queryTimeout, "queryTimeout");
    ConfigValidation.validateRange(
      config.retryAttempts,
      0,
      10,
      "retryAttempts"
    );

    return config;
  }

  static memory(
    params: Partial<GraphStoreMemoryConfig> = {}
  ): GraphStoreMemoryConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreMemoryConfig>("graphstore.memory");
    const builtInDefaults: GraphStoreMemoryConfig = {
      maxHeapSize: "8G",
      offHeapEnabled: true,
      offHeapSize: "16G",
      compressionEnabled: true,
      garbageCollectionTuning: "G1GC",
      memoryMappedFiles: true,
    };

    const config = { ...builtInDefaults, ...fileDefaults, ...params };

    ConfigValidation.validateRequired(config.maxHeapSize, "maxHeapSize");
    ConfigValidation.validateMemorySize(config.maxHeapSize);
    if (config.offHeapEnabled && config.offHeapSize) {
      ConfigValidation.validateMemorySize(config.offHeapSize);
    }

    return config;
  }

  static cache(
    params: Partial<GraphStoreCacheConfig> = {}
  ): GraphStoreCacheConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreCacheConfig>("graphstore.cache");
    const builtInDefaults: GraphStoreCacheConfig = {
      nodeCacheSize: 1000000,
      relationshipCacheSize: 5000000,
      propertyCacheSize: 2000000,
      cacheEvictionPolicy: "LRU",
      cacheCompressionEnabled: true,
      cacheStatisticsEnabled: true,
    };

    const config = { ...builtInDefaults, ...fileDefaults, ...params };

    ConfigValidation.validatePositive(config.nodeCacheSize, "nodeCacheSize");
    ConfigValidation.validatePositive(
      config.relationshipCacheSize,
      "relationshipCacheSize"
    );
    ConfigValidation.validatePositive(
      config.propertyCacheSize,
      "propertyCacheSize"
    );
    ConfigValidation.validateCacheEvictionPolicy(config.cacheEvictionPolicy);

    return config;
  }

  static compute(
    params: Partial<GraphStoreComputeConfig> = {}
  ): GraphStoreComputeConfig {
    const fileDefaults =
      ConfigLoader.getDefaults<GraphStoreComputeConfig>("graphstore.compute");
    const builtInDefaults: GraphStoreComputeConfig = {
      computeConcurrency: 8,
      algorithmTimeout: 1800000,
      writeTimeout: 300000,
      computeMemoryLimit: "4G",
      enableComputeStatistics: true,
      tempDirectoryPath: "/tmp/graphstore-compute",
    };

    const config = { ...builtInDefaults, ...fileDefaults, ...params };

    ConfigValidation.validatePositive(
      config.computeConcurrency,
      "computeConcurrency"
    );
    ConfigValidation.validateRange(
      config.computeConcurrency,
      1,
      256,
      "computeConcurrency"
    );
    ConfigValidation.validatePositive(
      config.algorithmTimeout,
      "algorithmTimeout"
    );
    ConfigValidation.validatePositive(config.writeTimeout, "writeTimeout");
    ConfigValidation.validateMemorySize(config.computeMemoryLimit);
    ConfigValidation.validatePath(config.tempDirectoryPath);

    return config;
  }

  /**
   * Create a complete GraphStore configuration with all subsystem configs
   */
  static complete(
    params: Partial<{
      fileExporter: Partial<GraphStoreFileExporterConfig>;
      databaseExporter: Partial<GraphStoreDatabaseExporterConfig>;
      fileImporter: Partial<GraphStoreFileImporterConfig>;
      databaseImporter: Partial<GraphStoreDatabaseImporterConfig>;
      memory: Partial<GraphStoreMemoryConfig>;
      cache: Partial<GraphStoreCacheConfig>;
      compute: Partial<GraphStoreComputeConfig>;
    }> = {}
  ) {
    return {
      fileExporter: GraphStoreConfigFactory.fileExporter(
        params.fileExporter
      ),
      databaseExporter: GraphStoreConfigFactory.databaseExporter(
        params.databaseExporter
      ),
      fileImporter: GraphStoreConfigFactory.fileImporter(
        params.fileImporter
      ),
      databaseImporter: GraphStoreConfigFactory.databaseImporter(
        params.databaseImporter
      ),
      memory: GraphStoreConfigFactory.memory(params.memory),
      cache: GraphStoreConfigFactory.cache(params.cache),
      compute: GraphStoreConfigFactory.compute(params.compute),
    };
  }
}
