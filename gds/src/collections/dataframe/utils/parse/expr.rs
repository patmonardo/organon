//! Expression parsing helpers (py-polars inspired).

use std::collections::HashMap;

use polars::error::PolarsError;
use polars::lazy::dsl::all_horizontal;
use polars::prelude::{col, lit, AnyValue, DataFrame, Expr, PlSmallStr, Series};

use crate::collections::dataframe::selectors::{expand_exprs, Selector};

/// Input types that can be parsed into Polars expressions.
#[derive(Debug, Clone)]
pub enum ExprInput {
    Expr(Expr),
    /// Raw string input (interpreted as column name or literal).
    String(String),
    /// Explicit column name (always treated as a column).
    Column(String),
    Literal(AnyValue<'static>),
    Series(Series),
    Selector(Selector),
}

impl ExprInput {
    pub fn column(name: impl Into<String>) -> Self {
        Self::Column(name.into())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    pub fn literal(value: AnyValue<'static>) -> Self {
        Self::Literal(value)
    }

    pub fn series(value: Series) -> Self {
        Self::Series(value)
    }

    pub fn selector(value: Selector) -> Self {
        Self::Selector(value)
    }
}

impl From<Expr> for ExprInput {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl From<&str> for ExprInput {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ExprInput {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<AnyValue<'static>> for ExprInput {
    fn from(value: AnyValue<'static>) -> Self {
        Self::Literal(value)
    }
}

impl From<Series> for ExprInput {
    fn from(value: Series) -> Self {
        Self::Series(value)
    }
}

impl From<Selector> for ExprInput {
    fn from(value: Selector) -> Self {
        Self::Selector(value)
    }
}

/// Options controlling expression parsing behavior.
#[derive(Debug, Clone, Copy)]
pub struct ParseExprOptions {
    pub str_as_lit: bool,
    pub structify: bool,
    pub require_selector: bool,
}

impl Default for ParseExprOptions {
    fn default() -> Self {
        Self {
            str_as_lit: false,
            structify: false,
            require_selector: false,
        }
    }
}

/// Parse a single input into a Polars expression.
pub fn parse_into_expression(
    input: impl Into<ExprInput>,
    options: ParseExprOptions,
) -> Result<Expr, PolarsError> {
    match input.into() {
        ExprInput::Expr(expr) => Ok(expr),
        ExprInput::String(value) => {
            if options.str_as_lit {
                Ok(lit(value))
            } else if options.require_selector {
                Ok(col(&value))
            } else {
                Ok(col(&value))
            }
        }
        ExprInput::Column(value) => Ok(col(&value)),
        ExprInput::Literal(value) => {
            if options.require_selector {
                Err(PolarsError::ComputeError(
                    "selector required; literal provided".into(),
                ))
            } else {
                let series = Series::from_any_values(PlSmallStr::from(""), &[value], true)?;
                Ok(lit(series))
            }
        }
        ExprInput::Series(value) => {
            if options.require_selector {
                Err(PolarsError::ComputeError(
                    "selector required; series provided".into(),
                ))
            } else {
                Ok(lit(value))
            }
        }
        ExprInput::Selector(_) => Err(PolarsError::ComputeError(
            "selector requires DataFrame context".into(),
        )),
    }
    .map(|expr| if options.structify { expr } else { expr })
}

/// Parse inputs into a list of expressions.
pub fn parse_into_list_of_expressions(
    inputs: &[ExprInput],
    named_inputs: Option<&HashMap<String, ExprInput>>,
    options: ParseExprOptions,
) -> Result<Vec<Expr>, PolarsError> {
    parse_into_list_of_expressions_with_df(None, inputs, named_inputs, options)
}

/// Parse inputs into a list of expressions with DataFrame context.
pub fn parse_into_list_of_expressions_for_df(
    df: &DataFrame,
    inputs: &[ExprInput],
    named_inputs: Option<&HashMap<String, ExprInput>>,
    options: ParseExprOptions,
) -> Result<Vec<Expr>, PolarsError> {
    parse_into_list_of_expressions_with_df(Some(df), inputs, named_inputs, options)
}

fn parse_into_list_of_expressions_with_df(
    df: Option<&DataFrame>,
    inputs: &[ExprInput],
    named_inputs: Option<&HashMap<String, ExprInput>>,
    options: ParseExprOptions,
) -> Result<Vec<Expr>, PolarsError> {
    let mut exprs = Vec::new();

    for input in inputs {
        match input {
            ExprInput::Selector(selector) => {
                let df = df.ok_or_else(|| {
                    PolarsError::ComputeError("selector requires DataFrame context".into())
                })?;
                exprs.extend(expand_exprs(df, selector));
            }
            _ => exprs.push(parse_into_expression(input.clone(), options)?),
        }
    }

    if let Some(named) = named_inputs {
        for (name, input) in named {
            let expr = match input {
                ExprInput::Selector(selector) => {
                    let df = df.ok_or_else(|| {
                        PolarsError::ComputeError(
                            "selector requires DataFrame context for named input".into(),
                        )
                    })?;
                    let mut expanded = expand_exprs(df, selector);
                    if expanded.len() != 1 {
                        return Err(PolarsError::ComputeError(
                            "named selector must resolve to a single column".into(),
                        ));
                    }
                    expanded.pop().expect("selector expanded to one expression")
                }
                _ => parse_into_expression(input.clone(), options)?,
            };
            exprs.push(expr.alias(name));
        }
    }

    Ok(exprs)
}

/// Parse predicates and constraints into a single expression (AND-reduction).
pub fn parse_predicates_constraints_into_expression(
    predicates: &[ExprInput],
    constraints: &HashMap<String, ExprInput>,
) -> Result<Expr, PolarsError> {
    let mut parsed = Vec::new();

    for predicate in predicates {
        parsed.push(parse_into_expression(
            predicate.clone(),
            ParseExprOptions::default(),
        )?);
    }

    for (name, value) in constraints {
        let value_expr = parse_into_expression(
            value.clone(),
            ParseExprOptions {
                str_as_lit: true,
                ..ParseExprOptions::default()
            },
        )?;
        parsed.push(col(name).eq(value_expr));
    }

    if parsed.is_empty() {
        return Err(PolarsError::ComputeError(
            "at least one predicate or constraint must be provided".into(),
        ));
    }

    if parsed.len() == 1 {
        return Ok(parsed.remove(0));
    }

    Ok(all_horizontal(parsed)?)
}
