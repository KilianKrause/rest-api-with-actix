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
        let mut app = test::init_service(App::new().service(request_handler::get));
        let req = test::TestRequest::get().uri("/persons/123").to_request();

        let future = test::run_on(|| app.call(req));
        let resp = test::block_on(future).unwrap();

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_returns_success() {
        let mut app = test::init_service(App::new().service(request_handler::get));
        let req = test::TestRequest::get().uri("/persons/3").to_request();

        let future = test::run_on(|| app.call(req));
        let resp = test::block_on(future).unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
