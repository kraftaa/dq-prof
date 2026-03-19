#[derive(Debug, Clone)]
pub struct SamplingDecision {
    pub full_scan: bool,
    pub sample_rows: Option<usize>,
    pub sample_mode: crate::types::SampleMode,
    pub sample_rate: Option<f64>,
    pub sample_seed: Option<u64>,
}

pub fn decide(
    sample_rows: Option<usize>,
    full_scan: bool,
    sample_mode: crate::types::SampleMode,
    sample_rate: Option<f64>,
    sample_seed: Option<u64>,
) -> SamplingDecision {
    SamplingDecision {
        full_scan,
        sample_rows,
        sample_mode,
        sample_rate,
        sample_seed,
    }
}

use polars::prelude::*;
use rand::rngs::StdRng;
use rand::{seq::index::sample, SeedableRng};

pub fn apply_frame_sampling(
    df: &mut DataFrame,
    decision: &SamplingDecision,
    total_rows: usize,
) -> Result<(), crate::error::Error> {
    if decision.full_scan {
        return Ok(());
    }
    let target = decision
        .sample_rows
        .or_else(|| {
            decision
                .sample_rate
                .map(|r| ((r.clamp(0.0, 1.0)) * total_rows as f64) as usize)
        })
        .unwrap_or(50_000)
        .max(1);

    match decision.sample_mode {
        crate::types::SampleMode::Head => {
            *df = df.head(Some(target));
        }
        crate::types::SampleMode::Random => {
            let seed = decision.sample_seed.unwrap_or(42);
            let mut rng = StdRng::seed_from_u64(seed);
            let n = target.min(df.height()).max(1);
            let idx = sample(&mut rng, df.height(), n)
                .into_iter()
                .map(|i| i as u32)
                .collect::<Vec<_>>();
            let idx_ca = UInt32Chunked::from_vec("idx", idx);
            *df = df
                .take(&idx_ca)
                .map_err(|e| crate::error::Error::Io(e.to_string()))?;
        }
        crate::types::SampleMode::TableSample => {
            // Not applicable to in-memory frames; fallback to random
            let seed = decision.sample_seed.unwrap_or(42);
            let mut rng = StdRng::seed_from_u64(seed);
            let n = target.min(df.height()).max(1);
            let idx = sample(&mut rng, df.height(), n)
                .into_iter()
                .map(|i| i as u32)
                .collect::<Vec<_>>();
            let idx_ca = UInt32Chunked::from_vec("idx", idx);
            *df = df
                .take(&idx_ca)
                .map_err(|e| crate::error::Error::Io(e.to_string()))?;
        }
    }
    Ok(())
}
