use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    id: u32,
    name: String,
    age: u32
}

impl Person {
    pub fn new(id: u32, name: String, age: u32) -> Self {
        Person { id, name, age }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}