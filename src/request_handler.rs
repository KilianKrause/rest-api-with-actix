use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put};
use serde_json::json;

use crate::person::{NewPerson, UpdatePerson};
use crate::person_repository;
use actix_web::error;
use actix_web::error::Error;

#[get("/persons")]
pub fn get_all() -> Result<HttpResponse, Error> {
    let persons = person_repository::get_all();
    if !persons.is_empty() {
        Ok(HttpResponse::Ok().json(persons))
    } else {
        let json_err = json!({"error" : "No persons persisted yet."});
        Err(error::ErrorNotFound(json_err))
    }
}

#[get("/persons/{id}")]
pub fn get(id: Path<u32>) -> Result<HttpResponse, Error> {
    match person_repository::get(*id) {
        Some(person) => Ok(HttpResponse::Ok().json(person)),
        None => {
            let err_msg = format!("Person with id {} does not exist.", id);
            let json_err = json!({ "error": err_msg });
            Err(error::ErrorNotFound(json_err))
        }
    }
}

#[delete("/persons/{id}")]
pub fn delete(id: Path<u32>) -> Result<HttpResponse, Error> {
    match person_repository::delete(*id) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Ok())),
        Err(err_msg) => Err(error::ErrorNotFound(json!({ "error": err_msg }))),
    }
}

#[post("/persons")]
pub fn create(person: Json<NewPerson>) -> Result<HttpResponse, Error> {
    match person_repository::create(person.into_inner()) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Created())),
        Err(err_msg) => Err(error::ErrorBadRequest(json!({ "error": err_msg }))),
    }
}

#[put("/persons/{id}")]
pub fn update(id: Path<u32>, person: Json<UpdatePerson>) -> Result<HttpResponse, Error> {
    match person_repository::update(*id, person.into_inner()) {
        Ok(_) => Ok(HttpResponse::from(HttpResponse::Ok())),
        Err(err_msg) => Err(error::ErrorNotFound(json!({ "error": err_msg }))),
    }
}
