# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete project reorganization with proper src/ directory structure
- Comprehensive documentation in docs/ directory
  - API.md - Complete API documentation with examples
  - ARCHITECTURE.md - System architecture and design decisions
  - DEPLOYMENT.md - Deployment guides for various platforms
- GitHub Actions CI/CD workflows
  - Continuous Integration with tests, formatting, and clippy checks
  - Docker image building and publishing
  - Security audit checks
- Examples directory with usage examples
- CONTRIBUTING.md with contribution guidelines
- .env.example for environment configuration template
- rustfmt.toml and clippy.toml for code quality standards
- .dockerignore for optimized Docker builds
- Utility scripts in scripts/ directory
- Enhanced README.md with better structure and badges

### Changed
- Moved all Rust source files from root to src/ directory
- Updated lib.rs to include all module declarations
- Improved project structure following Rust best practices

### Fixed
- Project structure now follows standard Rust conventions

## [0.1.0] - 2024-01-15

### Added
- Initial release of Rust URL Shortener
- Core URL shortening functionality
- RESTful API with Actix-web
- SQLite database support with Diesel ORM
- Database migrations for:
  - URLs table
  - Usage logs
  - Redirect statistics
  - URL expiration dates
- Docker support with multi-stage builds
- Docker Compose configuration
- Makefile for common operations
- Basic integration tests
- Logging with env_logger
- Environment variable configuration

### Features
- Create shortened URLs
- List all shortened URLs
- Redirect to original URLs
- URL statistics tracking
- Expiration date support

### Technical
- Actix-web 4.x for web framework
- Diesel 2.x for ORM
- SQLite for database
- Connection pooling with r2d2
- Async/await support
- Error handling with custom error types
- Module-based code organization

---

## Version History

### [Unreleased] - TBD
New features and enhancements under development

### [0.1.0] - 2024-01-15
Initial release with core functionality

---

## Categories

This changelog uses the following categories:

- **Added** - New features
- **Changed** - Changes in existing functionality
- **Deprecated** - Soon-to-be removed features
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements

---

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.
