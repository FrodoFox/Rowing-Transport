use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write, BufReader};

use crate::models::{Person, Minibus};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SystemState {
    pub people: Vec<Person>,
    pub minibuses: Vec<Minibus>,
}

impl SystemState {
    const PEOPLE_FILE: &'static str = "people.json";
    const MINIBUSES_FILE: &'static str = "minibuses.json";

    pub fn load_all(&mut self) -> io::Result<()> {
        self.load_people(Self::PEOPLE_FILE)?;
        self.load_minibuses(Self::MINIBUSES_FILE)?;
        Ok(())
    }

    pub fn save_all(&self) -> io::Result<()> {
        self.save_people(Self::PEOPLE_FILE)?;
        self.save_minibuses(Self::MINIBUSES_FILE)?;
        Ok(())
    }

    pub fn load_people(&mut self, filename: &str) -> io::Result<()> {
        if let Ok(file) = File::open(filename) {
            let reader  = BufReader::new(file);
            self.people = serde_json::from_reader(reader).unwrap_or_default();
        } else {
            self.people = Vec::new();
        }
        Ok(())
    }

    pub fn save_people(&self, filename: &str) -> io::Result<()> {
        let data     = serde_json::to_string_pretty(&self.people).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())
    }

    pub fn load_minibuses(&mut self, filename: &str) -> io::Result<()> {
        if let Ok(file)    = File::open(filename) {
            let reader     = BufReader::new(file);
            self.minibuses = serde_json::from_reader(reader).unwrap_or_default();
        } else {
            self.minibuses = Vec::new();
        }
        Ok(())
    }

    pub fn save_minibuses(&self, filename: &str) -> io::Result<()> {
        let data     = serde_json::to_string_pretty(&self.minibuses).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())
    }

    pub fn get_person(&self, id: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.student_id == id)
    }
}
