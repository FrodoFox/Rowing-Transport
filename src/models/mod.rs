pub mod person;
pub mod car;
pub mod minibus;
pub mod allocations;

pub use person::{Person, Gender};
pub use car::Car;
pub use minibus::Minibus;
pub use allocations::{Allocation, TransportGroup};

use serde::{Serialize, Deserialize};

// --- CENTRAL DESTINATION ENUM ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Destination {
    StrathclydePark,
    Auchenstarry,
}

// --- DESTINATION LOOKUP DATA ---
impl Destination {

    // Defining location venue labels for transport sheet
    pub fn label(&self) -> &'static str {
        match self {
            Destination::StrathclydePark => "Strathclyde Park",
            Destination::Auchenstarry => "Auchenstarry",
        }
    }

    // Defining destination specific colours for transport sheet
    pub fn color_rgb(&self) -> (f32, f32, f32) {
        match self {
            Destination::StrathclydePark => (1.0, 0.9, 0.8),
            Destination::Auchenstarry => (0.9, 0.8, 0.9),
        }
    }
}