use crate::baseline::model::BaselineFile;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, _baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let mut issues = Vec::new();
    for col in &current.columns {
        if let Some(cat) = &col.categorical {
            if cat.is_constant {
                issues.push(Issue {
                    rule_id: "constant_column".into(),
                    severity: Severity::Warning,
                    column: Some(col.name.clone()),
                    message: "column is constant".into(),
                    observed: None,
                    expected: None,
                });
            }
        }
        if let Some(num) = &col.numeric {
            if num.stddev.unwrap_or(0.0) == 0.0 {
                issues.push(Issue {
                    rule_id: "constant_numeric".into(),
                    severity: Severity::Warning,
                    column: Some(col.name.clone()),
                    message: "numeric column has zero variance".into(),
                    observed: num.mean.map(|m| format!("{m:.4}")),
                    expected: None,
                });
            }
        }
    }
    issues
}
