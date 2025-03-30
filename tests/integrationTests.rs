
use reqwest;
use serde_json::json;
use std::{thread, time};

/// This test sends a POST request to create a shortened URL and verifies
/// that the response contains the expected fields.
#[test]
fn test_create_url() {
    // Wait briefly to ensure the server is up (adjust if needed)
    thread::sleep(time::Duration::from_secs(2));

    let client = reqwest::blocking::Client::new();

    // Send a POST request to create a shortened URL
    let response = client
        .post("http://localhost:8080/")
        .json(&json!({ "original_url": "https://example.com" }))
        .send()
        .expect("Failed to send POST request");

    // Ensure we receive a "Created" (201) status
    assert_eq!(response.status(), 201, "Expected status 201 Created");

    // Parse the JSON response and check for the presence of required fields
    let body: serde_json::Value = response.json().expect("Failed to parse JSON response");
    assert!(
        body.get("short_code").is_some(),
        "Response JSON should contain 'short_code'"
    );
    assert!(
        body.get("short_url").is_some(),
        "Response JSON should contain 'short_url'"
    );
    println!("Created URL response: {}", body);
}

/// This test sends a GET request to list all shortened URLs and verifies
/// that the response is a JSON array.
#[test]
fn test_list_urls() {
    // Wait briefly to ensure the server is up (adjust if needed)
    thread::sleep(time::Duration::from_secs(2));

    let client = reqwest::blocking::Client::new();

    // Send a GET request to fetch all shortened URLs
    let response = client
        .get("http://localhost:8080/")
        .send()
        .expect("Failed to send GET request");

    // Ensure we receive a successful (200 OK) status
    assert_eq!(response.status(), 200, "Expected status 200 OK");

    // Parse the JSON response and verify that it is an array
    let urls: serde_json::Value = response.json().expect("Failed to parse JSON response");
    assert!(urls.is_array(), "Response should be a JSON array");
    println!("List of URLs: {}", urls);
}
