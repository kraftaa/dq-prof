use polars::prelude::*;

use crate::types::NumericProfile;

pub fn profile_numeric(series: &Series) -> NumericProfile {
    let cast = match series.cast(&DataType::Float64) {
        Ok(s) => s,
        Err(_) => return empty_numeric_profile(),
    };
    let ca = match cast.f64() {
        Ok(ca) => ca,
        Err(_) => return empty_numeric_profile(),
    };
    let len = ca.len() as f64;
    let valid: Vec<f64> = ca.into_iter().flatten().collect();

    let valid_len = valid.len() as f64;
    let min = ca.min();
    let max = ca.max();
    let mean = ca.mean();
    let stddev = ca.std(1);

    let (zero_ratio, negative_ratio, outlier_ratio) = if valid_len > 0.0 {
        let mut zero = 0usize;
        let mut negative = 0usize;
        let mut outliers = 0usize;
        let mean_v = mean.unwrap_or(0.0);
        let std_v = stddev.unwrap_or(0.0);
        for v in &valid {
            if *v == 0.0 {
                zero += 1;
            }
            if *v < 0.0 {
                negative += 1;
            }
            if std_v > 0.0 && ((v - mean_v) / std_v).abs() > 3.0 {
                outliers += 1;
            }
        }
        (
            Some(zero as f64 / len.max(1.0)),
            Some(negative as f64 / len.max(1.0)),
            Some(outliers as f64 / len.max(1.0)),
        )
    } else {
        (None, None, None)
    };

    let q25 = ca.quantile(0.25, QuantileInterpolOptions::Nearest).ok().flatten();
    let q50 = ca.quantile(0.50, QuantileInterpolOptions::Nearest).ok().flatten();
    let q75 = ca.quantile(0.75, QuantileInterpolOptions::Nearest).ok().flatten();

    NumericProfile {
        min,
        max,
        mean,
        stddev,
        q25,
        q50,
        q75,
        zero_ratio,
        negative_ratio,
        outlier_ratio,
    }
}

fn empty_numeric_profile() -> NumericProfile {
    NumericProfile {
        min: None,
        max: None,
        mean: None,
        stddev: None,
        q25: None,
        q50: None,
        q75: None,
        zero_ratio: None,
        negative_ratio: None,
        outlier_ratio: None,
    }
}
