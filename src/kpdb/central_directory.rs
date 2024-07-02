use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CentralDirectory {
    pub files: Vec<FileMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileMetadata {
    pub file_name: String,
    pub file_offset: u64,
    pub file_size: u64,
    pub crc: u32,
    pub mac: Vec<u8>,
    pub metadata_offset: u64,
    pub metadata_size: u64,
    pub preview_offset: Option<u64>,
    pub preview_size: Option<u64>,
}

impl CentralDirectory {
    pub fn new() -> Self {
        CentralDirectory { files: Vec::new() }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
