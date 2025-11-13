# Contributing to Rust URL Shortener

Thank you for your interest in contributing to the Rust URL Shortener project! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/rust-url-shortener.git
   cd rust-url-shortener
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL-OWNER/rust-url-shortener.git
   ```

## Development Setup

### Prerequisites

- Rust 1.56 or higher
- SQLite 3
- Diesel CLI
- Git

### Setup Steps

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Diesel CLI**:
   ```bash
   cargo install diesel_cli --no-default-features --features sqlite
   ```

3. **Set up environment**:
   ```bash
   cp .env.example .env
   ```

4. **Run migrations**:
   ```bash
   diesel migration run
   ```

5. **Build the project**:
   ```bash
   cargo build
   ```

6. **Run tests**:
   ```bash
   cargo test
   ```

## Making Changes

### Branch Naming Convention

Use descriptive branch names:

- `feature/short-description` - for new features
- `fix/short-description` - for bug fixes
- `docs/short-description` - for documentation
- `refactor/short-description` - for code refactoring
- `test/short-description` - for test additions/changes

Example:
```bash
git checkout -b feature/add-expiration-dates
```

### Commit Messages

Follow the conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(api): add URL expiration functionality

Add support for setting expiration dates on shortened URLs.
URLs will automatically become inactive after expiration.

Closes #123
```

```
fix(handlers): handle empty URL input correctly

Previously, empty URLs would cause a panic. Now returns
a proper error response.
```

## Coding Standards

### Rust Style Guide

We follow the official Rust style guide. Key points:

1. **Formatting**: Use `rustfmt`
   ```bash
   cargo fmt
   ```

2. **Linting**: Use `clippy`
   ```bash
   cargo clippy
   ```

3. **Naming Conventions**:
   - `snake_case` for functions, variables, and modules
   - `CamelCase` for types and traits
   - `SCREAMING_SNAKE_CASE` for constants

4. **Error Handling**:
   - Use `Result` types
   - Provide meaningful error messages
   - Don't use `unwrap()` in production code

5. **Documentation**:
   - Add doc comments for public APIs
   - Include examples in doc comments
   - Document complex logic

Example:
```rust
/// Creates a new shortened URL.
///
/// # Arguments
///
/// * `original_url` - The full URL to be shortened
///
/// # Returns
///
/// Returns a `Result` containing the shortened URL or an error
///
/// # Examples
///
/// ```
/// let short_url = create_short_url("https://example.com")?;
/// ```
pub fn create_short_url(original_url: &str) -> Result<String, Error> {
    // Implementation
}
```

### Code Organization

- Keep functions small and focused
- Separate concerns into appropriate modules
- Use meaningful variable and function names
- Avoid deep nesting (max 3-4 levels)
- Add comments for complex logic

## Testing

### Writing Tests

1. **Unit Tests**: Test individual functions
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_generate_short_code() {
           let code = generate_short_code();
           assert_eq!(code.len(), 6);
       }
   }
   ```

2. **Integration Tests**: Place in `tests/` directory
   ```rust
   #[actix_web::test]
   async fn test_create_url_endpoint() {
       // Test implementation
   }
   ```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

### Test Coverage

Aim for:
- 80%+ code coverage
- All public APIs tested
- Edge cases covered
- Error paths tested

## Submitting Changes

### Before Submitting

1. **Run tests**:
   ```bash
   cargo test
   ```

2. **Format code**:
   ```bash
   cargo fmt
   ```

3. **Check with clippy**:
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Update documentation**:
   - Update README if needed
   - Add/update API documentation
   - Update CHANGELOG.md

### Pull Request Process

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your changes**:
   ```bash
   git push origin your-branch-name
   ```

3. **Create Pull Request**:
   - Go to GitHub and create a PR
   - Fill in the PR template
   - Link related issues
   - Add screenshots if UI changes

4. **PR Title Format**:
   ```
   [Type] Brief description
   ```
   Example: `[Feature] Add URL expiration support`

5. **PR Description Should Include**:
   - What changes were made
   - Why the changes were made
   - How to test the changes
   - Screenshots (if applicable)
   - Related issues

### PR Review Process

- Maintainers will review your PR
- Address feedback and comments
- Push additional commits if needed
- Once approved, maintainers will merge

## Reporting Issues

### Bug Reports

Use the bug report template and include:

- Clear, descriptive title
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Error messages or logs
- Screenshots if applicable

Example:
```markdown
## Bug Description
URL creation fails when original URL contains special characters

## Steps to Reproduce
1. Send POST request to /
2. Use URL: https://example.com/path?param=hello&other=world
3. Observe error response

## Expected Behavior
URL should be shortened successfully

## Actual Behavior
Returns 400 error: "Invalid URL format"

## Environment
- OS: Ubuntu 22.04
- Rust: 1.70.0
- Database: SQLite 3.40
```

### Feature Requests

Include:

- Clear description of the feature
- Use cases and benefits
- Possible implementation approach
- Alternatives considered

## Development Tips

### Database Migrations

Create a new migration:
```bash
diesel migration generate migration_name
```

Apply migrations:
```bash
diesel migration run
```

Revert last migration:
```bash
diesel migration revert
```

### Debugging

Enable debug logs:
```bash
RUST_LOG=debug cargo run
```

### Performance Testing

```bash
# Build with optimizations
cargo build --release

# Run benchmarks (if available)
cargo bench
```

## Questions?

- Check existing issues and discussions
- Join our community chat (if available)
- Ask in discussions section
- Email maintainers

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project README

Thank you for contributing! 🎉
