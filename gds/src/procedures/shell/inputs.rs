use crate::shell::ShellComponentCall;
use crate::shell::ShellComponentMode;
use serde_json::Map;
use serde_json::Value;

use super::ShellProcedureError;

pub(super) struct PropertyMetricInput {
    pub name: String,
    pub metric: Option<String>,
}

pub(super) fn output_property(
    call: &ShellComponentCall,
) -> Result<Option<String>, ShellProcedureError> {
    let names = match call.mode {
        ShellComponentMode::Mutate => &["mutateProperty", "mutate_property"][..],
        ShellComponentMode::Write => &["writeProperty", "write_property"][..],
        _ => return Ok(None),
    };
    let value = find_input(call, names).ok_or(ShellProcedureError::MissingInput(names[0]))?;
    value
        .as_str()
        .filter(|value| !value.is_empty())
        .map(|value| Some(value.to_string()))
        .ok_or(ShellProcedureError::InvalidInput {
            input: names[0],
            expected: "a non-empty string",
        })
}

pub(super) fn required_output_property(value: Option<&str>) -> Result<&str, ShellProcedureError> {
    value.ok_or(ShellProcedureError::MissingInput("outputProperty"))
}

pub(super) fn required_u64(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<u64, ShellProcedureError> {
    optional_u64(call, name, aliases)?.ok_or(ShellProcedureError::MissingInput(name))
}

pub(super) fn required_string(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<String, ShellProcedureError> {
    optional_str(call, name, aliases)?
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or(ShellProcedureError::MissingInput(name))
}

pub(super) fn optional_u64(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<u64>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    value
        .as_u64()
        .map(Some)
        .ok_or(ShellProcedureError::InvalidInput {
            input: name,
            expected: "an unsigned integer",
        })
}

pub(super) fn optional_f64(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<f64>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    value
        .as_f64()
        .map(Some)
        .ok_or(ShellProcedureError::InvalidInput {
            input: name,
            expected: "a number",
        })
}

pub(super) fn optional_usize(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<usize>, ShellProcedureError> {
    optional_u64(call, name, aliases)?
        .map(|value| {
            usize::try_from(value).map_err(|_| ShellProcedureError::InvalidInput {
                input: name,
                expected: "a platform-sized unsigned integer",
            })
        })
        .transpose()
}

pub(super) fn optional_bool(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<bool>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    value
        .as_bool()
        .map(Some)
        .ok_or(ShellProcedureError::InvalidInput {
            input: name,
            expected: "a boolean",
        })
}

pub(super) fn optional_str<'a>(
    call: &'a ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<&'a str>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    value
        .as_str()
        .map(Some)
        .ok_or(ShellProcedureError::InvalidInput {
            input: name,
            expected: "a string",
        })
}

pub(super) fn optional_object<'a>(
    call: &'a ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<&'a Map<String, Value>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    value
        .as_object()
        .map(Some)
        .ok_or(ShellProcedureError::InvalidInput {
            input: name,
            expected: "an object",
        })
}

pub(super) fn optional_u64_array(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<Vec<u64>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    let values = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "an array of unsigned integers",
    })?;
    values
        .iter()
        .map(|value| {
            value.as_u64().ok_or(ShellProcedureError::InvalidInput {
                input: name,
                expected: "an array of unsigned integers",
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

pub(super) fn optional_i64_array(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<Vec<i64>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    let values = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "an array of integers",
    })?;
    values
        .iter()
        .map(|value| {
            value.as_i64().ok_or(ShellProcedureError::InvalidInput {
                input: name,
                expected: "an array of integers",
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

pub(super) fn optional_f64_array(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<Vec<f64>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    let values = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "an array of numbers",
    })?;
    values
        .iter()
        .map(|value| {
            value.as_f64().ok_or(ShellProcedureError::InvalidInput {
                input: name,
                expected: "an array of numbers",
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

pub(super) fn optional_string_array(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<Vec<String>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    let values = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "an array of strings",
    })?;
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or(ShellProcedureError::InvalidInput {
                    input: name,
                    expected: "an array of strings",
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

pub(super) fn optional_string_or_array(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<Vec<String>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    if let Some(value) = value.as_str() {
        return if value.is_empty() {
            Err(ShellProcedureError::InvalidInput {
                input: name,
                expected: "a non-empty string or an array of non-empty strings",
            })
        } else {
            Ok(Some(vec![value.to_string()]))
        };
    }
    let values = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "a non-empty string or an array of non-empty strings",
    })?;
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty())
                .map(str::to_string)
                .ok_or(ShellProcedureError::InvalidInput {
                    input: name,
                    expected: "a non-empty string or an array of non-empty strings",
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

pub(super) fn required_property_metrics(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Vec<PropertyMetricInput>, ShellProcedureError> {
    let value =
        find_named_input(call, name, aliases).ok_or(ShellProcedureError::MissingInput(name))?;
    let values = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "a non-empty array of property names or {name, metric} objects",
    })?;
    if values.is_empty() {
        return Err(ShellProcedureError::InvalidInput {
            input: name,
            expected: "a non-empty array of property names or {name, metric} objects",
        });
    }
    values
        .iter()
        .map(|value| {
            if let Some(property) = value.as_str().filter(|value| !value.is_empty()) {
                return Ok(PropertyMetricInput {
                    name: property.to_string(),
                    metric: None,
                });
            }
            let object = value.as_object().ok_or(ShellProcedureError::InvalidInput {
                input: name,
                expected: "a non-empty array of property names or {name, metric} objects",
            })?;
            let property = object
                .get("name")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or(ShellProcedureError::InvalidInput {
                    input: name,
                    expected: "a non-empty array of property names or {name, metric} objects",
                })?;
            let metric = object
                .get("metric")
                .map(|value| {
                    value
                        .as_str()
                        .map(str::to_string)
                        .ok_or(ShellProcedureError::InvalidInput {
                            input: name,
                            expected:
                                "a non-empty array of property names or {name, metric} objects",
                        })
                })
                .transpose()?;
            Ok(PropertyMetricInput {
                name: property.to_string(),
                metric,
            })
        })
        .collect()
}

pub(super) fn optional_f64_matrix(
    call: &ShellComponentCall,
    name: &'static str,
    aliases: &[&str],
) -> Result<Option<Vec<Vec<f64>>>, ShellProcedureError> {
    let Some(value) = find_named_input(call, name, aliases) else {
        return Ok(None);
    };
    let rows = value.as_array().ok_or(ShellProcedureError::InvalidInput {
        input: name,
        expected: "an array of numeric arrays",
    })?;
    rows.iter()
        .map(|row| {
            let values = row.as_array().ok_or(ShellProcedureError::InvalidInput {
                input: name,
                expected: "an array of numeric arrays",
            })?;
            values
                .iter()
                .map(|value| {
                    value.as_f64().ok_or(ShellProcedureError::InvalidInput {
                        input: name,
                        expected: "an array of numeric arrays",
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn find_named_input<'a>(
    call: &'a ShellComponentCall,
    name: &str,
    aliases: &[&str],
) -> Option<&'a Value> {
    call.inputs.get(name).or_else(|| find_input(call, aliases))
}

fn find_input<'a>(call: &'a ShellComponentCall, names: &[&str]) -> Option<&'a Value> {
    names.iter().find_map(|name| call.inputs.get(*name))
}
