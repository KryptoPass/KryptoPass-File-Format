use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub custom_fields: HashMap<String, String>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            custom_fields: HashMap::new(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
