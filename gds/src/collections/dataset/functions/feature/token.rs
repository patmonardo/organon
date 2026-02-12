//! Token/tag feature extractors.

use crate::collections::dataset::expressions::feature::FeatureValue;

pub type TaggedToken = (String, String);

pub fn token_text(tokens: &[TaggedToken], index: usize) -> Option<&str> {
    tokens.get(index).map(|t| t.0.as_str())
}

pub fn token_tag(tokens: &[TaggedToken], index: usize) -> Option<&str> {
    tokens.get(index).map(|t| t.1.as_str())
}

pub fn extract_property(property: &str, token: &TaggedToken) -> Option<FeatureValue> {
    match property {
        "word" | "token" | "text" => Some(FeatureValue::text(token.0.clone())),
        "pos" | "tag" => Some(FeatureValue::text(token.1.clone())),
        "lower" => Some(FeatureValue::text(token.0.to_lowercase())),
        "len" => Some(FeatureValue::Number(token.0.len() as i64)),
        "is_upper" => Some(FeatureValue::Bool(
            token.0.chars().all(|c| !c.is_lowercase()),
        )),
        "is_lower" => Some(FeatureValue::Bool(
            token.0.chars().all(|c| !c.is_uppercase()),
        )),
        _ => None,
    }
}

pub fn extract_at(property: &str, tokens: &[TaggedToken], index: usize) -> Option<FeatureValue> {
    tokens
        .get(index)
        .and_then(|token| extract_property(property, token))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_property_reads_token() {
        let token = ("Mary".to_string(), "NNP".to_string());
        let value = extract_property("word", &token).unwrap();
        assert_eq!(value, FeatureValue::Text("Mary".to_string()));
    }
}
