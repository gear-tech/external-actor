use gstd::prelude::*;
use codec::{Encode, Decode};

#[derive(Encode, Decode, Debug)]
pub enum ExecutionOutcome {
    Ok(Option<Vec<u8>>),
    Trap,
}

#[derive(Encode, Decode, Debug)]
pub struct ProofData {
    pub index: u64,
    pub new_actor_state: [u8; 32],
    pub reply: Vec<u8>,
    pub proof: Vec<u8>,
    pub outcome: ExecutionOutcome,
}

#[derive(Encode, Decode, Debug)]
pub enum Incoming {
    New(Vec<u8>),
    Proof(ProofData),
}
