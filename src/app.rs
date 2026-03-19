use std::path::Path;

use crate::baseline;
use crate::error::{Error, Result};
use crate::io;
use crate::profile;
use crate::report::summary;
use crate::rules;
use crate::types::{AnalyzeOptions, ProfileReport, Timings};

pub fn run(path: &Path, opts: &AnalyzeOptions) -> Result<ProfileReport> {
    let t_start = std::time::Instant::now();

    let t_load = std::time::Instant::now();
    let loaded = io::reader::read_dataset(path, opts)?;
    let load_ms = t_load.elapsed().as_millis();

    let t_profile = std::time::Instant::now();
    let dataset_profile = profile::dataset::build_dataset_profile(&loaded, opts.max_top_values)?;
    let profile_ms = t_profile.elapsed().as_millis();

    let baseline_profile = match &opts.baseline_path {
        Some(p) => Some(
            baseline::store::load_baseline(p)
                .map_err(|e| Error::Baseline(format!("{} ({})", e, p.display())))?,
        ),
        None => None,
    };

    let t_rules = std::time::Instant::now();
    let issues = rules::engine::run_rules(&dataset_profile, baseline_profile.as_ref());
    let rules_ms = t_rules.elapsed().as_millis();

    let summary = summary::summarize(&issues, &dataset_profile, baseline_profile.as_ref());

    let report = ProfileReport {
        dataset: dataset_profile.clone(),
        issues,
        summary,
        timings: Timings {
            load_ms,
            profile_ms,
            rules_ms,
            total_ms: t_start.elapsed().as_millis(),
        },
        meta: crate::types::ReportMeta {
            sampled: dataset_profile.sampled,
            sampled_rows: dataset_profile.sampled_rows,
            total_rows: dataset_profile.row_count,
            sample_mode: dataset_profile.sample_mode.clone(),
        },
    };

    if let Some(save_path) = &opts.save_baseline_path {
        if report.dataset.sampled && !opts.allow_sampled_baseline {
            return Err(Error::Baseline(
                "refusing to save baseline from sampled run; rerun with --full-scan or --allow-sampled-baseline".into(),
            ));
        }
        baseline::store::save_baseline(&report.dataset, save_path)
            .map_err(|e| Error::Baseline(e.to_string()))?;
    }

    Ok(report)
}
