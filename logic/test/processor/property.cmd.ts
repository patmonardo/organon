import { Neo4jConnection } from "../connection";
import { PropertyDefinitionRepository } from "./property.def";
import { PropertyShapeRepository } from "../../src/repository/property";
import { FormPropertyDefinition, FormProperty } from "@/form/schema/property";
import neo4j from "neo4j-driver";
import { v4 as uuidv4 } from "uuid";
import * as dotenv from "dotenv";

// Load environment variables
dotenv.config();

// --- Configuration ---
const NEO4J_URI = process.env.NEO4J_URI || "neo4j://localhost:7687";
const NEO4J_USER = process.env.NEO4J_USER || "neo4j";
const NEO4J_PASSWORD = process.env.NEO4J_PASSWORD || "password";
// --- End Configuration ---

async function testPropertyRepositories() {
  console.log("🚀 Starting Property Repository Test Script...");

  let connection: Neo4jConnection | null = null;
  // Create and initialize connection
  connection = new Neo4jConnection({
    uri: process.env.NEO4J_URI || "neo4j://localhost:7687",
    username: process.env.NEO4J_USERNAME || "neo4j",
    password: process.env.NEO4J_PASSWORD || "neo4j",
    useDefaultDriver: false,
  });
  const defRepo = new PropertyDefinitionRepository(connection);
  const shapeRepo = new PropertyShapeRepository(connection);

  // Generate unique test IDs
  const testContextId = `ctx-${uuidv4()}`;
  const testEntityId = `ent-${uuidv4()}`;
  const testPropertyId = `prop-${uuidv4()}`;
  const testDefinitionId = `def-${uuidv4()}`;

  try {
    await connection.initialize();
    console.log("✅ Neo4j Connection Established");

    // --- Create prerequisite nodes ---
    console.log("\n🔧 Creating prerequisite nodes...");
    const session = connection.getSession({ defaultAccessMode: "WRITE" });
    try {
      await session.run("MERGE (c:FormContext {id: $id}) SET c.name = $name", {
        id: testContextId,
        name: "Test Context",
      });
      await session.run("MERGE (e:FormEntity {id: $id}) SET e.name = $name", {
        id: testEntityId,
        name: "Test Entity",
      });
      console.log(`   - Created Context ID: ${testContextId}`);
      console.log(`   - Created Entity ID: ${testEntityId}`);
    } finally {
      await session.close();
    }

    // --- Test PropertyShapeRepository first ---
    console.log("\n🧪 Testing PropertyShapeRepository...");

    // 1. Create a Property Shape
    console.log("   - Creating property shape...");
    const newProperty: FormProperty = {
      id: testPropertyId,
      name: "Test Property",
      description: "A property for testing",
      propertyType: "intrinsic",
      contextId: testContextId,
      entityId: testEntityId,
      staticValue: "Static test value",
      qualitative: {
        essential: true,
        observable: true,
        mutable: true,
        inherent: false,
      },
      quantitative: {
        dataType: "string",
        precision: 0,
      },
    };

    const savedProperty = await shapeRepo.saveProperty(newProperty);
    console.log(`   ✅ Created property shape: ${savedProperty.id}`);

    // 2. Get Property by ID
    console.log(`\n   - Getting property shape by ID: ${testPropertyId}`);
    const fetchedProperty = await shapeRepo.getPropertyById(testPropertyId);
    if (fetchedProperty && fetchedProperty.id === testPropertyId) {
      console.log(`   ✅ Fetched property: ${fetchedProperty.name}`);
      console.log("      Static value:", fetchedProperty.staticValue);
    } else {
      throw new Error("❌ Failed to fetch property by ID");
    }

    // 3. Find Properties by Context
    console.log(`\n   - Finding properties by context ID: ${testContextId}`);
    const contextProps = await shapeRepo.findPropertiesByFormContext(
      testContextId
    );
    if (contextProps.some((p) => p.id === testPropertyId)) {
      console.log(
        `   ✅ Found ${contextProps.length} properties in context (including test property)`
      );
    } else {
      throw new Error("❌ Failed to find property by context");
    }

    // 4. Find Properties by Entity
    console.log(`\n   - Finding properties by entity ID: ${testEntityId}`);
    const entityProps = await shapeRepo.findPropertiesByFormEntity(
      testEntityId
    );
    if (entityProps.some((p) => p.id === testPropertyId)) {
      console.log(
        `   ✅ Found ${entityProps.length} properties for entity (including test property)`
      );
    } else {
      throw new Error("❌ Failed to find property by entity");
    }

    // 5. Update Property via saveProperty
    console.log("\n   - Updating property via saveProperty...");
    fetchedProperty!.staticValue = "Updated value";
    fetchedProperty!.description = "Updated description";
    const updatedProperty = await shapeRepo.saveProperty(fetchedProperty!);
    if (
      updatedProperty.staticValue === "Updated value" &&
      updatedProperty.description === "Updated description"
    ) {
      console.log("   ✅ Updated property successfully");
    } else {
      throw new Error("❌ Failed to update property");
    }

    // --- Test PropertyDefinitionRepository ---
    console.log("\n🧪 Testing PropertyDefinitionRepository...");

    // 6. Create Property Definition
    console.log("   - Creating property definition...");
    const now = new Date();
    const newDefinition: FormPropertyDefinition = {
      id: testDefinitionId,
      name: "Test Definition",
      description: "A definition for testing",
      scriptType: "validator",
      contextId: testContextId,
      propertyId: testPropertyId, // Link to the property we created
      code: `
        function compute(context, entity) {
          return "Computed value for " + entity.name;
        }
      `,
      dependencies: [], // No dependencies for this simple test
      created: now,
      updated: now,
    };

    const savedDefinition = await defRepo.savePropertyDefinition(newDefinition);
    console.log(`   ✅ Created property definition: ${savedDefinition.id}`);

    // 7. Update Property to Link to the Definition
    console.log("\n   - Linking property to definition...");
    fetchedProperty!.scriptId = testDefinitionId;
    fetchedProperty!.staticValue = undefined; // Remove static value as it's now computed
    const linkedProperty = await shapeRepo.saveProperty(fetchedProperty!);
    console.log(
      `   ✅ Linked property to definition: ${linkedProperty.scriptId}`
    );

    // --- Verify Relationships ---
    console.log("\n🔍 Verifying Relationships...");
    const verifySession = connection.getSession({ defaultAccessMode: "READ" });
    try {
      // Check property -> definition relationship
      const defResult = await verifySession.run(
        `
        MATCH (p:FormProperty {id: $propId})-[r:DEFINED_BY]->(def:FormPropertyDefinition)
        RETURN count(r) as relCount
      `,
        { propId: testPropertyId }
      );

      const defRelCount = defResult.records[0].get("relCount");
      if (defRelCount > 0) {
        console.log("   ✅ Property correctly linked to definition");
      } else {
        console.warn("   ⚠️ Property not linked to definition");
      }

      // Check property -> entity relationship
      const entityResult = await verifySession.run(
        `
        MATCH (p:FormProperty {id: $propId})-[r:BELONGS_TO_ENTITY]->(e:FormEntity)
        RETURN count(r) as relCount
      `,
        { propId: testPropertyId }
      );

      const entityRelCount = entityResult.records[0].get("relCount");
      if (entityRelCount > 0) {
        console.log("   ✅ Property correctly linked to entity");
      } else {
        console.warn("   ⚠️ Property not linked to entity");
      }
    } finally {
      await verifySession.close();
    }

    // --- Cleanup ---
    console.log("\n🧹 Cleaning up test data...");
    const cleanupSession = connection.getSession({
      defaultAccessMode: "WRITE",
    });
    try {
      // Delete property and its relationships
      await cleanupSession.run(
        `
        MATCH (p:FormProperty {id: $propId})
        DETACH DELETE p
      `,
        { propId: testPropertyId }
      );
      console.log("   ✅ Deleted test property");

      // Delete definition and its relationships
      await cleanupSession.run(
        `
        MATCH (d:FormPropertyDefinition {id: $defId})
        DETACH DELETE d
      `,
        { defId: testDefinitionId }
      );
      console.log("   ✅ Deleted test definition");

      // Delete code node
      await cleanupSession.run(
        `
        MATCH (c:DefinitionCode {definitionId: $defId})
        DETACH DELETE c
      `,
        { defId: testDefinitionId }
      );
      console.log("   ✅ Deleted definition code");

      // Leave context and entity nodes for potential reuse
    } finally {
      await cleanupSession.close();
    }

    console.log("\n🎉 Property Repository Test Script Completed Successfully!");
  } catch (error) {
    console.error("\n❌ ERROR DURING PROPERTY REPOSITORY TEST:", error);
  } finally {
    await connection.close();
    console.log("\nClosing connection...");
    console.log("✅ Connection closed.");
  }
}

// Run the test function
testPropertyRepositories();
