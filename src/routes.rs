// src/routes.rs
// Route configuration for the URL shortener service

use actix_web::web;
use crate::handlers::{create_url_handler, list_urls_handler, redirect_handler};

/// Initializes and configures all application routes
///
/// This function sets up the HTTP endpoints for the URL shortener:
/// - POST / - Create a new shortened URL
/// - GET / - List all shortened URLs
/// - GET /{code} - Redirect to the original URL using the short code
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(create_url_handler))
            .route(web::get().to(list_urls_handler))
    )
    .service(
        web::resource("/{code}")
            .route(web::get().to(redirect_handler))
    );
}
