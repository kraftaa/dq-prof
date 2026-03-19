use std::io::Read;

use polars::prelude::*;
use polars::prelude::CsvReader;
use postgres::{Client, NoTls};

use crate::error::{Error, Result};
use crate::io::sampling::SamplingDecision;

pub fn read_postgres(
    table: &str,
    url: &str,
    sampling: &SamplingDecision,
) -> Result<(DataFrame, usize, bool)> {
    validate_table(table)?;
    let mut client = Client::connect(url, NoTls).map_err(|e| Error::Io(e.to_string()))?;

    let safe_table = quote_ident(table)?;

    let count_sql = format!("SELECT COUNT(*) FROM {safe_table}");
    let total_rows: i64 = client
        .query_one(&count_sql, &[])
        .map_err(|e| Error::Io(e.to_string()))?
        .get(0);

    let target_rows = sampling
        .sample_rows
        .or_else(|| {
            sampling
                .sample_rate
                .map(|r| ((r.clamp(0.0, 1.0)) * total_rows as f64) as usize)
        })
        .unwrap_or(50_000)
        .max(1);

    let (query_sql, was_sampled) = if sampling.full_scan {
        (
            format!("COPY (SELECT * FROM {safe_table}) TO STDOUT WITH CSV HEADER"),
            false,
        )
    } else {
        match sampling.sample_mode {
            crate::types::SampleMode::Head => (
                format!(
                    "COPY (SELECT * FROM {safe_table} LIMIT {target_rows}) TO STDOUT WITH CSV HEADER"
                ),
                true,
            ),
            crate::types::SampleMode::Random => {
                let seed_comment = sampling
                    .sample_seed
                    .map(|s| format!(" /* seed:{s} */"))
                    .unwrap_or_default();
                (
                    format!(
                        "COPY (SELECT * FROM {safe_table} ORDER BY random(){seed_comment} LIMIT {target_rows}) TO STDOUT WITH CSV HEADER"
                    ),
                    true,
                )
            }
            crate::types::SampleMode::TableSample => {
                let pct = sampling
                    .sample_rate
                    .map(|r| (r.clamp(0.0, 1.0) * 100.0).max(0.0001))
                    .unwrap_or_else(|| {
                        let fraction = target_rows as f64 / (total_rows as f64).max(1.0);
                        (fraction * 100.0).max(0.0001)
                    })
                    .min(100.0);
                (
                    format!(
                        "COPY (SELECT * FROM {safe_table} TABLESAMPLE BERNOULLI ({pct}) LIMIT {target_rows}) TO STDOUT WITH CSV HEADER"
                    ),
                    true,
                )
            }
        }
    };

    let mut copy_out = client
        .copy_out(query_sql.as_str())
        .map_err(|e| Error::Io(e.to_string()))?;

    let mut buf = Vec::new();
    copy_out
        .read_to_end(&mut buf)
        .map_err(|e| Error::Io(e.to_string()))?;

    let cursor = std::io::Cursor::new(buf);
    let df = CsvReader::new(cursor)
        .finish()
        .map_err(|e| Error::Io(e.to_string()))?;

    Ok((df, total_rows as usize, was_sampled))
}

fn validate_table(table: &str) -> Result<()> {
    if table.is_empty() {
        return Err(Error::Io("table name required for postgres source".into()));
    }
    Ok(())
}

fn quote_ident(raw: &str) -> Result<String> {
    // Basic guard: reject dangerous characters outright.
    if raw
        .chars()
        .any(|c| c == ';' || c == '\'' || c == '"' || c == '\\' || c == '\n' || c == '\r')
    {
        return Err(Error::Io("table name contains unsupported characters".into()));
    }

    let parts: Vec<String> = raw
        .split('.')
        .map(|p| {
            let escaped = p.replace('"', "\"\"");
            format!("\"{escaped}\"")
        })
        .collect();
    if parts.is_empty() {
        return Err(Error::Io("table name required for postgres source".into()));
    }
    Ok(parts.join("."))
}
