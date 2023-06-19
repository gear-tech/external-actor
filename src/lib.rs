#![no_std]

#[cfg(feature = "std")]
mod code {
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
}

#[cfg(feature = "std")]
pub use code::WASM_BINARY_OPT as WASM_BINARY;

extern crate gstd;
extern crate hashbrown;

mod io;
mod queue;
#[cfg(test)]
mod tests;

use gstd::{prelude::*, MessageId};
use hashbrown::HashMap;

use io::{ExecutionOutcome, Incoming, ProofData};
use queue::{NewMessage, Queue};

static mut ACTOR_CODE_HASH: [u8; 32] = [0u8; 32];
static mut ACTOR_STATE_HASH: [u8; 32] = [0u8; 32];
static mut WAKERS: Option<HashMap<u64, MessageId>> = None;
static mut PROOFS: Option<HashMap<MessageId, ProofData>> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    queue::QUEUE = Some(vec![]);
    WAKERS = Some(Default::default());
    PROOFS = Some(Default::default());
    gcore::msg::read_at(0, &mut ACTOR_CODE_HASH[..]).expect("Unable to read actor code hash");
    gcore::msg::read_at(32, &mut ACTOR_STATE_HASH[..]).expect("Unable to read actor state hash");
}

fn push_waker(index: u64) {
    let wakers = unsafe {
        WAKERS
            .as_mut()
            .expect("WAKERS should have been initialized!")
    };
    wakers.insert(index, gstd::msg::id());
}

fn pop_waker(index: u64) -> Option<MessageId> {
    let wakers = unsafe {
        WAKERS
            .as_mut()
            .expect("WAKERS should have been initialized!")
    };
    wakers.remove(&index)
}

fn pop_proof() -> Option<ProofData> {
    let proofs = unsafe {
        PROOFS
            .as_mut()
            .expect("PROOFS should have been initialized!")
    };
    proofs.remove(&gstd::msg::id())
}

fn validate_proof(_proof: &ProofData) -> bool {
    true
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let msg: Incoming = gstd::msg::load().expect("Unable to parse incoming");

    match msg {
        Incoming::New(payload) => {
            if let Some(proof) = pop_proof() {
                match proof.outcome {
                    ExecutionOutcome::Ok(Some(reply)) => {
                        gcore::msg::reply(&reply[..], 0).expect("Failed to reply");
                    }
                    // nothing to do in case of error / no reply
                    // TODO: find a way to generate error reply
                    _ => {}
                }
            } else {
                let new_index = Queue::push(NewMessage {
                    payload: payload,
                    sender: gstd::msg::source(),
                    value: gstd::msg::value(),
                });

                push_waker(new_index);

                gcore::exec::wait()
            }
        }
        Incoming::Proof(proof) => {
            if validate_proof(&proof) {
                if let Some(wake_id) = pop_waker(proof.index) {
                    gcore::exec::wake(wake_id.into()).expect("Failed to wake");
                }
            } else {
                // report error about proof
            }
        }
    }
}
