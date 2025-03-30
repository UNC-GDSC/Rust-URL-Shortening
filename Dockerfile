# =========================================
# Stage 1 - Build the application
# =========================================
FROM rust:1.69 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rust-url-shortener
WORKDIR /rust-url-shortener

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Install system dependencies needed by Diesel CLI, if you plan to run migrations inside the container
RUN apt-get update && apt-get install -y libsqlite3-dev

# Build for release
RUN cargo build --release

# =========================================
# Stage 2 - Create a minimal runtime image
# =========================================
FROM debian:bullseye-slim

# Install SQLite if you need to run migrations or use local sqlite
RUN apt-get update && apt-get install -y sqlite3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /rust-url-shortener/target/release/rust-url-shortener /usr/local/bin/rust-url-shortener

# Create a non-root user
RUN useradd -m -s /bin/bash appuser
USER appuser

# Expose the port your application runs on (8080 by default)
EXPOSE 8080

ENV DATABASE_URL=rust_url_shortener.db
ENV BASE_URL=http://localhost:8080

CMD ["rust-url-shortener"]
