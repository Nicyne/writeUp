//! # writeUp
//!
//! The writeUp-Crate provides the backend to 'writeUp',
//! an extensive webapp all about writing and sharing notes using markdown.
//! It contains support for MongoDB and a REST-API as an interface.

#![allow(non_snake_case)]
mod web;
mod db_access;

use std::env;
use std::sync::Mutex;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::db_access::connect_to_database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Check for required environment-variables
    let api_port = env::var("API_PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    let db_uri = env::var("DB_URI").expect("Env-Variable 'DB_URI' needs to be set");
    let db_port = env::var("DB_PORT").expect("Env-Variable 'DB_PORT' needs to be set");
    let db_user = env::var("DB_USER").expect("Env-Variable 'DB_USER' needs to be set");
    let db_passwd = env::var("DB_PASSWD").expect("Env-Variable 'DB_PASSWD' needs to be set");

    // Connect to the Database
    let db = connect_to_database((db_uri, db_port), (db_user, db_passwd)).await;
    if db.is_err() {
        println!("Failed to establish a connection to the Database. Shutting down");
        return Ok(());
    }
    // Prepare the connection for use by the web-server
    let data = Data::new(Mutex::new(db.unwrap()));

    // Start the web-server
    HttpServer::new(move ||
        App::new()
            .app_data(data.clone())
            .service(actix_web::web::scope("/api").configure(web::handler_config)))
        .bind(("127.0.0.1", api_port))?
        .run()
        .await
}
