use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Car {
    pub vehicle_type: String,
    pub registration: String,
    pub seats: u8,
}