use crate::types::{DatasetProfile, FailOn, HealthStatus, Issue, ReportSummary, Severity};

pub fn summarize(
    issues: &[Issue],
    current: &DatasetProfile,
    baseline: Option<&crate::baseline::model::BaselineFile>,
) -> ReportSummary {
    let warning_count = issues.iter().filter(|i| i.severity == Severity::Warning).count();
    let critical_count = issues.iter().filter(|i| i.severity == Severity::Critical).count();
    let status = if critical_count > 0 {
        HealthStatus::Fail
    } else if warning_count > 0 {
        HealthStatus::Warn
    } else {
        HealthStatus::Pass
    };

    let sampling_note = baseline.and_then(|b| {
        let base_sampled = b.dataset.sampled;
        if current.sampled != base_sampled {
            Some(format!(
                "sampling differs: current sampled={}, baseline sampled={}",
                current.sampled, base_sampled
            ))
        } else {
            None
        }
    });

    ReportSummary {
        status,
        warning_count,
        critical_count,
        sampling_note,
    }
}

pub fn exit_code(summary: &ReportSummary, fail_on: FailOn) -> i32 {
    match fail_on {
        FailOn::None => 0,
        FailOn::Warning => {
            if summary.warning_count > 0 || summary.critical_count > 0 {
                1
            } else {
                0
            }
        }
        FailOn::Critical => {
            if summary.critical_count > 0 {
                1
            } else {
                0
            }
        }
    }
}
