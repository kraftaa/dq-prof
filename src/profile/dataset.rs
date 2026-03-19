use crate::error::Result;
use crate::io::reader::LoadedData;
use crate::profile::{column, fingerprint};
use crate::types::{DatasetProfile, LogicalType};

pub fn build_dataset_profile(
    data: &LoadedData,
    max_top_values: usize,
) -> Result<DatasetProfile> {
    let columns = column::profile_columns(&data.df, max_top_values)?;

    let schema_cols: Vec<(String, LogicalType)> = columns
        .iter()
        .map(|c| (c.name.clone(), c.logical_type.clone()))
        .collect();
    let fingerprint = fingerprint::compute(&schema_cols);

    Ok(DatasetProfile {
        dataset_name: data.dataset_name.clone(),
        row_count: data.total_rows,
        sampled: data.sampled,
        sampled_rows: Some(data.sampled_rows),
        sample_mode: Some(data.sample_mode.clone()),
        column_count: data.df.width(),
        schema_fingerprint: fingerprint,
        columns,
    })
}
