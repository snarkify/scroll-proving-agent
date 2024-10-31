use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnarkifyConfig {
    pub service_id: String,
}

impl SnarkifyConfig {
    pub fn from_reader<R>(reader: R) -> anyhow::Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn from_file(file_name: String) -> anyhow::Result<Self> {
        let file = File::open(file_name)?;
        SnarkifyConfig::from_reader(&file)
    }
}
