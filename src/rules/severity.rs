use crate::types::Severity;

pub fn max(a: Severity, b: Severity) -> Severity {
    match (a, b) {
        (Severity::Critical, _) | (_, Severity::Critical) => Severity::Critical,
        _ => Severity::Warning,
    }
}
