use crate::datetime_utils::deserialize_datetime;
use crate::proof_type::SnarkifyProofType;
use crate::task_state::SnarkifyTaskState;
use anyhow::Result;
use chrono::{DateTime, Utc};
use scroll_proving_sdk::prover::proving_service::ProveRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SnarkifyGetVkResponse {
    /// Base64 encoded verification key, which will be used in the login request to the Scroll coordinator.
    pub vks: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct SnarkifyGetTaskResponse {
    /// Task ID in Snarkify platform. It can be UUID or an empty string.
    pub task_id: String,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub started: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub finished: Option<DateTime<Utc>>,
    pub state: SnarkifyTaskState,
    /// Task input data necessary for the proof generation.
    pub input: String,
    /// Serialized JSON string including the base64 encoded proof and its metadata.
    pub proof: Option<String>,
    pub error: Option<String>,
    pub proof_type: SnarkifyProofType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnarkifyCreateTaskInput {
    pub circuit_version: String,
    pub hard_fork_name: String,
    pub task_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnarkifyCreateTaskRequest {
    pub input: SnarkifyCreateTaskInput,
    pub proof_type: SnarkifyProofType,
}

impl SnarkifyCreateTaskRequest {
    pub fn from_prove_request(request: &ProveRequest) -> Result<Self> {
        Ok(Self {
            input: SnarkifyCreateTaskInput {
                circuit_version: request.circuit_version.clone(),
                hard_fork_name: request.hard_fork_name.clone(),
                task_data: request.input.clone(),
            },
            proof_type: request
                .proof_type
                .try_into()
                .map_err(|e| anyhow::anyhow!("Failed to convert proof type: {}", e))?,
        })
    }
}
