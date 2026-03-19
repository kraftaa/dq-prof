use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{ColumnProfile, DatasetProfile};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaselineFile {
    pub version: u8,
    pub generated_at: DateTime<Utc>,
    pub dataset: DatasetProfile,
}

impl BaselineFile {
    pub const CURRENT_VERSION: u8 = 1;

    pub fn new(dataset: DatasetProfile) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            generated_at: Utc::now(),
            dataset,
        }
    }
}

#[derive(Debug)]
pub struct BaselineLookup<'a> {
    pub columns: std::collections::HashMap<&'a str, &'a ColumnProfile>,
    pub dataset: &'a DatasetProfile,
}

impl<'a> From<&'a BaselineFile> for BaselineLookup<'a> {
    fn from(file: &'a BaselineFile) -> Self {
        let columns = file
            .dataset
            .columns
            .iter()
            .map(|c| (c.name.as_str(), c))
            .collect();
        Self {
            columns,
            dataset: &file.dataset,
        }
    }
}
