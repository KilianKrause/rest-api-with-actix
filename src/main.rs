use actix_web::{HttpServer, App};

pub mod person;
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



// Beispiel Request handler
/*#[get("/")]
fn request_handler() -> impl Responder {
    "Hello world!".to_owned()
}*/
