use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::sync::Arc;

use crate::projection::NodeLabel;
use crate::shell::builtin_component;
use crate::shell::ShellComponentCall;
use crate::shell::ShellComponentExecutionKind;
use crate::shell::ShellComponentId;
use crate::shell::ShellComponentMode;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;

use super::GraphFacade;
use super::ShellProcedureBinding;
use super::ShellProcedureError;
use super::ShellProcedureResult;

pub(super) fn bind_store_api(
    graph: &GraphFacade,
    graph_catalog: Arc<dyn GraphCatalog>,
    call: &ShellComponentCall,
) -> Result<ShellProcedureBinding, ShellProcedureError> {
    let descriptor = builtin_component(call.component.as_str())
        .ok_or(ShellProcedureError::UnknownComponent(call.component))?
        .descriptor();

    if descriptor.execution_kind != ShellComponentExecutionKind::StoreApi {
        return Err(ShellProcedureError::UnboundComponent(descriptor.id));
    }

    if !descriptor.supports(call.mode) {
        return Err(ShellProcedureError::UnsupportedMode {
            component: descriptor.id,
            mode: call.mode,
        });
    }

    let store = graph.store().clone();
    let mut input = Map::new();
    for (key, value) in &call.inputs {
        input.insert(key.clone(), value.clone());
    }

    Ok(ShellProcedureBinding::StoreApi {
        component: descriptor.id,
        mode: call.mode,
        graph_catalog,
        store,
        input: Value::Object(input),
    })
}

pub(super) fn invoke_store_api(
    component: ShellComponentId,
    mode: ShellComponentMode,
    graph_catalog: &dyn GraphCatalog,
    store: &DefaultGraphStore,
    input: Value,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    let component_id = component.as_str();
    let node_count = store.node_count();
    let relationship_count = store.relationship_count();
    let graph_name = string_input(&input, "graphName").unwrap_or("graphframe.active");

    let result = match component_id {
        "gds.store.graph.put" => {
            graph_catalog.set(graph_name, Arc::new(store.clone()));
            json!({
                "component": component_id,
                "mode": format!("{:?}", mode),
                "applied": true,
                "graphName": graph_name,
                "nodeCount": node_count,
                "relationshipCount": relationship_count,
                "input": input,
            })
        }
        "gds.store.catalog.exists" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "graphName": graph_name,
            "exists": graph_catalog.get(graph_name).is_some(),
            "input": input,
        }),
        "gds.store.catalog.list" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "graphs": graph_catalog
                .list(string_input(&input, "graphName"), false)
                .into_iter()
                .map(|entry| {
                    json!({
                        "name": entry.name,
                        "nodeCount": entry.node_count,
                        "relationshipCount": entry.relationship_count,
                    })
                })
                .collect::<Vec<_>>(),
            "input": input,
        }),
        "gds.store.catalog.memory_usage" => match graph_catalog.size_of(graph_name) {
            Ok(mu) => json!({
                "component": component_id,
                "mode": format!("{:?}", mode),
                "graphName": graph_name,
                "memoryUsage": mu.bytes,
                "nodeCount": mu.nodes,
                "relationshipCount": mu.relationships,
                "input": input,
            }),
            Err(_) => json!({
                "component": component_id,
                "mode": format!("{:?}", mode),
                "graphName": graph_name,
                "memoryUsage": 0_u64,
                "nodeCount": 0_u64,
                "relationshipCount": 0_u64,
                "status": "missing",
                "input": input,
            }),
        },
        "gds.store.catalog.drop" => {
            let fail_if_missing = bool_input(&input, "failIfMissing").unwrap_or(false);
            match graph_catalog.drop(&[graph_name], fail_if_missing) {
                Ok(dropped) => json!({
                    "component": component_id,
                    "mode": format!("{:?}", mode),
                    "applied": true,
                    "dropped": dropped
                        .into_iter()
                        .map(|entry| {
                            json!({
                                "name": entry.name,
                                "nodeCount": entry.node_count,
                                "relationshipCount": entry.relationship_count,
                            })
                        })
                        .collect::<Vec<_>>(),
                    "input": input,
                }),
                Err(error) => json!({
                    "component": component_id,
                    "mode": format!("{:?}", mode),
                    "applied": false,
                    "status": "error",
                    "error": error.to_string(),
                    "input": input,
                }),
            }
        }
        "gds.store.catalog.drop_many" => {
            let names = string_array_input(&input, "graphNames").unwrap_or_default();
            let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
            if refs.is_empty() {
                planned_mutation_response(component_id, mode, node_count, relationship_count, input)
            } else {
                let fail_if_missing = bool_input(&input, "failIfMissing").unwrap_or(false);
                match graph_catalog.drop(&refs, fail_if_missing) {
                    Ok(dropped) => json!({
                        "component": component_id,
                        "mode": format!("{:?}", mode),
                        "applied": true,
                        "dropped": dropped
                            .into_iter()
                            .map(|entry| {
                                json!({
                                    "name": entry.name,
                                    "nodeCount": entry.node_count,
                                    "relationshipCount": entry.relationship_count,
                                })
                            })
                            .collect::<Vec<_>>(),
                        "input": input,
                    }),
                    Err(error) => json!({
                        "component": component_id,
                        "mode": format!("{:?}", mode),
                        "applied": false,
                        "status": "error",
                        "error": error.to_string(),
                        "input": input,
                    }),
                }
            }
        }
        "gds.store.node_properties.drop" => {
            let keys = string_array_input(&input, "nodeProperties").unwrap_or_default();
            if keys.is_empty() {
                planned_mutation_response(component_id, mode, node_count, relationship_count, input)
            } else {
                let mut mutator = |store: &mut DefaultGraphStore| {
                    for key in &keys {
                        let _ = store.remove_node_property(key);
                    }
                };
                let outcome = graph_catalog.with_store_mut(graph_name, &mut mutator);
                json!({
                    "component": component_id,
                    "mode": format!("{:?}", mode),
                    "applied": outcome.is_ok(),
                    "status": if outcome.is_ok() {"applied"} else {"error"},
                    "error": outcome.err().map(|error| error.to_string()),
                    "nodeProperties": keys,
                    "input": input,
                })
            }
        }
        "gds.store.graph_property.drop" => {
            let graph_property = string_input(&input, "graphProperty")
                .or_else(|| string_input(&input, "propertyKey"));
            if let Some(property_key) = graph_property {
                let mut mutator = |store: &mut DefaultGraphStore| {
                    let _ = store.remove_graph_property(property_key);
                };
                let outcome = graph_catalog.with_store_mut(graph_name, &mut mutator);
                json!({
                    "component": component_id,
                    "mode": format!("{:?}", mode),
                    "applied": outcome.is_ok(),
                    "status": if outcome.is_ok() {"applied"} else {"error"},
                    "error": outcome.err().map(|error| error.to_string()),
                    "graphProperty": property_key,
                    "input": input,
                })
            } else {
                planned_mutation_response(component_id, mode, node_count, relationship_count, input)
            }
        }
        "gds.store.node_label.mutate" => {
            let label = string_input(&input, "label").or_else(|| string_input(&input, "nodeLabel"));
            if let Some(label) = label {
                let mut mutator = |store: &mut DefaultGraphStore| {
                    let _ = store.add_node_label(NodeLabel::of(label));
                };
                let outcome = graph_catalog.with_store_mut(graph_name, &mut mutator);
                json!({
                    "component": component_id,
                    "mode": format!("{:?}", mode),
                    "applied": outcome.is_ok(),
                    "status": if outcome.is_ok() {"applied"} else {"error"},
                    "error": outcome.err().map(|error| error.to_string()),
                    "label": label,
                    "input": input,
                })
            } else {
                planned_mutation_response(component_id, mode, node_count, relationship_count, input)
            }
        }
        "gds.store.relationships.drop"
        | "gds.store.node_properties.write"
        | "gds.store.relationship_properties.write"
        | "gds.store.relationships.write"
        | "gds.store.node_label.write"
        | "gds.store.graph.generate"
        | "gds.store.graph.sample"
        | "gds.store.subgraph.project" => {
            planned_mutation_response(component_id, mode, node_count, relationship_count, input)
        }
        "gds.store.graph_property.stream" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "graphPropertyKeys": store.graph_property_keys().into_iter().collect::<Vec<_>>(),
            "input": input,
        }),
        "gds.store.node_properties.stream" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "nodePropertyKeys": store.node_property_keys().into_iter().collect::<Vec<_>>(),
            "input": input,
        }),
        "gds.store.relationship_properties.stream" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "relationshipPropertyKeys": store.relationship_property_keys().into_iter().collect::<Vec<_>>(),
            "input": input,
        }),
        "gds.store.relationships.stream" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "relationshipTypes": store
                .relationship_types()
                .into_iter()
                .map(|t| t.name().to_string())
                .collect::<Vec<_>>(),
            "relationshipCount": relationship_count,
            "input": input,
        }),
        "gds.store.native_project.estimate"
        | "gds.store.common_neighbour_aware_random_walk.estimate" => json!({
            "component": component_id,
            "mode": format!("{:?}", mode),
            "nodeCount": node_count,
            "relationshipCount": relationship_count,
            "estimatedMemory": {
                "min": ((node_count + relationship_count) as u64) * 8,
                "max": ((node_count + relationship_count) as u64) * 24,
            },
            "input": input,
        }),
        _ => return Err(ShellProcedureError::UnboundComponent(component)),
    };

    Ok(ShellProcedureResult::StoreApi(result))
}

fn planned_mutation_response(
    component: &str,
    mode: ShellComponentMode,
    node_count: usize,
    relationship_count: usize,
    input: Value,
) -> Value {
    json!({
        "component": component,
        "mode": format!("{:?}", mode),
        "applied": false,
        "status": "planned",
        "reason": "store-api mutation executor is not yet connected to catalog successor writes",
        "nodeCount": node_count,
        "relationshipCount": relationship_count,
        "input": input,
    })
}

fn string_input<'a>(input: &'a Value, key: &str) -> Option<&'a str> {
    input
        .get(key)
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn bool_input(input: &Value, key: &str) -> Option<bool> {
    input.get(key).and_then(|value| value.as_bool())
}

fn string_array_input(input: &Value, key: &str) -> Option<Vec<String>> {
    let values = input
        .get(key)
        .and_then(|value| value.as_array())?
        .iter()
        .filter_map(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    Some(values)
}
