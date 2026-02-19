mod models;
mod state;
mod ui;
mod pdf;

use crate::ui::{RowingApp};
use crate::state::SystemState;

fn main() -> eframe::Result<()> {
    let mut state = SystemState::default();

    // --- GETTING INPUT DATA FROM THE JSON FILES ---
    state.load_all().ok();
    
    // --- GEENERATE TRANSPORT SHEET ---
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rowing Transport Manager",
        native_options,
        Box::new(|_cc| Box::new(RowingApp { 
            state, 
            boats: vec![], 
            selected_id: None,
            error_message: None,
            show_add_person: false,
            add_person_form: Default::default(),
            edit_person_form: None,
            edit_minibus_form: None,
            wants_to_drive: std::collections::HashSet::new(),
        })),
    )
}