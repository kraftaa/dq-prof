use polars::prelude::*;

use crate::profile::{categorical, datetime, numeric, schema};
use crate::types::{ColumnProfile, LogicalType};

pub fn profile_columns(
    df: &DataFrame,
    max_top_values: usize,
) -> crate::error::Result<Vec<ColumnProfile>> {
    let mut profiles = Vec::with_capacity(df.width());
    let row_count = df.height() as f64;

    for name in df.get_column_names() {
        let series = df.column(name).map_err(|e| crate::error::Error::Profile(e.to_string()))?;
        let logical_type = schema::infer_logical_type(series.dtype());

        let null_ratio = series.null_count() as f64 / row_count.max(1.0);
        let distinct_count = series.n_unique().ok();
        let distinct_ratio = distinct_count.map(|c| c as f64 / row_count.max(1.0));

        let (numeric, categorical, datetime_prof) = match logical_type {
            LogicalType::Numeric => (
                Some(numeric::profile_numeric(series)),
                None,
                None,
            ),
            LogicalType::Categorical | LogicalType::Boolean => (
                None,
                Some(categorical::profile_categorical(series, max_top_values)),
                None,
            ),
            LogicalType::Datetime => (None, None, Some(datetime::profile_datetime(series))),
            _ => (None, None, None),
        };

        profiles.push(ColumnProfile {
            name: name.to_string(),
            logical_type,
            null_ratio,
            distinct_count,
            distinct_ratio,
            numeric,
            categorical,
            datetime: datetime_prof,
        });
    }

    Ok(profiles)
}
