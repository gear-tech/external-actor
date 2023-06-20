use gstd::prelude::*;

// change this to the external-actor specific event destination id
pub static EVENT_DESTINATION: [u8; 32] = *b"gear::external_actor::risc0::001";

#[derive(Debug, Encode, Decode)]
pub enum Event {
    NewPayload { index: u64, size: u32 },
    InvalidProof { index: u64 },
}

pub fn send(event: Event) {
    gstd::msg::send(EVENT_DESTINATION.into(), event, 0)
        .expect("Failed to send event");
}
