use eframe::egui;
use chrono::Local;
use crate::models::{Person, Car, Allocation, Minibus, Destination, Gender};
use crate::state::SystemState;

// --- CREATING A SIMPLE ENUM FOR DIFFERENT BOAT CLASSIFICATIONS ---
#[derive(Clone, Copy, PartialEq)]
pub enum BoatType {
    Single, 
    Double, 
    Pair, 
    Quad, 
    Four, 
    FourCoxed, 
    EightCoxed,
}

// --- IMPLEMENTING METHODS TO RETURN SEAT COUNTS AND LABELS TO BE DISPLAYED FOR DIFFERENT BOAT TYPES ---
impl BoatType {
    fn seat_count(&self) -> usize {
        match self {
            BoatType::Single => 1,
            BoatType::Double | BoatType::Pair => 2,
            BoatType::Quad | BoatType::Four => 4,
            BoatType::FourCoxed => 5,
            BoatType::EightCoxed => 9,
        }
    }

    fn label(&self) -> &str {
        match self {
            BoatType::Single => "1x",
            BoatType::Double => "2x",
            BoatType::Pair => "2-",
            BoatType::Quad => "4x-",
            BoatType::Four => "4-",
            BoatType::FourCoxed => "4+",
            BoatType::EightCoxed => "8+",
        }
    }
}

// --- DEFINING A STRUCTURE TO REPRESENT EACH BOAT ON THE LAKE (AND HANDLE UI STUFF (pos)) ---
pub struct Boat {
    pub boat_type: BoatType,
    pub seats: Vec<Option<String>>,         // Each seat can be None (empty) or Some(student_id)
    pub pos: egui::Pos2,                    // Position of the "boat" on the frame
    pub departure_time: String,             // Departure time (e.g. "07:00")
    pub destination: Option<Destination>,   // Venue for specific boat (e.g. auchenstarry or strathclyde)
}

// --- FORM STATE FOR THE ADD PERSON POPUP WINDOW ---
#[derive(Default)]
pub struct AddPersonForm {
    name: String,
    gender: Option<Gender>,
    student_id: String,
    year_of_entry: String,
    pickup_locations: String,   // Comma-separated input
    can_drive_minibus: bool,
    has_car: bool,              // Whether the person owns a car
    car_type: String,
    car_registration: String,
    car_seats: String,
}

// --- FORM STATE FOR THE EDIT PERSON POPUP WINDOW ---
pub struct EditPersonForm {
    index: usize,               // Index into state.people for in-place updates
    name: String,
    gender: Option<Gender>,
    student_id: String,
    year_of_entry: String,
    pickup_locations: String,
    can_drive_minibus: bool,
    has_car: bool,
    wants_to_drive: bool,       // Whether the person has opted in to drive
    car_type: String,
    car_registration: String,
    car_seats: String,
}

// --- FORM STATE FOR THE EDIT MINIBUS POPUP WINDOW ---
pub struct EditMinibusForm {
    index: usize,               // Index into state.minibuses for in-place updates
    registration: String,
    seats: String,
}

// --- DEFINING THE MAIN APPLICATION STRUCTURE TO DESCRIBE THE SYSTEM ---
pub struct RowingApp {
    pub state: SystemState,                                // Contains all the people and minibuses data loaded from JSON
    pub boats: Vec<Boat>,                                  // Represents the boats currently on the frame and their assigned rowers
    pub selected_id: Option<String>,                       // Tracks which person (by student_id) is currently selected for training
    pub error_message: Option<String>,                     // Used to display error messages
    pub show_add_person: bool,                             // Controls whether the Add Person popup is open
    pub add_person_form: AddPersonForm,                    // Holds the in-progress input data for the Add Person form
    pub edit_person_form: Option<EditPersonForm>,          // Holds in-progress edits for a person
    pub edit_minibus_form: Option<EditMinibusForm>,        // Holds in-progress edits for a minibus
    pub wants_to_drive: std::collections::HashSet<String>, // Tracks which person IDs have "Wants to Drive" checked
}

impl eframe::App for RowingApp {

    // --- THE MAIN UPDATE LOOP WHERE ALL THE UI LOGIC HAPPENS ---
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let assigned_ids: std::collections::HashSet<String> = self.boats.iter()
                                                                        .flat_map(|b| b.seats.iter().flatten().cloned())
                                                                        .collect();

        // --- DISPLAY: ERROR WINDOW ---
        if let Some(msg) = self.error_message.clone() {     // Clones the error message to release the borrow on self.error_message
            egui::Window::new("Error")                      // Creates the error window
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(msg);                          // Puts the error message on the window
                    ui.add_space(10.0);                     // Adds some space before the close button
                    if ui.button("Close").clicked() {
                        self.error_message = None;
                    }
                });
        }

        // --- ADD PERSON POPUP WINDOW ---
        if self.show_add_person {

            let mut still_open = true;
            egui::Window::new("Add New Person")
                .collapsible(false)
                .resizable(false)
                .open(&mut still_open)
                .show(ctx, |ui| {   // The form for adding a new person

                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.add_person_form.name);

                    ui.label("Student ID:");
                    ui.text_edit_singleline(&mut self.add_person_form.student_id);

                    ui.label("Year of Entry:");
                    ui.text_edit_singleline(&mut self.add_person_form.year_of_entry);

                    ui.label("Pickup Locations (comma-separated):");
                    ui.text_edit_singleline(&mut self.add_person_form.pickup_locations);

                    ui.label("Gender:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.add_person_form.gender, Some(Gender::Male), "Male");
                        ui.selectable_value(&mut self.add_person_form.gender, Some(Gender::Female), "Female");
                    });

                    ui.checkbox(&mut self.add_person_form.can_drive_minibus, "Can drive minibus");

                    ui.separator();

                    // --- CAR OWNERSHIP TOGGLE — ungrey car fields only when "Has a car" is checked ---
                    ui.checkbox(&mut self.add_person_form.has_car, "Has a car");

                    ui.add_enabled_ui(self.add_person_form.has_car, |ui| {
                        ui.label("Car Type:");
                        ui.text_edit_singleline(&mut self.add_person_form.car_type);

                        ui.label("Car Registration:");
                        ui.text_edit_singleline(&mut self.add_person_form.car_registration);

                        ui.label("Car Seats:");
                        ui.text_edit_singleline(&mut self.add_person_form.car_seats);
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Add Person").clicked() {

                            // Attempt to build and save the new person from form data
                            if let Some(person) = Self::build_person_from_add_form(&self.add_person_form) {
                                self.state.people.push(person);
                                self.state.save_all().ok();

                                // Resets form and closes it
                                self.add_person_form = AddPersonForm::default();
                                self.show_add_person = false;
                            } else {
                                self.error_message = Some("Please fill in all required fields correctly.".to_string());
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            // Reset and close form on cancel
                            self.add_person_form = AddPersonForm::default();
                            self.show_add_person = false;
                        }
                    });
                });

            // Handles the X button on the window closing the popup cleanly
            if !still_open {
                self.add_person_form = AddPersonForm::default();
                self.show_add_person = false;
            }
        }

        // --- EDIT PERSON POPUP WINDOW ---
        let mut commit_edit_person = false;
        let mut cancel_edit_person = false;
        if let Some(form) = &mut self.edit_person_form {
            let mut still_open = true;
            egui::Window::new("Edit Person")
                .collapsible(false)
                .resizable(false)
                .open(&mut still_open)
                .show(ctx, |ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut form.name);

                    ui.label("Student ID:");
                    ui.text_edit_singleline(&mut form.student_id);

                    ui.label("Year of Entry:");
                    ui.text_edit_singleline(&mut form.year_of_entry);

                    ui.label("Pickup Locations (comma-separated):");
                    ui.text_edit_singleline(&mut form.pickup_locations);

                    ui.label("Gender:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut form.gender, Some(Gender::Male), "Male");
                        ui.selectable_value(&mut form.gender, Some(Gender::Female), "Female");
                    });

                    ui.checkbox(&mut form.can_drive_minibus, "Can drive minibus");

                    ui.separator();

                    // --- CAR OWNERSHIP TOGGLE — ungrey car fields only when "Has a car" is checked ---
                    ui.checkbox(&mut form.has_car, "Has a car");

                    ui.add_enabled_ui(form.has_car, |ui| {
                        ui.label("Car Type:");
                        ui.text_edit_singleline(&mut form.car_type);

                        ui.label("Car Registration:");
                        ui.text_edit_singleline(&mut form.car_registration);

                        ui.label("Car Seats:");
                        ui.text_edit_singleline(&mut form.car_seats);

                        // "Wants to Drive" — only shown when the person has their own car
                        ui.checkbox(&mut form.wants_to_drive, "Wants to Drive");
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked()   { commit_edit_person = true; }
                        if ui.button("Cancel").clicked() { cancel_edit_person = true; }
                    });
                });

            if !still_open { cancel_edit_person = true; }
        }

        // Apply edits or cancel outside the borrow to avoid double-borrow of self
        if commit_edit_person {
            self.apply_edit_person();
        } else if cancel_edit_person {
            self.edit_person_form = None;
        }

        // --- EDIT MINIBUS POPUP WINDOW ---
        let mut commit_edit_minibus = false;
        let mut cancel_edit_minibus = false;
        if let Some(form) = &mut self.edit_minibus_form {
            let mut still_open = true;
            egui::Window::new("Edit Minibus")
                .collapsible(false)
                .resizable(false)
                .open(&mut still_open)
                .show(ctx, |ui| {
                    ui.label("Registration:");
                    ui.text_edit_singleline(&mut form.registration);

                    ui.label("Seats:");
                    ui.text_edit_singleline(&mut form.seats);

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked()   { commit_edit_minibus = true; }
                        if ui.button("Cancel").clicked() { cancel_edit_minibus = true; }
                    });
                });

            if !still_open { cancel_edit_minibus = true; }
        }

        // Apply minibus edits or cancel outside the borrow
        if commit_edit_minibus {
            self.apply_edit_minibus();
        } else if cancel_edit_minibus {
            self.edit_minibus_form = None;
        }

        // --- SQUAD ---
        egui::SidePanel::left("Rowing Transport").show(ctx, |ui| {
            ui.heading("Transport Automation");                                 // Heading for the sidebar
            ui.add_space(4.0);
            ui.separator();                                                     // A seperator line to make it look cleaner   

            ui.heading("Squad");
            ui.add_space(4.0);

            // Collect indices to avoid borrowing issues when opening edit form or deleting
            let mut open_edit_for: Option<usize> = None;
            let mut delete_person_idx: Option<usize> = None;

            egui::ScrollArea::vertical().show(ui, |ui| {

                // --- HELPER FUNCTION TO LIST SQUADS BY GENDER ---
                let mut render_gender_group = |ui: &mut egui::Ui, heading: &str, gender: &Gender| {
                    ui.label(egui::RichText::new(heading).underline());
                    ui.add_space(2.0);

                    for p in self.state.people.iter().filter(|p| &p.gender == gender) {
                        let is_assigned = assigned_ids.contains(&p.student_id);
                        let is_selected = self.selected_id.as_ref() == Some(&p.student_id);

                        // Updates people in sidebar to indicate if they're assigned to a boat
                        let mut label_text = "".to_string();
                        if is_assigned {
                            label_text = format!("✔ {}", p.name);                                              // Displays a checkmark if the person is already assigned to a boat

                            if is_selected {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(50, 150, 50));  // Highlights selected person's name in green
                            }
                        } else {
                            label_text = p.name.clone();                                                            // Simply displays the persons name as intended
                        }

                        // Handles label selection and deselection logic when clicking a persons name
                        let label_resp = ui.selectable_label(is_selected, label_text);  // Creates a selectable label for each person

                        if label_resp.clicked() {
                            if is_selected && is_assigned {                         // logic to search through every boat and remove the selected person from their seat
                                for boat in &mut self.boats {
                                    for seat in &mut boat.seats {
                                        if seat.as_ref() == Some(&p.student_id) {
                                            *seat = None;                           // Setting the seat to none (removes the person from that boat)
                                            break;
                                        }
                                    }
                                }
                                self.selected_id = None;                            // Then simply deselects the persons ID after they've been removed
                            } else if is_selected {                                 // If the person is selected but not assigned then it deselects them
                                self.selected_id = None;
                            } else {
                                self.selected_id = Some(p.student_id.clone());      // Otherwise if they're being clicked for the first time then it selects them
                            }
                        }

                        ui.visuals_mut().override_text_color = None;    // Resets the colour override after each label to prevent all labels from being coloured when only one is selected
                    }

                    ui.add_space(6.0);
                };

                // Calling helper function for mens and womens squads in order to list them seperately (also makes possibly adding beginners easy)
                render_gender_group(ui, "Mens", &Gender::Male);
                render_gender_group(ui, "Womens", &Gender::Female);

                ui.add_space(8.0);

                // Create the add person button bellow the squads
                if ui.button("＋ Add Person").clicked() {
                    self.show_add_person = true;
                }

                // Find the index of the currently selected person (used by both Edit and Delete buttons)
                let selected_person_idx = self.selected_id.as_ref().and_then(|id| {
                    self.state.people.iter().position(|p| &p.student_id == id)
                });

                // Create the edit person button — enabled only when a person is selected
                ui.add_enabled_ui(selected_person_idx.is_some(), |ui| {             // If a person is selected then open the edit window for that ID index
                    if ui.button("✎ Edit Person").clicked() {
                        open_edit_for = selected_person_idx;
                    }
                });

                // Create the delete person button — enabled only when a person is selected
                ui.add_enabled_ui(selected_person_idx.is_some(), |ui| {             // Only active when someone is selected, same pattern as Edit
                    if ui.button("🗑 Delete Person").clicked() {
                        delete_person_idx = selected_person_idx;
                    }
                });

                ui.add_space(12.0);
                ui.separator();

                // --- MINIBUSES SECTION ---
                ui.heading("Minibuses");
                ui.add_space(4.0);

                // Collect minibus edit requests to avoid borrow issues
                let mut open_minibus_edit_for: Option<usize> = None;

                for (mb_idx, mb) in self.state.minibuses.iter().enumerate() {

                    // Selecting the last 3 characters of the registration
                    let short_reg = if mb.registration.len() >= 3 {
                        &mb.registration[mb.registration.len() - 3..]
                    } else {
                        &mb.registration
                    };

                    // Displaying the last characters and seat count of associated minibus
                    ui.horizontal(|ui| {
                        ui.label(format!("{} — {} seats", short_reg, mb.seats));
                        if ui.small_button("Edit").clicked() {
                            open_minibus_edit_for = Some(mb_idx);
                        }
                    });
                }

                ui.add_space(8.0);

                // If there is a minibus edit request, this simply opens the edit window for that specific minibus and populates the form
                if let Some(mb_idx) = open_minibus_edit_for {
                    let mb = &self.state.minibuses[mb_idx];
                    self.edit_minibus_form = Some(EditMinibusForm {
                        index: mb_idx,
                        registration: mb.registration.clone(),
                        seats: mb.seats.to_string(),
                    });
                }
            });

            // If there is a minibus edit request then this opens up the window for that specific person and popualates the form
            if let Some(idx) = open_edit_for {
                let p = &self.state.people[idx];
                let has_car = p.car.is_some();
                self.edit_person_form = Some(EditPersonForm {
                    index: idx,
                    name: p.name.clone(),
                    gender: Some(p.gender.clone()),
                    student_id: p.student_id.clone(),
                    year_of_entry: p.year_of_entry.to_string(),
                    pickup_locations: p.pickup_locations.join(", "),
                    can_drive_minibus: p.can_drive_minibus,
                    has_car,
                    wants_to_drive:   self.wants_to_drive.contains(&p.student_id),  // Pre-populate from current state
                    car_type:         p.car.as_ref().map(|c| c.vehicle_type.clone()).unwrap_or_default(),
                    car_registration: p.car.as_ref().map(|c| c.registration.clone()).unwrap_or_default(),
                    car_seats:        p.car.as_ref().map(|c| c.seats.to_string()).unwrap_or_default(),
                });
            }

            // Handle deletion outside the scroll area borrow — removes the person, clears their seat
            // assignments from any boats, removes their wants_to_drive entry, and saves
            if let Some(idx) = delete_person_idx {
                let removed_id = self.state.people[idx].student_id.clone();

                self.state.people.remove(idx);                              // Remove the person from the squad

                for boat in &mut self.boats {                               // Clear any seat they were assigned to on the lake
                    for seat in &mut boat.seats {
                        if seat.as_ref() == Some(&removed_id) {
                            *seat = None;
                        }
                    }
                }

                self.wants_to_drive.remove(&removed_id);                   // Remove their wants-to-drive preference

                if self.selected_id.as_ref() == Some(&removed_id) {        // Deselect them if they were selected
                    self.selected_id = None;
                }

                self.state.save_all().ok();                                 // Persist the change to JSON
            }
        });

        // --- MAIN FRAME ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {                        // Top horizontal bar for adding boats and publishing
                ui.label("Add Boat:");
                let types = [
                    BoatType::Single, 
                    BoatType::Double, 
                    BoatType::Pair,
                    BoatType::Quad, 
                    BoatType::Four, 
                    BoatType::FourCoxed, 
                    BoatType::EightCoxed
                ];

                // logic for each boat to instantiate a boat of that type and add it to the frame
                for t in types {
                    if ui.button(t.label()).clicked() { self.add_boat(t); }
                }
                
                ui.separator();                                                             // A seperator line to make it look cleaner
                if ui.button("Publish & PDF").clicked() { self.validate_and_publish(); }    // Button to trigger the validation and PDF generation process
                if ui.button("Clear Lake")   .clicked() { self.boats.clear(); }             // Button to clear all boats from the frame
            });
            
            for (b_idx, boat) in self.boats.iter_mut().enumerate() {

                // Calculating the dimensions for the boat based on the number of seats
                let num_seats     = boat.seats.len();
                let seat_spacing  = 40.0;
                let boat_width    = 30.0; 
                let header_height = 60.0; 
                let total_height  = (num_seats as f32 * seat_spacing) + 140.0 + header_height; 
                
                // Calculating edge points for the boat based on the position and calculated height
                let top    = boat.pos.y - (total_height / 2.0);
                let bottom = boat.pos.y + (total_height / 2.0);
                let left   = boat.pos.x - (boat_width   / 2.0);
                let right  = boat.pos.x + (boat_width   / 2.0);

                // --- INPUTS ABOVE THE BOAT ---
                // handling the main box to hold both the input lines for departure time and destination selection
                let input_rect = egui::Rect::from_min_size(
                    egui::pos2(left - 40.0, top),           // Positioning the box
                    egui::vec2(110.0, 60.0)                 // Size of the input box
                );
                
                // configuring what the box contains (input lines)
                ui.put(input_rect, |ui: &mut egui::Ui| {
                    ui.vertical(|ui| {
                        ui.set_width(100.0);                                                                                    // Setting a fixed width for the input area
                        ui.text_edit_singleline(&mut boat.departure_time).on_hover_text("Departure Time (e.g. 07:00)");         // Input for departure time
                        
                        egui::ComboBox::from_id_source(b_idx)
                            .selected_text(boat.destination.map(|d| d.label()).unwrap_or("Location"))                           // Dropdown for destination selection
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut boat.destination, Some(Destination::StrathclydePark), "Strathclyde");
                                ui.selectable_value(&mut boat.destination, Some(Destination::Auchenstarry),    "Auchenstarry");
                            });
                    })
                    .response   // Adding a response to the entire input area to allow dragging the boat by clicking and dragging on the inputs as well
                });

                // --- DRAWING THE BOAT ITSELF ---
                let painter = ui.painter();                     // Instantiating the painter

                let shell_start_y = top + header_height;        // Starting Y position for the shell (where the first seat is - below the input area)
                let shell_points  = vec![                       // Defining the points for the polygon to draw the boat shell
                    egui::pos2(boat.pos.x, shell_start_y), 
                    egui::pos2(right, shell_start_y + 75.0),
                    egui::pos2(right, bottom - 75.0), 
                    egui::pos2(boat.pos.x, bottom),
                    egui::pos2(left, bottom - 75.0), 
                    egui::pos2(left, shell_start_y + 75.0),
                ];

                // Drawing the boat shell as a filled polygon with a stroke
                painter.add(egui::Shape::convex_polygon(
                    shell_points, egui::Color32::from_rgb(30, 30, 30),              // Fill color for the boat
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(200, 170, 40))   // Thickness and color for the boat outline
                ));

                // --- DRAWING SEATS ---
                for (s_idx, seat) in boat.seats.iter_mut().enumerate() {
                    let seat_pos = egui::pos2(boat.pos.x, (shell_start_y + 85.0) + (s_idx as f32 * seat_spacing));                              // Positioning each seat with some spacing below the boat shell
                    let is_cox = (boat.boat_type == BoatType::FourCoxed || boat.boat_type == BoatType::EightCoxed) && s_idx == (num_seats - 1); // Identifying if the current seat is the coxswain seat (last seat in coxed boats)

                    let seat_hitbox = egui::Rect::from_center_size(seat_pos, egui::vec2(30.0, 30.0));   // Creating a hitbox around each seat (for clicking and assigning passengers)
                    
                    // Handling the logic for when a seat is clicked to assign or unassign a person
                    if ui.rect_contains_pointer(seat_hitbox) && ui.input(|i| i.pointer.any_click()) {
                        if let Some(id) = &self.selected_id {
                            *seat            = Some(id.clone());
                            self.selected_id = None;
                        }
                    }

                    // Defining colours for seats based on their status
                    let color = if seat.is_some() { egui::Color32::from_rgb(50, 200, 50) }  // Seats that are filled with a person are colored green 
                                else if is_cox { egui::Color32::from_rgb(180, 60, 60) }     // Coxswain seats that are empty are colored red
                                else { egui::Color32::WHITE };                              // Regular empty seats are colored white
                    
                    painter.circle_filled(seat_pos, 10.0, color);   // Drawing the seat as a filled circle with the determined color

                    // Determining the name of each seat to be displayed on the boat
                    let name  = seat.as_ref().and_then(|id| self.state.get_person(id)).map(|p| p.name.as_str()).unwrap_or("—"); // Getting the name of the assigned person for that seat (or "-" if the seat is empty)
                    let has_cox: bool = (boat.boat_type == BoatType::FourCoxed || boat.boat_type == BoatType::EightCoxed);

                    let mut label; // Initializing the label (name to be given to the seat)
                    if is_cox {
                        label = format!("COX: {}", name);           // Label for coxswain seat
                    } else if s_idx == 0 {
                        label = format!("BOW: {}", name);           // Label for bow seat
                    } else if (has_cox && s_idx == num_seats - 2) {
                        label = format!("STR: {}", name);           // Label for stroke seat - condition that boat is coxed
                    } else {
                        if(s_idx == num_seats - 1) {
                            label = format!("STR: {}", name);       // Label for stroke seat - condition that boat is coxless
                        } else {
                        label = format!("{}: {}", s_idx + 1, name); // Label for regular seats (e.g. "2: Alice")
                        }
                    }

                    // Drawing the label for each seat (Showing each seat number)
                    painter.text(seat_pos + egui::vec2(28.0, 0.0), 
                                 egui::Align2::LEFT_CENTER, 
                                 label, 
                                 egui::FontId::proportional(14.0), 
                                 egui::Color32::WHITE);
                }

                // --- DRAGGING BOATS LOGIC ---
                // Positioning the draggable area for the boat (the entire boat including the input area)
                let boat_rect = egui::Rect::from_center_size(egui::pos2(boat.pos.x,
                                                                        boat.pos.y + (header_height/2.0)),
                                                                        egui::vec2(boat_width, total_height));
                
                // Adding an interaction response to the boat area to allow dragging the boat around the frame
                let resp      = ui.interact(boat_rect, 
                                            ui.make_persistent_id(format!("boat_{}", b_idx)), 
                                            egui::Sense::drag());

                // Updating the boat's position based on the drag response
                if resp.dragged() { boat.pos += resp.drag_delta(); }
            }
        });
    }
}

// --- IMPLEMENTING METHODS FOR THE MAIN APPLICATION STRUCTURE ---
impl RowingApp {

    // --- METHOD TO ADD A NEW BOAT TO THE FRAME BASED ON THE SELECTED BOAT TYPE ---
    fn add_boat(&mut self, bt: BoatType) {
        self.boats.push(Boat { 
            boat_type: bt, 
            seats: vec![None; bt.seat_count()], 
            pos: egui::pos2(400.0, 300.0),
            departure_time: String::new(),
            destination: None,
        });
    }

    // --- ERROR CHECK METHOD FOR MISSING DEPARTURE TIMES, DESTINATIONS, OR UNFILLED SEATS ---
    fn validate_and_publish(&mut self) {
        if self.boats.iter().any(|b| b.departure_time.trim().is_empty()) {
            self.error_message = Some("Error: All boat departure times must be entered.".to_string());
            return;
        }
        if self.boats.iter().any(|b| b.destination.is_none()) {
            self.error_message = Some("Error: All destination locations must be selected.".to_string());
            return;
        }
        if self.boats.iter().any(|b| b.seats.iter().any(|s| s.is_none())) {
            self.error_message = Some("Error: All boat seats must be filled.".to_string());
            return;
        }

        self.publish();
    }

    // --- METHOD TO HANDLE THE LOGIC FOR PUBLISHING THE FINAL ALLOCATIONS AND GENERATING THE PDF ---
    fn publish(&mut self) {

        // Grouping people by their destination and departure time to prepare for the allocation algorithm
        let mut groups: Vec<(Destination, String, Vec<Person>)> = Vec::new();

        // Creating groups based on the boats currently on frame and grouping based on destination and departure time
        for boat in &self.boats {
            let dest = boat.destination.unwrap();
            let time = boat.departure_time.clone();
            
            let people: Vec<Person> = boat.seats.iter()
                .filter_map(|s| s.as_ref())
                .filter_map(|id| self.state.get_person(id).cloned())
                .collect();

            if let Some(existing) = groups.iter_mut().find(|(d, t, _)| *d == dest && *t == time) {
                existing.2.extend(people);
            } else {
                groups.push((dest, time, people));
            }
        }

        // Getting the current date for transport sheet creation and documentation
        let curr_date = Local::now().format("%Y-%m-%d").to_string();

        // Handing off results of UI to other allocation algorithm and PDF generation,
        match Allocation::assign_transport_global(groups, &self.state.minibuses, &self.wants_to_drive) {

            // If all allocations could be done successfully
            Ok(all_allocations) => {
                if let Err(e) = crate::pdf::generate_pdf(&all_allocations, &format!("transport_sheet_{}.pdf", curr_date)) {
                    self.error_message = Some(format!("PDF Generation failed: {}", e));     // Displaying an error message if PDF generation fails
                } else {
                    self.error_message = Some("PDF generated successfully.".to_string());   // Displaying a success message (using error message logic...)
                }
            }

            // If there was an error during the allocation that caused someone to be missed
            Err(unallocated_names) => {
                self.error_message = Some(format!(
                    "Error: Not everyone could be assigned -\n{}", 
                    unallocated_names.join(",\n")
                ));
            }
        }
    }

    // --- HELPER FUNCTION TO BUILD PERSON STRUCT FROM ADD PERSON FORM ---
    fn build_person_from_add_form(form: &AddPersonForm) -> Option<Person> {

        // Formatting the input data from the form before validation
        let name       = form.name.trim().to_string();
        let student_id = form.student_id.trim().to_string();
        let gender     = form.gender.clone()?;

        // Basic input validation for main person fields
        if name.is_empty() || student_id.is_empty() { return None; }
        let year_of_entry: u16 = form.year_of_entry.trim().parse().ok()?; 

        let pickup_locations: Vec<String> = form.pickup_locations           // Formatting entered pickup locations
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let car = if form.has_car {                                         // Build car details only if the person has a car and all car fields are filled
            let seats: u8 = form.car_seats.trim().parse().ok()?;
            Some(crate::models::Car {
                vehicle_type: form.car_type.trim().to_string(),
                registration: form.car_registration.trim().to_string(),
                seats,
            })
        } else {
            None                                                            // Returns none if for some reason validation fails
        };

        Some(Person {                                                       // Instantiating the NEW person
            name,
            gender,
            student_id,
            year_of_entry,
            pickup_locations,
            car,
            can_drive_minibus: form.can_drive_minibus,
        })
    }

    // --- APPLY EDITS FROM THE EDIT PERSON FORM BACK INTO STATE ---
    fn apply_edit_person(&mut self) {

        if let Some(form) = &self.edit_person_form {

            // Copying form data into local variables and formatting it before applying it back into system state
            let year_of_entry: u16 = form.year_of_entry.trim().parse().unwrap_or(0);
            let pickup_locations: Vec<String> = form.pickup_locations
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            // Doing the same data input parsing but for the optional car fields
            let car = if form.has_car {
                let seats: u8 = form.car_seats.trim().parse().unwrap_or(0);

                Some(crate::models::Car {                                   // Building a car struct from the form data if the person has a car
                    vehicle_type: form.car_type.trim().to_string(),
                    registration: form.car_registration.trim().to_string(),
                    seats,
                })

            } else {
                None
            };

            if let Some(gender) = form.gender.clone() {
                let idx = form.index;
                if idx < self.state.people.len() {                          // Safety check to ensure the index is within bounds of the people vector
                    self.state.people[idx] = Person {                       // Applying edits from the form back into the main system state at the correct index
                        name: form.name.trim().to_string(),
                        gender,
                        student_id: form.student_id.trim().to_string(),
                        year_of_entry,
                        pickup_locations,
                        car,
                        can_drive_minibus: form.can_drive_minibus,
                    };
                    self.state.save_all().ok();                             // Saving the updated state back to the JSON files

                    let sid = self.state.people[idx].student_id.clone();    // Handling the "Wants to Drive" logic back to app-level
                    if form.wants_to_drive && form.has_car {
                        self.wants_to_drive.insert(sid);                    // Inserts student ID into the list of IDs that want to drive
                    } else {
                        self.wants_to_drive.remove(&sid);
                    }
                }
            }
        }
        self.edit_person_form = None;                                       // Closing the edit person window after applying edits
    }

    // --- APPLY EDITS FROM THE EDIT MINIBUS FORM BACK INTO STATE ---
    fn apply_edit_minibus(&mut self) {
        if let Some(form) = &self.edit_minibus_form {
            let seats: u8 = form.seats.trim().parse().unwrap_or(0);         // Parsing the seat input (or 0 if fails)
            let idx = form.index;
            if idx < self.state.minibuses.len() {                           // Safety check to ensure the index is within bounds of the minibuses vector
                self.state.minibuses[idx] = Minibus {                       // Updating edits from the form back into the main system state at the correct index
                    registration: form.registration.trim().to_string(),
                    seats,
                };
                self.state.save_all().ok();
            }
        }
        self.edit_minibus_form = None;                                      // Closing the minibus edit window
    }
}