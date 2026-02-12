//! Binary namespace for expressions (py-polars inspired).

use polars::prelude::{lit, DataType, DataTypeExpr, Expr, PolarsError, PolarsResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryEncoding {
    Hex,
    Base64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryEndianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinarySizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
}

impl BinarySizeUnit {
    fn scale_factor(self) -> f64 {
        match self {
            BinarySizeUnit::Bytes => 1.0,
            BinarySizeUnit::Kilobytes => 1024.0,
            BinarySizeUnit::Megabytes => 1024.0 * 1024.0,
            BinarySizeUnit::Gigabytes => 1024.0 * 1024.0 * 1024.0,
            BinarySizeUnit::Terabytes => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprBinary {
    expr: Expr,
}

impl ExprBinary {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn contains(self, literal: &[u8]) -> Expr {
        self.expr.binary().contains_literal(lit(literal.to_vec()))
    }

    pub fn contains_expr(self, literal: Expr) -> Expr {
        self.expr.binary().contains_literal(literal)
    }

    pub fn ends_with(self, suffix: &[u8]) -> Expr {
        self.expr.binary().ends_with(lit(suffix.to_vec()))
    }

    pub fn ends_with_expr(self, suffix: Expr) -> Expr {
        self.expr.binary().ends_with(suffix)
    }

    pub fn starts_with(self, prefix: &[u8]) -> Expr {
        self.expr.binary().starts_with(lit(prefix.to_vec()))
    }

    pub fn starts_with_expr(self, prefix: Expr) -> Expr {
        self.expr.binary().starts_with(prefix)
    }

    pub fn size_bytes(self) -> Expr {
        self.expr.binary().size_bytes()
    }

    pub fn size(self, unit: BinarySizeUnit) -> Expr {
        let size = self.size_bytes();
        if unit == BinarySizeUnit::Bytes {
            size
        } else {
            size.cast(DataType::Float64) / lit(unit.scale_factor())
        }
    }

    pub fn decode(self, encoding: BinaryEncoding, strict: bool) -> Expr {
        match encoding {
            BinaryEncoding::Hex => self.expr.binary().hex_decode(strict),
            BinaryEncoding::Base64 => self.expr.binary().base64_decode(strict),
        }
    }

    pub fn encode(self, encoding: BinaryEncoding) -> Expr {
        match encoding {
            BinaryEncoding::Hex => self.expr.binary().hex_encode(),
            BinaryEncoding::Base64 => self.expr.binary().base64_encode(),
        }
    }

    pub fn reinterpret(self, dtype: impl Into<DataTypeExpr>, endianness: BinaryEndianness) -> Expr {
        let is_little_endian = matches!(endianness, BinaryEndianness::Little);
        self.expr
            .binary()
            .reinterpret(dtype.into(), is_little_endian)
    }

    pub fn slice(self, _offset: i64, _length: i64) -> PolarsResult<Expr> {
        Err(PolarsError::ComputeError(
            "binary.slice is not supported in Polars 0.52 expression API".into(),
        ))
    }

    pub fn head(self, _n: i64) -> PolarsResult<Expr> {
        Err(PolarsError::ComputeError(
            "binary.head is not supported in Polars 0.52 expression API".into(),
        ))
    }

    pub fn tail(self, _n: i64) -> PolarsResult<Expr> {
        Err(PolarsError::ComputeError(
            "binary.tail is not supported in Polars 0.52 expression API".into(),
        ))
    }
}
