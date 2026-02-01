//! DataFrame interchange wrapper for Polars DataFrame.

use polars::prelude::DataFrame;

use crate::collections::dataframe::interchange::column::PolarsInterchangeColumn;
use crate::collections::dataframe::interchange::protocol::{
    Column, DataFrame as InterchangeDataFrame, InterchangeError, MetadataMap, SupportsInterchange,
};

#[derive(Clone)]
pub struct PolarsInterchangeDataFrame {
    df: DataFrame,
    allow_copy: bool,
}

impl PolarsInterchangeDataFrame {
    pub fn new(df: DataFrame, allow_copy: bool) -> Self {
        Self { df, allow_copy }
    }

    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }
}

impl InterchangeDataFrame for PolarsInterchangeDataFrame {
    fn version(&self) -> u8 {
        0
    }

    fn metadata(&self) -> Result<MetadataMap, InterchangeError> {
        Ok(MetadataMap::new())
    }

    fn num_columns(&self) -> Result<usize, InterchangeError> {
        Ok(self.df.width())
    }

    fn num_rows(&self) -> Result<Option<usize>, InterchangeError> {
        Ok(Some(self.df.height()))
    }

    fn num_chunks(&self) -> Result<usize, InterchangeError> {
        Ok(self.df.first_col_n_chunks())
    }

    fn column_names(&self) -> Result<Vec<String>, InterchangeError> {
        Ok(self
            .df
            .get_column_names_str()
            .iter()
            .map(|name| name.to_string())
            .collect())
    }

    fn get_column(&self, index: usize) -> Result<Box<dyn Column>, InterchangeError> {
        let series = self
            .df
            .select_at_idx(index)
            .and_then(|column| column.as_series())
            .ok_or_else(|| {
                InterchangeError::InvalidArgument(format!("column index {index} out of bounds"))
            })?;
        Ok(Box::new(PolarsInterchangeColumn::new(
            series.clone(),
            self.allow_copy,
        )) as Box<dyn Column>)
    }

    fn get_column_by_name(&self, name: &str) -> Result<Box<dyn Column>, InterchangeError> {
        let series = self.df.column(name)?.as_series().ok_or_else(|| {
            InterchangeError::InvalidArgument(format!("column {name} is not a Series"))
        })?;
        Ok(Box::new(PolarsInterchangeColumn::new(
            series.clone(),
            self.allow_copy,
        )) as Box<dyn Column>)
    }

    fn get_columns(&self) -> Result<Vec<Box<dyn Column>>, InterchangeError> {
        Ok(self
            .df
            .get_columns()
            .iter()
            .filter_map(|column| column.as_series())
            .map(|series| {
                Box::new(PolarsInterchangeColumn::new(
                    series.clone(),
                    self.allow_copy,
                )) as Box<dyn Column>
            })
            .collect())
    }

    fn select_columns(
        &self,
        indices: &[usize],
    ) -> Result<Box<dyn InterchangeDataFrame>, InterchangeError> {
        let names = self.df.get_column_names_str();
        let mut selected = Vec::with_capacity(indices.len());
        for &index in indices {
            let name = names.get(index).ok_or_else(|| {
                InterchangeError::InvalidArgument(format!("column index {index} out of bounds"))
            })?;
            selected.push(*name);
        }
        let df = self.df.select(selected)?;
        Ok(Box::new(PolarsInterchangeDataFrame::new(
            df,
            self.allow_copy,
        )))
    }

    fn select_columns_by_name(
        &self,
        names: &[&str],
    ) -> Result<Box<dyn InterchangeDataFrame>, InterchangeError> {
        let selection: Vec<&str> = names.iter().copied().collect();
        let df = self.df.select(selection)?;
        Ok(Box::new(PolarsInterchangeDataFrame::new(
            df,
            self.allow_copy,
        )))
    }

    fn get_chunks(
        &self,
        n_chunks: Option<usize>,
    ) -> Result<Vec<Box<dyn InterchangeDataFrame>>, InterchangeError> {
        let total_n_chunks = self.df.first_col_n_chunks();
        let mut chunks = Vec::new();

        if self.df.width() == 0 {
            chunks.push(Box::new(self.clone()) as Box<dyn InterchangeDataFrame>);
            return Ok(chunks);
        }

        let first_col = self
            .df
            .select_at_idx(0)
            .and_then(|column| column.as_series())
            .ok_or_else(|| {
                InterchangeError::InvalidArgument("dataframe has no columns".to_string())
            })?;
        let chunk_sizes: Vec<usize> = first_col.chunks().iter().map(|chunk| chunk.len()).collect();

        if n_chunks.is_none() || n_chunks == Some(total_n_chunks) {
            let mut offset = 0usize;
            for size in chunk_sizes {
                let mut chunk = self.df.slice(offset as i64, size);
                if chunk.max_n_chunks() > 1 {
                    if !self.allow_copy {
                        return Err(InterchangeError::CopyNotAllowed(
                            "unevenly chunked columns must be rechunked".to_string(),
                        ));
                    }
                    chunk.align_chunks();
                }
                chunks.push(
                    Box::new(PolarsInterchangeDataFrame::new(chunk, self.allow_copy))
                        as Box<dyn InterchangeDataFrame>,
                );
                offset += size;
            }
            return Ok(chunks);
        }

        let n_chunks = n_chunks.unwrap_or(total_n_chunks);
        if n_chunks == 0 || n_chunks % total_n_chunks != 0 {
            return Err(InterchangeError::InvalidArgument(format!(
                "n_chunks must be a multiple of the number of chunks ({total_n_chunks})"
            )));
        }

        let subchunks_per_chunk = n_chunks / total_n_chunks;
        let mut offset = 0usize;
        for size in chunk_sizes {
            let mut step = size / subchunks_per_chunk;
            if size % subchunks_per_chunk != 0 {
                step += 1;
            }
            for start in (0..step * subchunks_per_chunk).step_by(step) {
                let end = (start + step).min(size);
                if start >= end {
                    continue;
                }
                let mut chunk = self.df.slice((offset + start) as i64, end - start);
                if chunk.max_n_chunks() > 1 {
                    if !self.allow_copy {
                        return Err(InterchangeError::CopyNotAllowed(
                            "unevenly chunked columns must be rechunked".to_string(),
                        ));
                    }
                    chunk.align_chunks();
                }
                chunks.push(
                    Box::new(PolarsInterchangeDataFrame::new(chunk, self.allow_copy))
                        as Box<dyn InterchangeDataFrame>,
                );
            }
            offset += size;
        }

        Ok(chunks)
    }
}

impl SupportsInterchange for DataFrame {
    fn dataframe_interchange(
        &self,
        nan_as_null: bool,
        allow_copy: bool,
    ) -> Result<Box<dyn InterchangeDataFrame>, InterchangeError> {
        if nan_as_null {
            return Err(InterchangeError::NotImplemented(
                "nan_as_null is not implemented".to_string(),
            ));
        }
        Ok(Box::new(PolarsInterchangeDataFrame::new(
            self.clone(),
            allow_copy,
        )))
    }
}
