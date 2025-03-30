// src/handlers.rs
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::Url;
use crate::service::create_short_url;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    pub original_url: String,
}

/// Handler for creating a shortened URL.
pub async fn create_url_handler(
    pool: web::Data<DbPool>,
    item: web::Json<CreateUrlRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    match web::block(move || create_short_url(&mut conn, &item.original_url)).await {
        Ok(url_entry) => {
            let base = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
            let short_url = format!("{}/{}", base, url_entry.short_code);
            HttpResponse::Created().json(serde_json::json!({
                "original_url": url_entry.original_url,
                "short_code": url_entry.short_code,
                "short_url": short_url,
                "created_at": url_entry.created_at
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// Handler for listing all shortened URLs.
pub async fn list_urls_handler(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::urls::dsl::*;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    match web::block(move || urls.order(created_at.desc()).load::<Url>(&mut conn)).await {
        Ok(urls_list) => HttpResponse::Ok().json(urls_list),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// Handler for redirecting a short URL to its original URL.
pub async fn redirect_handler(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> impl Responder {
    let code = req.match_info().get("code").unwrap_or("").to_string();
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    use crate::schema::urls::dsl::*;
    match web::block(move || urls.filter(short_code.eq(code)).first::<Url>(&mut conn)).await {
        Ok(url_entry) => HttpResponse::Found()
            .append_header(("Location", url_entry.original_url))
            .finish(),
        Err(_) => HttpResponse::NotFound().body("URL not found"),
    }
}
