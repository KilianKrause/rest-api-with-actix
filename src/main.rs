use actix_web::{HttpServer, Responder, App, Error, error, HttpResponse};
use actix_web::web::{Json, Path};
use actix_web::{get, delete, post, put};
use serde::{Serialize, Deserialize};

fn main() {
    HttpServer::new(|| {
        App::new()
          .service(get_all)
          .service(get)
          .service(delete)
          .service(create)
          .service(update)
    })
    .bind("127.0.0.1:8099")
    .unwrap()
    .run()
    .unwrap();
}

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
        let err_msg = format!("Person with name {} already exists.", person.name);
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
}





// Beispiel Request handler
#[get("/")]
fn request_handler() -> impl Responder {
    "Hello world!".to_owned()
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use actix_web::test;
    use actix_web::web;

    // Unit test
    #[test]
    fn test_get_person() {
        let request = test::TestRequest::with_uri("/persons/123").to_http_request();
        //let response = test::block_on(get(request)).unwrap();
    }


    // Integration test
    #[test]
    fn test_get_all() {
        let mut app = test::init_service(App::new().route("/persons", web::get().to(get_all())));
        let request = test::TestRequest::get().uri("/persons").to_request();
        let response = test::block_on(||, app.call(request)).unwrap();
        let future = test::run_on(|| app.call(request));
        assert!(response.status().is_success());
    }
    */
}
