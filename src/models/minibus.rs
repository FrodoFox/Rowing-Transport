use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minibus {
    pub registration: String,
    pub seats: u8,
}