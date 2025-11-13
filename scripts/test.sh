#!/bin/bash
# Test script for Rust URL Shortener
# Runs all tests with coverage and quality checks

set -e

echo "🧪 Running Tests"
echo "==============="
echo

# Run tests
echo "⚙️  Running unit and integration tests..."
cargo test --verbose

echo
echo "📊 Running code coverage (if tarpaulin is installed)..."
if command -v cargo-tarpaulin &> /dev/null; then
    cargo tarpaulin --out Html --output-dir coverage
    echo "✓ Coverage report generated in coverage/"
else
    echo "⚠️  cargo-tarpaulin not installed, skipping coverage"
    echo "   Install with: cargo install cargo-tarpaulin"
fi

echo
echo "🔍 Running clippy..."
cargo clippy -- -D warnings

echo
echo "📝 Checking code formatting..."
cargo fmt -- --check

echo
echo "✅ All tests passed!"
