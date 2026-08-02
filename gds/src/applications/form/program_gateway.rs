use crate::form::ApplicationForm;
use crate::form::Context;
use crate::form::FormShape;
use crate::form::Morph;
use crate::form::ProgramSpec;
use crate::form::Shape;
use crate::form::Specification;
use serde_json::Value;
use std::collections::HashMap;

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

pub fn parse_program_spec(program_value: &Value) -> Result<ProgramSpec, String> {
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
        .get("pureFormComponents")
        .or_else(|| program.get("pure_form_components"))
        .or_else(|| program.get("components"))
        .or_else(|| program.get("applicationForms"))
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
            .get("selectedPureFormComponents")
            .or_else(|| program.get("selected_pure_form_components"))
            .or_else(|| program.get("selectedComponents"))
            .or_else(|| program.get("selected_components"))
            .or_else(|| program.get("selectedForms"))
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

pub fn parse_program_value<'a>(request: &'a Value) -> Result<&'a Value, String> {
    if let Some(program) = request.get("program") {
        return Ok(program);
    }

    if let Some(given_form) = request
        .get("givenForm")
        .or_else(|| request.get("given_form"))
    {
        return Ok(given_form);
    }

    if let Some(given_forms) = request
        .get("givenForms")
        .or_else(|| request.get("given_forms"))
    {
        if let Some(forms) = given_forms.as_array() {
            return forms
                .first()
                .ok_or_else(|| "givenForms must contain at least one entry".to_string());
        }

        return Ok(given_forms);
    }

    Err("Missing required field: program|givenForm|givenForms".to_string())
}

pub fn parse_service_id(request: &Value) -> Result<Option<&str>, String> {
    request
        .get("serviceId")
        .or_else(|| request.get("service_id"))
        .map(|value| {
            value
                .as_str()
                .map(str::trim)
                .filter(|service_id| !service_id.is_empty())
                .ok_or_else(|| "serviceId must be a non-empty string when provided".to_string())
        })
        .transpose()
}

pub fn program_features_json(program: &ProgramSpec) -> Result<Value, String> {
    let features = program
        .define_features()
        .map_err(|error| format!("Program feature extraction failed: {error}"))?;

    let feature_rows = features
        .features
        .iter()
        .map(|feature| {
            serde_json::json!({
                "kind": feature.kind.as_str(),
                "value": feature.value,
                "source": feature.source,
            })
        })
        .collect::<Vec<_>>();

    Ok(serde_json::json!({
        "programName": features.program_name,
        "selectedForms": features.selected_forms,
        "selectedComponents": features.selected_forms,
        "features": feature_rows,
    }))
}
