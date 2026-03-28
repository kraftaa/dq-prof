use chrono::{TimeZone, Utc};
use polars::prelude::*;

use crate::types::DateTimeProfile;

pub fn profile_datetime(series: &Series) -> DateTimeProfile {
    let dt_series = match series.dtype() {
        DataType::Datetime(_, _) => series.clone(),
        DataType::Date => series
            .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))
            .unwrap_or_else(|_| series.clone()),
        _ => series.clone(),
    };

    let ca = match dt_series.datetime() {
        Ok(ca) => ca,
        Err(_) => return empty_datetime_profile(),
    };
    let min = ca.min();
    let max = ca.max();
    let now_ms = Utc::now().timestamp_millis();

    let min_ts = min.and_then(|v| Utc.timestamp_millis_opt(v).single()).map(|dt| dt.to_rfc3339());
    let max_ts = max.and_then(|v| Utc.timestamp_millis_opt(v).single()).map(|dt| dt.to_rfc3339());

    let freshness_lag_seconds = max.map(|v| (now_ms - v) / 1000);

    let mut future = 0usize;
    let mut total = 0usize;
    for val in ca.into_iter().flatten() {
        total += 1;
        if val > now_ms {
            future += 1;
        }
    }
    let future_ratio = if total > 0 {
        Some(future as f64 / total as f64)
    } else {
        None
    };

    DateTimeProfile {
        min_ts,
        max_ts,
        freshness_lag_seconds,
        future_ratio,
    }
}

fn empty_datetime_profile() -> DateTimeProfile {
    DateTimeProfile {
        min_ts: None,
        max_ts: None,
        freshness_lag_seconds: None,
        future_ratio: None,
    }
}
