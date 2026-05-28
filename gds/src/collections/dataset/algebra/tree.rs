//! Tree algebra canonical surface.
//!
//! Phase A: re-export from legacy `dataset::expressions::tree` so callers can
//! migrate without behavior changes.

pub use crate::collections::dataset::expressions::tree::*;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::collections::dataset::expressions::tree as legacy;

	#[test]
	fn test_tree_algebra_reexport_compatibility() {
		let pos_new = TreePos::new(vec![0, 1]);
		let pos_old: legacy::TreePos = pos_new.clone();
		assert_eq!(pos_old.path(), &[0, 1]);

		let expr_new = TreeExpr::pos(vec![2, 0]);
		let expr_old: legacy::TreeExpr = expr_new;
		assert!(matches!(expr_old, legacy::TreeExpr::Pos(_)));
	}
}
