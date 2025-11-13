# 🦀 Rust URL Shortener Service

<div align="center">

[![Rust Version](https://img.shields.io/badge/Rust-1.56+-orange.svg?logo=rust)](https://www.rust-lang.org)
[![CI](https://img.shields.io/github/workflow/status/UNC-GDSC/Rust-URL-Shortening/CI?logo=github)](https://github.com/UNC-GDSC/Rust-URL-Shortening/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![PostgreSQL](https://img.shields.io/badge/Database-PostgreSQL-blue.svg?logo=postgresql&logoColor=white)](#)
[![MySQL](https://img.shields.io/badge/Database-MySQL-4479A1.svg?logo=mysql&logoColor=white)](#)
[![SQLite](https://img.shields.io/badge/Database-SQLite-lightgrey.svg?logo=sqlite&logoColor=blue)](#)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED.svg?logo=docker&logoColor=white)](#docker-deployment)

**A blazingly fast, production-ready URL shortening service built entirely in Rust**

[Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Contributing](#contributing)

</div>

---

## 📋 Overview

A full-stack URL shortening service built with **Rust**, leveraging Actix-web for high performance and Diesel ORM for type-safe database operations. Perfect for learning Rust web development or deploying a production URL shortener.

### Key Capabilities

- 🔗 **Create Short URLs:** Transform long URLs into short, shareable links
- 📊 **List & Manage:** Retrieve and manage all shortened URLs
- ⚡ **Fast Redirects:** Lightning-fast redirection to original URLs
- 📈 **Analytics:** Track URL usage and statistics
- ⏰ **Expiration Support:** Set expiration dates for URLs

---

## ✨ Features

- ⚡ **High Performance** - Built with Actix-web, one of the fastest web frameworks
- 🔒 **Type Safety** - Diesel ORM provides compile-time query verification
- 🐳 **Docker Ready** - Multi-stage Docker builds for easy deployment
- 📊 **Statistics Tracking** - Monitor URL usage and access patterns
- 🔄 **Database Flexibility** - Supports SQLite, PostgreSQL, and MySQL
- 🧪 **Well Tested** - Comprehensive test suite with integration tests
- 📖 **Great Documentation** - Extensive API and architecture documentation
- 🚀 **CI/CD Ready** - GitHub Actions workflows included

---

## 🎯 Why Rust?

| Feature | Benefit |
|---------|---------|
| ⚡ **Performance** | Near C/C++ speed with zero-cost abstractions |
| 🔒 **Memory Safety** | No garbage collector, no data races |
| 🔄 **Concurrency** | Fearless concurrency without data races |
| 🛠️ **Modern Tooling** | Cargo, rustfmt, clippy for productive development |
| 🐛 **Reliability** | Catch bugs at compile time, not runtime |

---

## 🛠️ Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Web Framework** | [Actix-web 4.x](https://actix.rs/) | High-performance async HTTP server |
| **ORM** | [Diesel 2.x](https://diesel.rs/) | Type-safe SQL query builder |
| **Database** | SQLite / PostgreSQL / MySQL | Flexible data storage |
| **Async Runtime** | Tokio | Asynchronous runtime |
| **Serialization** | Serde | JSON serialization/deserialization |
| **Environment** | dotenvy | Environment variable management |
| **Utilities** | rand, chrono, uuid | Various utilities |

---

## 📁 Project Structure

```
rust-url-shortener/
├── .github/
│   └── workflows/          # CI/CD pipelines
│       ├── ci.yml
│       └── docker.yml
├── docs/                   # Documentation
│   ├── API.md
│   ├── ARCHITECTURE.md
│   └── DEPLOYMENT.md
├── examples/               # Usage examples
│   ├── basic_usage.rs
│   └── README.md
├── migrations/             # Database migrations
│   ├── 20230310123456_create_urls/
│   ├── 20230310123567_create_usage_logs/
│   ├── 20230310123678_create_redirect_stats/
│   └── 20230310123789_add_expiration_date_to_urls/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── lib.rs             # Library root
│   ├── config.rs          # Configuration
│   ├── db.rs              # Database connection
│   ├── error.rs           # Error handling
│   ├── handlers.rs        # Request handlers
│   ├── loggers.rs         # Logging setup
│   ├── models.rs          # Data models
│   ├── routes.rs          # Route definitions
│   ├── schema.rs          # Database schema
│   └── utils.rs           # Utilities
├── scripts/                # Utility scripts
├── tests/                  # Integration tests
│   └── integrationTests.rs
├── .dockerignore
├── .env.example           # Example environment file
├── .gitignore
├── CHANGELOG.md           # Version history
├── CONTRIBUTING.md        # Contribution guidelines
├── Cargo.toml             # Rust dependencies
├── clippy.toml            # Linting configuration
├── docker-compose.yml     # Docker Compose setup
├── Dockerfile             # Docker image definition
├── LICENSE                # MIT License
├── Makefile               # Build automation
├── README.md              # This file
└── rustfmt.toml           # Code formatting rules
```

---

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.56+ - [Install via rustup](https://rustup.rs/)
- **SQLite** - System SQLite library
- **Diesel CLI** - Database migration tool

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/UNC-GDSC/Rust-URL-Shortening.git
   cd Rust-URL-Shortening
   ```

2. **Install Diesel CLI**

   ```bash
   cargo install diesel_cli --no-default-features --features sqlite
   ```

3. **Set up environment**

   ```bash
   cp .env.example .env
   ```

   Edit `.env` to configure your settings:
   ```env
   DATABASE_URL=rust_url_shortener.db
   BASE_URL=http://localhost:8080
   RUST_LOG=info
   ```

4. **Run database migrations**

   ```bash
   diesel migration run
   ```

5. **Build and run**

   ```bash
   cargo run
   ```

   The server will start at `http://localhost:8080` 🎉

---

## 📖 API Usage

### Create a Short URL

```bash
curl -X POST http://localhost:8080/ \
  -H "Content-Type: application/json" \
  -d '{"original_url": "https://example.com"}'
```

**Response:**
```json
{
  "id": 1,
  "original_url": "https://example.com",
  "short_code": "abc123",
  "created_at": "2024-01-15T10:30:00Z",
  "expires_at": null
}
```

### List All URLs

```bash
curl http://localhost:8080/
```

### Use Short URL

Simply visit: `http://localhost:8080/abc123`

For complete API documentation, see [docs/API.md](docs/API.md)

---

## 🐳 Docker Deployment

### Using Docker

```bash
# Build image
docker build -t rust-url-shortener .

# Run container
docker run -d -p 8080:8080 \
  -e DATABASE_URL=rust_url_shortener.db \
  -e BASE_URL=http://localhost:8080 \
  rust-url-shortener
```

### Using Docker Compose

```bash
docker-compose up -d
```

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for production deployment guides.

---

## 📚 Documentation

- **[API Documentation](docs/API.md)** - Complete API reference with examples
- **[Architecture](docs/ARCHITECTURE.md)** - System design and architecture decisions
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Deploy to various platforms
- **[Examples](examples/)** - Code examples for different use cases

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integrationTests
```

---

## 🔧 Development

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run checks
make lint
```

### Using Makefile

```bash
make build      # Build project
make run        # Run application
make test       # Run tests
make fmt        # Format code
make lint       # Run clippy
```

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📋 Roadmap

- [ ] Rate limiting per IP
- [ ] Custom short codes
- [ ] Analytics dashboard
- [ ] QR code generation
- [ ] Batch URL creation
- [ ] API authentication
- [ ] Redis caching layer
- [ ] Prometheus metrics

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👥 Authors

**UNC-CH Google Developer Student Club (GDSC)**

---

## 🙏 Acknowledgments

- Built with [Actix-web](https://actix.rs/)
- Database management with [Diesel](https://diesel.rs/)
- Inspired by the Rust community

---

## 📞 Support

- 📫 [Open an issue](https://github.com/UNC-GDSC/Rust-URL-Shortening/issues)
- 💬 [Discussions](https://github.com/UNC-GDSC/Rust-URL-Shortening/discussions)
- 📖 [Documentation](docs/)

---

<div align="center">

**[⬆ back to top](#-rust-url-shortener-service)**

Made with ❤️ by UNC-GDSC

</div>
