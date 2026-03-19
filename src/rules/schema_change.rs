use crate::baseline::model::BaselineFile;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let mut issues = Vec::new();

    if let Some(base) = baseline {
        // fingerprint change
        if current.schema_fingerprint != base.dataset.schema_fingerprint {
            issues.push(Issue {
                rule_id: "schema_fingerprint".into(),
                severity: Severity::Critical,
                column: None,
                message: "schema fingerprint changed".into(),
                observed: Some(current.schema_fingerprint.clone()),
                expected: Some(base.dataset.schema_fingerprint.clone()),
            });
        }

        // missing and new columns
        let base_names: std::collections::HashSet<_> = base
            .dataset
            .columns
            .iter()
            .map(|c| c.name.as_str())
            .collect();
        let cur_names: std::collections::HashSet<_> = current.columns.iter().map(|c| c.name.as_str()).collect();

        for missing in base_names.difference(&cur_names) {
            issues.push(Issue {
                rule_id: "missing_column".into(),
                severity: Severity::Critical,
                column: Some((*missing).to_string()),
                message: "column missing vs baseline".into(),
                observed: None,
                expected: None,
            });
        }

        for new_col in cur_names.difference(&base_names) {
            issues.push(Issue {
                rule_id: "new_column".into(),
                severity: Severity::Warning,
                column: Some((*new_col).to_string()),
                message: "new column vs baseline".into(),
                observed: None,
                expected: None,
            });
        }
    }

    issues
}
