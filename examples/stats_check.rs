/// Example: Checking server health and listing URLs
/// Demonstrates how to verify server status and retrieve URL statistics
///
/// Run with: cargo run --example stats_check

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    database: String,
    timestamp: String,
}

#[derive(Debug, Deserialize)]
struct UrlEntry {
    id: i32,
    original_url: String,
    short_code: String,
    created_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    println!("=== URL Shortener Stats Check ===\n");

    // Check server health
    println!("1. Checking server health...");
    let health_response = client
        .get(format!("{}/health", base_url))
        .send()
        .await?;

    if health_response.status().is_success() {
        let health: HealthResponse = health_response.json().await?;
        println!("   ✓ Status: {}", health.status);
        println!("   ✓ Database: {}", health.database);
        println!("   ✓ Timestamp: {}", health.timestamp);
    } else {
        println!("   ✗ Server is not healthy: {}", health_response.status());
        return Ok(());
    }

    println!();

    // List all URLs
    println!("2. Fetching all shortened URLs...");
    let list_response = client.get(base_url).send().await?;

    if list_response.status().is_success() {
        let urls: Vec<UrlEntry> = list_response.json().await?;
        println!("   ✓ Total URLs: {}\n", urls.len());

        if urls.is_empty() {
            println!("   No URLs found. Create some using the basic_usage example!");
        } else {
            println!("   Recent URLs:");
            for (idx, url) in urls.iter().take(10).enumerate() {
                println!(
                    "   {}. {} -> {}/{}",
                    idx + 1,
                    url.original_url,
                    base_url,
                    url.short_code
                );
            }

            if urls.len() > 10 {
                println!("   ... and {} more", urls.len() - 10);
            }
        }
    } else {
        println!("   ✗ Failed to fetch URLs: {}", list_response.status());
    }

    println!("\n=== Stats check complete ===");

    Ok(())
}
