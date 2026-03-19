use std::io::{self, Write};

use crate::types::ProfileReport;

pub fn print(report: &ProfileReport) -> io::Result<()> {
    let json = serde_json::to_string_pretty(report).expect("serialize report");
    let mut out = io::stdout();
    writeln!(out, "{}", json)
}
