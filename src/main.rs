//! # writeUp
//!
//! The writeUp-Crate provides the backend to 'writeUp',
//! an extensive webapp all about writing and sharing notes using markdown.
//! It contains support for MongoDB and a REST-API as an interface.
//!
//! # Usage
//!
//! 1. Make sure the database is up and running
//!
//! 2. Make sure the following environment variables are set:
//!     * `DB_URI` - The address under which to find the Database
//!     * `DB_PORT` - The port under which to find the Database
//!     * `DB_USER` - The user under which writeUp will use the database
//!     * `DB_PASSWD` - The password of above's user
//!     * `API_PORT` - The port under which to find the REST-API *[default: `8080`]*
//!     * `JWT_SECRET` - The secret used in creating and verifying JWTs *[default: random]*
//!     * `SHARE_SECRET` - The secret used in creating and verifying invitation-codes *[default: random]*
//!
//! 3. Start up the server by executing `writeUp` and wait for
//!     ```text
//!     > Starting up writeUp
//!     > Checking for environment-variables
//!     > Connecting to Database
//!     > Starting up webserver
//!     ```
//!
//! 4. Verify it is running by making a request to `GET /api/system`
//!
//!     If the request gets rejected, check your console for error messages
//!
//! For a comprehensive list of all Endpoints and how to use them please refer to [[`web`](crate::web)]

#![allow(rustdoc::private_intra_doc_links)]
#![allow(non_snake_case)]
mod web;
mod db_access;

use std::env;
use std::sync::Mutex;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::db_access::connect_to_database;

/// The name of the environment-variable containing the jwt-secret
pub const JWT_SECRET_ENV_VAR_KEY: &str = "JWT_SECRET";
/// The name of the environment-variable containing the share-secret
pub const SHARE_SECRET_ENV_VAR_KEY: &str = "SHARE_SECRET";
/// The default length at which a given secret-string gets generated
const SECRET_SIZE: usize = 16;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting up writeUp");

    println!("Checking for environment-variables");
    // Set random secrets for encryption if not predefined
    if env::var(JWT_SECRET_ENV_VAR_KEY).is_err() {
        let new_secret: String = rand::thread_rng().sample_iter(&Alphanumeric)
            .take(SECRET_SIZE).map(char::from).collect();
        env::set_var(JWT_SECRET_ENV_VAR_KEY, new_secret);
    }
    if env::var(SHARE_SECRET_ENV_VAR_KEY).is_err() {
        let new_secret: String = rand::thread_rng().sample_iter(&Alphanumeric)
            .take(SECRET_SIZE).map(char::from).collect();
        env::set_var(SHARE_SECRET_ENV_VAR_KEY, new_secret);
    }
    // The port to listen to
    let api_port = env::var("API_PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    // Database-related environment variables
    let db_uri = env::var("DB_URI").expect("Env-Variable 'DB_URI' needs to be set"); //TODO? Combine the following four vars to one big 'CONFIG_MONGODB_URL'
    let db_port = env::var("DB_PORT").expect("Env-Variable 'DB_PORT' needs to be set");
    let db_user = env::var("DB_USER").expect("Env-Variable 'DB_USER' needs to be set");
    let db_passwd = env::var("DB_PASSWD").expect("Env-Variable 'DB_PASSWD' needs to be set");

    // Connect to the Database
    println!("Connecting to Database");
    let db = connect_to_database((db_uri, db_port), (db_user, db_passwd)).await;
    if db.is_err() {
        println!("Failed to establish a connection to the Database. Shutting down");
        return Ok(());
    }
    // Prepare the connection for use by the web-server
    let data = Data::new(Mutex::new(db.unwrap()));

    // Start the web-server
    println!("Starting up webserver");
    HttpServer::new(move ||
        App::new()
            .app_data(data.clone())
            .service(actix_web::web::scope("/api").configure(web::handler_config)))
        .bind(("127.0.0.1", api_port))?
        .run()
        .await
}
