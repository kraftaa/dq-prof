use crate::baseline::model::BaselineFile;
use crate::types::{DatasetProfile, Issue, Severity};

pub fn evaluate(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut low_cards: Vec<&str> = Vec::new();

    for col in &current.columns {
        if let Some(distinct_ratio) = col.distinct_ratio {
            if distinct_ratio < 0.01 {
                // Only flag individually if the column looks ID-like.
                if is_id_like(&col.name) {
                    issues.push(Issue {
                        rule_id: "low_cardinality".into(),
                        severity: Severity::Warning,
                        column: Some(col.name.clone()),
                        message: "low cardinality (few unique values)".into(),
                        observed: Some(format!("{:.4}", distinct_ratio)),
                        expected: Some("> 0.01".into()),
                    });
                } else {
                    low_cards.push(col.name.as_str());
                }
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
                            message: "low cardinality dropped vs baseline".into(),
                            observed: Some(format!("{:.4}", cur)),
                            expected: Some(format!("~{:.4}", prev)),
                        });
                    }
                }
            }
        }
    }

    if !low_cards.is_empty() {
        let preview_len = low_cards.len().min(5);
        let mut preview = low_cards[..preview_len].join(", ");
        if low_cards.len() > preview_len {
            preview.push_str(", ...");
        }
        issues.push(Issue {
            rule_id: "low_cardinality_grouped".into(),
            severity: Severity::Warning,
            column: None,
            message: format!(
                "{} columns have low cardinality: {}",
                low_cards.len(),
                preview
            ),
            observed: None,
            expected: Some("> 0.01".into()),
        });
    }

    issues
}

fn is_id_like(name: &str) -> bool {
    let lname = name.to_ascii_lowercase();
    lname.contains("id") || lname.contains("uuid") || lname.contains("key")
}
