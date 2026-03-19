use dq_prof::types::*;
use serde_json::json;

#[test]
fn json_shape_stable() {
    let report = ProfileReport {
        dataset: DatasetProfile {
            dataset_name: "sample".into(),
            row_count: 100,
            sampled: true,
            sampled_rows: Some(25),
            sample_mode: Some(SampleMode::Random),
            column_count: 2,
            schema_fingerprint: "abc".into(),
            columns: vec![],
        },
        issues: vec![Issue {
            rule_id: "null_ratio".into(),
            severity: Severity::Warning,
            column: Some("col1".into()),
            message: "high null ratio".into(),
            observed: Some("0.12".into()),
            expected: Some("< 0.05".into()),
        }],
        summary: ReportSummary {
            status: HealthStatus::Warn,
            warning_count: 1,
            critical_count: 0,
            sampling_note: Some("sampling differs".into()),
        },
        timings: Timings {
            load_ms: 10,
            profile_ms: 20,
            rules_ms: 5,
            total_ms: 40,
        },
        meta: ReportMeta {
            sampled: true,
            sampled_rows: Some(25),
            total_rows: 100,
            sample_mode: Some(SampleMode::Random),
        },
    };

    let val = serde_json::to_value(&report).unwrap();
    let expected = json!({
        "dataset": {
            "dataset_name": "sample",
            "row_count": 100,
            "sampled": true,
            "sampled_rows": 25,
            "sample_mode": "Random",
            "column_count": 2,
            "schema_fingerprint": "abc",
            "columns": []
        },
        "issues": [
            {
                "rule_id": "null_ratio",
                "severity": "Warning",
                "column": "col1",
                "message": "high null ratio",
                "observed": "0.12",
                "expected": "< 0.05"
            }
        ],
        "summary": {
            "status": "Warn",
            "warning_count": 1,
            "critical_count": 0,
            "sampling_note": "sampling differs"
        },
        "timings": {
            "load_ms": 10,
            "profile_ms": 20,
            "rules_ms": 5,
            "total_ms": 40
        },
        "meta": {
            "sampled": true,
            "sampled_rows": 25,
            "total_rows": 100,
            "sample_mode": "Random"
        }
    });

    assert_eq!(val, expected);
}
