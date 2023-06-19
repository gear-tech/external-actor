extern crate gtest;

use self::gtest::{Log, Program, System};
use crate::io;

const INIT_MSG: [u8; 64] =
    hex_literal::hex!("abf3746e72a6e8740bd9e12b879fbdd59e052cb390f116454e9116c22021ae4aabf3746e72a6e8740bd9e12b879fbdd59e052cb390f116454e9116c22021ae4a");

#[test]
fn smoky() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    let from = 42;
    let _res = program.send_bytes(from, INIT_MSG);
    let _res = program.send(from, io::Incoming::New(b"ping".to_vec()));

    let res = program.send(
        from,
        io::Incoming::Proof(io::ProofData {
            index: 0,
            new_actor_state: [0u8; 32],
            proof: Default::default(),
            outcome: io::ExecutionOutcome::Ok(Some(b"pong".to_vec())),
        }),
    );

    let log = Log::builder().source(program.id()).dest(from);
    assert!(res.contains(&log));
}
