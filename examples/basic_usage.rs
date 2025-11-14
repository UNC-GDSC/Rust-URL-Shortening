/// Basic usage example for the URL shortener API
/// This example demonstrates how to interact with the URL shortener service
/// using reqwest for HTTP requests.
///
/// To run this example:
/// 1. Make sure the server is running: `cargo run`
/// 2. In another terminal: `cargo run --example basic_usage`

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct CreateUrlRequest {
    original_url: String,
}

#[derive(Debug, Deserialize)]
struct UrlResponse {
    id: i32,
    original_url: String,
    short_code: String,
    created_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    println!("=== URL Shortener API Example ===\n");

    // Example 1: Create a short URL
    println!("1. Creating a short URL...");
    let create_request = CreateUrlRequest {
        original_url: "https://www.rust-lang.org/".to_string(),
    };

    let response = client
        .post(base_url)
        .json(&create_request)
        .send()
        .await?;

    if response.status().is_success() {
        let url_data: UrlResponse = response.json().await?;
        println!("   ✓ Short URL created!");
        println!("   - Short Code: {}", url_data.short_code);
        println!("   - Original URL: {}", url_data.original_url);
        println!("   - Full Short URL: {}/{}\n", base_url, url_data.short_code);

        // Example 2: List all URLs
        println!("2. Fetching all shortened URLs...");
        let list_response = client.get(base_url).send().await?;

        if list_response.status().is_success() {
            let urls: Vec<UrlResponse> = list_response.json().await?;
            println!("   ✓ Found {} URL(s):", urls.len());
            for (idx, url) in urls.iter().enumerate() {
                println!(
                    "   {}. {} -> {}/{}",
                    idx + 1,
                    url.original_url,
                    base_url,
                    url.short_code
                );
            }
            println!();
        }

        // Example 3: Test redirect
        println!("3. Testing redirect...");
        let redirect_url = format!("{}/{}", base_url, url_data.short_code);
        let redirect_response = client
            .get(&redirect_url)
            .send()
            .await?;

        println!("   ✓ Redirect successful!");
        println!("   - Requested: {}", redirect_url);
        println!("   - Redirected to: {}", redirect_response.url());
    } else {
        println!("   ✗ Failed to create short URL: {}", response.status());
    }

    println!("\n=== Example completed ===");
    Ok(())
}
