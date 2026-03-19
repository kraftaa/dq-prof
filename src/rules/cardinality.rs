use crate::baseline::model::BaselineFile;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let mut issues = Vec::new();

    for col in &current.columns {
        if let Some(distinct_ratio) = col.distinct_ratio {
            if distinct_ratio < 0.01 {
                issues.push(Issue {
                    rule_id: "low_cardinality".into(),
                    severity: Severity::Warning,
                    column: Some(col.name.clone()),
                    message: "distinct ratio very low".into(),
                    observed: Some(format!("{:.4}", distinct_ratio)),
                    expected: Some("> 0.01".into()),
                });
            }
        }

        if let Some(base) = baseline {
            if let Some(base_col) = base.dataset.columns.iter().find(|c| c.name == col.name) {
                if let (Some(cur), Some(prev)) = (col.distinct_ratio, base_col.distinct_ratio) {
                    if prev > 0.0 && cur / prev < 0.2 {
                        issues.push(Issue {
                            rule_id: "distinct_ratio_drop".into(),
                            severity: Severity::Warning,
                            column: Some(col.name.clone()),
                            message: "distinct ratio collapsed vs baseline".into(),
                            observed: Some(format!("{:.4}", cur)),
                            expected: Some(format!("~{:.4}", prev)),
                        });
                    }
                }
            }
        }
    }

    issues
}
