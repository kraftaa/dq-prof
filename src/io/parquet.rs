use std::fs::File;
use std::path::Path;

use polars::prelude::*;

use crate::error::{Error, Result};
use crate::io::sampling::{apply_frame_sampling, SamplingDecision};

pub fn read_parquet(path: &Path, sampling: &SamplingDecision) -> Result<(DataFrame, usize, bool)> {
    let file = File::open(path).map_err(|e| Error::Io(e.to_string()))?;
    let reader = ParquetReader::new(file);
    let mut df = reader.finish().map_err(|e| Error::Io(e.to_string()))?;
    let total_rows = df.height();

    let was_sampled = !sampling.full_scan;
    apply_frame_sampling(&mut df, sampling, total_rows)?;

    Ok((df, total_rows, was_sampled))
}
