#[macro_use]
extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;

use actix_web::{middleware::Logger, web, App, HttpServer};
use db::establish_connection_pool;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Initialize the logger (env_logger logs info to stdout)
    env_logger::init();

    // Retrieve the DATABASE_URL from the environment. The application will panic if it's not set.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish a connection pool using the provided database URL
    let pool = establish_connection_pool(&database_url);

    // Define the server address to bind to (listening on port 8080)
    let server_address = "0.0.0.0:8080";
    println!("Starting server at: {}", server_address);

    // Create and run the HTTP server using Actix-web
    HttpServer::new(move || {
        App::new()
            // Share the database pool across all application routes
            .app_data(web::Data::new(pool.clone()))
            // Use default logging middleware to log HTTP requests
            .wrap(Logger::default())
            // Configure the application routes defined in the routes module
            .configure(routes::init_routes)
    })
    .bind(server_address)?
    .run()
    .await
}
