use crate::baseline::compare;
use crate::baseline::model::BaselineFile;
use crate::rules::builtins::Thresholds;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let thresholds = Thresholds::default();
    let mut issues = Vec::new();

    if current.row_count == 0 {
        issues.push(Issue {
            rule_id: "rowcount_zero".into(),
            severity: Severity::Critical,
            column: None,
            message: "dataset has zero rows".into(),
            observed: Some("0".into()),
            expected: Some("> 0".into()),
        });
    }

    if let Some(base) = baseline {
        let delta = compare::dataset_delta(current, base);
        if let Some(rc_delta) = delta.row_count_delta {
            let base_rows = base.dataset.row_count as f64;
            if base_rows > 0.0 {
                let change_ratio = rc_delta as f64 / base_rows;
                if change_ratio <= -thresholds.rowcount_drop_critical {
                    issues.push(Issue {
                        rule_id: "rowcount_drop".into(),
                        severity: Severity::Critical,
                        column: None,
                        message: format!(
                            "row count dropped by {:.1}% ({} -> {})",
                            change_ratio * 100.0,
                            base.dataset.row_count,
                            current.row_count
                        ),
                        observed: Some(current.row_count.to_string()),
                        expected: Some(format!("> {}", (base_rows * (1.0 - thresholds.rowcount_drop_critical)) as usize)),
                    });
                }
            }
        }
    }

    issues
}
