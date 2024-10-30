use scroll_proving_sdk::prover::types::CircuitType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SnarkifyProofType {
    Chunk,
    Batch,
    Bundle,
}

impl From<CircuitType> for SnarkifyProofType {
    fn from(circuit_type: CircuitType) -> Self {
        match circuit_type {
            CircuitType::Chunk => SnarkifyProofType::Chunk,
            CircuitType::Batch => SnarkifyProofType::Batch,
            CircuitType::Bundle => SnarkifyProofType::Bundle,
            CircuitType::Undefined => unreachable!("CircuitType::Undefined should not be used"),
        }
    }
}
