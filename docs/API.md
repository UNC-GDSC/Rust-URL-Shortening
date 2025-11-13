# API Documentation

## Base URL

```
http://localhost:8080
```

## Endpoints

### 1. Create Short URL

Creates a new shortened URL.

**Endpoint:** `POST /`

**Request Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "original_url": "https://example.com/very/long/url"
}
```

**Response:** `200 OK`
```json
{
  "id": 1,
  "original_url": "https://example.com/very/long/url",
  "short_code": "abc123",
  "created_at": "2024-01-15T10:30:00Z",
  "expires_at": null
}
```

**Error Responses:**
- `400 Bad Request` - Invalid URL format
- `500 Internal Server Error` - Database error

---

### 2. List All URLs

Retrieves a list of all shortened URLs.

**Endpoint:** `GET /`

**Response:** `200 OK`
```json
[
  {
    "id": 1,
    "original_url": "https://example.com/page1",
    "short_code": "abc123",
    "created_at": "2024-01-15T10:30:00Z",
    "expires_at": null
  },
  {
    "id": 2,
    "original_url": "https://example.com/page2",
    "short_code": "def456",
    "created_at": "2024-01-15T11:00:00Z",
    "expires_at": null
  }
]
```

---

### 3. Redirect to Original URL

Redirects to the original URL using the short code.

**Endpoint:** `GET /{short_code}`

**Response:** `302 Found`
- Redirects to the original URL
- Response Header: `Location: https://example.com/original/url`

**Error Responses:**
- `404 Not Found` - Short code doesn't exist or has expired

---

### 4. Get URL Statistics

Retrieves statistics for a shortened URL.

**Endpoint:** `GET /stats/{short_code}`

**Response:** `200 OK`
```json
{
  "short_code": "abc123",
  "original_url": "https://example.com/page",
  "click_count": 42,
  "created_at": "2024-01-15T10:30:00Z",
  "last_accessed": "2024-01-16T15:45:00Z"
}
```

**Error Responses:**
- `404 Not Found` - Short code doesn't exist

---

## Error Format

All error responses follow this format:

```json
{
  "error": "Error description message"
}
```

## Rate Limiting

Currently, there is no rate limiting implemented. Consider implementing rate limiting for production use.

## Authentication

Currently, the API doesn't require authentication. For production deployment, consider implementing:
- API key authentication
- OAuth 2.0
- JWT tokens

## Examples

### Using cURL

**Create a short URL:**
```bash
curl -X POST http://localhost:8080/ \
  -H "Content-Type: application/json" \
  -d '{"original_url": "https://example.com"}'
```

**List all URLs:**
```bash
curl http://localhost:8080/
```

**Test redirection:**
```bash
curl -I http://localhost:8080/abc123
```

### Using Python

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
print(response.json())
```

### Using JavaScript (fetch)

```javascript
// Create short URL
fetch('http://localhost:8080/', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    original_url: 'https://example.com'
  })
})
  .then(response => response.json())
  .then(data => console.log(data));

// List all URLs
fetch('http://localhost:8080/')
  .then(response => response.json())
  .then(data => console.log(data));
```
