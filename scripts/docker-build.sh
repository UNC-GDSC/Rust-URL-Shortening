#!/bin/bash
# Docker build script for Rust URL Shortener

set -e

IMAGE_NAME="rust-url-shortener"
TAG=${1:-latest}

echo "🐳 Building Docker Image"
echo "======================="
echo "Image: $IMAGE_NAME:$TAG"
echo

# Build the Docker image
echo "⚙️  Building image..."
docker build -t "$IMAGE_NAME:$TAG" .

echo
echo "✓ Docker image built successfully!"
echo
echo "To run the container:"
echo "  docker run -d -p 8080:8080 $IMAGE_NAME:$TAG"
echo
echo "Or use docker-compose:"
echo "  docker-compose up -d"
