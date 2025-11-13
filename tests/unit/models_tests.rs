// Unit tests for models

#[cfg(test)]
mod tests {
    use rust_url_shortener::models::NewUrl;

    #[test]
    fn test_new_url_creation() {
        let new_url = NewUrl {
            original_url: "https://example.com".to_string(),
            short_code: "abc123".to_string(),
        };

        assert_eq!(new_url.original_url, "https://example.com");
        assert_eq!(new_url.short_code, "abc123");
    }

    #[test]
    fn test_new_url_with_long_url() {
        let long_url = format!("https://example.com/{}", "a".repeat(1000));
        let new_url = NewUrl {
            original_url: long_url.clone(),
            short_code: "test123".to_string(),
        };

        assert_eq!(new_url.original_url, long_url);
    }

    #[test]
    fn test_new_url_with_special_characters() {
        let url_with_params = "https://example.com/path?param1=value1&param2=value2#section";
        let new_url = NewUrl {
            original_url: url_with_params.to_string(),
            short_code: "xyz789".to_string(),
        };

        assert_eq!(new_url.original_url, url_with_params);
    }
}
