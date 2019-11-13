use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
}

impl Person {
    pub fn new(id: u32, name: String, age: u32) -> Self {
        Person { id, name, age }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewPerson {
    pub name: String,
    pub age: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePerson {
    pub name: Option<String>,
    pub age: Option<u32>,
}
