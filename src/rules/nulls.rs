use crate::baseline::model::BaselineFile;
use crate::rules::builtins::Thresholds;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, _baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let thresholds = Thresholds::default();
    let mut issues = Vec::new();

    for col in &current.columns {
        let ratio = col.null_ratio;
        let severity = if ratio >= thresholds.null_critical {
            Some(Severity::Critical)
        } else if ratio >= thresholds.null_warning {
            Some(Severity::Warning)
        } else {
            None
        };

        if let Some(sev) = severity {
            issues.push(Issue {
                rule_id: "null_ratio".into(),
                severity: sev,
                column: Some(col.name.clone()),
                message: format!("high null ratio {:.2}%", ratio * 100.0),
                observed: Some(format!("{:.4}", ratio)),
                expected: Some(format!("< {:.2}", thresholds.null_warning)),
            });
        }
    }

    issues
}
