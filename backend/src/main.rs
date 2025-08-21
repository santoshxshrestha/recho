#![allow(unused_imports)]
use actix_web::get;
use actix_web::{self, App, HttpServer, Responder, web};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || App::new().service(index))
        // why is copilot not working right now
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
