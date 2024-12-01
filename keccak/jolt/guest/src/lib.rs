use sha3::{Digest, Keccak256};

#[jolt::provable]
fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

#[jolt::provable]
pub fn guest_main() -> String {
    let input: &[u8] = &[5u8; 32];
    let mut output = keccak(input);
    let n: u32 = 1000;
    for _ in 1..n {
        output = keccak(&output);
    }
    
    let output_hex = hex::encode(output);
    output_hex
}