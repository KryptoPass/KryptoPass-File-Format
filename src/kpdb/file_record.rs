use super::metadata::Metadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Preview {
    mime_type: String,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRecord {
    pub data: Vec<u8>,
    pub metadata: Metadata,
    pub preview: Option<Preview>,
}

impl FileRecord {
    pub fn new(data: Vec<u8>, metadata: Metadata, preview: Option<Preview>) -> Self {
        FileRecord {
            data,
            metadata,
            preview,
        }
    }
}
