use actix_web::{Error, error, HttpResponse};
use actix_web::web::{Json, Path};
use actix_web::{get, delete, post, put};

use crate::person::Person;

#[get("/persons")]
pub fn get_all() -> Result<HttpResponse, Error> {
    let micheal = Person::new(1, "Micheal".to_owned(), 32);
    let frank = Person::new(2, "Frank".to_owned(), 28);
    let persons = vec![micheal, frank];
    if all_correct() {
        Ok(HttpResponse::Ok().json(persons))
    } else {
        Err(error::ErrorInternalServerError("bad"))
    }
}

#[get("/persons/{id}")]
pub fn get(id: Path<u32>) -> Result<HttpResponse, Error> {
    // Communication with persistence layer
    let person = Person::new(*id, "Tom".to_owned(), 38);
    if *id == 123 {
        Ok(HttpResponse::Ok().json(person))
    } else {
        let err_msg = format!("person with id {} not found", id);
        Err(error::ErrorNotFound(err_msg))
    }
}

#[delete("/persons/{id}")]
pub fn delete(id: Path<u32>) -> Result<HttpResponse, Error> {
    if *id == 123 {
        Ok(HttpResponse::from("Delete successfully."))
    } else {
        let err_msg = format!("person with id {} not found", id);
        Err(error::ErrorNotFound(err_msg))
    }
}

#[post("/persons")]
pub fn create(person: Json<Person>) -> Result<HttpResponse, Error> {
    // Communication with persistence layer
    if all_correct() {
        Ok(HttpResponse::from(HttpResponse::Created()))
    } else {
        let err_msg = format!("Person with name {} already exists.", person.name());
        Err(error::ErrorConflict(err_msg))
    }
}

#[put("/persons/{id}")]
pub fn update(id: Path<u32>, person: Json<Person>) -> Result<HttpResponse, Error> {
    if *id == 123 {
        // Communication with DB
        Ok(HttpResponse::from("Update successfully."))
    } else {
        let err_msg = format!("person with id {} not found", id);
        Err(error::ErrorNotFound(err_msg))
    }
}

fn all_correct() -> bool {
    true
}