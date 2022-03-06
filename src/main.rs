//! # writeUp
//!
//! The writeUp-Crate provides the backend to 'writeUp',
//! an extensive webapp all about writing and sharing notes using markdown.
//! It contains support for MongoDB and a REST-API as an interface.

#![allow(non_snake_case)]
mod web;

use std::env;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Check for required environment-variables
    let api_port = env::var("API_PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();

    // Start the web-server
    HttpServer::new(||
        App::new()
            .service(actix_web::web::scope("/api").configure(web::handler_config)))
        .bind(("127.0.0.1", api_port))?
        .run()
        .await
}
