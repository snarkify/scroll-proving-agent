use scroll_proving_sdk::prover::types::ProofType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SnarkifyProofType {
    Chunk,
    Batch,
    Bundle,
}

impl TryFrom<ProofType> for SnarkifyProofType {
    type Error = anyhow::Error;

    fn try_from(proof_type: ProofType) -> Result<Self, Self::Error> {
        match proof_type {
            ProofType::Chunk => Ok(SnarkifyProofType::Chunk),
            ProofType::Batch => Ok(SnarkifyProofType::Batch),
            ProofType::Bundle => Ok(SnarkifyProofType::Bundle),
            ProofType::Undefined => Err(anyhow::anyhow!("ProofType::Undefined should not be used")),
        }
    }
}

impl From<SnarkifyProofType> for ProofType {
    fn from(proof_type: SnarkifyProofType) -> Self {
        match proof_type {
            SnarkifyProofType::Chunk => ProofType::Chunk,
            SnarkifyProofType::Batch => ProofType::Batch,
            SnarkifyProofType::Bundle => ProofType::Bundle,
        }
    }
}
