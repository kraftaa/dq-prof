use dq_prof::baseline::model::BaselineFile;
use dq_prof::report::summary::summarize;
use dq_prof::types::{DatasetProfile, ReportSummary, SampleMode};

fn make_profile(sampled: bool) -> DatasetProfile {
    DatasetProfile {
        dataset_name: "test".into(),
        row_count: 100,
        sampled,
        sampled_rows: Some(if sampled { 20 } else { 100 }),
        sample_mode: Some(SampleMode::Random),
        column_count: 1,
        schema_fingerprint: "abc".into(),
        columns: vec![],
    }
}

#[test]
fn sampling_warning_when_differs() {
    let current = make_profile(true);
    let baseline_profile = BaselineFile {
        version: BaselineFile::CURRENT_VERSION,
        generated_at: chrono::Utc::now(),
        dataset: make_profile(false),
    };
    let issues: Vec<dq_prof::types::Issue> = vec![];
    let summary: ReportSummary = summarize(&issues, &current, Some(&baseline_profile));
    assert!(summary.sampling_note.is_some());
}

#[test]
fn no_warning_when_same_sampling() {
    let current = make_profile(true);
    let baseline_profile = BaselineFile {
        version: BaselineFile::CURRENT_VERSION,
        generated_at: chrono::Utc::now(),
        dataset: make_profile(true),
    };
    let issues: Vec<dq_prof::types::Issue> = vec![];
    let summary: ReportSummary = summarize(&issues, &current, Some(&baseline_profile));
    assert!(summary.sampling_note.is_none());
}
