use crate::baseline::model::BaselineFile;
use crate::types::{ColumnProfile, DatasetProfile};

#[derive(Debug, Default)]
pub struct DatasetDelta {
    pub row_count_delta: Option<i64>,
}

#[derive(Debug, Default)]
pub struct ColumnDelta {
    pub null_ratio_delta: Option<f64>,
    pub top_ratio_delta: Option<f64>,
    pub outlier_ratio_delta: Option<f64>,
}

pub fn dataset_delta(current: &DatasetProfile, baseline: &BaselineFile) -> DatasetDelta {
    let mut delta = DatasetDelta::default();
    let base_rows = baseline.dataset.row_count as i64;
    delta.row_count_delta = Some(current.row_count as i64 - base_rows);
    delta
}

pub fn column_delta(current: &ColumnProfile, baseline: &ColumnProfile) -> ColumnDelta {
    let mut delta = ColumnDelta::default();

    delta.null_ratio_delta = Some(current.null_ratio - baseline.null_ratio);

    if let (Some(cur), Some(base)) = (
        current
            .categorical
            .as_ref()
            .and_then(|c| c.top_value_ratio),
        baseline
            .categorical
            .as_ref()
            .and_then(|c| c.top_value_ratio),
    ) {
        delta.top_ratio_delta = Some(cur - base);
    }

    if let (Some(cur), Some(base)) = (
        current
            .numeric
            .as_ref()
            .and_then(|c| c.outlier_ratio),
        baseline
            .numeric
            .as_ref()
            .and_then(|c| c.outlier_ratio),
    ) {
        delta.outlier_ratio_delta = Some(cur - base);
    }

    delta
}
