use serde_json;
use std::fs;

use crate::person::{NewPerson, Person, UpdatePerson};

const FILE_NAME: &str = "data.json";

pub fn get(id: u32) -> Option<Person> {
    let persons = read_values_from_file();

    persons.into_iter().find(|p| p.id() == id)
}

pub fn get_all() -> Vec<Person> {
    read_values_from_file()
}

pub fn create(new_person: NewPerson) -> Result<(), String> {
    let mut persons = read_values_from_file();
    let id = calculate_new_id(&persons);
    persons.push(Person::new(id, new_person.name, new_person.age));
    write_values_to_file(persons);
    Ok(())
}

pub fn update(id: u32, person: UpdatePerson) -> Result<(), String> {
    let mut persons = read_values_from_file();

    if let Some(index) = persons.iter().position(|person| person.id() == id) {
        let old = persons.remove(index);
        persons.insert(
            index,
            Person::new(
                id,
                person.name.unwrap_or_else(|| old.name().clone()),
                person.age.unwrap_or_else(|| old.age()),
            ),
        );
        write_values_to_file(persons);
        return Ok(());
    }
    Err(format!("Person with id {} does not exist.", id))
}

pub fn delete(id: u32) -> Result<(), String> {
    let mut persons = read_values_from_file();

    if let Some(index) = persons.iter().position(|person| person.id() == id) {
        persons.remove(index);
        write_values_to_file(persons);
        return Ok(());
    }
    Err(format!("Person with id {} does not exist.", id))
}

fn calculate_new_id(persons: &[Person]) -> u32 {
    let last_id = persons.get(persons.len() - 1).unwrap().id(); // get id of the last person in vec
    last_id + 1
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
