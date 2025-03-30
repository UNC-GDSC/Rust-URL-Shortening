use rand::{distributions::Alphanumeric, Rng};

/// Generates a random alphanumeric short code with the specified length.
pub fn generate_short_code(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
