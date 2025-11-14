#!/bin/bash
# Cleanup script for Rust URL Shortener
# Removes build artifacts and temporary files

set -e

echo "🧹 Cleaning up..."
echo "================"
echo

# Clean cargo build artifacts
echo "⚙️  Cleaning cargo build artifacts..."
cargo clean

# Remove database file
if [ -f "rust_url_shortener.db" ]; then
    read -p "Remove database file (rust_url_shortener.db)? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm rust_url_shortener.db
        echo "✓ Database removed"
    fi
fi

# Remove SQLite journal files
rm -f *.db-shm *.db-wal

# Remove log files
rm -f *.log

# Remove coverage reports
if [ -d "coverage" ]; then
    rm -rf coverage
    echo "✓ Coverage reports removed"
fi

echo
echo "✅ Cleanup complete!"
