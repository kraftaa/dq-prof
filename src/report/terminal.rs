use std::io::{self, Write};

use yansi::{Paint, Painted};

use crate::types::{ColorChoice, HealthStatus, Issue, ProfileReport, Severity};

pub fn print(report: &ProfileReport, color: ColorChoice) -> io::Result<()> {
    let mut out = io::stdout();
    print_to(report, color, &mut out)
}

pub fn print_to(
    report: &ProfileReport,
    color: ColorChoice,
    out: &mut dyn Write,
) -> io::Result<()> {
    apply_color_choice(color);
    writeln!(
        out,
        "{}: {} ({} {}, {} {})",
        Paint::new("DATA HEALTH").bold(),
        color_status(&report.summary.status),
        report.summary.critical_count,
        pluralize(report.summary.critical_count, "critical"),
        report.summary.warning_count,
        pluralize(report.summary.warning_count, "warning")
    )?;
    writeln!(out, "Rows: {}", format_rows(report))?;
    if let Some(note) = &report.summary.sampling_note {
        writeln!(out, "Sampling: {note}")?;
    }
    writeln!(
        out,
        "Timing: load {} ms | profile {} ms | rules {} ms | total {} ms",
        report.timings.load_ms, report.timings.profile_ms, report.timings.rules_ms, report.timings.total_ms
    )?;
    writeln!(out)?;

    if report.issues.is_empty() {
        writeln!(out, "No issues detected.")?;
        return Ok(());
    }

    render_group(out, "CRITICAL", Severity::Critical, &report.issues)?;
    render_group(out, "WARNING", Severity::Warning, &report.issues)?;
    Ok(())
}

fn render_group(
    out: &mut dyn Write,
    title: &str,
    severity: Severity,
    issues: &[Issue],
) -> io::Result<()> {
    let bucket: Vec<&Issue> = issues.iter().filter(|i| i.severity == severity).collect();
    if bucket.is_empty() {
        return Ok(());
    }
    writeln!(out, "{}", color_sev_title(title, severity))?;
    for issue in bucket {
        let scope = issue.column.as_deref().unwrap_or("<dataset>");
        if let (Some(obs), Some(exp)) = (&issue.observed, &issue.expected) {
            writeln!(out, "- {scope}: {} (obs={}, exp={})", issue.message, obs, exp)?;
        } else if let Some(obs) = &issue.observed {
            writeln!(out, "- {scope}: {} (obs={})", issue.message, obs)?;
        } else {
            writeln!(out, "- {scope}: {}", issue.message)?;
        }
    }
    writeln!(out)?;
    Ok(())
}

fn render_status(status: &HealthStatus) -> &'static str {
    match status {
        HealthStatus::Pass => "PASS",
        HealthStatus::Warn => "WARN",
        HealthStatus::Fail => "FAIL",
    }
}

fn format_rows(report: &ProfileReport) -> String {
    let sampled_rows = report.dataset.sampled_rows.unwrap_or(report.dataset.row_count);
    if report.dataset.sampled {
        format!(
            "{} sampled of {} ({} sampling)",
            sampled_rows,
            report.dataset.row_count,
            match report.dataset.sample_mode {
                Some(ref m) => format!("{:?}", m).to_ascii_lowercase(),
                None => "head".into()
            }
        )
    } else {
        format!("{}", report.dataset.row_count)
    }
}

fn color_status(status: &HealthStatus) -> Painted<&'static str> {
    match status {
        HealthStatus::Pass => Paint::green(render_status(status)).bold(),
        HealthStatus::Warn => Paint::yellow(render_status(status)).bold(),
        HealthStatus::Fail => Paint::red(render_status(status)).bold(),
    }
}

fn pluralize(n: usize, word: &str) -> String {
    if n == 1 {
        word.to_string()
    } else {
        format!("{word}s")
    }
}

fn color_sev_title(title: &str, sev: Severity) -> Painted<&str> {
    match sev {
        Severity::Critical => Paint::red(title).bold(),
        Severity::Warning => Paint::yellow(title).bold(),
    }
}

fn apply_color_choice(choice: ColorChoice) {
    match choice {
        ColorChoice::Auto => {
            // yansi auto by default; do nothing
        }
        ColorChoice::Always => {
            yansi::enable();
        }
        ColorChoice::Never => {
            yansi::disable();
        }
    }
}
