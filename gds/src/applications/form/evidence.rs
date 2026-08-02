use crate::applications::form::capability_source::FormCapabilitySnapshot;
use crate::applications::form::service_manifest::FormServiceManifest;
use serde::Serialize;
use serde_json::Value;

pub fn compose<T: Serialize>(
    program_form: &T,
    program_features: Value,
    form_capabilities: &FormCapabilitySnapshot,
    service_manifest: &FormServiceManifest,
    artifact_hooks: Value,
) -> Result<Value, String> {
    let program_form = serde_json::to_value(program_form)
        .map_err(|error| format!("ProgramForm evidence serialization failed: {error}"))?;
    let service_manifest = serde_json::to_value(service_manifest)
        .map_err(|error| format!("serviceManifest serialization failed: {error}"))?;
    let form_capabilities = serde_json::to_value(form_capabilities)
        .map_err(|error| format!("formCapabilities serialization failed: {error}"))?;

    Ok(serde_json::json!({
        "programForm": program_form,
        "programFeatures": program_features,
        "formCapabilities": form_capabilities,
        "serviceManifest": service_manifest,
        "artifactHooks": artifact_hooks,
    }))
}
