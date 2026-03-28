use std::io::{self, Write};

use crate::types::ProfileReport;

pub fn print(report: &ProfileReport) -> io::Result<()> {
    let json = serde_json::to_string_pretty(report)
        .map_err(|e| io::Error::other(format!("failed to serialize report: {e}")))?;
    let mut out = io::stdout();
    writeln!(out, "{}", json)
}
