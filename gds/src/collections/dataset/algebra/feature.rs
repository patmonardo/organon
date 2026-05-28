//! Feature algebra canonical surface.
//!
//! Phase A: re-export from legacy `dataset::expressions::feature` so callers
//! can migrate without behavior changes.

pub use crate::collections::dataset::expressions::feature::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::expressions::feature as legacy;

    #[test]
    fn test_feature_algebra_reexport_compatibility() {
        let value_new = FeatureValue::text("token");
        let value_old: legacy::FeatureValue = value_new.clone();
        assert_eq!(value_old, legacy::FeatureValue::Text("token".to_string()));

        let expr_new = FeatureExpr::from(value_new);
        let expr_old: legacy::FeatureExpr = expr_new;
        assert!(matches!(expr_old, legacy::FeatureExpr::Value(_)));
    }
}
