use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeOptions {
    pub baseline_path: Option<PathBuf>,
    pub save_baseline_path: Option<PathBuf>,
    pub output_format: OutputFormat,
    pub fail_on: FailOn,
    pub color: ColorChoice,
    pub sample_rows: Option<usize>,
    pub sample_mode: SampleMode,
    pub sample_rate: Option<f64>,
    pub sample_seed: Option<u64>,
    pub full_scan: bool,
    pub max_top_values: usize,
    pub pg_url: Option<String>,
    pub pg_table: Option<String>,
    pub allow_sampled_baseline: bool,
    pub max_rows: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FailOn {
    None,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SampleMode {
    Head,
    Random,
    TableSample,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileReport {
    pub dataset: DatasetProfile,
    pub issues: Vec<Issue>,
    pub summary: ReportSummary,
    pub timings: Timings,
    pub meta: ReportMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetProfile {
    pub dataset_name: String,
    pub row_count: usize,
    pub sampled: bool,
    pub sampled_rows: Option<usize>,
    pub sample_mode: Option<SampleMode>,
    pub column_count: usize,
    pub schema_fingerprint: String,
    pub columns: Vec<ColumnProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnProfile {
    pub name: String,
    pub logical_type: LogicalType,
    pub null_ratio: f64,
    pub distinct_count: Option<usize>,
    pub distinct_ratio: Option<f64>,
    pub numeric: Option<NumericProfile>,
    pub categorical: Option<CategoricalProfile>,
    pub datetime: Option<DateTimeProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalType {
    Numeric,
    Categorical,
    Datetime,
    Boolean,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericProfile {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub mean: Option<f64>,
    pub stddev: Option<f64>,
    pub q25: Option<f64>,
    pub q50: Option<f64>,
    pub q75: Option<f64>,
    pub zero_ratio: Option<f64>,
    pub negative_ratio: Option<f64>,
    pub outlier_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoricalProfile {
    pub top_values: Vec<ValueCount>,
    pub top_value_ratio: Option<f64>,
    pub avg_length: Option<f64>,
    pub empty_string_ratio: Option<f64>,
    pub is_constant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeProfile {
    pub min_ts: Option<String>,
    pub max_ts: Option<String>,
    pub freshness_lag_seconds: Option<i64>,
    pub future_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueCount {
    pub value: String,
    pub count: usize,
    pub ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub rule_id: String,
    pub severity: Severity,
    pub column: Option<String>,
    pub message: String,
    pub observed: Option<String>,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub status: HealthStatus,
    pub warning_count: usize,
    pub critical_count: usize,
    pub sampling_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Timings {
    pub load_ms: u128,
    pub profile_ms: u128,
    pub rules_ms: u128,
    pub total_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportMeta {
    pub sampled: bool,
    pub sampled_rows: Option<usize>,
    pub total_rows: usize,
    pub sample_mode: Option<SampleMode>,
}

// Helper maps for fast lookup
pub type ColumnProfileMap = HashMap<String, ColumnProfile>;
