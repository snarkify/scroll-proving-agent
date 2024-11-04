use scroll_proving_sdk::prover::proving_service::TaskStatus;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SnarkifyTaskState {
    Pending,
    Success,
    Failure,
}

impl From<SnarkifyTaskState> for TaskStatus {
    fn from(state: SnarkifyTaskState) -> Self {
        match state {
            SnarkifyTaskState::Pending => TaskStatus::Proving,
            SnarkifyTaskState::Success => TaskStatus::Success,
            SnarkifyTaskState::Failure => TaskStatus::Failed,
        }
    }
}
