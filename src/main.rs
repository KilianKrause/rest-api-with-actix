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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, App};

    #[test]
    fn test_returns_error() {
        let mut app = test::init_service(App::new());
        let req = test::TestRequest::get().uri("/persons/5").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_returns_success() {
        let mut app = test::init_service(App::new().service(request_handler::get));
        let req = test::TestRequest::get().uri("/persons/2").to_request();

        let result: person::Person = test::read_response_json(&mut app, req);

        assert_eq!(result.id(), 2);
        assert_eq!(result.age(), 24);
        assert_eq!(result.name(), "Bob");
    }
}
