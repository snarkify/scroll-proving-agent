use anyhow::{anyhow, Result};
use std::fs::File;

use scroll_proving_sdk::config::Config as SdkConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnarkifyConfig {
    pub sdk_config: SdkConfig,
    pub base_url: String,
    pub api_key: String,
    pub service_id: String,
}

impl SnarkifyConfig {
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).map_err(|e| anyhow!(e))
    }

    pub fn from_file(file_name: String) -> Result<Self> {
        let file = File::open(file_name)?;
        Self::from_reader(&file)
    }
}
