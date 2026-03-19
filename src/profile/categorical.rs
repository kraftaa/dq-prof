use std::collections::HashMap;

use polars::prelude::*;

use crate::types::{CategoricalProfile, ValueCount};

pub fn profile_categorical(series: &Series, max_top_values: usize) -> CategoricalProfile {
    let len = series.len().max(1) as f64;
    let mut freq: HashMap<String, usize> = HashMap::new();
    let mut total_length: usize = 0;
    let mut total_values: usize = 0;
    let mut empty_strings: usize = 0;

    for val in series.iter() {
        if let AnyValue::Null = val {
            continue;
        }
        let s = val.to_string();
        if s.is_empty() {
            empty_strings += 1;
        }
        total_length += s.len();
        total_values += 1;
        *freq.entry(s).or_insert(0) += 1;
    }

    let mut counts: Vec<(String, usize)> = freq.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));

    let top_values: Vec<ValueCount> = counts
        .iter()
        .take(max_top_values)
        .map(|(v, c)| ValueCount {
            value: v.clone(),
            count: *c,
            ratio: *c as f64 / len,
        })
        .collect();

    let distinct_count = counts.len();
    let top_value_ratio = top_values.first().map(|v| v.ratio);
    let avg_length = if total_values > 0 {
        Some(total_length as f64 / total_values as f64)
    } else {
        None
    };
    let empty_string_ratio = Some(empty_strings as f64 / len);
    let is_constant = distinct_count <= 1;

    CategoricalProfile {
        top_values,
        top_value_ratio,
        avg_length,
        empty_string_ratio,
        is_constant,
    }
}
