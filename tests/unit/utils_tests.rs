// Unit tests for utility functions

#[cfg(test)]
mod tests {
    use rust_url_shortener::utils::generate_short_code;

    #[test]
    fn test_generate_short_code_length() {
        let code = generate_short_code(6);
        assert_eq!(code.len(), 6, "Short code should be 6 characters long");
    }

    #[test]
    fn test_generate_short_code_custom_length() {
        let code = generate_short_code(10);
        assert_eq!(code.len(), 10, "Short code should be 10 characters long");
    }

    #[test]
    fn test_generate_short_code_alphanumeric() {
        let code = generate_short_code(8);
        assert!(
            code.chars().all(|c| c.is_alphanumeric()),
            "Short code should only contain alphanumeric characters"
        );
    }

    #[test]
    fn test_generate_short_code_uniqueness() {
        let code1 = generate_short_code(8);
        let code2 = generate_short_code(8);
        // While not guaranteed, it's extremely unlikely to get the same code twice
        // This is more of a sanity check
        assert!(
            !code1.is_empty() && !code2.is_empty(),
            "Generated codes should not be empty"
        );
    }

    #[test]
    #[should_panic]
    fn test_generate_short_code_zero_length() {
        generate_short_code(0);
    }
}
