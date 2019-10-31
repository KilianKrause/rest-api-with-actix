use std::fs;
use serde_json;

use crate::person::Person;

const FILE_NAME: &str = "data.json";

pub fn get(id: u32) -> Result<Person, String> {
    let persons = read_values_from_file();
    for person in &persons {
        if person.id() == id {
            // TODO: return person here
        }
    }
    let err_msg = format!("Person with id {} does not exist.", id);
    Err(err_msg)
}

pub fn get_all() -> Vec<Person> {
    read_values_from_file()
}

pub fn create(person: Person) -> Result<String, String> {
    let mut persons = read_values_from_file();
    for persisted_person in &persons {
        if person.id() == persisted_person.id() {
            let err_msg = format!("Person with id {} already exists.", person.id());
            return Err(err_msg);
        }
    }
    persons.push(person);
    write_values_to_file(persons);
    Ok("successfully created.".to_owned())
}

pub fn update(person: Person) -> Result<String, String> {
    let persons = read_values_from_file();
    for persisted_person in &persons {
        if person.id() == persisted_person.id() {
            // TODO: update person here
            write_values_to_file(persons);
            return Ok("Updated successfully".to_owned()); // without return statement
        }
    }
    let err_msg = format!("Person with id {} does not exist.", person.id());
    Err(err_msg)
}

pub fn delete(id: u32) -> Result<String, String> {
    let persons = read_values_from_file();
    for person in &persons {
        if person.id() == id {
            // TODO: delete person here
            write_values_to_file(persons);
            return Ok("success".to_owned()); // without return statement
        }
    }
    let err_msg = format!("Person with id {} does not exist.", id);
    Err(err_msg)
}

fn read_values_from_file() -> Vec<Person> {
    let data = fs::read_to_string(FILE_NAME).unwrap();
    let persons: Vec<Person> = serde_json::from_str(&data).unwrap();
    persons
}

fn write_values_to_file(persons: Vec<Person>) {
    let data = serde_json::to_string(&persons).unwrap();
    fs::write(FILE_NAME, data).unwrap();
}
