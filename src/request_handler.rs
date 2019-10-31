use actix_web::{Error, error, HttpResponse};
use actix_web::web::{Json, Path};
use actix_web::{get, delete, post, put};

use crate::person::Person;
use crate::person_repository;

#[get("/persons")]
pub fn get_all() -> Result<HttpResponse, Error> {
    let persons = person_repository::get_all();
    if !persons.is_empty() {
        Ok(HttpResponse::Ok().json(persons))
    } else {
        Err(error::ErrorNotFound("No persons persisted yet.".to_owned()))
    }
}

#[get("/persons/{id}")]
pub fn get(id: Path<u32>) -> Result<HttpResponse, Error> {
    let person = person_repository::get(*id);
    match person {
        Ok(found) => Ok(HttpResponse::Ok().json(found)),
        Err(e) => Err(error::ErrorNotFound(e)),
    }
}

#[delete("/persons/{id}")]
pub fn delete(id: Path<u32>) -> Result<HttpResponse, Error> {
    match person_repository::delete(*id) {
        Ok(msg) => Ok(HttpResponse::from(msg)),
        Err(err_msg) => Err(error::ErrorNotFound(err_msg))
    }
}

#[post("/persons")]
pub fn create(person: Json<Person>) -> Result<HttpResponse, Error> {
    match person_repository::create(person.into_inner()) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Created())),
        Err(err_msg) => Err(error::ErrorConflict(err_msg))
    }
}

#[put("/persons/{id}")]
pub fn update(id: Path<u32>, person: Json<Person>) -> Result<HttpResponse, Error> {
    println!("{}", id);
    match person_repository::update(person.into_inner()) {
        Ok(msg) => Ok(HttpResponse::from(msg)),
        Err(err_msg) => Err(error::ErrorNotFound(err_msg))
    }
}
