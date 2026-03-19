use crate::baseline::model::BaselineFile;
use crate::baseline::compare;
use crate::rules::builtins::Thresholds;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let thresholds = Thresholds::default();
    let mut issues = Vec::new();

    for col in &current.columns {
        if let Some(num) = &col.numeric {
            if let Some(ratio) = num.outlier_ratio {
                let severity = if ratio >= thresholds.outlier_critical {
                    Some(Severity::Critical)
                } else if ratio >= thresholds.outlier_warning {
                    Some(Severity::Warning)
                } else {
                    None
                };

                if let Some(sev) = severity {
                    issues.push(Issue {
                        rule_id: "outlier_ratio".into(),
                        severity: sev,
                        column: Some(col.name.clone()),
                        message: format!("outlier ratio {:.2}%", ratio * 100.0),
                        observed: Some(format!("{:.4}", ratio)),
                        expected: Some(format!("< {:.2}", thresholds.outlier_warning)),
                    });
                }

                if let Some(base) = baseline {
                    if let Some(base_col) = base.dataset.columns.iter().find(|c| c.name == col.name)
                    {
                        let delta = compare::column_delta(col, base_col);
                        if let Some(out_delta) = delta.outlier_ratio_delta {
                            if out_delta > 0.05 {
                                issues.push(Issue {
                                    rule_id: "outlier_shift".into(),
                                    severity: Severity::Warning,
                                    column: Some(col.name.clone()),
                                    message: format!(
                                        "outlier ratio increased by {:.1}pp",
                                        out_delta * 100.0
                                    ),
                                    observed: Some(format!("{:.4}", ratio)),
                                    expected: base_col
                                        .numeric
                                        .as_ref()
                                        .and_then(|n| n.outlier_ratio)
                                        .map(|v| format!("{:.4}", v)),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    issues
}
