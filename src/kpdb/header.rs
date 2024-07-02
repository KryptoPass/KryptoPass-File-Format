use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const KPDB_MAGIC_NUMBER: [u8; 4] = [b'K', b'P', b'D', b'B'];
const HEADER_PADDING_SIZE: usize = 10;

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    magic_number: [u8; 4],
    version: Version,
    timestamp: u64,
    pub central_directory_offset: u64,
    pub central_directory_size: u64,
    pub padding_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Header {
    pub fn new() -> Self {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        let timestamp_sec = since_epoch.as_secs();

        Header {
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
            },
            magic_number: KPDB_MAGIC_NUMBER,
            timestamp: timestamp_sec,
            central_directory_offset: 0,
            central_directory_size: 0,
            padding_size: HEADER_PADDING_SIZE as u64,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
