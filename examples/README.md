# Examples

This directory contains example code demonstrating how to use the Rust URL Shortener.

## Running Examples

Make sure the server is running first:

```bash
cargo run
```

Then, in another terminal, run any example:

```bash
cargo run --example basic_usage
```

## Available Examples

### basic_usage.rs

Demonstrates basic API interactions:
- Creating a short URL
- Listing all URLs
- Testing redirects

**Prerequisites:** Add `reqwest` and `tokio` to your `Cargo.toml` dev-dependencies:

```toml
[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

## API Examples in Other Languages

### Python Example

```python
import requests

# Create short URL
response = requests.post(
    'http://localhost:8080/',
    json={'original_url': 'https://example.com'}
)
print(response.json())

# List all URLs
response = requests.get('http://localhost:8080/')
for url in response.json():
    print(f"{url['short_code']}: {url['original_url']}")
```

### JavaScript/Node.js Example

```javascript
const fetch = require('node-fetch');

// Create short URL
async function createShortUrl() {
  const response = await fetch('http://localhost:8080/', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      original_url: 'https://example.com'
    })
  });
  const data = await response.json();
  console.log(data);
}

createShortUrl();
```

### cURL Example

```bash
# Create short URL
curl -X POST http://localhost:8080/ \
  -H "Content-Type: application/json" \
  -d '{"original_url": "https://example.com"}'

# List all URLs
curl http://localhost:8080/

# Test redirect
curl -I http://localhost:8080/abc123
```
