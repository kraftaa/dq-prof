use crate::baseline::model::BaselineFile;
use crate::baseline::compare;
use crate::rules::builtins::Thresholds;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let thresholds = Thresholds::default();
    let mut issues = Vec::new();

    for col in &current.columns {
        if let Some(cat) = &col.categorical {
            if let Some(ratio) = cat.top_value_ratio {
                let severity = if ratio >= thresholds.top_share_critical {
                    Some(Severity::Critical)
                } else if ratio >= thresholds.top_share_warning {
                    Some(Severity::Warning)
                } else {
                    None
                };

                if let Some(sev) = severity {
                    issues.push(Issue {
                        rule_id: "top_value_skew".into(),
                        severity: sev,
                        column: Some(col.name.clone()),
                        message: format!(
                            "top value dominates {:.1}% of rows",
                            ratio * 100.0
                        ),
                        observed: Some(format!("{:.4}", ratio)),
                        expected: Some(format!("< {:.2}", thresholds.top_share_warning)),
                    });
                }

                if let Some(base) = baseline {
                    if let Some(base_col) = base.dataset.columns.iter().find(|c| c.name == col.name)
                    {
                        let delta = compare::column_delta(col, base_col);
                        if let Some(top_delta) = delta.top_ratio_delta {
                            if top_delta > 0.2 {
                                issues.push(Issue {
                                    rule_id: "top_value_shift".into(),
                                    severity: Severity::Warning,
                                    column: Some(col.name.clone()),
                                    message: format!(
                                        "top value share increased by {:.1}pp",
                                        top_delta * 100.0
                                    ),
                                    observed: Some(format!("{:.4}", ratio)),
                                    expected: base_col
                                        .categorical
                                        .as_ref()
                                        .and_then(|c| c.top_value_ratio)
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
