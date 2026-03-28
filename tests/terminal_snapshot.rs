use dq_prof::report::terminal::print_to;
use dq_prof::types::*;

#[test]
fn terminal_output_stable_no_color() {
    let report = ProfileReport {
        dataset: DatasetProfile {
            dataset_name: "sample".into(),
            row_count: 100,
            sampled: true,
            sampled_rows: Some(20),
            sample_mode: Some(SampleMode::Head),
            column_count: 1,
            schema_fingerprint: "abc".into(),
            columns: vec![],
        },
        issues: vec![
            Issue {
                rule_id: "null_ratio".into(),
                severity: Severity::Warning,
                column: Some("col1".into()),
                message: "high null ratio".into(),
                observed: Some("0.12".into()),
                expected: Some("< 0.05".into()),
            },
            Issue {
                rule_id: "rowcount_drop".into(),
                severity: Severity::Critical,
                column: None,
                message: "row count dropped".into(),
                observed: Some("10".into()),
                expected: Some("> 50".into()),
            },
        ],
        summary: ReportSummary {
            status: HealthStatus::Fail,
            warning_count: 1,
            critical_count: 1,
            sampling_note: Some("sampling differs".into()),
        },
        timings: Timings {
            load_ms: 1,
            profile_ms: 2,
            rules_ms: 3,
            total_ms: 6,
        },
        meta: ReportMeta {
            sampled: true,
            sampled_rows: Some(20),
            total_rows: 100,
            sample_mode: Some(SampleMode::Head),
        },
    };

    let mut buf: Vec<u8> = Vec::new();
    print_to(&report, ColorChoice::Never, &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let expected = "\
DATA HEALTH: FAIL (1 critical, 1 warning)\n\
Rows: 20 sampled of 100 (head sampling)\n\
Sampling: sampling differs\n\
Timing: load 1 ms | profile 2 ms | rules 3 ms | total 6 ms\n\
\n\
CRITICAL\n\
- <dataset>: row count dropped (obs=10, exp=> 50)\n\
\n\
WARNING\n\
- col1: high null ratio (obs=0.12, exp=< 0.05)\n\
\n";
    assert_eq!(output, expected);
}
