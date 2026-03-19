use crate::rules::builtins::Thresholds;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile) -> Vec<Issue> {
    let thresholds = Thresholds::default();
    let mut issues = Vec::new();

    for col in &current.columns {
        if let Some(dt) = &col.datetime {
            if let Some(lag) = dt.freshness_lag_seconds {
                let severity = if lag >= thresholds.freshness_critical_secs {
                    Some(Severity::Critical)
                } else if lag >= thresholds.freshness_warning_secs {
                    Some(Severity::Warning)
                } else {
                    None
                };
                if let Some(sev) = severity {
                    issues.push(Issue {
                        rule_id: "stale_datetime".into(),
                        severity: sev,
                        column: Some(col.name.clone()),
                        message: format!("stale timestamps: lag {}s", lag),
                        observed: dt.max_ts.clone(),
                        expected: None,
                    });
                }
            }
            if let Some(future_ratio) = dt.future_ratio {
                if future_ratio > 0.01 {
                    issues.push(Issue {
                        rule_id: "future_timestamps".into(),
                        severity: Severity::Warning,
                        column: Some(col.name.clone()),
                        message: format!("future timestamp ratio {:.2}%", future_ratio * 100.0),
                        observed: Some(format!("{:.4}", future_ratio)),
                        expected: Some("< 0.01".into()),
                    });
                }
            }
        }
    }

    issues
}
