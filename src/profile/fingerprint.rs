use sha2::{Digest, Sha256};

use crate::types::LogicalType;

pub fn compute(columns: &[(String, LogicalType)]) -> String {
    let mut hasher = Sha256::new();
    for (name, logical) in columns {
        hasher.update(name.as_bytes());
        hasher.update(b":");
        hasher.update(logical_name(logical).as_bytes());
        hasher.update(b";");
    }
    let bytes = hasher.finalize();
    hex::encode(bytes)
}

fn logical_name(logical: &LogicalType) -> &'static str {
    match logical {
        LogicalType::Numeric => "numeric",
        LogicalType::Categorical => "categorical",
        LogicalType::Datetime => "datetime",
        LogicalType::Boolean => "boolean",
        LogicalType::Other => "other",
    }
}
