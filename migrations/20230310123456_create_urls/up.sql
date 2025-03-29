CREATE TABLE urls (
                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                      original_url TEXT NOT NULL,
                      short_code TEXT NOT NULL UNIQUE,
                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
