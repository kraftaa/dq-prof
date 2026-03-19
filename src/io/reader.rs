use std::path::Path;

use polars::prelude::*;

use crate::error::{Error, Result};
use crate::io::{csv, parquet, postgres, sampling};
use crate::types::AnalyzeOptions;

#[derive(Debug, Clone)]
pub struct LoadedData {
    pub df: DataFrame,
    pub dataset_name: String,
    pub total_rows: usize,
    pub sampled: bool,
    pub sampled_rows: usize,
    pub sample_mode: crate::types::SampleMode,
}

pub fn read_dataset(path: &Path, opts: &AnalyzeOptions) -> Result<LoadedData> {
    let sampling_decision = sampling::decide(
        opts.sample_rows,
        opts.full_scan,
        opts.sample_mode.clone(),
        opts.sample_rate,
        opts.sample_seed,
    );
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let (df, total_rows, dataset_name, sampled) = match extension.as_str() {
        "parquet" | "pq" => {
            if !path.exists() {
                return Err(Error::Io(format!("file not found: {}", path.display())));
            }
            let (df, rows, was_sampled) = parquet::read_parquet(path, &sampling_decision)?;
            (
                df,
                rows,
                path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("dataset")
                    .to_string(),
                was_sampled,
            )
        }
        "csv" => {
            if !path.exists() {
                return Err(Error::Io(format!("file not found: {}", path.display())));
            }
            let (df, rows, was_sampled) = csv::read_csv(path, &sampling_decision)?;
            (
                df,
                rows,
                path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("dataset")
                    .to_string(),
                was_sampled,
            )
        }
        _ => {
            if let Some(pg_url) = &opts.pg_url {
                let table = opts
                    .pg_table
                    .as_deref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());
                let (df, rows, was_sampled) =
                    postgres::read_postgres(&table, pg_url, &sampling_decision)?;
                (df, rows, table, was_sampled)
            } else {
                return Err(Error::UnsupportedFile(extension.to_string()));
            }
        }
    };

    if df.height() == 0 {
        return Err(Error::EmptyDataset);
    }

    let mut df = df;
    let mut sampled_rows = df.height();
    let mut sampled_flag = sampled;

    if let Some(cap) = opts.max_rows {
        if sampled_rows > cap {
            df = df.head(Some(cap));
            sampled_rows = cap;
            sampled_flag = true;
        }
    }

    Ok(LoadedData {
        dataset_name,
        df,
        total_rows: if total_rows > 0 {
            total_rows.min(opts.max_rows.unwrap_or(total_rows))
        } else {
            sampled_rows
        },
        sampled: sampled_flag,
        sampled_rows,
        sample_mode: sampling_decision.sample_mode.clone(),
    })
}
