use actix_web::{App, HttpServer};

pub mod error;
pub mod person;
pub mod person_repository;
pub mod request_handler;

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(request_handler::get_all)
            .service(request_handler::get)
            .service(request_handler::delete)
            .service(request_handler::create)
            .service(request_handler::update)
    })
    .bind("127.0.0.1:8099")
    .unwrap()
    .run()
    .unwrap();
}
