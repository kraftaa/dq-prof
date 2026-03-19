use crate::baseline::model::BaselineFile;
use crate::rules::{
    cardinality, constants, freshness, nulls, outliers, rowcount, schema_change, skew,
};
use crate::types::{DatasetProfile, Issue};

pub fn run_rules(current: &DatasetProfile, baseline: Option<&BaselineFile>) -> Vec<Issue> {
    let mut issues = Vec::new();
    issues.extend(nulls::evaluate(current, baseline));
    issues.extend(skew::evaluate(current, baseline));
    issues.extend(outliers::evaluate(current, baseline));
    issues.extend(freshness::evaluate(current));
    issues.extend(rowcount::evaluate(current, baseline));
    issues.extend(schema_change::evaluate(current, baseline));
    issues.extend(constants::evaluate(current, baseline));
    issues.extend(cardinality::evaluate(current, baseline));
    issues
}
