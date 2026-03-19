use dq_prof::rules::engine::run_rules;
use dq_prof::types::{ColumnProfile, DatasetProfile, LogicalType, NumericProfile};

fn make_dataset(columns: Vec<ColumnProfile>, rows: usize) -> DatasetProfile {
    DatasetProfile {
        dataset_name: "test".into(),
        row_count: rows,
        sampled: false,
        sampled_rows: Some(rows),
        sample_mode: Some(dq_prof::types::SampleMode::Head),
        column_count: columns.len(),
        schema_fingerprint: "test".into(),
        columns,
    }
}

#[test]
fn detects_nulls_and_constants() {
    let cols = vec![
        ColumnProfile {
            name: "a".into(),
            logical_type: LogicalType::Numeric,
            null_ratio: 0.12,
            distinct_count: Some(1),
            distinct_ratio: Some(0.01),
            numeric: Some(NumericProfile {
                min: Some(1.0),
                max: Some(1.0),
                mean: Some(1.0),
                stddev: Some(0.0),
                q25: Some(1.0),
                q50: Some(1.0),
                q75: Some(1.0),
                zero_ratio: Some(0.0),
                negative_ratio: Some(0.0),
                outlier_ratio: Some(0.0),
            }),
            categorical: None,
            datetime: None,
        },
    ];
    let ds = make_dataset(cols, 100);
    let issues = run_rules(&ds, None);
    assert!(issues.iter().any(|i| i.rule_id == "null_ratio"));
    assert!(issues.iter().any(|i| i.rule_id == "constant_numeric"));
}

#[test]
fn detects_outliers() {
    let cols = vec![
        ColumnProfile {
            name: "b".into(),
            logical_type: LogicalType::Numeric,
            null_ratio: 0.0,
            distinct_count: Some(10),
            distinct_ratio: Some(0.5),
            numeric: Some(NumericProfile {
                min: Some(0.0),
                max: Some(100.0),
                mean: Some(10.0),
                stddev: Some(1.0),
                q25: Some(1.0),
                q50: Some(2.0),
                q75: Some(3.0),
                zero_ratio: Some(0.0),
                negative_ratio: Some(0.0),
                outlier_ratio: Some(0.2),
            }),
            categorical: None,
            datetime: None,
        },
    ];
    let ds = make_dataset(cols, 20);
    let issues = run_rules(&ds, None);
    assert!(issues.iter().any(|i| i.rule_id == "outlier_ratio"));
}
