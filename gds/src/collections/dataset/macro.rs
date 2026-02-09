//! Lightweight macros for concise Dataset DSL usage.
//!
//! Design goal: macros provide an ergonomic *surface language* for datasets,
//! while delegating execution/semantics to typed functions and methods.
//!
//! Key idea: treat a Dataset as a **virtual DataFrame** *and* as a table of
//! **TypedDict-like records**.
//!
//! Polars `Struct` is a useful “record/TypedDict” substrate: a column whose
//! values are records with named fields. The dataset layer should lean into
//! this for data cleanup and unstructured→structured workflows.
//!
//! These macros intentionally compose with the existing DataFrame DSL macros
//! (`select!`, `filter!`, `sel!`, `expr!`, etc.), since `Dataset` exposes
//! DataFrame-like methods.
//!
//! Examples:
//! ```rust
//! use gds::collections::dataset::prelude::*;
//! use gds::{ds, io, stream, split, td, get};
//! use gds::collections::dataframe::prelude::*;
//!
//! let train = ds!(io!(csv "./data/train.csv"))?;
//! let batched = stream!(train, batch = 1024);
//! let _s = split!(train);
//!
//! // TypedDict-like record construction as a Struct expression
//! // (field values can be Exprs; bare idents are treated as columns).
//! let record = td!(id: id, score_x2: (score * 2.0));
//! let _field = get!(record, "id");
//! # Ok::<(), GDSFrameError>(())
//! ```

/// IO shorthand for loading a `Dataset` from common formats.
///
/// This keeps *format* vocabulary (`csv`, `parquet`, …) out of the core `ds!`
/// “dataset language”.
#[macro_export]
macro_rules! io {
    (csv $path:expr) => {
        $crate::collections::dataset::Dataset::from_csv($path)
    };
    (parquet $path:expr) => {
        $crate::collections::dataset::Dataset::from_parquet($path)
    };
    (ipc $path:expr) => {
        $crate::collections::dataset::Dataset::from_ipc($path)
    };
    (json $path:expr) => {
        $crate::collections::dataset::Dataset::from_json($path)
    };
}

/// Dataset constructor shorthand.
///
/// Expands to `Result<Dataset, GDSFrameError>` for IO-backed constructors.
///
/// Supported forms:
/// - `ds!(io!(csv "path"))` (preferred)
/// - `ds!(io csv "path")` (preferred)
///
/// Back-compat forms (may be removed later):
/// - `ds!(csv "path")`, `ds!(parquet "path")`, ...
/// - `ds!(tbl ... )` delegates to the DataFrame `tbl!` macro and wraps it.
#[macro_export]
macro_rules! ds {
    (io $($rest:tt)+) => {
        $crate::io!($($rest)+)
    };
    ($loaded:expr) => {
        $loaded
    };

	(csv $path:expr) => {
        $crate::io!(csv $path)
	};
	(parquet $path:expr) => {
        $crate::io!(parquet $path)
	};
	(ipc $path:expr) => {
        $crate::io!(ipc $path)
	};
	(json $path:expr) => {
        $crate::io!(json $path)
	};
	(tbl $($spec:tt)+) => {
		$crate::tbl!($($spec)+).map($crate::collections::dataset::Dataset::new)
	};
    (corpus $($rest:tt)+) => {
        $crate::corpus!($($rest)+).map($crate::collections::dataset::Corpus::into_dataset)
    };
}

/// TypedDict-like record (Polars `Struct`) expression builder.
///
/// This macro builds a single `polars::Expr` whose output is a Struct/record.
/// Each key becomes a field name.
///
/// Supported field value forms:
/// - `field: col_ident` (bare ident is treated as a column expression)
/// - `field: (arithmetic / comparisons ...)` routed through `expr!`
/// - `field: some_expr` where `some_expr` is already a `polars::Expr`
///
/// Example:
/// ```rust
/// use gds::collections::dataframe::prelude::*;
/// use gds::{td, get};
/// let r = td!(id: id, score_x2: (score * 2.0));
/// let id_expr = get!(r, "id");
/// ```
#[macro_export]
macro_rules! td {
    ( $( $key:ident : $value:tt ),* $(,)? ) => {{
        let mut __fields: ::std::vec::Vec<polars::prelude::Expr> = ::std::vec::Vec::new();
        $( $crate::td!(@push __fields, $key : $value); )*
        $crate::collections::dataframe::record(__fields)
    }};

    (@push $vec:ident, $key:ident : ( $($e:tt)+ ) ) => {{
        $vec.push($crate::expr!($($e)+).alias(stringify!($key)));
    }};
    (@push $vec:ident, $key:ident : $id:ident ) => {{
        $vec.push($crate::collections::dataframe::col(stringify!($id)).alias(stringify!($key)));
    }};
    (@push $vec:ident, $key:ident : $expr:expr ) => {{
        $vec.push(($expr).alias(stringify!($key)));
    }};
}

/// TypedDict/Struct field access helper.
///
/// Supported forms:
/// - `get!(struct_expr, "field")`
/// - `get!(struct_col_ident.field_ident)` (struct column + field)
#[macro_export]
macro_rules! get {
    ($struct_expr:expr, $field:expr) => {
        ($struct_expr).struct_().field_by_name($field)
    };
    ($struct_col:ident . $field:ident) => {
        $crate::collections::dataframe::col(stringify!($struct_col))
            .struct_()
            .field_by_name(stringify!($field))
    };
}

/// Tree expression builder.
///
/// Example:
/// ```rust
/// use gds::{tree, tleaf};
/// let t = tree!("S" => [tleaf!("Mary"), tleaf!("walks")]);
/// ```
#[macro_export]
macro_rules! tree {
    ($label:expr => [ $( $child:expr ),* $(,)? ]) => {
        $crate::collections::dataset::tree::node_expr($label, vec![ $( $child ),* ])
    };
}

/// Tree leaf expression builder.
#[macro_export]
macro_rules! tleaf {
    ($value:expr) => {
        $crate::collections::dataset::tree::leaf_expr($value)
    };
}

/// Tree position expression builder.
#[macro_export]
macro_rules! tpos {
    ( $( $idx:expr ),+ $(,)? ) => {
        $crate::collections::dataset::tree::pos_expr(vec![ $( $idx as usize ),+ ])
    };
}

/// Tree root position expression builder.
#[macro_export]
macro_rules! troot {
    () => {
        $crate::collections::dataset::tree::pos_expr(Vec::<usize>::new())
    };
}

/// Tree span expression builder.
#[macro_export]
macro_rules! tspan {
    ($start:expr, $end:expr $(,)?) => {
        $crate::collections::dataset::tree::span_expr($start as usize, $end as usize)
    };
}

/// Tree transform expression builder.
#[macro_export]
macro_rules! ttransform {
    ($input:expr, $op:expr $(,)?) => {
        $crate::collections::dataset::tree::transform_expr($input, $op)
    };
}

/// Feature position builder.
#[macro_export]
macro_rules! fpos {
    ( $( $idx:expr ),+ $(,)? ) => {
        $crate::collections::dataset::feature::FeaturePosition::new(vec![ $( $idx as i32 ),+ ])
            .expect("feature positions cannot be empty")
    };
}

/// Feature position range builder (inclusive).
#[macro_export]
macro_rules! frange {
    ( $start:expr, $end:expr $(,)? ) => {
        $crate::collections::dataset::feature::FeaturePosition::from_range(
            $start as i32,
            $end as i32,
        )
        .expect("illegal feature position range")
    };
}

/// Feature spec builder for offset-based positions.
#[macro_export]
macro_rules! fspec {
    ( $property:expr, $positions:expr $(,)? ) => {
        $crate::collections::dataset::feature::FeatureSpec::new($property, $positions)
    };
}

/// Feature spec builder for tree-position paths.
#[macro_export]
macro_rules! fspec_tree {
    ( $property:expr, [ $( $idx:expr ),+ $(,)? ] ) => {
        $crate::collections::dataset::feature::FeatureSpec::new_tree(
            $property,
            $crate::collections::dataset::expressions::tree::TreePos::new(vec![
                $( $idx as usize ),+
            ]),
        )
    };
}

/// Feature condition builder.
#[macro_export]
macro_rules! fcond {
    ( $spec:expr, $value:expr $(,)? ) => {
        $crate::collections::dataset::feature::FeatureCondition::new($spec, $value)
    };
}

/// Feature template builder.
#[macro_export]
macro_rules! ftemplate {
    ( $( $spec:expr ),+ $(,)? ) => {
        $crate::collections::dataset::feature::FeatureTemplate::new(vec![ $( $spec ),+ ])
    };
}

/// Feature expression wrapper.
#[macro_export]
macro_rules! fexpr {
    ($expr:expr) => {
        $crate::collections::dataset::expressions::feature::FeatureExpr::from($expr)
    };
}

/// Build a lazy Dataset Plan (DataOps graph) with a small, schemish pipe grammar.
///
/// Grammar (minimal):
/// - `plan!(-> X (filter score > 20.0) (select id, (score * 2.0) as "score_x2") (item id: id, score: score))`
/// - `plan!(-> (my_dataset()) (filter score > 20.0))`
///
/// Notes:
/// - `X` is a dataset variable bound in a `PlanEnv`.
/// - `item ...` constructs/overwrites the canonical `item` Struct record column.
/// - This macro is intentionally conservative; we can grow it as we learn.
#[macro_export]
macro_rules! plan {
    (-> $src:ident $( ( $op:ident $($args:tt)* ) )* $(,)?) => {{
        let mut __p = $crate::collections::dataset::plan::Plan::new(
            $crate::collections::dataset::plan::Source::Var(stringify!($src).to_string()),
        );
        $( __p = $crate::plan!(@step __p, $op $($args)*); )*
        __p
    }};
    (-> ( $src:expr ) $( ( $op:ident $($args:tt)* ) )* $(,)?) => {{
        let mut __p = $crate::collections::dataset::plan::Plan::new(
            $crate::collections::dataset::plan::Source::Value($src),
        );
        $( __p = $crate::plan!(@step __p, $op $($args)*); )*
        __p
    }};

    (@step $p:expr, name $name:expr) => { $p.named($name) };

    (@step $p:expr, filter $($pred:tt)+) => {
        $p.push_step($crate::collections::dataset::plan::Step::Filter($crate::expr!($($pred)+)))
    };

    (@step $p:expr, select $($spec:tt)+) => {{
        let __exprs = $crate::plan!(@exprs $($spec)+);
        $p.push_step($crate::collections::dataset::plan::Step::Select(__exprs))
    }};

    (@step $p:expr, with $($spec:tt)+) => {{
        let __exprs = $crate::plan!(@exprs $($spec)+);
        $p.push_step($crate::collections::dataset::plan::Step::WithColumns(__exprs))
    }};

    // Canonical record column: item
    (@step $p:expr, item $($fields:tt)+) => {{
        let __item = $crate::td!($($fields)+).alias("item");
        $p.push_step($crate::collections::dataset::plan::Step::Item(__item))
    }};

    (@step $p:expr, split $split:tt) => {
        $p.push_step($crate::collections::dataset::plan::Step::Split($crate::split!($split)))
    };
    (@step $p:expr, split $split:expr) => {
        $p.push_step($crate::collections::dataset::plan::Step::Split($crate::split!($split)))
    };

    (@step $p:expr, batch $n:expr) => {
        $p.push_step($crate::collections::dataset::plan::Step::Batch($n))
    };

    // Convert a select/with spec into a Vec<Expr> (borrow the feel of select!).
    (@exprs $($rest:tt)+) => {{
        let mut __exprs: ::std::vec::Vec<polars::prelude::Expr> = ::std::vec::Vec::new();
        $crate::plan!(@push_expr __exprs, $($rest)+);
        __exprs
    }};
    (@push_expr $vec:ident,) => {};
    (@push_expr $vec:ident) => {};

    (@push_expr $vec:ident, ( $($e:tt)+ ) as $alias:expr $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+).alias($alias));
        $( $crate::plan!(@push_expr $vec, $($rest)*); )?
    }};
    (@push_expr $vec:ident, ( $($e:tt)+ ) as $alias:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+).alias(stringify!($alias)));
        $( $crate::plan!(@push_expr $vec, $($rest)*); )?
    }};
    (@push_expr $vec:ident, ( $($e:tt)+ ) => $alias:expr $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+).alias($alias));
        $( $crate::plan!(@push_expr $vec, $($rest)*); )?
    }};
    (@push_expr $vec:ident, ( $($e:tt)+ ) $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+));
        $( $crate::plan!(@push_expr $vec, $($rest)*); )?
    }};
    (@push_expr $vec:ident, $id:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::collections::dataframe::col(stringify!($id)));
        $( $crate::plan!(@push_expr $vec, $($rest)*); )?
    }};
    (@push_expr $vec:ident, $name:literal $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::collections::dataframe::col($name));
        $( $crate::plan!(@push_expr $vec, $($rest)*); )?
    }};
}

/// Alias for `plan!` when you want to lean into the skrub-inspired naming.
#[macro_export]
macro_rules! dop {
    ($($t:tt)+) => { $crate::plan!($($t)+) };
}

/// Corpus constructor shorthand.
///
/// Supported forms:
/// - `corpus!(texts [&str, ...])`
/// - `corpus!(file "path")`
/// - `corpus!(dir "path")` (non-recursive)
#[macro_export]
macro_rules! corpus {
	(texts [ $($t:expr),* $(,)? ]) => {
		$crate::collections::dataset::Corpus::from_texts(&[ $($t),* ])
	};
	(file $path:expr) => {
		$crate::collections::dataset::Corpus::from_text_file($path)
	};
	(dir $path:expr) => {
		$crate::collections::dataset::scan_text_dir($path)
	};
}

/// Dataset split shorthand.
///
/// Supported forms:
/// - `split!(train)` / `split!(validation)` / `split!(test)` / `split!(all)`
/// - `split!("my_split")` for `DatasetSplit::Custom`.
#[macro_export]
macro_rules! split {
    (train) => {
        $crate::collections::dataset::DatasetSplit::Train
    };
    (val) => {
        $crate::collections::dataset::DatasetSplit::Validation
    };
    (valid) => {
        $crate::collections::dataset::DatasetSplit::Validation
    };
    (validation) => {
        $crate::collections::dataset::DatasetSplit::Validation
    };
    (test) => {
        $crate::collections::dataset::DatasetSplit::Test
    };
    (all) => {
        $crate::collections::dataset::DatasetSplit::All
    };
    ($name:literal) => {
        $crate::collections::dataset::DatasetSplit::Custom($name.to_string())
    };
    ($name:expr) => {
        $crate::collections::dataset::DatasetSplit::Custom(($name).to_string())
    };
}

/// Streaming dataset shorthand.
///
/// Supported forms:
/// - `stream!(dataset, batch = 1024)`
/// - `stream!(dataset, batch = 1024, transform = |lf| { ... })`
#[macro_export]
macro_rules! stream {
    ($dataset:expr, batch = $batch:expr) => {
        $crate::collections::dataset::StreamingDataset::new($dataset, $batch)
    };
    ($dataset:expr, batch = $batch:expr, transform = $transform:expr) => {
        $crate::collections::dataset::StreamingDataset::new($dataset, $batch)
            .with_transform($transform)
    };
}
