# Makefile for Rust URL Shortener

PROJECT_NAME = rust-url-shortener
DOCKER_IMAGE = rust-url-shortener

# Default target
all: build

# Build the project in debug mode
build:
	cargo build

# Build the project in release mode
release:
	cargo build --release

# Run the application locally (debug mode)
run:
	cargo run

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Check for lint
lint:
	cargo clippy -- -D warnings

# Docker build
docker-build:
	docker build -t $(DOCKER_IMAGE) .

# Docker run (detached)
docker-run:
	docker run -d -p 8080:8080 --name $(DOCKER_IMAGE) $(DOCKER_IMAGE)

# Docker stop and remove container
docker-stop:
	docker stop $(DOCKER_IMAGE) || true
	docker rm $(DOCKER_IMAGE) || true

# Docker compose
compose-up:
	docker-compose up -d

compose-down:
	docker-compose down

# Clean up
clean:
	cargo clean
