//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha3::{Digest, Keccak256};

fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

pub fn main() {
    // Compared to the original form with hardcoded value we:
    // 1. Read from stdin
    let input: Vec<u8> = sp1_zkvm::io::read();
    let mut output = keccak(input);
    let n: u32 = 1000;
    for _ in 1..n {
        output = keccak(&output);
    }
    
    // 2. Commit to bytes directly with no hex conversion
    sp1_zkvm::io::commit(&output);
}
