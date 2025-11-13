// src/handlers.rs
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{Url, NewUrl};
use serde::Deserialize;
use std::env;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    pub original_url: String,
}

/// Handler for creating a shortened URL.
pub async fn create_url_handler(
    pool: web::Data<DbPool>,
    item: web::Json<CreateUrlRequest>,
) -> impl Responder {
    use crate::schema::urls;
    use crate::schema::urls::dsl::*;

    if item.original_url.trim().is_empty() {
        return HttpResponse::BadRequest().body("Original URL is required");
    }

    let generated_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let new_url = NewUrl {
        original_url: item.original_url.clone(),
        short_code: generated_code.clone(),
    };

    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match web::block(move || {
        diesel::insert_into(urls::table)
            .values(&new_url)
            .execute(&mut conn)
            .and_then(|_| {
                urls.filter(short_code.eq(&generated_code))
                    .first::<Url>(&mut conn)
            })
    }).await {
        Ok(Ok(url_entry)) => {
            let base = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
            let short_url = format!("{}/{}", base, url_entry.short_code);
            HttpResponse::Created().json(serde_json::json!({
                "original_url": url_entry.original_url,
                "short_code": url_entry.short_code,
                "short_url": short_url,
                "created_at": url_entry.created_at
            }))
        }
        _ => HttpResponse::InternalServerError().body("Error creating short URL"),
    }
}

/// Handler for listing all shortened URLs.
pub async fn list_urls_handler(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::urls::dsl::*;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");
    match web::block(move || urls.order(created_at.desc()).load::<Url>(&mut conn)).await {
        Ok(Ok(urls_list)) => HttpResponse::Ok().json(urls_list),
        _ => HttpResponse::InternalServerError().body("Error loading URLs"),
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
        Ok(Ok(url_entry)) => HttpResponse::Found()
            .append_header(("Location", url_entry.original_url))
            .finish(),
        _ => HttpResponse::NotFound().body("URL not found"),
    }
}
