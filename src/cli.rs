use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::types::{AnalyzeOptions, FailOn, OutputFormat};

#[derive(Parser, Debug)]
#[command(name = "dq-prof", author, version, about = "Fast data sanity checker")]
struct Cli {
    /// Path to the CSV or Parquet file
    path: PathBuf,

    /// Existing baseline file to compare against
    #[arg(long)]
    baseline: Option<PathBuf>,

    /// Save computed profile to this baseline file
    #[arg(long = "save-baseline")]
    save_baseline: Option<PathBuf>,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormatArg::Text)]
    format: OutputFormatArg,

    /// Exit on warnings or critical issues
    #[arg(long = "fail-on", value_enum, default_value_t = FailOnArg::Warning)]
    fail_on: FailOnArg,

    /// Color output: auto|always|never
    #[arg(long = "color", value_enum, default_value_t = ColorArg::Auto)]
    color: ColorArg,

    /// Sample at most N rows (use --full-scan to disable)
    #[arg(long = "sample-rows")]
    sample_rows: Option<usize>,

    /// Sampling mode: head, random, tablesample (pg only)
    #[arg(long = "sample-mode", value_enum, default_value_t = SampleModeArg::Head)]
    sample_mode: SampleModeArg,

    /// Sampling rate (0-1) for random/table sample modes
    #[arg(long = "sample-rate")]
    sample_rate: Option<f64>,

    /// Sampling seed for deterministic random sampling
    #[arg(long = "sample-seed")]
    sample_seed: Option<u64>,

    /// Scan full file instead of sampling
    #[arg(long = "full-scan")]
    full_scan: bool,

    /// Max top values to keep for categorical columns
    #[arg(long = "max-top-values", default_value_t = 10)]
    max_top_values: usize,

    /// Postgres connection string (postgres://user:pass@host:port/db)
    #[arg(long = "pg-url")]
    pg_url: Option<String>,

    /// Postgres table or view name to query (defaults to positional path when pg-url is set)
    #[arg(long = "pg-table")]
    pg_table: Option<String>,

    /// Allow saving a baseline when sampling is enabled
    #[arg(long = "allow-sampled-baseline")]
    allow_sampled_baseline: bool,

    /// Maximum rows to load (safety cap)
    #[arg(long = "max-rows")]
    max_rows: Option<usize>,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormatArg {
    Text,
    Json,
}

#[derive(ValueEnum, Clone, Debug)]
enum FailOnArg {
    None,
    Warning,
    Critical,
}

#[derive(ValueEnum, Clone, Debug)]
enum ColorArg {
    Auto,
    Always,
    Never,
}

#[derive(ValueEnum, Clone, Debug)]
enum SampleModeArg {
    Head,
    Random,
    TableSample,
}

pub fn parse_args() -> (PathBuf, AnalyzeOptions) {
    let args = Cli::parse();
    let opts = AnalyzeOptions {
        baseline_path: args.baseline,
        save_baseline_path: args.save_baseline,
        output_format: match args.format {
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
        },
        fail_on: match args.fail_on {
            FailOnArg::None => FailOn::None,
            FailOnArg::Warning => FailOn::Warning,
            FailOnArg::Critical => FailOn::Critical,
        },
        color: match args.color {
            ColorArg::Auto => crate::types::ColorChoice::Auto,
            ColorArg::Always => crate::types::ColorChoice::Always,
            ColorArg::Never => crate::types::ColorChoice::Never,
        },
        sample_rows: args.sample_rows,
        sample_mode: match args.sample_mode {
            SampleModeArg::Head => crate::types::SampleMode::Head,
            SampleModeArg::Random => crate::types::SampleMode::Random,
            SampleModeArg::TableSample => crate::types::SampleMode::TableSample,
        },
        sample_rate: args.sample_rate,
        sample_seed: args.sample_seed,
        full_scan: args.full_scan,
        max_top_values: args.max_top_values,
        pg_url: args.pg_url,
        pg_table: args.pg_table,
        allow_sampled_baseline: args.allow_sampled_baseline,
        max_rows: args.max_rows,
    };
    (args.path, opts)
}
