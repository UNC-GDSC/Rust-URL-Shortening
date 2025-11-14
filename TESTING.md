# Testing Guide

This document provides comprehensive information about testing the Rust URL Shortener.

## Table of Contents

- [Test Structure](#test-structure)
- [Running Tests](#running-tests)
- [Unit Tests](#unit-tests)
- [Integration Tests](#integration-tests)
- [Benchmarks](#benchmarks)
- [Performance Testing](#performance-testing)
- [Code Coverage](#code-coverage)
- [Continuous Integration](#continuous-integration)

## Test Structure

```
rust-url-shortener/
├── tests/
│   ├── integrationTests.rs    # Integration tests
│   └── unit/                   # Unit tests
│       ├── utils_tests.rs
│       └── models_tests.rs
├── benches/                    # Benchmarks
│   └── url_generation.rs
└── src/                        # Inline unit tests in modules
```

## Running Tests

### All Tests

```bash
cargo test
```

### Specific Test

```bash
cargo test test_name
```

### With Output

```bash
cargo test -- --nocapture
```

### Parallel vs Sequential

```bash
# Run tests in parallel (default)
cargo test

# Run tests sequentially
cargo test -- --test-threads=1
```

## Unit Tests

Unit tests are located in the `tests/unit/` directory and inline within source files.

### Utils Tests

Tests for utility functions like short code generation:

```bash
cargo test --test utils_tests
```

### Models Tests

Tests for data models:

```bash
cargo test --test models_tests
```

### Writing Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let result = function_to_test();
        assert_eq!(result, expected_value);
    }

    #[test]
    #[should_panic]
    fn test_panic_case() {
        function_that_should_panic();
    }
}
```

## Integration Tests

Integration tests verify the interaction between components.

### Running Integration Tests

```bash
cargo test --test integrationTests
```

### Database Integration Tests

Integration tests that require database setup:

1. Ensure test database is configured
2. Run migrations:
   ```bash
   DATABASE_URL=test.db diesel migration run
   ```
3. Run tests:
   ```bash
   cargo test --test integrationTests
   ```

### Writing Integration Tests

```rust
#[actix_web::test]
async fn test_create_url_endpoint() {
    // Setup
    let app = test::init_service(App::new().configure(init_routes)).await;

    // Test
    let req = test::TestRequest::post()
        .uri("/")
        .set_json(&json!({"original_url": "https://example.com"}))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
}
```

## Benchmarks

Benchmarks measure performance of critical code paths.

### Running Benchmarks

```bash
cargo bench
```

### Specific Benchmark

```bash
cargo bench url_generation
```

### Writing Benchmarks

Benchmarks are in the `benches/` directory:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| function_to_benchmark(black_box(input)))
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

## Performance Testing

Load testing with Apache Bench or similar tools.

### Using the Performance Test Script

```bash
./scripts/performance-test.sh
```

### Custom Performance Tests

```bash
# With Apache Bench
ab -n 1000 -c 10 http://localhost:8080/health

# With wrk
wrk -t4 -c100 -d30s http://localhost:8080/health

# With hey
hey -n 1000 -c 10 http://localhost:8080/health
```

### Performance Metrics to Track

- **Requests per second**: Higher is better
- **Latency (p50, p95, p99)**: Lower is better
- **Error rate**: Should be 0%
- **CPU usage**: Should be reasonable
- **Memory usage**: Should be stable

## Code Coverage

### Install cargo-tarpaulin

```bash
cargo install cargo-tarpaulin
```

### Generate Coverage Report

```bash
cargo tarpaulin --out Html --output-dir coverage
```

### View Coverage

Open `coverage/index.html` in a browser.

### Coverage Goals

- **Overall**: > 80%
- **Critical paths**: > 90%
- **Handlers**: > 85%
- **Utils**: > 90%

## Continuous Integration

Tests run automatically on every push via GitHub Actions.

### CI Workflow

1. **Unit tests**: `cargo test`
2. **Integration tests**: Full test suite
3. **Clippy**: `cargo clippy -- -D warnings`
4. **Format check**: `cargo fmt -- --check`
5. **Security audit**: `cargo audit`

### Local CI Simulation

Run all CI checks locally:

```bash
# Format check
cargo fmt -- --check

# Clippy
cargo clippy -- -D warnings

# Tests
cargo test

# Security audit (install first: cargo install cargo-audit)
cargo audit
```

## Test Best Practices

### 1. Naming Conventions

```rust
#[test]
fn test_create_url_success() { }  // Good

#[test]
fn test1() { }  // Bad
```

### 2. Arrange-Act-Assert Pattern

```rust
#[test]
fn test_example() {
    // Arrange
    let input = setup_test_data();

    // Act
    let result = function_under_test(input);

    // Assert
    assert_eq!(result, expected);
}
```

### 3. Use Descriptive Assertions

```rust
assert_eq!(result, expected, "URLs should match");  // Good
assert_eq!(result, expected);  // Okay
```

### 4. Test Edge Cases

- Empty input
- Maximum length input
- Invalid input
- Null/None values
- Concurrent access

### 5. Keep Tests Independent

Each test should:
- Set up its own test data
- Clean up after itself
- Not depend on other tests
- Be able to run in any order

## Troubleshooting

### Tests Fail Intermittently

- Check for race conditions
- Use `--test-threads=1` to run sequentially
- Increase timeouts if needed

### Database Tests Fail

- Ensure database is initialized
- Check migrations are up to date
- Verify DATABASE_URL is correct

### Performance Tests Vary

- Close other applications
- Run multiple times and average
- Use dedicated hardware for CI

## Additional Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Actix-web Testing](https://actix.rs/docs/testing/)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)

## Questions?

If you have questions about testing:

1. Check this guide
2. Look at existing tests
3. Ask in GitHub Discussions
4. Open an issue with the `question` label
