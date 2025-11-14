/// Example: Creating multiple URLs in batch
/// This demonstrates how to create multiple short URLs efficiently
///
/// Run with: cargo run --example batch_urls

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct CreateUrlRequest {
    original_url: String,
}

#[derive(Debug, Deserialize)]
struct UrlResponse {
    short_code: String,
    original_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    println!("=== Batch URL Creation Example ===\n");

    // List of URLs to shorten
    let urls_to_shorten = vec![
        "https://www.rust-lang.org/",
        "https://docs.rs/",
        "https://crates.io/",
        "https://github.com/rust-lang/rust",
        "https://actix.rs/",
    ];

    let mut created_urls = Vec::new();

    println!("Creating {} short URLs...\n", urls_to_shorten.len());

    for (index, url) in urls_to_shorten.iter().enumerate() {
        let request = CreateUrlRequest {
            original_url: url.to_string(),
        };

        match client.post(base_url).json(&request).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let url_data: UrlResponse = response.json().await?;
                    println!(
                        "{}. Created: {}/{} -> {}",
                        index + 1,
                        base_url,
                        url_data.short_code,
                        url_data.original_url
                    );
                    created_urls.push(url_data);
                } else {
                    eprintln!(
                        "{}. Failed to create URL for {}: {}",
                        index + 1,
                        url,
                        response.status()
                    );
                }
            }
            Err(e) => {
                eprintln!("{}. Error creating URL for {}: {}", index + 1, url, e);
            }
        }

        // Small delay to avoid overwhelming the server
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("\n=== Summary ===");
    println!("Total created: {}/{}", created_urls.len(), urls_to_shorten.len());

    Ok(())
}
