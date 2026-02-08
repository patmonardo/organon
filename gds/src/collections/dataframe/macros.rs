//! Lightweight macros for concise DataFrame DSL usage.
//!
//! These wrap the existing `col`, `lit`, and `when` helpers to provide a
//! more Pythonic, terser call style in examples and user code.
//!
//! Examples:
//! ```rust
//! use gds::collections::dataframe::prelude::*;
//! let expr = col!(score).gt(lit!(20.0));
//! ```

/// Shorthand for `col("name")`. Accepts an identifier or an expression/string.
#[macro_export]
macro_rules! col {
    ($name:ident) => {
        $crate::collections::dataframe::col(stringify!($name))
    };
    ($name:expr) => {
        $crate::collections::dataframe::col($name)
    };
}

/// Shorthand for `lit(value)`.
#[macro_export]
macro_rules! lit {
    ($e:expr) => {
        $crate::collections::dataframe::lit($e)
    };
}

/// Expression shorthand macro.
///
/// - `expr!(col)` expands to `col("col")`.
/// - `expr!(1)` expands to `lit(1)`.
/// - `expr!(a + b)` expands recursively to `(expr!(a) + expr!(b))`.
/// - Chained arithmetic is left-associative; use parentheses for precedence.
/// - Parentheses are supported: `expr!((a * 2.0))`.
#[macro_export]
macro_rules! expr {
    // Identifier -> column
    ($id:ident) => {
        $crate::collections::dataframe::col(stringify!($id))
    };
    // Literal -> literal expr
    ($lit:literal) => {
        $crate::collections::dataframe::lit($lit)
    };
    // Parenthesized group
    ( ( $($inner:tt)+ ) ) => {
        $crate::expr!($($inner)+)
    };
    // Left-associative chaining for arithmetic
    ($lhs:tt + $rhs:tt $($rest:tt)+) => {
        $crate::expr!(($lhs + $rhs) $($rest)+)
    };
    ($lhs:tt - $rhs:tt $($rest:tt)+) => {
        $crate::expr!(($lhs - $rhs) $($rest)+)
    };
    ($lhs:tt * $rhs:tt $($rest:tt)+) => {
        $crate::expr!(($lhs * $rhs) $($rest)+)
    };
    ($lhs:tt / $rhs:tt $($rest:tt)+) => {
        $crate::expr!(($lhs / $rhs) $($rest)+)
    };
    // Relational operators -> call the corresponding method on Expr
    ($lhs:tt > $rhs:tt) => {
        $crate::expr!($lhs).gt($crate::expr!($rhs))
    };
    ($lhs:tt < $rhs:tt) => {
        $crate::expr!($lhs).lt($crate::expr!($rhs))
    };
    ($lhs:tt >= $rhs:tt) => {
        $crate::expr!($lhs).gt_eq($crate::expr!($rhs))
    };
    ($lhs:tt <= $rhs:tt) => {
        $crate::expr!($lhs).lt_eq($crate::expr!($rhs))
    };
    ($lhs:tt == $rhs:tt) => {
        $crate::expr!($lhs).eq($crate::expr!($rhs))
    };
    ($lhs:tt != $rhs:tt) => {
        $crate::expr!($lhs).neq($crate::expr!($rhs))
    };
    // Fallback to allow arithmetic ops (like *) to use regular operator overloading
    ($lhs:tt $op:tt $rhs:tt) => {
        ($crate::expr!($lhs) $op $crate::expr!($rhs))
    };
}

/// Prefix expression macro for concise arithmetic and comparisons.
///
/// Examples:
/// ```rust
/// let a = s!(+ score 2.0);
/// let b = s!(> score 20.0);
/// ```
#[macro_export]
macro_rules! s {
    // parenthesized group -> recurse
    ( ( $($inner:tt)+ ) ) => {
        $crate::s!($($inner)+)
    };

    // arithmetic
    ( + $a:tt $b:tt ) => { ($crate::expr!($a) + $crate::expr!($b)) };
    ( - $a:tt $b:tt ) => { ($crate::expr!($a) - $crate::expr!($b)) };
    ( * $a:tt $b:tt ) => { ($crate::expr!($a) * $crate::expr!($b)) };
    ( / $a:tt $b:tt ) => { ($crate::expr!($a) / $crate::expr!($b)) };

    // comparisons -> use Expr methods
    ( > $a:tt $b:tt ) => { $crate::expr!($a).gt($crate::expr!($b)) };
    ( < $a:tt $b:tt ) => { $crate::expr!($a).lt($crate::expr!($b)) };
    ( >= $a:tt $b:tt ) => { $crate::expr!($a).gt_eq($crate::expr!($b)) };
    ( <= $a:tt $b:tt ) => { $crate::expr!($a).lt_eq($crate::expr!($b)) };
    ( == $a:tt $b:tt ) => { $crate::expr!($a).eq($crate::expr!($b)) };
    ( != $a:tt $b:tt ) => { $crate::expr!($a).neq($crate::expr!($b)) };

    // fallback to `expr!` for single tokens
    ($e:tt) => { $crate::expr!($e) };
}

/// Shorthand for `when(expr)`.
#[macro_export]
macro_rules! when {
    ($e:expr) => {
        $crate::collections::dataframe::when($e)
    };
}

/// Build a table from a concise column spec.
///
/// Supported forms for each column:
/// - `name: ["a", "b"]` for string columns (literals converted via `to_string()`)
/// - `name: i64 => [1, 2, 3]` for integer columns
/// - `name: f64 => [1.0, 2.0]` for float columns
/// `name` can be an identifier or a string literal. Columns are comma-separated.
/// The macro expands to a `TableBuilder::new()... .build()` expression returning `Result<_, _>`.
#[macro_export]
macro_rules! tbl {
    // Entry point: capture comma-separated column specs as `name: body` pairs
    ( $( $name:ident : $( $body:tt )+ ),* $(,)? ) => {{
        let mut builder = $crate::collections::dataframe::TableBuilder::new();
        $( tbl!(@col builder; $name : $($body)+); )*
        builder.build()
    }};

    // Support literal column names
    ( $( $name:literal : $( $body:tt )+ ),* $(,)? ) => {{
        let mut builder = $crate::collections::dataframe::TableBuilder::new();
        $( tbl!(@col builder; $name : $($body)+); )*
        builder.build()
    }};

    // String column, ident
    (@col $builder:ident; $name:ident : [ $($s:literal),* ]) => {{
        $builder = $builder.with_string_column(stringify!($name), &[$($s.to_string()),*]);
    }};

    // String column, literal
    (@col $builder:ident; $name:literal : [ $($s:literal),* ]) => {{
        $builder = $builder.with_string_column($name, &[$($s.to_string()),*]);
    }};

    // i64 column, ident
    (@col $builder:ident; $name:ident : i64 => [ $($v:expr),* ]) => {{
        $builder = $builder.with_i64_column(stringify!($name), &[$($v),*]);
    }};

    // i64 column, literal
    (@col $builder:ident; $name:literal : i64 => [ $($v:expr),* ]) => {{
        $builder = $builder.with_i64_column($name, &[$($v),*]);
    }};

    // f64 column, ident
    (@col $builder:ident; $name:ident : f64 => [ $($v:expr),* ]) => {{
        $builder = $builder.with_f64_column(stringify!($name), &[$($v),*]);
    }};

    // f64 column, literal
    (@col $builder:ident; $name:literal : f64 => [ $($v:expr),* ]) => {{
        $builder = $builder.with_f64_column($name, &[$($v),*]);
    }};

    // Fallback to provide better error message
    (@col $builder:ident; $($tok:tt)+) => {{
        compile_error!("invalid tbl! column specification")
    }};
}

#[macro_export]
macro_rules! tbl_def {
    // Accept mixed identifier and literal column names by matching a token tree
    // for the column name and delegating to the `tbl!` helper which handles
    // ident vs literal cases.
    ( $( ( $name:tt : $( $body:tt )+ ) ),* $(,)? ) => {{
        let mut builder = $crate::collections::dataframe::TableBuilder::new();
        $( $crate::tbl!(@col builder; $name : $($body)+); )*
        builder.build()
    }};
}

// Simpler unambiguous table builder macro that uses parenthesized column specs.
// Example:
// tbl_def!(
//   (id: i64 => [1,2,3]),
//   (score: f64 => [1.0,2.0]),
//   (group: ["A","B"]),
// )

/// Aggregate shorthand for `col("name").agg().alias("...")`.
///
/// Example:
/// ```rust
/// let a = agg!(score.mean => "avg_score");
/// ```
#[macro_export]
macro_rules! agg {
    // simple form: `col.method => "alias"`
    ($col:ident . $meth:ident => $alias:expr) => {
        $crate::collections::dataframe::col(stringify!($col))
            .$meth()
            .alias($alias)
    };
    // allow literal alias without quotes: `as alias`
    ($col:ident . $meth:ident as $alias:ident) => {
        $crate::collections::dataframe::col(stringify!($col))
            .$meth()
            .alias(stringify!($alias))
    };

    // convenience: accept `sum(col) => "alias"` form and common agg wrappers
    ($meth:ident ( $col:ident ) => $alias:expr) => {
        $crate::collections::dataframe::col(stringify!($col))
            .$meth()
            .alias($alias)
    };
    ($meth:ident ( $col:ident ) as $alias:ident) => {
        $crate::collections::dataframe::col(stringify!($col))
            .$meth()
            .alias(stringify!($alias))
    };
}

/// Convenience macro to perform `group_by_columns` on a table with a concise
/// aggregate list built from `agg!` items.
///
/// Example:
/// ```rust
/// let g = group_by!(table, [group], [agg!(score.mean => "avg_score"), agg!(id.count => "rows")])?;
/// ```
#[macro_export]
macro_rules! group_by {
    // table, list of ident keys, list of aggs
    ($table:expr, [ $($key:ident),* ], [ $($agg:expr),* ]) => {
        $crate::group_by!(@impl $table, &[ $(stringify!($key)),* ], &[ $($agg),* ])
    };

    // single key version: table, key ident, list of aggs
    ($table:expr, $key:ident, [ $($agg:expr),* ]) => {
        $crate::group_by!($table, [ $key ], [ $($agg),* ])
    };

    // allow literal keys
    ($table:expr, [ $($key:literal),* ], [ $($agg:expr),* ]) => {
        $crate::group_by!(@impl $table, &[ $($key),* ], &[ $($agg),* ])
    };

    // internal implementation helper
    (@impl $table:expr, $keys:expr, $aggs:expr) => {
        $table.group_by_columns($keys, $aggs)
    };
}

/// Select shorthand to build a `select(&[...])` call.
/// Examples:
/// ```rust
/// let s = select!(table, id, (score * 2.0) as "score_x2", name);
/// ```
#[macro_export]
macro_rules! select {
    // Entry point: use a tt-muncher so multi-token items (like `(a * 2.0) as "x"`) work.
    ($table:expr, $($rest:tt)+) => {{
        let mut __exprs = ::std::vec::Vec::new();
        $crate::select!(@push __exprs, $($rest)+);
        $table.select(&__exprs)
    }};

    // End of list
    (@push $vec:ident,) => {};
    (@push $vec:ident) => {};

    // Parenthesized expression with alias using literal
    (@push $vec:ident, ( $($e:tt)+ ) as $alias:expr $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+).alias($alias));
        $( $crate::select!(@push $vec, $($rest)*); )?
    }};

    // Parenthesized expression with alias using ident
    (@push $vec:ident, ( $($e:tt)+ ) as $alias:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+).alias(stringify!($alias)));
        $( $crate::select!(@push $vec, $($rest)*); )?
    }};

    // Parenthesized expression with alias using =>
    (@push $vec:ident, ( $($e:tt)+ ) => $alias:expr $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+).alias($alias));
        $( $crate::select!(@push $vec, $($rest)*); )?
    }};

    // Parenthesized expression without alias
    (@push $vec:ident, ( $($e:tt)+ ) $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::expr!($($e)+));
        $( $crate::select!(@push $vec, $($rest)*); )?
    }};

    // Identifier -> col("id")
    (@push $vec:ident, $id:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::collections::dataframe::col(stringify!($id)));
        $( $crate::select!(@push $vec, $($rest)*); )?
    }};

    // Literal column name -> col("name")
    (@push $vec:ident, $name:literal $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::collections::dataframe::col($name));
        $( $crate::select!(@push $vec, $($rest)*); )?
    }};
}

/// Filter shorthand to build a `filter_expr(...)` call.
/// Examples:
/// ```rust
/// let f = filter!(table, score > 20.0)?;
/// let g = filter!(table, (score * 2.0) > 40.0)?;
/// ```
#[macro_export]
macro_rules! filter {
    ($table:expr, $($e:tt)+) => {
        $table.filter_expr($crate::expr!($($e)+))
    };
}

/// Build a selector list from identifiers or string literals.
/// Example: `selector![id, score, "group"]`
#[macro_export]
macro_rules! selector {
    [ $($sel:tt),* $(,)? ] => {{
        let mut __cols: ::std::vec::Vec<&'static str> = ::std::vec::Vec::new();
        $( $crate::selector!(@push __cols, $sel); )*
        __cols
    }};

    (@push $vec:ident, $id:ident) => {
        $vec.push(stringify!($id));
    };

    (@push $vec:ident, $lit:literal) => {
        $vec.push($lit);
    };
}

/// Selector expression macro for selector composition and common selector helpers.
///
/// Examples:
/// ```rust
/// let s = sel!(temporal | string | starts_with("e"));
/// let t = sel!(temporal & matches("opp|JJK"));
/// let u = sel!(all - by_dtype([Duration(Milliseconds), Time]));
/// ```
#[macro_export]
macro_rules! sel {
    // Parenthesized group
    ( ( $($inner:tt)+ ) ) => {
        $crate::sel!($($inner)+)
    };

    // Union / intersection / difference operators
    ($lhs:tt | $($rest:tt)+) => {
        $crate::sel!($lhs).or($crate::sel!($($rest)+))
    };
    ($lhs:tt & $($rest:tt)+) => {
        $crate::sel!($lhs).and($crate::sel!($($rest)+))
    };
    ($lhs:tt - $($rest:tt)+) => {
        $crate::sel!($lhs).exclude($crate::sel!($($rest)+))
    };

    // Explicit union/intersection/difference helpers
    (union($first:tt, $second:tt $(, $rest:tt)* $(,)?)) => {{
        let mut __sel = $crate::sel!($first).or($crate::sel!($second));
        $( __sel = __sel.or($crate::sel!($rest)); )*
        __sel
    }};
    (intersection($first:tt, $second:tt $(, $rest:tt)* $(,)?)) => {{
        let mut __sel = $crate::sel!($first).and($crate::sel!($second));
        $( __sel = __sel.and($crate::sel!($rest)); )*
        __sel
    }};
    (difference($base:tt, $remove:tt)) => {
        $crate::sel!($base).exclude($crate::sel!($remove))
    };

    // Common selector atoms
    (all) => {
        $crate::collections::dataframe::selector_all()
    };
    (numeric) => {
        $crate::collections::dataframe::selector_numeric()
    };
    (string) => {
        $crate::collections::dataframe::selector_string()
    };
    (temporal) => {
        $crate::collections::dataframe::selector_temporal()
    };
    (struct) => {
        $crate::collections::dataframe::selectors::structure()
    };
    (structure) => {
        $crate::collections::dataframe::selectors::structure()
    };
    (record) => {
        $crate::collections::dataframe::selectors::record()
    };
    (starts_with($prefix:expr)) => {
        $crate::collections::dataframe::selector_starts_with($prefix)
    };
    (contains($needle:expr)) => {
        $crate::collections::dataframe::selector_contains($needle)
    };
    (matches($pattern:expr)) => {
        $crate::collections::dataframe::selector_matches($pattern)
            .expect("sel!(matches(...)) requires a valid regex")
    };
    (by_dtype([ $( $dtype:ident $( ( $($args:tt)* ) )? ),* $(,)? ])) => {
        $crate::collections::dataframe::selector_by_dtype(&[
            $( $crate::dtype!($dtype $( ( $($args)* ) )?) ),*
        ])
    };
    (by_name([ $($name:tt),* $(,)? ])) => {
        $crate::collections::dataframe::selector_by_name(&[ $( $crate::sel!(@name $name) ),* ])
    };

    (@name $name:ident) => {
        stringify!($name)
    };
    (@name $name:literal) => {
        $name
    };
}

/// Shorthand for DataType variants (avoids `DataType::` in schemas and fields).
#[macro_export]
macro_rules! dtype {
    (Boolean) => { polars::prelude::DataType::Boolean };
    (Binary) => { polars::prelude::DataType::Binary };
    (Date) => { polars::prelude::DataType::Date };
    (Time) => { polars::prelude::DataType::Time };
    (String) => { polars::prelude::DataType::String };
    (UInt8) => { polars::prelude::DataType::UInt8 };
    (UInt16) => { polars::prelude::DataType::UInt16 };
    (UInt32) => { polars::prelude::DataType::UInt32 };
    (UInt64) => { polars::prelude::DataType::UInt64 };
    (Int8) => { polars::prelude::DataType::Int8 };
    (Int16) => { polars::prelude::DataType::Int16 };
    (Int32) => { polars::prelude::DataType::Int32 };
    (Int64) => { polars::prelude::DataType::Int64 };
    (Float32) => { polars::prelude::DataType::Float32 };
    (Float64) => { polars::prelude::DataType::Float64 };
    (Duration($unit:ident)) => {
        polars::prelude::DataType::Duration($crate::dtype!(@time_unit $unit))
    };
    (Datetime($unit:ident, $tz:expr)) => {
        polars::prelude::DataType::Datetime($crate::dtype!(@time_unit $unit), $tz)
    };
    (Decimal($precision:expr, $scale:expr)) => {
        polars::prelude::DataType::Decimal($precision, $scale)
    };

    (@time_unit Milliseconds) => { polars::prelude::TimeUnit::Milliseconds };
    (@time_unit Microseconds) => { polars::prelude::TimeUnit::Microseconds };
    (@time_unit Nanoseconds) => { polars::prelude::TimeUnit::Nanoseconds };

    // Fallback to allow passing a DataType expression/path directly
    ($other:expr) => { $other };
}

/// Shorthand for Field construction.
///
/// Examples:
/// ```rust
/// let f = field!(score: Float64);
/// let g = field!("ts": Datetime(Milliseconds, None));
/// ```
#[macro_export]
macro_rules! field {
    ($name:ident : $dtype:ident) => {
        polars::prelude::Field::new(stringify!($name).into(), $crate::dtype!($dtype))
    };
    ($name:ident : $dtype:ident ( $($args:tt)* )) => {
        polars::prelude::Field::new(
            stringify!($name).into(),
            $crate::dtype!($dtype($($args)*)),
        )
    };
    ($name:ident => $dtype:ident) => {
        polars::prelude::Field::new(stringify!($name).into(), $crate::dtype!($dtype))
    };
    ($name:ident => $dtype:ident ( $($args:tt)* )) => {
        polars::prelude::Field::new(
            stringify!($name).into(),
            $crate::dtype!($dtype($($args)*)),
        )
    };
    ($name:literal : $dtype:ident) => {
        polars::prelude::Field::new($name.into(), $crate::dtype!($dtype))
    };
    ($name:literal : $dtype:ident ( $($args:tt)* )) => {
        polars::prelude::Field::new(
            $name.into(),
            $crate::dtype!($dtype($($args)*)),
        )
    };
    ($name:literal => $dtype:ident) => {
        polars::prelude::Field::new($name.into(), $crate::dtype!($dtype))
    };
    ($name:literal => $dtype:ident ( $($args:tt)* )) => {
        polars::prelude::Field::new(
            $name.into(),
            $crate::dtype!($dtype($($args)*)),
        )
    };
    ($name:expr, $dtype:expr) => {
        polars::prelude::Field::new($name.into(), $dtype)
    };
}

/// Shorthand for Schema construction.
///
/// Examples:
/// ```rust
/// let s = schema![id: UInt32, score: Float64];
/// let t = schema![field!("name": String)];
/// ```
#[macro_export]
macro_rules! schema {
    ( $($rest:tt)+ ) => {
        polars::prelude::Schema::from_iter($crate::fields!( $($rest)+ ))
    };
}

/// Expand field definitions into a `Vec<Field>`.
///
/// Examples:
/// ```rust
/// let fields = fields![id => UInt32, score => Float64];
/// let schema = schema!(fields);
/// ```
#[macro_export]
macro_rules! fields {
    (@push $vec:ident,) => {};
    (@push $vec:ident) => {};

    // ident name with => and args
    (@push $vec:ident, $name:ident => $dtype:ident ( $($args:tt)* ) $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name => $dtype($($args)*)));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};
    // ident name with =>
    (@push $vec:ident, $name:ident => $dtype:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name => $dtype));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};
    // ident name with : and args
    (@push $vec:ident, $name:ident : $dtype:ident ( $($args:tt)* ) $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name: $dtype($($args)*)));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};
    // ident name with :
    (@push $vec:ident, $name:ident : $dtype:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name: $dtype));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};

    // literal name with => and args
    (@push $vec:ident, $name:literal => $dtype:ident ( $($args:tt)* ) $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name => $dtype($($args)*)));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};
    // literal name with =>
    (@push $vec:ident, $name:literal => $dtype:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name => $dtype));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};
    // literal name with : and args
    (@push $vec:ident, $name:literal : $dtype:ident ( $($args:tt)* ) $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name: $dtype($($args)*)));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};
    // literal name with :
    (@push $vec:ident, $name:literal : $dtype:ident $(, $($rest:tt)*)? ) => {{
        $vec.push($crate::field!($name: $dtype));
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};

    // Fallback: allow raw Field expressions
    (@push $vec:ident, $field:expr $(, $($rest:tt)*)? ) => {{
        $vec.push($field);
        $( $crate::fields!(@push $vec, $($rest)*); )?
    }};

    ( $($rest:tt)+ ) => {{
        let mut __fields = ::std::vec::Vec::new();
        $crate::fields!(@push __fields, $($rest)+);
        __fields
    }};
}

/// Select columns using a selector list: `select_columns!(table, [id, score])`.
#[macro_export]
macro_rules! select_columns {
    ($table:expr, [ $($sel:tt),* $(,)? ]) => {
        $table.select_columns(&$crate::selector![ $($sel),* ])
    };
}

/// Python-like shorthand for `select_columns!` accepting either a bracketed
/// selector list or bare selector arguments: `sc!(table, id, score)` or
/// `sc!(table, [id, "name"])`.
#[macro_export]
macro_rules! sc {
    ($table:expr, [ $($sel:tt),* $(,)? ]) => {
        $crate::select_columns!($table, [ $($sel),* ])
    };
    ($table:expr, $($sel:tt),* $(,)? ) => {
        $crate::select_columns!($table, [ $($sel),* ])
    };
}

/// Order by columns with a concise direction syntax.
/// Examples:
/// `order_by!(table, [score], desc)`
/// `order_by!(table, ["score"], asc)`
#[macro_export]
macro_rules! order_by {
    ($table:expr, [ $($sel:tt),* $(,)? ]) => {
        $table.order_by_columns(
            &$crate::selector![ $($sel),* ],
            $crate::collections::dataframe::PolarsSortMultipleOptions::new()
                .with_order_descending(false),
        )
    };

    ($table:expr, [ $($sel:tt),* $(,)? ], desc) => {
        $table.order_by_columns(
            &$crate::selector![ $($sel),* ],
            $crate::collections::dataframe::PolarsSortMultipleOptions::new()
                .with_order_descending(true),
        )
    };

    ($table:expr, [ $($sel:tt),* $(,)? ], asc) => {
        $table.order_by_columns(
            &$crate::selector![ $($sel),* ],
            $crate::collections::dataframe::PolarsSortMultipleOptions::new()
                .with_order_descending(false),
        )
    };
}
