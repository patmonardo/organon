//! PureForm -> Shell algorithm dispatch fixture.
//!
//! Demonstrates Form-first ingress through TS-JSON:
//! 1) materialize a graph through graph_store facade
//! 2) submit a PureForm Program with algo.pagerank
//! 3) inspect proof payload for algorithm/shell activation and apply evidence
//!
//! Run with:
//!   cargo run -p gds --example form_pureform_shell_dispatch

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::applications::services::tsjson;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== PureForm Shell Dispatch ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;

    let graph_name = "form-shell-dispatch-graph";

    let put_request = serde_json::json!({
        "facade": "graph_store",
        "op": "put",
        "user": { "username": "alice", "isAdmin": true },
        "databaseId": "db1",
        "graphName": graph_name,
        "snapshot": {
            "nodes": [0, 1, 2, 3, 4],
            "relationships": [
                {"type": "REL", "source": 0, "target": 1},
                {"type": "REL", "source": 0, "target": 2},
                {"type": "REL", "source": 1, "target": 3},
                {"type": "REL", "source": 2, "target": 3},
                {"type": "REL", "source": 3, "target": 4}
            ]
        }
    });

    let put_response = invoke_json(&put_request)?;
    ensure_ok(&put_response, "graph_store.put")?;

    let form_request = serde_json::json!({
        "facade": "form_eval",
        "op": "evaluate",
        "user": { "username": "alice", "isAdmin": true },
        "databaseId": "db1",
        "graphName": graph_name,
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
        }
    });

    let form_response = invoke_json(&form_request)?;
    ensure_ok(&form_response, "form_eval.evaluate")?;

    let proof = form_response
        .get("data")
        .and_then(|v| v.get("proof"))
        .ok_or("proof payload missing")?;

    let activated_services = proof
        .get("serviceManifest")
        .and_then(|v| v.get("activatedServices"))
        .and_then(Value::as_array)
        .ok_or("serviceManifest.activatedServices missing")?;

    let form_capabilities = proof
        .get("formCapabilities")
        .ok_or("formCapabilities missing")?;
    let capability_source = form_capabilities
        .get("source")
        .and_then(|v| v.get("kind"))
        .and_then(Value::as_str)
        .ok_or("formCapabilities.source.kind missing")?;
    let capability_persistent = form_capabilities
        .get("source")
        .and_then(|v| v.get("persistent"))
        .and_then(Value::as_bool)
        .ok_or("formCapabilities.source.persistent missing")?;
    let application_form_count = form_capabilities
        .get("applicationForms")
        .and_then(Value::as_array)
        .map(Vec::len)
        .ok_or("formCapabilities.applicationForms missing")?;

    let executed_op = proof
        .get("programForm")
        .and_then(|v| v.get("apply"))
        .and_then(|v| v.get("executed"))
        .and_then(Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("op"))
        .and_then(Value::as_str)
        .unwrap_or("none");
    let bus_receipt = proof
        .get("programForm")
        .and_then(|v| v.get("apply"))
        .and_then(|v| v.get("executed"))
        .and_then(Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("response"))
        .and_then(|response| response.get("busReceipt"))
        .ok_or("programForm.apply execution busReceipt missing")?;
    let bus_service = bus_receipt
        .get("service")
        .and_then(Value::as_str)
        .ok_or("busReceipt.service missing")?;
    let bus_runtime = bus_receipt
        .get("runtime")
        .and_then(Value::as_str)
        .ok_or("busReceipt.runtime missing")?;

    if bus_service != "form.shell" || bus_runtime != "ShellProcedureRuntime" {
        return Err("PageRank did not execute through the canonical Form Bus Nexus".into());
    }

    let summary = format!(
        "PureForm Shell Dispatch Summary\n\
         graph={graph_name}\n\
         activated_services={activated_services:?}\n\
         capability_source={capability_source}\n\
         capability_persistent={capability_persistent}\n\
         application_form_count={application_form_count}\n\
         executed_op={executed_op}\n\
         bus_service={bus_service}\n\
         bus_runtime={bus_runtime}\n\
         note=program ingress is Form-first; adapter remains transport-only\n"
    );

    let put_path = fixture_root.join("00-graph-put-response.json");
    let request_path = fixture_root.join("01-form-request.json");
    let response_path = fixture_root.join("02-form-response.json");
    let summary_path = fixture_root.join("03-dispatch-summary.txt");
    let manifest_path = fixture_root.join("README.txt");

    fs::write(&put_path, serde_json::to_string_pretty(&put_response)?)?;
    fs::write(&request_path, serde_json::to_string_pretty(&form_request)?)?;
    fs::write(
        &response_path,
        serde_json::to_string_pretty(&form_response)?,
    )?;
    fs::write(&summary_path, summary)?;
    fs::write(
        &manifest_path,
        manifest(&put_path, &request_path, &response_path, &summary_path),
    )?;

    println!("persisted: {}", fixture_path(&put_path));
    println!("persisted: {}", fixture_path(&request_path));
    println!("persisted: {}", fixture_path(&response_path));
    println!("persisted: {}", fixture_path(&summary_path));
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn invoke_json(request: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let response_text = tsjson::invoke(request.to_string());
    Ok(serde_json::from_str(&response_text)?)
}

fn ensure_ok(response: &Value, op: &str) -> Result<(), Box<dyn std::error::Error>> {
    if response.get("ok").and_then(Value::as_bool) == Some(true) {
        return Ok(());
    }

    let message = response
        .get("error")
        .and_then(|v| v.get("message"))
        .and_then(Value::as_str)
        .unwrap_or("unknown error");
    Err(format!("{op} failed: {message}").into())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/form/form_pureform_shell_dispatch")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/form/form_pureform_shell_dispatch/{file_name}")
}

fn manifest(
    put_path: &Path,
    request_path: &Path,
    response_path: &Path,
    summary_path: &Path,
) -> String {
    format!(
        "PureForm Shell Dispatch Fixture\n\n\
         Namespace: form::shell_dispatch\n\n\
         00 Graph Put Response\n\
         artifact: {}\n\
         meaning: adapter-level graph materialization through graph_store.put.\n\n\
         01 Form Request\n\
         artifact: {}\n\
         meaning: PureForm Program requesting algo.pagerank through form_eval.\n\n\
         02 Form Response\n\
         artifact: {}\n\
         meaning: proof payload including programForm, formCapabilities, and serviceManifest activation.\n\n\
         03 Dispatch Summary\n\
         artifact: {}\n\
         meaning: concise witness of Form capability knowledge and dispatch into shell algorithm runtime.\n",
        fixture_path(put_path),
        fixture_path(request_path),
        fixture_path(response_path),
        fixture_path(summary_path),
    )
}
