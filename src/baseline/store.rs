use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::baseline::model::BaselineFile;
use crate::error::Result;
use crate::types::DatasetProfile;

pub fn save_baseline(dataset: &DatasetProfile, path: &Path) -> Result<()> {
    let file = BaselineFile::new(dataset.clone());
    let contents = serde_json::to_string_pretty(&file)
        .map_err(|e| crate::error::Error::Baseline(e.to_string()))?;
    fs::write(path, contents).map_err(|e| crate::error::Error::Baseline(e.to_string()))?;
    Ok(())
}

pub fn load_baseline(path: &Path) -> Result<BaselineFile> {
    let data = fs::read_to_string(path).map_err(|e| crate::error::Error::Baseline(e.to_string()))?;
    let json: Value =
        serde_json::from_str(&data).map_err(|e| crate::error::Error::Baseline(e.to_string()))?;

    // quick version gate
    if let Some(v) = json.get("version").and_then(|v| v.as_u64()) {
        if v != BaselineFile::CURRENT_VERSION as u64 {
            return Err(crate::error::Error::Baseline(format!(
                "unsupported baseline version {}",
                v
            )));
        }
    }

    serde_json::from_value(json).map_err(|e| crate::error::Error::Baseline(e.to_string()))
}
