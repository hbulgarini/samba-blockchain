use blake2::{Blake2s256, Digest};
use hex::encode;

pub fn to_hash<'a>(data: &Vec<u8>) -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(&data);
    let res = hasher.finalize();
    encode(res)
}
