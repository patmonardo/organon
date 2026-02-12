//! LazyFrame facade (py-polars shaped).

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use polars::prelude::{
    col, CsvWriterOptions, DataFrame, DataType, DynamicGroupOptions, Expr, IdxSize,
    InterpolationMethod, IntoLazy, IpcWriterOptions, JoinArgs, JsonWriterOptions, LazyFrame,
    ParquetWriteOptions, PlSmallStr, PolarsResult, QuantileMethod, RollingGroupOptions, Schema,
    SchemaRef, Selector, SortMultipleOptions, UniqueKeepStrategy,
};
use polars_ops::frame::join::AsOfOptions;
use polars_ops::frame::pivot::UnpivotDF;
use polars_plan::dsl::{
    ExtraColumnsPolicy, MatchToSchemaPerColumn, SinkOptions, SinkTarget, SpecialEq,
};
use polars_plan::frame::OptFlags;
use polars_plan::plans::{HintIR, Sorted as SortedHint};

use crate::collections::dataframe::utils::parse::{
    parse_into_list_of_expressions_for_df, ExprInput, ParseExprOptions,
};

fn lazyframe_schema_df(lf: &LazyFrame) -> PolarsResult<DataFrame> {
    let mut cloned = lf.clone();
    let schema = cloned.collect_schema()?;
    Ok(DataFrame::empty_with_schema(schema.as_ref()))
}

fn selector_from_names(names: &[&str], strict: bool) -> Selector {
    let names = names
        .iter()
        .copied()
        .map(PlSmallStr::from)
        .collect::<Vec<_>>();
    Selector::ByName {
        names: Arc::from(names),
        strict,
    }
}

/// Optimization flags wrapper mirroring py-polars QueryOptFlags.
#[derive(Debug, Clone)]
pub struct QueryOptFlags {
    opt_flags: OptFlags,
}

impl Default for QueryOptFlags {
    fn default() -> Self {
        Self {
            opt_flags: OptFlags::default(),
        }
    }
}

impl QueryOptFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn none() -> Self {
        Self {
            opt_flags: OptFlags::empty(),
        }
    }

    pub fn from_opt_flags(opt_flags: OptFlags) -> Self {
        Self { opt_flags }
    }

    pub fn opt_flags(&self) -> OptFlags {
        self.opt_flags
    }

    pub fn into_opt_flags(self) -> OptFlags {
        self.opt_flags
    }

    pub fn predicate_pushdown(&self) -> bool {
        self.opt_flags.contains(OptFlags::PREDICATE_PUSHDOWN)
    }

    pub fn set_predicate_pushdown(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::PREDICATE_PUSHDOWN, value);
        self
    }

    pub fn projection_pushdown(&self) -> bool {
        self.opt_flags.contains(OptFlags::PROJECTION_PUSHDOWN)
    }

    pub fn set_projection_pushdown(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::PROJECTION_PUSHDOWN, value);
        self
    }

    pub fn simplify_expression(&self) -> bool {
        self.opt_flags.contains(OptFlags::SIMPLIFY_EXPR)
    }

    pub fn set_simplify_expression(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::SIMPLIFY_EXPR, value);
        self
    }

    pub fn slice_pushdown(&self) -> bool {
        self.opt_flags.contains(OptFlags::SLICE_PUSHDOWN)
    }

    pub fn set_slice_pushdown(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::SLICE_PUSHDOWN, value);
        self
    }

    pub fn comm_subplan_elim(&self) -> bool {
        self.opt_flags.contains(OptFlags::COMM_SUBPLAN_ELIM)
    }

    pub fn set_comm_subplan_elim(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::COMM_SUBPLAN_ELIM, value);
        self
    }

    pub fn comm_subexpr_elim(&self) -> bool {
        self.opt_flags.contains(OptFlags::COMM_SUBEXPR_ELIM)
    }

    pub fn set_comm_subexpr_elim(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::COMM_SUBEXPR_ELIM, value);
        self
    }

    pub fn cluster_with_columns(&self) -> bool {
        self.opt_flags.contains(OptFlags::CLUSTER_WITH_COLUMNS)
    }

    pub fn set_cluster_with_columns(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::CLUSTER_WITH_COLUMNS, value);
        self
    }

    pub fn check_order_observe(&self) -> bool {
        self.opt_flags.contains(OptFlags::CHECK_ORDER_OBSERVE)
    }

    pub fn set_check_order_observe(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::CHECK_ORDER_OBSERVE, value);
        self
    }

    pub fn fast_projection(&self) -> bool {
        self.opt_flags.contains(OptFlags::FAST_PROJECTION)
    }

    pub fn set_fast_projection(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::FAST_PROJECTION, value);
        self
    }

    pub fn type_coercion(&self) -> bool {
        self.opt_flags.contains(OptFlags::TYPE_COERCION)
    }

    pub fn set_type_coercion(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::TYPE_COERCION, value);
        self
    }

    pub fn type_check(&self) -> bool {
        self.opt_flags.contains(OptFlags::TYPE_CHECK)
    }

    pub fn set_type_check(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::TYPE_CHECK, value);
        self
    }

    pub fn eager(&self) -> bool {
        self.opt_flags.contains(OptFlags::EAGER)
    }

    pub fn set_eager(&mut self, value: bool) -> &mut Self {
        self.opt_flags.set(OptFlags::EAGER, value);
        self
    }

    pub fn update(
        &mut self,
        predicate_pushdown: Option<bool>,
        projection_pushdown: Option<bool>,
        simplify_expression: Option<bool>,
        slice_pushdown: Option<bool>,
        comm_subplan_elim: Option<bool>,
        comm_subexpr_elim: Option<bool>,
        cluster_with_columns: Option<bool>,
        check_order_observe: Option<bool>,
        fast_projection: Option<bool>,
        type_coercion: Option<bool>,
        type_check: Option<bool>,
    ) -> &mut Self {
        if let Some(value) = predicate_pushdown {
            self.set_predicate_pushdown(value);
        }
        if let Some(value) = projection_pushdown {
            self.set_projection_pushdown(value);
        }
        if let Some(value) = simplify_expression {
            self.set_simplify_expression(value);
        }
        if let Some(value) = slice_pushdown {
            self.set_slice_pushdown(value);
        }
        if let Some(value) = comm_subplan_elim {
            self.set_comm_subplan_elim(value);
        }
        if let Some(value) = comm_subexpr_elim {
            self.set_comm_subexpr_elim(value);
        }
        if let Some(value) = cluster_with_columns {
            self.set_cluster_with_columns(value);
        }
        if let Some(value) = check_order_observe {
            self.set_check_order_observe(value);
        }
        if let Some(value) = fast_projection {
            self.set_fast_projection(value);
        }
        if let Some(value) = type_coercion {
            self.set_type_coercion(value);
        }
        if let Some(value) = type_check {
            self.set_type_check(value);
        }
        self
    }
}

/// Polars LazyFrame wrapper for the Collections SDK.
#[derive(Clone)]
pub struct GDSLazyFrame {
    lf: LazyFrame,
}

impl GDSLazyFrame {
    pub fn new(lf: LazyFrame) -> Self {
        Self { lf }
    }

    pub fn from_dataframe(df: DataFrame) -> Self {
        Self { lf: df.lazy() }
    }

    pub fn lazyframe(&self) -> &LazyFrame {
        &self.lf
    }

    pub fn lazy(&self) -> Self {
        self.clone()
    }

    pub fn into_lazyframe(self) -> LazyFrame {
        self.lf
    }

    pub fn clone(&self) -> Self {
        Self {
            lf: self.lf.clone(),
        }
    }

    pub fn clear(self) -> Self {
        Self {
            lf: self.lf.slice(0, 0),
        }
    }

    pub fn columns(&mut self) -> PolarsResult<Vec<String>> {
        let schema = self.lf.collect_schema()?;
        Ok(schema.iter().map(|(name, _)| name.to_string()).collect())
    }

    pub fn dtypes(&mut self) -> PolarsResult<Vec<DataType>> {
        let schema = self.lf.collect_schema()?;
        Ok(schema.iter().map(|(_, dtype)| dtype.clone()).collect())
    }

    pub fn schema(&mut self) -> PolarsResult<Schema> {
        Ok(self.lf.collect_schema()?.as_ref().clone())
    }

    pub fn width(&mut self) -> PolarsResult<usize> {
        Ok(self.lf.collect_schema()?.len())
    }

    pub fn with_optimizations(self, optimizations: QueryOptFlags) -> Self {
        Self {
            lf: self.lf.with_optimizations(optimizations.into_opt_flags()),
        }
    }

    pub fn cast(self, dtypes: HashMap<String, DataType>, strict: bool) -> Self {
        if dtypes.is_empty() {
            return self;
        }

        let exprs = dtypes
            .into_iter()
            .map(|(name, dtype)| {
                if strict {
                    col(name).strict_cast(dtype)
                } else {
                    col(name).cast(dtype)
                }
            })
            .collect::<Vec<_>>();

        self.with_columns_exprs(exprs)
    }

    pub fn cast_all(self, dtype: DataType, strict: bool) -> Self {
        let expr = if strict {
            col("*").strict_cast(dtype)
        } else {
            col("*").cast(dtype)
        };
        self.with_columns_exprs(vec![expr])
    }

    pub fn collect(self) -> PolarsResult<DataFrame> {
        self.lf.collect()
    }

    pub fn collect_async(self) -> PolarsResult<DataFrame> {
        self.collect()
    }

    pub fn fetch(self, n_rows: usize) -> PolarsResult<DataFrame> {
        self.lf.limit(n_rows as IdxSize).collect()
    }

    pub fn explain(&self, optimized: bool) -> PolarsResult<String> {
        self.lf.explain(optimized)
    }

    pub fn show(&self, n_rows: Option<usize>) -> PolarsResult<String> {
        let rows = n_rows.unwrap_or(10);
        let df = self.clone().fetch(rows)?;
        Ok(format!("{df:?}"))
    }

    pub fn show_graph(&self, optimized: bool) -> PolarsResult<String> {
        self.explain(optimized)
    }

    pub fn collect_with_optimizations(
        self,
        optimizations: QueryOptFlags,
    ) -> PolarsResult<DataFrame> {
        self.lf
            .with_optimizations(optimizations.into_opt_flags())
            .collect()
    }

    pub fn profile(self) -> PolarsResult<(DataFrame, DataFrame)> {
        self.lf.profile()
    }

    pub fn describe(
        self,
        _percentiles: Option<Vec<f64>>,
        _interpolation: QuantileMethod,
    ) -> PolarsResult<DataFrame> {
        self.collect()
    }

    pub fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    pub fn pipe_with_schema<F, R>(mut self, f: F) -> PolarsResult<R>
    where
        F: FnOnce(Self, Schema) -> R,
    {
        let schema = self.schema()?;
        Ok(f(self, schema))
    }

    pub fn map_batches<F>(self, function: F) -> PolarsResult<Self>
    where
        F: Fn(DataFrame) -> PolarsResult<DataFrame>,
    {
        let out = function(self.collect()?)?;
        Ok(Self::from_dataframe(out))
    }

    pub fn inspect(self, fmt: &str) -> PolarsResult<Self> {
        self.map_batches(|df| {
            println!("{}", fmt.replace("{}", &format!("{df:?}")));
            Ok(df)
        })
    }

    pub fn serialize(self) -> PolarsResult<Vec<u8>> {
        let df = self.collect()?;
        bincode::serialize(&df).map_err(|error| {
            polars::error::PolarsError::ComputeError(
                format!("lazyframe serialize fallback failed: {error}").into(),
            )
        })
    }

    pub fn deserialize(source: &[u8]) -> PolarsResult<Self> {
        let df: DataFrame = bincode::deserialize(source).map_err(|error| {
            polars::error::PolarsError::ComputeError(
                format!("lazyframe deserialize fallback failed: {error}").into(),
            )
        })?;
        Ok(Self::from_dataframe(df))
    }

    pub fn collect_schema(&mut self) -> PolarsResult<SchemaRef> {
        self.lf.collect_schema()
    }

    pub fn select_exprs(self, exprs: Vec<Expr>) -> Self {
        Self {
            lf: self.lf.select(exprs),
        }
    }

    pub fn select_seq_exprs(self, exprs: Vec<Expr>) -> Self {
        Self {
            lf: self.lf.select_seq(exprs),
        }
    }

    pub fn select(
        self,
        inputs: Vec<ExprInput>,
        named_inputs: Option<HashMap<String, ExprInput>>,
    ) -> PolarsResult<Self> {
        let df = lazyframe_schema_df(&self.lf)?;
        let exprs = parse_into_list_of_expressions_for_df(
            &df,
            &inputs,
            named_inputs.as_ref(),
            ParseExprOptions::default(),
        )?;
        Ok(Self {
            lf: self.lf.select(exprs),
        })
    }

    pub fn select_seq(
        self,
        inputs: Vec<ExprInput>,
        named_inputs: Option<HashMap<String, ExprInput>>,
    ) -> PolarsResult<Self> {
        let df = lazyframe_schema_df(&self.lf)?;
        let exprs = parse_into_list_of_expressions_for_df(
            &df,
            &inputs,
            named_inputs.as_ref(),
            ParseExprOptions::default(),
        )?;
        Ok(Self {
            lf: self.lf.select_seq(exprs),
        })
    }

    pub fn with_columns_exprs(self, exprs: Vec<Expr>) -> Self {
        Self {
            lf: self.lf.with_columns(exprs),
        }
    }

    pub fn with_columns_seq_exprs(self, exprs: Vec<Expr>) -> Self {
        Self {
            lf: self.lf.with_columns_seq(exprs),
        }
    }

    pub fn with_columns(
        self,
        inputs: Vec<ExprInput>,
        named_inputs: Option<HashMap<String, ExprInput>>,
    ) -> PolarsResult<Self> {
        let df = lazyframe_schema_df(&self.lf)?;
        let exprs = parse_into_list_of_expressions_for_df(
            &df,
            &inputs,
            named_inputs.as_ref(),
            ParseExprOptions::default(),
        )?;
        Ok(Self {
            lf: self.lf.with_columns(exprs),
        })
    }

    pub fn with_columns_seq(
        self,
        inputs: Vec<ExprInput>,
        named_inputs: Option<HashMap<String, ExprInput>>,
    ) -> PolarsResult<Self> {
        let df = lazyframe_schema_df(&self.lf)?;
        let exprs = parse_into_list_of_expressions_for_df(
            &df,
            &inputs,
            named_inputs.as_ref(),
            ParseExprOptions::default(),
        )?;
        Ok(Self {
            lf: self.lf.with_columns_seq(exprs),
        })
    }

    pub fn filter(self, expr: Expr) -> Self {
        Self {
            lf: self.lf.filter(expr),
        }
    }

    pub fn remove(self, expr: Expr) -> Self {
        Self {
            lf: self.lf.remove(expr),
        }
    }

    pub fn drop(self, columns: &[&str], strict: bool) -> Self {
        Self {
            lf: self.lf.drop(selector_from_names(columns, strict)),
        }
    }

    pub fn explode(self, columns: &[&str], strict: bool) -> Self {
        Self {
            lf: self.lf.explode(selector_from_names(columns, strict)),
        }
    }

    pub fn join(
        self,
        other: GDSLazyFrame,
        left_on: Vec<Expr>,
        right_on: Vec<Expr>,
        args: JoinArgs,
    ) -> Self {
        Self {
            lf: self
                .lf
                .join(other.into_lazyframe(), left_on, right_on, args),
        }
    }

    pub fn join_asof(
        self,
        other: GDSLazyFrame,
        left_on: Vec<Expr>,
        right_on: Vec<Expr>,
        options: AsOfOptions,
    ) -> Self {
        let args = JoinArgs::new(polars::prelude::JoinType::AsOf(Box::new(options)));
        self.join(other, left_on, right_on, args)
    }

    pub fn join_where(
        self,
        other: GDSLazyFrame,
        left_on: Vec<Expr>,
        right_on: Vec<Expr>,
        predicates: Vec<Expr>,
    ) -> Self {
        Self {
            lf: self
                .lf
                .join_builder()
                .with(other.into_lazyframe())
                .left_on(left_on)
                .right_on(right_on)
                .join_where(predicates),
        }
    }

    pub fn rename(self, existing: &[&str], new: &[&str], strict: bool) -> Self {
        Self {
            lf: self
                .lf
                .rename(existing.iter().copied(), new.iter().copied(), strict),
        }
    }

    pub fn top_k(self, k: IdxSize, by_exprs: Vec<Expr>, sort_options: SortMultipleOptions) -> Self {
        Self {
            lf: self.lf.top_k(k, by_exprs, sort_options),
        }
    }

    pub fn bottom_k(
        self,
        k: IdxSize,
        by_exprs: Vec<Expr>,
        sort_options: SortMultipleOptions,
    ) -> Self {
        Self {
            lf: self.lf.bottom_k(k, by_exprs, sort_options),
        }
    }

    pub fn reverse(self) -> Self {
        Self {
            lf: self.lf.reverse(),
        }
    }

    pub fn slice(self, offset: i64, len: IdxSize) -> Self {
        Self {
            lf: self.lf.slice(offset, len),
        }
    }

    pub fn first(self) -> Self {
        Self {
            lf: self.lf.first(),
        }
    }

    pub fn last(self) -> Self {
        Self { lf: self.lf.last() }
    }

    pub fn head(self, n: Option<usize>) -> Self {
        let n = n.unwrap_or(5) as IdxSize;
        Self {
            lf: self.lf.limit(n),
        }
    }

    pub fn tail(self, n: Option<usize>) -> Self {
        let n = n.unwrap_or(5) as IdxSize;
        Self {
            lf: self.lf.tail(n),
        }
    }

    pub fn cache(self) -> Self {
        Self {
            lf: self.lf.cache(),
        }
    }

    pub fn shift(self, n: Expr) -> Self {
        Self {
            lf: self.lf.shift(n),
        }
    }

    pub fn fill_null(self, fill_value: Expr) -> Self {
        Self {
            lf: self.lf.fill_null(fill_value),
        }
    }

    pub fn fill_nan(self, fill_value: Expr) -> Self {
        Self {
            lf: self.lf.fill_nan(fill_value),
        }
    }

    pub fn max(self) -> Self {
        Self { lf: self.lf.max() }
    }

    pub fn approx_n_unique(self) -> Self {
        Self {
            lf: self.lf.select([col("*").n_unique()]),
        }
    }

    pub fn min(self) -> Self {
        Self { lf: self.lf.min() }
    }

    pub fn sum(self) -> Self {
        Self { lf: self.lf.sum() }
    }

    pub fn mean(self) -> Self {
        Self { lf: self.lf.mean() }
    }

    pub fn median(self) -> Self {
        Self {
            lf: self.lf.median(),
        }
    }

    pub fn quantile(self, quantile: Expr, method: QuantileMethod) -> Self {
        Self {
            lf: self.lf.quantile(quantile, method),
        }
    }

    pub fn std(self, ddof: u8) -> Self {
        Self {
            lf: self.lf.std(ddof),
        }
    }

    pub fn var(self, ddof: u8) -> Self {
        Self {
            lf: self.lf.var(ddof),
        }
    }

    pub fn null_count(self) -> Self {
        Self {
            lf: self.lf.null_count(),
        }
    }

    pub fn count(self) -> Self {
        Self {
            lf: self.lf.count(),
        }
    }

    pub fn gather_every(self, n: usize, offset: usize) -> Self {
        Self {
            lf: self.lf.select([col("*").gather_every(n, offset)]),
        }
    }

    pub fn drop_nans(self) -> Self {
        Self {
            lf: self.lf.drop_nans(None),
        }
    }

    pub fn drop_nulls(self) -> Self {
        Self {
            lf: self.lf.drop_nulls(None),
        }
    }

    pub fn interpolate(self) -> Self {
        Self {
            lf: self
                .lf
                .select([col("*").interpolate(InterpolationMethod::Linear)]),
        }
    }

    pub fn unique(self, subset: Option<&[&str]>, keep_strategy: UniqueKeepStrategy) -> Self {
        let subset = subset.map(|names| selector_from_names(names, false));
        Self {
            lf: self.lf.unique(subset, keep_strategy),
        }
    }

    pub fn unique_stable(self, subset: Option<&[&str]>, keep_strategy: UniqueKeepStrategy) -> Self {
        let subset = subset.map(|names| selector_from_names(names, false));
        Self {
            lf: self.lf.unique_stable(subset, keep_strategy),
        }
    }

    pub fn sort(self, columns: Vec<impl Into<PlSmallStr>>, options: SortMultipleOptions) -> Self {
        let columns: Vec<PlSmallStr> = columns.into_iter().map(Into::into).collect();
        Self {
            lf: self.lf.sort(columns, options),
        }
    }

    pub fn sort_by_exprs(self, exprs: Vec<Expr>, options: SortMultipleOptions) -> Self {
        Self {
            lf: self.lf.sort_by_exprs(exprs, options),
        }
    }

    pub fn limit(self, n: IdxSize) -> Self {
        Self {
            lf: self.lf.limit(n),
        }
    }

    pub fn with_row_index(self, name: &str, offset: Option<IdxSize>) -> Self {
        Self {
            lf: self.lf.with_row_index(name, offset),
        }
    }

    pub fn with_row_count(self, name: &str, offset: Option<IdxSize>) -> Self {
        self.with_row_index(name, offset)
    }

    pub fn prepare_missing_columns(self) -> Self {
        self
    }

    pub fn set_sorted(
        self,
        columns: Vec<&str>,
        descending: Vec<bool>,
        nulls_last: Vec<bool>,
    ) -> PolarsResult<Self> {
        if columns.is_empty() {
            return Ok(self);
        }

        let len = columns.len();
        let descending = normalize_sort_bools(descending, len, "descending")?;
        let nulls_last = normalize_sort_bools(nulls_last, len, "nulls_last")?;

        let hints = columns
            .into_iter()
            .zip(descending)
            .zip(nulls_last)
            .map(|((column, descending), nulls_last)| SortedHint {
                column: PlSmallStr::from(column),
                descending,
                nulls_last,
            })
            .collect::<Vec<_>>();

        Ok(Self {
            lf: self.lf.hint(HintIR::Sorted(Arc::from(hints)))?,
        })
    }

    pub fn group_by_dynamic(
        self,
        index_column: Expr,
        by: Vec<Expr>,
        options: DynamicGroupOptions,
    ) -> GDSLazyGroupBy {
        GDSLazyGroupBy::new(self.lf.group_by_dynamic(index_column, by, options))
    }

    pub fn rolling(
        self,
        index_column: Expr,
        group_by: Vec<Expr>,
        options: RollingGroupOptions,
    ) -> GDSLazyGroupBy {
        GDSLazyGroupBy::new(self.lf.rolling(index_column, group_by, options))
    }

    pub fn unnest(self, columns: &[&str], separator: Option<&str>) -> Self {
        let separator = separator.map(PlSmallStr::from);
        Self {
            lf: self
                .lf
                .unnest(selector_from_names(columns, true), separator),
        }
    }

    pub fn unpivot(
        self,
        on: &[&str],
        index: &[&str],
        variable_name: Option<&str>,
        value_name: Option<&str>,
    ) -> PolarsResult<Self> {
        let args = polars::prelude::UnpivotArgsIR {
            on: on.iter().map(|name| PlSmallStr::from(*name)).collect(),
            index: index.iter().map(|name| PlSmallStr::from(*name)).collect(),
            variable_name: variable_name.map(PlSmallStr::from),
            value_name: value_name.map(PlSmallStr::from),
        };
        let df = self.lf.collect()?;
        let out = df.unpivot2(args)?;
        Ok(Self { lf: out.lazy() })
    }

    pub fn pivot(
        self,
        _on: &[&str],
        _on_columns: DataFrame,
        _index: Option<&[&str]>,
        _values: Option<&[&str]>,
        _aggregate_function: Option<Expr>,
        _maintain_order: bool,
        _separator: &str,
    ) -> PolarsResult<Self> {
        Err(polars::error::PolarsError::ComputeError(
            "lazy pivot is not available in this Rust wrapper path yet; use eager DataFrame pivot fallback".into(),
        ))
    }

    pub fn melt(
        self,
        id_vars: &[&str],
        value_vars: &[&str],
        variable_name: Option<&str>,
        value_name: Option<&str>,
        _streamable: bool,
    ) -> PolarsResult<Self> {
        self.unpivot(value_vars, id_vars, variable_name, value_name)
    }

    pub fn sink_parquet(
        self,
        target: SinkTarget,
        options: ParquetWriteOptions,
        cloud_options: Option<polars_io::cloud::CloudOptions>,
        sink_options: SinkOptions,
    ) -> PolarsResult<Self> {
        Ok(Self {
            lf: self
                .lf
                .sink_parquet(target, options, cloud_options, sink_options)?,
        })
    }

    pub fn sink_ipc(
        self,
        target: SinkTarget,
        options: IpcWriterOptions,
        cloud_options: Option<polars_io::cloud::CloudOptions>,
        sink_options: SinkOptions,
    ) -> PolarsResult<Self> {
        Ok(Self {
            lf: self
                .lf
                .sink_ipc(target, options, cloud_options, sink_options)?,
        })
    }

    pub fn sink_csv(
        self,
        target: SinkTarget,
        options: CsvWriterOptions,
        cloud_options: Option<polars_io::cloud::CloudOptions>,
        sink_options: SinkOptions,
    ) -> PolarsResult<Self> {
        Ok(Self {
            lf: self
                .lf
                .sink_csv(target, options, cloud_options, sink_options)?,
        })
    }

    pub fn sink_ndjson(
        self,
        target: SinkTarget,
        options: JsonWriterOptions,
        cloud_options: Option<polars_io::cloud::CloudOptions>,
        sink_options: SinkOptions,
    ) -> PolarsResult<Self> {
        Ok(Self {
            lf: self
                .lf
                .sink_json(target, options, cloud_options, sink_options)?,
        })
    }

    pub fn sink_delta(self, _target: &str) -> PolarsResult<Self> {
        Err(polars::error::PolarsError::ComputeError(
            "sink_delta is not available in this Rust wrapper path yet".into(),
        ))
    }

    pub fn sink_batches(
        self,
        function: polars::prelude::PlanCallback<DataFrame, bool>,
        maintain_order: bool,
        chunk_size: Option<NonZeroUsize>,
    ) -> PolarsResult<Self> {
        Ok(Self {
            lf: self.lf.sink_batches(function, maintain_order, chunk_size)?,
        })
    }

    pub fn collect_batches(
        self,
        maintain_order: bool,
        chunk_size: Option<NonZeroUsize>,
    ) -> PolarsResult<Vec<DataFrame>> {
        let batches = Arc::new(Mutex::new(Vec::<DataFrame>::new()));
        let sink_batches = Arc::clone(&batches);
        let callback = polars::prelude::PlanCallback::Rust(SpecialEq::new(Arc::new(move |df| {
            let mut guard = sink_batches.lock().map_err(|_| {
                polars::error::PolarsError::ComputeError(
                    "collect_batches callback mutex poisoned".into(),
                )
            })?;
            guard.push(df);
            Ok(true)
        })));

        let _ = self.lf.sink_batches(callback, maintain_order, chunk_size)?;

        let out = Arc::try_unwrap(batches)
            .map_err(|_| {
                polars::error::PolarsError::ComputeError(
                    "collect_batches internal callback state still shared".into(),
                )
            })?
            .into_inner()
            .map_err(|_| {
                polars::error::PolarsError::ComputeError(
                    "collect_batches internal callback mutex poisoned".into(),
                )
            })?;
        Ok(out)
    }

    pub fn match_to_schema(
        self,
        schema: SchemaRef,
        per_column: Vec<MatchToSchemaPerColumn>,
        extra_columns: ExtraColumnsPolicy,
    ) -> Self {
        Self {
            lf: self
                .lf
                .match_to_schema(schema, Arc::from(per_column), extra_columns),
        }
    }

    pub fn with_context(self, contexts: Vec<GDSLazyFrame>) -> Self {
        let contexts = contexts
            .into_iter()
            .map(GDSLazyFrame::into_lazyframe)
            .collect::<Vec<_>>();
        Self {
            lf: self.lf.with_context(contexts),
        }
    }

    pub fn merge_sorted(
        self,
        other: GDSLazyFrame,
        key: impl Into<PlSmallStr>,
    ) -> PolarsResult<Self> {
        Ok(Self {
            lf: self.lf.merge_sorted(other.into_lazyframe(), key)?,
        })
    }

    pub fn remote(self) -> PolarsResult<Self> {
        Err(polars::error::PolarsError::ComputeError(
            "remote execution is not available in this Rust wrapper path yet".into(),
        ))
    }

    pub fn sql(self, _query: &str) -> PolarsResult<Self> {
        Err(polars::error::PolarsError::ComputeError(
            "lazyframe.sql is not available in this Rust wrapper path yet".into(),
        ))
    }

    pub fn skip_minmax(&self, dtype: &DataType) -> bool {
        matches!(dtype, DataType::Null | DataType::Unknown(_)) || dtype.is_nested()
    }

    pub fn group_by(self, by: Vec<Expr>, maintain_order: bool) -> GDSLazyGroupBy {
        let group_by = if maintain_order {
            self.lf.group_by_stable(by)
        } else {
            self.lf.group_by(by)
        };
        GDSLazyGroupBy::new(group_by)
    }
}

fn normalize_sort_bools(values: Vec<bool>, len: usize, name: &str) -> PolarsResult<Vec<bool>> {
    if values.is_empty() {
        return Ok(vec![false; len]);
    }
    if values.len() == 1 {
        return Ok(vec![values[0]; len]);
    }
    if values.len() != len {
        return Err(polars::error::PolarsError::ComputeError(
            format!("{name} must have length 1 or match columns length ({len})").into(),
        ));
    }
    Ok(values)
}

impl From<LazyFrame> for GDSLazyFrame {
    fn from(lf: LazyFrame) -> Self {
        Self::new(lf)
    }
}

impl From<DataFrame> for GDSLazyFrame {
    fn from(df: DataFrame) -> Self {
        Self::from_dataframe(df)
    }
}

impl From<GDSLazyFrame> for LazyFrame {
    fn from(wrapper: GDSLazyFrame) -> Self {
        wrapper.lf
    }
}

/// Polars LazyGroupBy wrapper for the Collections SDK.
pub struct GDSLazyGroupBy {
    group_by: polars::prelude::LazyGroupBy,
}

impl GDSLazyGroupBy {
    pub fn new(group_by: polars::prelude::LazyGroupBy) -> Self {
        Self { group_by }
    }

    pub fn agg(self, aggs: Vec<Expr>) -> GDSLazyFrame {
        GDSLazyFrame::new(self.group_by.agg(aggs))
    }

    pub fn head(self, n: Option<usize>) -> GDSLazyFrame {
        GDSLazyFrame::new(self.group_by.head(n))
    }

    pub fn tail(self, n: Option<usize>) -> GDSLazyFrame {
        GDSLazyFrame::new(self.group_by.tail(n))
    }
}

#[cfg(test)]
mod tests {
    use polars::prelude::{col, Column, DataFrame, IntoLazy, NamedFrom, Series};

    use super::GDSLazyFrame;
    use crate::collections::dataframe::utils::parse::ExprInput;

    #[test]
    fn lazyframe_select_parity() {
        let df = DataFrame::new(vec![
            Column::from(Series::new("a".into(), &[1i64, 2, 3])),
            Column::from(Series::new("b".into(), &[10i64, 20, 30])),
        ])
        .expect("dataframe");

        let out = GDSLazyFrame::from_dataframe(df.clone())
            .select(vec![ExprInput::column("a")], None)
            .expect("select")
            .collect()
            .expect("collect");

        let expected = df
            .lazy()
            .select([col("a")])
            .collect()
            .expect("collect expected");

        assert_eq!(out.shape(), expected.shape());
        assert_eq!(out.column("a").unwrap(), expected.column("a").unwrap());
    }
}
