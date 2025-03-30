// Import necessary Actix-web items for routing, handling HTTP responses, and asynchronous functionality.
use actix_web::{get, post, web, HttpResponse, Responder};
// Import Diesel prelude for database interaction.
use diesel::prelude::*;
// Import rand for generating random alphanumeric characters.
use rand::{distributions::Alphanumeric, Rng};
// Import std::env to access environment variables.
use std::env;

// Import the database connection pool type defined in our project.
use crate::db::DbPool;
// Import Diesel models for URL data and the corresponding insertable struct.
use crate::models::{NewUrl, Url};
// Import Diesel schema DSL for the "urls" table.
use crate::schema::urls::dsl::*;

/// Handler for creating a new shortened URL.
/// 
/// This function expects a JSON payload containing the `original_url` field.
/// It validates the input, generates a random 7-character short code, and then attempts
/// to insert the new URL into the database using Diesel.
/// 
/// On success, it returns a 201 Created response with a JSON payload containing the original URL,
/// generated short code, full short URL, and creation timestamp.
/// On failure, it returns an appropriate error response.
#[post("/")]
async fn create_url(
    // Web::Data provides shared state (database pool) to the handler.
    pool: web::Data<DbPool>,
    // Deserialize the incoming JSON into a NewUrlRequest struct.
    item: web::Json<NewUrlRequest>,
) -> impl Responder {
    // Retrieve a database connection from the pool.
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // Validate the provided URL. Here we simply check if it's not empty.
    if item.original_url.trim().is_empty() {
        return HttpResponse::BadRequest().body("Original URL is required");
    }

    // Generate a random alphanumeric short code of 7 characters.
    let generated_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    // Create a new URL entry using the provided original URL and generated short code.
    let new_url = NewUrl {
        original_url: item.original_url.clone(),
        short_code: generated_code.clone(),
    };

    // Insert the new URL into the database on a separate thread using web::block.
    let inserted_url = web::block(move || {
        // Use the Diesel insert_into function to add the new record into the urls table.
        use crate::schema::urls;
        diesel::insert_into(urls::table)
            .values(&new_url)
            // Execute the insertion and then, on success, query back the inserted record.
            .execute(&conn)
            .and_then(|_| {
                // Filter by the generated short code to fetch the new record.
                urls.filter(short_code.eq(&generated_code))
                    .first::<Url>(&conn)
            })
    })
    .await;

    // Match on the result of the insertion.
    match inserted_url {
        // If successful, build the full short URL using the BASE_URL environment variable.
        Ok(url_entry) => {
            let base = env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string());
            let short_url = format!("{}/{}", base, url_entry.short_code);
            // Return a 201 Created response with the URL details in JSON format.
            HttpResponse::Created().json(serde_json::json!({
                "original_url": url_entry.original_url,
                "short_url": short_url,
                "short_code": url_entry.short_code,
                "created_at": url_entry.created_at
            }))
        }
        // If an error occurs, log the error and return a 500 Internal Server Error.
        Err(err) => {
            eprintln!("Error inserting URL: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating short URL")
        }
    }
}

/// Handler for listing all shortened URLs.
///
/// This function fetches all URL entries from the database, ordered by creation date in descending order.
/// It then returns the data as JSON in a 200 OK response.
/// On failure, it logs the error and returns a 500 Internal Server Error.
#[get("/")]
async fn list_urls(pool: web::Data<DbPool>) -> impl Responder {
    // Retrieve a database connection from the pool.
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // Execute a query on a separate thread to fetch URL records.
    let urls_data = web::block(move || urls.order(created_at.desc()).load::<Url>(&conn)).await;
    match urls_data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            eprintln!("Error loading URLs: {:?}", err);
            HttpResponse::InternalServerError().body("Error loading URLs")
        }
    }
}

/// Handler for redirecting a short URL to its corresponding original URL.
///
/// This function extracts the `code` from the URL path, queries the database for a matching record,
/// and if found, returns a 302 Found response with the Location header set to the original URL.
/// If no matching record is found, it returns a 404 Not Found response.
#[get("/{code}")]
async fn redirect_url(
    pool: web::Data<DbPool>,
    // Extract the path parameter "code" as a String.
    web::Path(code): web::Path<String>,
) -> impl Responder {
    // Retrieve a database connection from the pool.
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // Execute a query to find the URL with the matching short code.
    let result = web::block(move || {
        urls.filter(short_code.eq(code))
            .first::<Url>(&conn)
    })
    .await;

    // If the URL is found, return a 302 Found response with the Location header set.
    match result {
        Ok(url_entry) => HttpResponse::Found()
            .append_header(("Location", url_entry.original_url))
            .finish(),
        // Otherwise, return a 404 Not Found response.
        Err(_) => HttpResponse::NotFound().body("URL not found"),
    }
}

/// Structure for deserializing a new URL creation request.
///
/// This struct expects a JSON object with an "original_url" field.
#[derive(serde::Deserialize)]
pub struct NewUrlRequest {
    pub original_url: String,
}

/// Initializes the application's routes.
///
/// This function registers the three handlers (create, list, and redirect) with the Actix-web service configuration.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_url);
    cfg.service(list_urls);
    cfg.service(redirect_url);
}
