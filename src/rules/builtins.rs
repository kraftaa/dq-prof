pub struct Thresholds {
    pub null_warning: f64,
    pub null_critical: f64,
    pub top_share_warning: f64,
    pub top_share_critical: f64,
    pub outlier_warning: f64,
    pub outlier_critical: f64,
    pub freshness_warning_secs: i64,
    pub freshness_critical_secs: i64,
    pub rowcount_drop_critical: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            null_warning: 0.05,
            null_critical: 0.10,
            top_share_warning: 0.75,
            top_share_critical: 0.90,
            outlier_warning: 0.03,
            outlier_critical: 0.10,
            freshness_warning_secs: 3 * 24 * 3600,
            freshness_critical_secs: 7 * 24 * 3600,
            rowcount_drop_critical: 0.5,
        }
    }
}
