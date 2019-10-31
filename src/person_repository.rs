use std::fs;
use serde_json;

use crate::person::Person;

pub fn get(id: u32) -> Result<Person, String> {
    Err("TODO".to_owned())
}

pub fn get_all() -> Vec<Person> {
    read_values_from_file()
}

pub fn create(person: Person) {

}

pub fn update(person: Person) -> Result<String, String> {
    Err("TODO".to_owned())
}

pub fn delete(id: u32) -> Result<String, String> {
    Err("TODO".to_owned())
}

fn read_values_from_file() -> Vec<Person> {
    let data = fs::read_to_string("data.json").unwrap();
    let persons: Vec<Person> = serde_json::from_str(&data).unwrap();
    persons
}
