use crate::models::{Person, Minibus, Gender, Destination};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TransportGroup {
    pub driver: Person,
    pub vehicle_label: String,
    pub passengers: Vec<Person>,
    pub capacity: usize,
    pub pickup_location: String,
    pub destination: Destination,
    pub departure_time: String,
}

pub struct Allocation;

impl Allocation {
    pub fn assign_transport_global(
        requests: Vec<(Destination, String, Vec<Person>)>,  // List of (Destination, Departure Time, People Requesting Transport)
        minibuses: &[Minibus],                              // List of available minibuses with their capacities
        wants_to_drive: &HashSet<String>,                   // Set of student IDs who have opted in to drive
    ) -> Result<Vec<TransportGroup>, Vec<String>> {
        
        // Final list of transport groups to be returned
        let mut final_allocations = Vec::new();

        // Collecing people from the requests
        let mut all_requested_people: Vec<Person> = Vec::new();
        for (_, _, people) in &requests {
            all_requested_people.extend(people.clone());
        }
        
        let mut available_minibuses = minibuses.to_vec();

        // Sort minibuses by capacity descending to create a priority queue for larger vehicles
        available_minibuses.sort_by(|a, b| b.seats.cmp(&a.seats));

        for (dest, time, mut group_people) in requests {
            
            // --- FILLING MINIBUSSES FIRST ---
            while group_people.len() > 1 && !available_minibuses.is_empty() {                   // As long as there are still people to allocate and a minibus left
                
                let d_idx = Self::find_willing_minibus_driver(&group_people, wants_to_drive)    // collect a willing minibus driver in the group
                    .or_else(|| group_people.iter().position(|p| p.can_drive_minibus));

                if let Some(d_idx) = d_idx {                    // If we found a driver (willing or not), allocate them to the minibus and fill up with passengers
                    let mb = available_minibuses.remove(0);
                    let driver = group_people.remove(d_idx);
                    let cap = mb.seats as usize;
                    
                    let mut t_group = TransportGroup {          // Create a new transport group for this minibus allocation
                        driver: driver.clone(),
                        vehicle_label: format!("Minibus {}", mb.registration),
                        passengers: Vec::new(),
                        capacity: cap,
                        pickup_location: "Pleasance".to_string(),
                        destination: dest,
                        departure_time: time.clone(),
                    };

                    // Fill the minibus with passengers
                    while t_group.passengers.len() < (cap - 1) && !group_people.is_empty() {
                        let p_idx = Self::find_best_passenger(&group_people, &t_group);
                        t_group.passengers.push(group_people.remove(p_idx));
                    }

                    final_allocations.push(t_group);    // Adding the filled minibus group to final allocations
                
                } else {
                    break;                              // No more minibus drivers in this group
                }
            }

            // --- FILL PERSONAL CARS (AGAIN, LARGEST FIRST) ---
            while !group_people.is_empty() {

                let best_driver_idx = Self::find_willing_car_driver(&group_people, wants_to_drive)  // First collect the people who actively want to drive their own car
                    .or_else(|| {
                        group_people.iter().enumerate()                         // Find the driver with the largest car capacity to minimize vehicle count
                            .filter(|(_, p)| p.car.is_some())
                            .max_by_key(|(_, p)| p.car.as_ref().unwrap().seats)
                            .map(|(idx, _)| idx)
                    });

                if let Some(d_idx) = best_driver_idx {                          // If we found a driver collect them and their car
                    let driver = group_people.remove(d_idx);
                    let car = driver.car.as_ref().unwrap().clone();
                    
                    let mut t_group = TransportGroup {                          // Allocating the driver and car to transport sheet
                        driver: driver.clone(),
                        vehicle_label: format!("Car {} ({})", car.registration, car.vehicle_type),
                        passengers: Vec::new(),
                        capacity: car.seats as usize,
                        pickup_location: driver.pickup_locations.first().cloned().unwrap_or("Home".to_string()),
                        destination: dest,
                        departure_time: time.clone(),
                    };

                    // Filling the car with passengers
                    while t_group.passengers.len() < (t_group.capacity - 1) && !group_people.is_empty() {
                        let p_idx = Self::find_best_passenger(&group_people, &t_group);
                        t_group.passengers.push(group_people.remove(p_idx));
                    }

                    final_allocations.push(t_group);                            // Adding the filled car to the final transport sheet
                
                } else {
                    break;                                                      // No more drivers available for remaining people 
                }
            }
        }

        // --- VALIDATION ---
        let allocated_ids: HashSet<String> = final_allocations.iter()                                   // Collecting all allocated peoples IDs (passengers and drivers)
            .flat_map(|g| {
                let mut ids: Vec<String> = g.passengers.iter().map(|p| p.student_id.clone()).collect();
                ids.push(g.driver.student_id.clone());
                ids
            })
            .collect();

        let unallocated_names: Vec<String> = all_requested_people.into_iter()                           // Collecting any names of people who aren't allocated to anything
            .filter(|p| !allocated_ids.contains(&p.student_id))
            .map(|p| p.name)
            .collect();

        if unallocated_names.is_empty() {
            Ok(final_allocations)                   // If everyone is allocated, return the final transport groups
        } else {
            Err(unallocated_names)                  // If not, return an error with the list of unallocated people
        }
    }

    // --- FIND THE BEST WILLING MINIBUS DRIVER (Wants to drive and can drive the minibus ---
    fn find_willing_minibus_driver(pool: &[Person], wants_to_drive: &HashSet<String>) -> Option<usize> {
        pool.iter().position(|p| p.can_drive_minibus && !wants_to_drive.contains(&p.student_id))
    }

    // --- FIND THE BEST WILLING CAR DRIVER (Must have their OWN car AND have opted in) ---
    fn find_willing_car_driver(pool: &[Person], wants_to_drive: &HashSet<String>) -> Option<usize> {
        pool.iter().enumerate()
            .filter(|(_, p)| p.car.is_some() && wants_to_drive.contains(&p.student_id))
            .max_by_key(|(_, p)| p.car.as_ref().unwrap().seats)
            .map(|(idx, _)| idx)
    }

    // --- FIND THE BEST PASSENGER TO FILL A VEHICLE (Based on location match, gender balance, and time at the club) ---
    fn find_best_passenger(pool: &[Person], group: &TransportGroup) -> usize {

        // Finding out the current gender distribution within the car being filled
        let driver_gender                   = &group.driver.gender;
        let passenger_genders: Vec<&Gender> = group.passengers.iter().map(|p| &p.gender).collect();
        
        // Counting the number of men and women and then deciding which gender is preferred for the transport based on which is less (aiming for 5050)
        let males   = passenger_genders.iter().filter(|g| ***g == Gender::Male).count() + (if *driver_gender == Gender::Male {1} else {0});
        let females = passenger_genders.iter().filter(|g| ***g == Gender::Female).count() + (if *driver_gender == Gender::Female {1} else {0});
        let preferred_gender = if males > females { Gender::Female } else { Gender::Male };

        // Simple find max of a caclulated score based on commonality of location, gender and years at the club
        let mut best_idx = 0;
        let mut best_score = -1;

        for (idx, p) in pool.iter().enumerate() {
            let mut score = 0;

            // Biggest priority - matching location
            if p.pickup_locations.contains(&group.pickup_location) {
                score += 1000;
            }

            // Middle priority - years and experience
            score += (3000 - p.year_of_entry as i32);       // Currently 974 (as of 2026)

            // Lowest priority - gender balancing
            if p.gender == preferred_gender {
                score += 100;
            }

            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }
        best_idx
    }
}