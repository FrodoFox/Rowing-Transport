use serde::{Serialize, Deserialize};
use super::car::Car;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub gender: Gender,
    pub student_id: String,
    pub year_of_entry: u16,
    pub pickup_locations: Vec<String>,
    pub car: Option<Car>,
    pub can_drive_minibus: bool,
}