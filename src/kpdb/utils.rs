use crc32fast::Hasher as CrcHasher;
use sha2::Sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;
const SECRET_KEY: &[u8] = b"your-very-secret-key";

pub fn calculate_crc(data: &[u8]) -> u32 {
    let mut hasher = CrcHasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn calculate_hmac(data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(SECRET_KEY).expect("HMAC puede tomar una llave de cualquier tamaño");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
