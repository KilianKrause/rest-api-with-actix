use actix_web::{HttpResponse};
use actix_web::web::{Json, Path};
use actix_web::{get, delete, post, put};
use serde_json::json;

use crate::person::{NewPerson, UpdatePerson};
use crate::person_repository;
use crate::error::Error;

#[get("/persons")]
pub fn get_all() -> Result<HttpResponse, Error> {
    let persons = person_repository::get_all();
    if !persons.is_empty() {
        Ok(HttpResponse::Ok().json(persons))
    } else {
        let json_err = json!({"error" : "No persons persisted yet."});
        Err(Error::NotFound(json_err))
    }
}

#[get("/persons/{id}")]
pub fn get(id: Path<u32>) -> Result<HttpResponse, Error> {
    let person = person_repository::get(*id);
    match person {
        Some(found) => Ok(HttpResponse::Ok().json(found)),
        None => {
            let err_msg = format!("Person with id {} does not exist.", id);
            let json_err = json!({"error" : err_msg});
            Err(Error::NotFound(json_err))
        }
    }
}

#[delete("/persons/{id}")]
pub fn delete(id: Path<u32>) -> Result<HttpResponse, Error> {
    match person_repository::delete(*id) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Ok())),
        Err(err_msg) => Err(Error::NotFound(json!({"error" : err_msg})))
    }
}

#[post("/persons")]
pub fn create(person: Json<NewPerson>) -> Result<HttpResponse, Error> {
    match person_repository::create(person.into_inner()) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Created())),
        Err(err_msg) => Err(Error::Conflict(json!({"error" : err_msg})))
    }
}

#[put("/persons/{id}")]
pub fn update(id: Path<u32>, person: Json<UpdatePerson>) -> Result<HttpResponse, Error> {
    match person_repository::update(*id, person.into_inner()) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Ok())),
        Err(err_msg) => Err(Error::NotFound(json!({"error" : err_msg})))
    }
}
