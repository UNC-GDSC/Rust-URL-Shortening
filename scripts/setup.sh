#!/bin/bash
# Setup script for Rust URL Shortener
# This script sets up the development environment

set -e

echo "🦀 Rust URL Shortener - Setup Script"
echo "====================================="
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust is installed: $(rustc --version)"

# Check if Diesel CLI is installed
if ! command -v diesel &> /dev/null; then
    echo "⚙️  Installing Diesel CLI..."
    cargo install diesel_cli --no-default-features --features sqlite
else
    echo "✓ Diesel CLI is installed"
fi

# Check if .env file exists
if [ ! -f .env ]; then
    echo "⚙️  Creating .env file from .env.example..."
    cp .env.example .env
    echo "✓ .env file created"
else
    echo "✓ .env file already exists"
fi

# Run database migrations
echo "⚙️  Running database migrations..."
diesel migration run
echo "✓ Database migrations completed"

# Build the project
echo "⚙️  Building the project..."
cargo build
echo "✓ Project built successfully"

echo
echo "🎉 Setup complete!"
echo
echo "To start the server, run:"
echo "  cargo run"
echo
echo "The server will be available at http://localhost:8080"
