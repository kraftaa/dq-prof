use std::path::Path;

use polars::lazy::frame::LazyFrame;
use polars::prelude::*;

use crate::error::{Error, Result};
use crate::io::sampling::{apply_frame_sampling, SamplingDecision};

pub fn read_csv(path: &Path, sampling: &SamplingDecision) -> Result<(DataFrame, usize, bool)> {
    let lazy = LazyCsvReader::new(path.to_string_lossy().to_string())
        .with_infer_schema_length(Some(1000))
        .with_ignore_errors(true)
        .finish()
        .map_err(|e| Error::Io(e.to_string()))?;

    collect_with_sampling(lazy, sampling)
}

fn collect_with_sampling(
    lf: LazyFrame,
    sampling: &SamplingDecision,
) -> Result<(DataFrame, usize, bool)> {
    use polars::lazy::dsl::len;

    let total_rows_df = lf
        .clone()
        .select([len().alias("row_count")])
        .collect()
        .map_err(|e| Error::Io(e.to_string()))?;
    let total_rows = total_rows_df
        .column("row_count")
        .ok()
        .and_then(|c| c.get(0).ok())
        .and_then(|v| v.to_string().parse::<usize>().ok())
        .unwrap_or(0);

    let mut df = lf.collect().map_err(|e| Error::Io(e.to_string()))?;
    let was_sampled = !sampling.full_scan;
    apply_frame_sampling(&mut df, sampling, total_rows)?;

    Ok((df, total_rows, was_sampled))
}
