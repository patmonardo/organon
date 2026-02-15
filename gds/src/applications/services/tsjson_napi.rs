use crate::applications::services::graph_store_dispatch;
use crate::applications::services::tsjson_support::{err, ok, parse_facade_context};
use crate::core::User;
use crate::form::{ApplicationForm, Context, FormShape, Morph, ProgramSpec, Shape, Specification};
use crate::projection::eval::algorithm::ExecutionMode;
use crate::projection::eval::form::{ProgramFormApi, ProgramFormRequest};
use crate::types::graph_store::GraphStore;
use serde_json::Value;
use std::collections::HashMap;

fn handle_graph_store(request: &serde_json::Value) -> serde_json::Value {
    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };
    graph_store_dispatch::handle_graph_store(request, &ctx)
}

fn handle_graph_store_catalog(request: &serde_json::Value) -> serde_json::Value {
    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    super::applications_dispatch::handle_graph_store_catalog(
        request,
        &ctx.user,
        &ctx.db,
        ctx.catalog,
    )
}

fn as_string_vec(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn as_string_map(value: Option<&Value>) -> HashMap<String, String> {
    value
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| {
                    v.as_str()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|s| (k.clone(), s.to_string()))
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default()
}

fn parse_program_spec(program_value: &Value) -> Result<ProgramSpec, String> {
    let program = program_value
        .as_object()
        .ok_or_else(|| "program must be an object".to_string())?;

    let morph = program
        .get("morph")
        .and_then(Value::as_object)
        .ok_or_else(|| "program.morph is required".to_string())?;

    let patterns = as_string_vec(morph.get("patterns"));
    if patterns.is_empty() {
        return Err("program.morph.patterns must contain at least one entry".to_string());
    }

    let shape_obj = program.get("shape").and_then(Value::as_object);
    let context_obj = program.get("context").and_then(Value::as_object);

    let shape = Shape {
        required_fields: as_string_vec(shape_obj.and_then(|o| o.get("required_fields"))),
        optional_fields: as_string_vec(shape_obj.and_then(|o| o.get("optional_fields"))),
        type_constraints: as_string_map(shape_obj.and_then(|o| o.get("type_constraints"))),
        validation_rules: as_string_map(shape_obj.and_then(|o| o.get("validation_rules"))),
    };

    let context = Context {
        dependencies: as_string_vec(context_obj.and_then(|o| o.get("dependencies"))),
        execution_order: as_string_vec(context_obj.and_then(|o| o.get("execution_order"))),
        runtime_strategy: context_obj
            .and_then(|o| o.get("runtime_strategy"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("kernel")
            .to_string(),
        conditions: as_string_vec(context_obj.and_then(|o| o.get("conditions"))),
    };

    let form = FormShape::new(shape, context, Morph::new(patterns));

    let application_forms = program
        .get("applicationForms")
        .or_else(|| program.get("application_forms"))
        .and_then(Value::as_array)
        .map(|forms| {
            forms
                .iter()
                .enumerate()
                .map(|(index, form)| {
                    let obj = form.as_object();
                    let name = obj
                        .and_then(|o| o.get("name"))
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(ToOwned::to_owned)
                        .unwrap_or_else(|| format!("form.{index}"));
                    let domain = obj
                        .and_then(|o| o.get("domain"))
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .unwrap_or("graph-ml")
                        .to_string();
                    let features = as_string_vec(obj.and_then(|o| o.get("features")));
                    let form_patterns = as_string_vec(obj.and_then(|o| o.get("patterns")));
                    let specifications = as_string_map(obj.and_then(|o| o.get("specifications")));
                    ApplicationForm::new(name, domain, features, form_patterns, specifications)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let selected_forms = as_string_vec(
        program
            .get("selectedForms")
            .or_else(|| program.get("selected_forms")),
    );

    let gdsl = Specification::new("form.program".to_string(), None, HashMap::new());

    Ok(ProgramSpec::new(
        form,
        gdsl,
        Vec::new(),
        application_forms,
        selected_forms,
    ))
}

fn parse_op_inputs(request: &Value) -> HashMap<String, Value> {
    request
        .get("opInputs")
        .or_else(|| request.get("op_inputs"))
        .and_then(Value::as_object)
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default()
}

fn parse_execution_mode(request: &Value) -> ExecutionMode {
    match request
        .get("mode")
        .and_then(Value::as_str)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("stats") => ExecutionMode::Stats,
        Some("train") => ExecutionMode::Train,
        Some("write") => ExecutionMode::WriteNodeProperty,
        Some("mutate") => ExecutionMode::MutateNodeProperty,
        _ => ExecutionMode::Stream,
    }
}

fn parse_form_eval_hook_artifacts(request: &Value) -> Result<Value, String> {
    let Some(artifacts) = request.get("artifacts") else {
        return Ok(serde_json::json!({
            "enabled": false,
            "validated": false,
        }));
    };

    let artifact_obj = artifacts
        .as_object()
        .ok_or_else(|| "artifacts must be an object when provided".to_string())?;

    let mut validated = false;
    let mut hook_payload = serde_json::Map::new();

    if let Some(store_contract) = artifact_obj.get("storeContract") {
        let store_obj = store_contract
            .as_object()
            .ok_or_else(|| "artifacts.storeContract must be an object".to_string())?;
        if !(store_obj.contains_key("formCatalog")
            && store_obj.contains_key("factStore")
            && store_obj.contains_key("knowledgeStore"))
        {
            return Err(
                "artifacts.storeContract must contain formCatalog, factStore, and knowledgeStore"
                    .to_string(),
            );
        }

        if let Some(fact_store) = store_obj.get("factStore") {
            let fact_store_obj = fact_store
                .as_object()
                .ok_or_else(|| "artifacts.storeContract.factStore must be an object".to_string())?;

            if let Some(data_access) = fact_store_obj.get("dataAccess") {
                let data_access_obj = data_access.as_object().ok_or_else(|| {
                    "artifacts.storeContract.factStore.dataAccess must be an object".to_string()
                })?;

                let polars = data_access_obj
                    .get("polars")
                    .and_then(Value::as_object)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.polars must be an object"
                            .to_string()
                    })?;
                let kernel = data_access_obj
                    .get("kernel")
                    .and_then(Value::as_object)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.kernel must be an object"
                            .to_string()
                    })?;

                let polars_postgres = polars
                    .get("postgres")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.polars.postgres must be a string"
                            .to_string()
                    })?;
                let polars_filesystem = polars
                    .get("filesystem")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.polars.filesystem must be a string"
                            .to_string()
                    })?;
                let kernel_neo4j =
                    kernel.get("neo4j").and_then(Value::as_str).ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.kernel.neo4j must be a string"
                            .to_string()
                    })?;
                let kernel_postgres = kernel
                    .get("postgres")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.kernel.postgres must be a string"
                            .to_string()
                    })?;
                let kernel_filesystem = kernel
                    .get("filesystem")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.dataAccess.kernel.filesystem must be a string"
                            .to_string()
                    })?;

                hook_payload.insert(
                    "factStoreDataAccess".to_string(),
                    serde_json::json!({
                        "polars": {
                            "postgres": polars_postgres,
                            "filesystem": polars_filesystem,
                        },
                        "kernel": {
                            "neo4j": kernel_neo4j,
                            "postgres": kernel_postgres,
                            "filesystem": kernel_filesystem,
                        }
                    }),
                );
            }

            if let Some(agent_monitoring) = fact_store_obj.get("agentMonitoring") {
                let monitoring_obj = agent_monitoring.as_object().ok_or_else(|| {
                    "artifacts.storeContract.factStore.agentMonitoring must be an object"
                        .to_string()
                })?;

                let enabled = monitoring_obj
                    .get("enabled")
                    .and_then(Value::as_bool)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.agentMonitoring.enabled must be a boolean"
                            .to_string()
                    })?;
                let scope = monitoring_obj
                    .get("scope")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.agentMonitoring.scope must be a string"
                            .to_string()
                    })?;
                let channels = monitoring_obj
                    .get("channels")
                    .and_then(Value::as_array)
                    .ok_or_else(|| {
                        "artifacts.storeContract.factStore.agentMonitoring.channels must be an array"
                            .to_string()
                    })?;

                hook_payload.insert(
                    "factStoreAgentMonitoring".to_string(),
                    serde_json::json!({
                        "enabled": enabled,
                        "scope": scope,
                        "channelCount": channels.len(),
                    }),
                );
            }
        }

        hook_payload.insert("storeContract".to_string(), store_contract.clone());
        validated = true;
    }

    if let Some(doctrine) = artifact_obj.get("doctrine") {
        let doctrine_obj = doctrine
            .as_object()
            .ok_or_else(|| "artifacts.doctrine must be an object".to_string())?;

        let authority = doctrine_obj
            .get("authority")
            .and_then(Value::as_str)
            .ok_or_else(|| "artifacts.doctrine.authority must be a string".to_string())?;
        let substrate = doctrine_obj
            .get("substrate")
            .and_then(Value::as_str)
            .ok_or_else(|| "artifacts.doctrine.substrate must be a string".to_string())?;
        let kernel_role = doctrine_obj
            .get("kernelRole")
            .and_then(Value::as_str)
            .ok_or_else(|| "artifacts.doctrine.kernelRole must be a string".to_string())?;

        hook_payload.insert(
            "doctrine".to_string(),
            serde_json::json!({
                "authority": authority,
                "substrate": substrate,
                "kernelRole": kernel_role,
            }),
        );
        validated = true;
    }

    if let Some(boundary_contract) = artifact_obj.get("boundaryContract") {
        let boundary_obj = boundary_contract
            .as_object()
            .ok_or_else(|| "artifacts.boundaryContract must be an object".to_string())?;

        let epistemic_processor = boundary_obj
            .get("epistemicProcessor")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                "artifacts.boundaryContract.epistemicProcessor must be an object".to_string()
            })?;
        let transcendental_kernel = boundary_obj
            .get("transcendentalKernel")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                "artifacts.boundaryContract.transcendentalKernel must be an object".to_string()
            })?;
        let handoff = boundary_obj
            .get("handoff")
            .and_then(Value::as_object)
            .ok_or_else(|| "artifacts.boundaryContract.handoff must be an object".to_string())?;

        let epistemic_mode = epistemic_processor
            .get("mode")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                "artifacts.boundaryContract.epistemicProcessor.mode must be a string".to_string()
            })?;
        let transcendental_mode = transcendental_kernel
            .get("mode")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                "artifacts.boundaryContract.transcendentalKernel.mode must be a string".to_string()
            })?;

        let invariants = handoff
            .get("invariants")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "artifacts.boundaryContract.handoff.invariants must be an array".to_string()
            })?;
        let proof_obligations = handoff
            .get("proofObligations")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "artifacts.boundaryContract.handoff.proofObligations must be an array".to_string()
            })?;

        if invariants.is_empty() {
            return Err(
                "artifacts.boundaryContract.handoff.invariants must contain at least one entry"
                    .to_string(),
            );
        }
        if proof_obligations.is_empty() {
            return Err("artifacts.boundaryContract.handoff.proofObligations must contain at least one entry".to_string());
        }

        hook_payload.insert(
            "boundaryContract".to_string(),
            serde_json::json!({
                "epistemicMode": epistemic_mode,
                "transcendentalMode": transcendental_mode,
                "invariantCount": invariants.len(),
                "proofObligationCount": proof_obligations.len(),
            }),
        );
        validated = true;
    }

    if let Some(ontology_image) = artifact_obj.get("ontologyImage") {
        let image_obj = ontology_image
            .as_object()
            .ok_or_else(|| "artifacts.ontologyImage must be an object".to_string())?;

        let tables = image_obj
            .get("tables")
            .and_then(Value::as_object)
            .ok_or_else(|| "artifacts.ontologyImage.tables must be an object".to_string())?;

        let models = tables
            .get("models")
            .and_then(Value::as_array)
            .ok_or_else(|| "artifacts.ontologyImage.tables.models must be an array".to_string())?;
        let features = tables
            .get("features")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "artifacts.ontologyImage.tables.features must be an array".to_string()
            })?;
        let queries = tables
            .get("queries")
            .and_then(Value::as_array)
            .ok_or_else(|| "artifacts.ontologyImage.tables.queries must be an array".to_string())?;
        let constraints = tables
            .get("constraints")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                "artifacts.ontologyImage.tables.constraints must be an array".to_string()
            })?;

        hook_payload.insert(
            "ontologyImage".to_string(),
            serde_json::json!({
                "imageId": image_obj.get("imageId").cloned().unwrap_or(Value::Null),
                "engine": image_obj.get("engine").cloned().unwrap_or(Value::Null),
                "tableCounts": {
                    "models": models.len(),
                    "features": features.len(),
                    "queries": queries.len(),
                    "constraints": constraints.len(),
                }
            }),
        );
        validated = true;
    }

    Ok(serde_json::json!({
        "enabled": validated,
        "validated": validated,
        "hooks": hook_payload,
    }))
}

fn handle_form_eval(request: &Value) -> Value {
    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if ctx.op != "evaluate" {
        return err(
            &ctx.op,
            "UNSUPPORTED_OP",
            "Unsupported form_eval operation.",
        );
    }

    let graph_name = match request
        .get("graphName")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(name) => name.to_string(),
        None => return err(&ctx.op, "INVALID_REQUEST", "graphName missing or empty"),
    };

    let output_graph_name = request
        .get("outputGraphName")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned);

    let program = match request.get("program") {
        Some(program_value) => match parse_program_spec(program_value) {
            Ok(program) => program,
            Err(message) => return err(&ctx.op, "INVALID_REQUEST", message),
        },
        None => {
            return err(
                &ctx.op,
                "INVALID_REQUEST",
                "Missing required field: program",
            )
        }
    };

    let mut form_request = ProgramFormRequest::for_agent_framework(program, ctx.user.username());
    form_request.execution_mode = parse_execution_mode(request);
    form_request.fail_fast = request
        .get("failFast")
        .or_else(|| request.get("fail_fast"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    form_request.default_input = serde_json::json!({
        "graphName": graph_name,
        "databaseId": request.get("databaseId").cloned().unwrap_or(Value::Null),
        "user": request.get("user").cloned().unwrap_or(Value::Null),
        "projectionId": request.get("projectionId").cloned().unwrap_or(Value::Null),
    });
    form_request.op_inputs = parse_op_inputs(request);

    let hook_artifacts = match parse_form_eval_hook_artifacts(request) {
        Ok(hooks) => hooks,
        Err(message) => return err(&ctx.op, "INVALID_REQUEST", message),
    };

    let api = ProgramFormApi::new();
    let print = match api.evaluate_apply_print(form_request, ctx.catalog.clone()) {
        Ok(result) => result,
        Err(error) => {
            return err(
                &ctx.op,
                "FORM_PROGRAM_ERROR",
                format!("Form Program evaluate failed: {error}"),
            )
        }
    };

    let (node_count, relationship_count) = ctx
        .catalog
        .get(&graph_name)
        .map(|store| {
            (
                GraphStore::node_count(store.as_ref()) as u64,
                GraphStore::relationship_count(store.as_ref()) as u64,
            )
        })
        .unwrap_or((0, 0));

    let proof = serde_json::json!({
        "programForm": serde_json::to_value(&print).unwrap_or_else(|_| serde_json::json!({})),
        "artifactHooks": hook_artifacts,
    });

    ok(
        &ctx.op,
        serde_json::json!({
            "graphName": graph_name,
            "outputGraphName": output_graph_name,
            "persistedOutputGraph": false,
            "operator": "program_form.evaluate_apply_print",
            "nodeCount": node_count,
            "relationshipCount": relationship_count,
            "proof": proof,
        }),
    )
}

fn handle_algorithms(request: &serde_json::Value) -> serde_json::Value {
    use super::applications_dispatch;

    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    applications_dispatch::handle_algorithms(request, ctx.catalog)
}

/// TS-JSON boundary for GDS.
///
/// This module is intentionally small and FFI-friendly:
/// - accepts/returns JSON strings
/// - uses stable operation names (`op`)
/// - returns handles for large results instead of materializing data
///
/// The internal Rust "applications" layer is free to mirror Java GDS closely.
pub fn invoke(request_json: String) -> String {
    let request: serde_json::Value = match serde_json::from_str(&request_json) {
        Ok(v) => v,
        Err(e) => {
            return err("", "INVALID_JSON", &format!("Invalid JSON request: {e}")).to_string();
        }
    };

    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    // Prefer facade-based routing when present.
    if let Some(facade) = request.get("facade").and_then(|v| v.as_str()) {
        let response = match facade {
            "graph_store" => handle_graph_store(&request),
            "graph_store_catalog" => handle_graph_store_catalog(&request),
            "algorithms" => handle_algorithms(&request),
            "form_eval" => handle_form_eval(&request),
            _ => err(op, "UNSUPPORTED_FACADE", "Unsupported facade."),
        };
        return response.to_string();
    }

    let response = match op {
        "ping" => {
            let nonce = request
                .get("nonce")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            ok("ping", serde_json::json!({ "nonce": nonce }))
        }
        "version" => ok(
            "version",
            serde_json::json!({
                "crate": "gds",
                "version": env!("CARGO_PKG_VERSION")
            }),
        ),
        _ => err(op, "UNSUPPORTED_OP", "Unsupported operation."),
    };

    response.to_string()
}

/// Convenience: returns the Rust crate version.
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
    use crate::applications::services::tsjson_support::{TsjsonUser, TSJSON_CATALOG_SERVICE};
    use crate::types::catalog::GraphCatalog;
    use crate::types::graph_store::DatabaseId;
    use std::sync::Arc;

    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, Randomizable};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn test_catalog(username: &str, is_admin: bool, database_id: &str) -> Arc<dyn GraphCatalog> {
        let user = TsjsonUser::new(username.to_string(), is_admin);
        let db = DatabaseId::new(database_id);
        TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db)
    }

    #[test]
    fn invoke_graph_store_catalog_list_graphs_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph1".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(0);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph1", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "listGraphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("listGraphs")
        );

        let entries = response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();

        assert!(entries
            .iter()
            .any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph1")));

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph1"], false);
    }

    #[test]
    fn invoke_graph_store_catalog_graph_memory_usage_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph2".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(1);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let expected_nodes = GraphStore::node_count(&store) as u64;
        let expected_rels = GraphStore::relationship_count(&store) as u64;
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph2", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "graphMemoryUsage",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph2"
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("graphMemoryUsage")
        );

        let data = response.get("data").unwrap();
        assert_eq!(
            data.get("graphName").and_then(|v| v.as_str()),
            Some("graph2")
        );
        assert_eq!(
            data.get("nodeCount").and_then(|v| v.as_u64()),
            Some(expected_nodes)
        );
        assert_eq!(
            data.get("relationshipCount").and_then(|v| v.as_u64()),
            Some(expected_rels)
        );

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph2"], false);
    }

    #[test]
    fn invoke_graph_store_catalog_drop_graph_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph_drop_1".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(30);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph_drop_1", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "dropGraph",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_drop_1",
            "failIfMissing": true
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("dropGraph")
        );

        // Verify it is gone.
        let list_request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "listGraphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });
        let list_json = invoke(list_request.to_string());
        let list_response: serde_json::Value = serde_json::from_str(&list_json).unwrap();
        let entries = list_response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(!entries
            .iter()
            .any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph_drop_1")));
    }

    #[test]
    fn invoke_graph_store_catalog_drop_graphs_round_trip() {
        let mut rng = StdRng::seed_from_u64(31);
        let catalog = test_catalog("alice", true, "db1");
        for name in ["graph_drop_a", "graph_drop_b"] {
            let config = RandomGraphConfig {
                graph_name: name.to_string(),
                database_name: "db1".to_string(),
                ..Default::default()
            };
            let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
            catalog.set(name, Arc::new(store));
        }

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "dropGraphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphNames": ["graph_drop_a", "graph_drop_b"],
            "failIfMissing": true
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("dropGraphs")
        );

        let dropped = response
            .get("data")
            .and_then(|v| v.get("dropped"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(dropped.len(), 2);
    }

    #[test]
    fn invoke_form_eval_with_store_contract_and_ontology_image_hooks() {
        let request = serde_json::json!({
            "facade": "form_eval",
            "op": "evaluate",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "hook-graph",
            "program": {
                "morph": {
                    "patterns": ["algo.pagerank"]
                },
                "applicationForms": [
                    {
                        "name": "centrality",
                        "domain": "ontology-runtime",
                        "features": ["feature.centrality.pagerank"],
                        "patterns": ["algo.pagerank"],
                        "specifications": { "binding": "spec.pagerank" }
                    }
                ],
                "selectedForms": ["centrality"]
            },
            "artifacts": {
                "boundaryContract": {
                    "epistemicProcessor": {
                        "runtime": "ts-agent-logic",
                        "processor": "reflective-form",
                        "mode": "epistemic",
                        "authority": "sdsl/zod"
                    },
                    "transcendentalKernel": {
                        "runtime": "gds-rust-kernel",
                        "processor": "program-form-evaluate-apply-print",
                        "mode": "transcendental-logic",
                        "role": "cache"
                    },
                    "handoff": {
                        "substrate": "cypher-driven",
                        "invariants": ["program-features-precede-kernel-compilation"],
                        "proofObligations": ["artifact-hooks-validated"]
                    }
                },
                "storeContract": {
                    "formCatalog": {"listRef": "neo4j://formdb/specifications/spec-1"},
                    "factStore": {
                        "graphRef": "neo4j://factstore/spec-1",
                        "dataAccess": {
                            "polars": {
                                "postgres": "partial-direct",
                                "filesystem": "partial-direct"
                            },
                            "kernel": {
                                "neo4j": "direct",
                                "postgres": "direct",
                                "filesystem": "direct"
                            }
                        },
                        "agentMonitoring": {
                            "enabled": true,
                            "scope": "factstore-structure-observability",
                            "channels": ["agent-runtime", "dataset-health", "cache-health"]
                        }
                    },
                    "knowledgeStore": {"targetRef": "spec-1-knowledge-store"}
                },
                "ontologyImage": {
                    "imageId": "ontology-image:spec-1",
                    "engine": "polars",
                    "tables": {
                        "models": [{"modelId": "m1"}],
                        "features": [{"featureId": "f1"}],
                        "queries": [{"queryId": "q1"}],
                        "constraints": [{"constraintId": "c1"}],
                        "provenance": []
                    }
                }
            }
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));

        let hooks = response
            .get("data")
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("artifactHooks"))
            .expect("artifactHooks expected in proof payload");

        assert_eq!(hooks.get("enabled").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(hooks.get("validated").and_then(|v| v.as_bool()), Some(true));

        let boundary = hooks
            .get("hooks")
            .and_then(|v| v.get("boundaryContract"))
            .expect("boundary contract hook expected");
        assert_eq!(
            boundary.get("epistemicMode").and_then(|v| v.as_str()),
            Some("epistemic")
        );
        assert_eq!(
            boundary.get("transcendentalMode").and_then(|v| v.as_str()),
            Some("transcendental-logic")
        );
        assert_eq!(
            boundary.get("invariantCount").and_then(|v| v.as_u64()),
            Some(1)
        );

        let data_access = hooks
            .get("hooks")
            .and_then(|v| v.get("factStoreDataAccess"))
            .expect("fact store data access hook expected");
        assert_eq!(
            data_access
                .get("polars")
                .and_then(|v| v.get("postgres"))
                .and_then(|v| v.as_str()),
            Some("partial-direct")
        );
        assert_eq!(
            data_access
                .get("kernel")
                .and_then(|v| v.get("neo4j"))
                .and_then(|v| v.as_str()),
            Some("direct")
        );

        let monitoring = hooks
            .get("hooks")
            .and_then(|v| v.get("factStoreAgentMonitoring"))
            .expect("fact store agent monitoring hook expected");
        assert_eq!(
            monitoring.get("enabled").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            monitoring.get("channelCount").and_then(|v| v.as_u64()),
            Some(3)
        );

        let table_counts = hooks
            .get("hooks")
            .and_then(|v| v.get("ontologyImage"))
            .and_then(|v| v.get("tableCounts"))
            .expect("ontology image table counts expected");

        assert_eq!(table_counts.get("models").and_then(|v| v.as_u64()), Some(1));
        assert_eq!(
            table_counts.get("features").and_then(|v| v.as_u64()),
            Some(1)
        );
    }
}
