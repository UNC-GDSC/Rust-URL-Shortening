#!/bin/bash
# Performance testing script for URL shortener
# Uses Apache Bench (ab) if available, otherwise provides instructions

set -e

BASE_URL="${BASE_URL:-http://localhost:8080}"
CONCURRENT="${CONCURRENT:-10}"
REQUESTS="${REQUESTS:-1000}"

echo "🚀 Performance Testing - Rust URL Shortener"
echo "==========================================="
echo "Base URL: $BASE_URL"
echo "Concurrent requests: $CONCURRENT"
echo "Total requests: $REQUESTS"
echo

# Check if server is running
echo "Checking if server is running..."
if ! curl -s -f "$BASE_URL/health" > /dev/null; then
    echo "❌ Server is not running at $BASE_URL"
    echo "Please start the server first with: cargo run"
    exit 1
fi

echo "✓ Server is running"
echo

# Check for Apache Bench
if ! command -v ab &> /dev/null; then
    echo "⚠️  Apache Bench (ab) is not installed"
    echo
    echo "To install:"
    echo "  Ubuntu/Debian: sudo apt-get install apache2-utils"
    echo "  macOS: brew install apr-util (ab comes with macOS)"
    echo "  RHEL/CentOS: sudo yum install httpd-tools"
    echo
    echo "Alternative: Use 'wrk' or 'hey' for load testing"
    exit 1
fi

echo "=== Test 1: Health Check Endpoint ==="
ab -n "$REQUESTS" -c "$CONCURRENT" "$BASE_URL/health" | tail -20

echo
echo "=== Test 2: List URLs (GET /) ==="
ab -n "$REQUESTS" -c "$CONCURRENT" "$BASE_URL/" | tail -20

echo
echo "=== Test 3: Create Short URL (POST /) ==="
# Create a temporary file with POST data
cat > /tmp/url_post.json <<EOF
{"original_url":"https://example.com/test"}
EOF

ab -n 100 -c 10 -p /tmp/url_post.json -T application/json "$BASE_URL/" | tail -20
rm /tmp/url_post.json

echo
echo "✅ Performance tests completed!"
echo
echo "Metrics to note:"
echo "  - Requests per second (higher is better)"
echo "  - Time per request (lower is better)"
echo "  - Failed requests (should be 0)"
echo "  - 50th percentile response time"
echo "  - 95th percentile response time"
echo "  - 99th percentile response time"
