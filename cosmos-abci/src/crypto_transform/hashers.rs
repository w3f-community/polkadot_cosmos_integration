use ripemd160::*;
use sha2::*;
use sp_std::vec::Vec;

/// Method for generate ripemd160 hash from value.
pub fn get_ripemd160_hash(from: &[u8]) -> Vec<u8> {
    let mut digest = Ripemd160::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}

/// Method for generate sha256 hash from value.
pub fn get_sha256_hash(from: &[u8]) -> Vec<u8> {
    let mut digest = Sha256::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}
