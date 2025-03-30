-- SQLite does not support dropping columns directly.
-- We will need to recreate the table without the column if rolling back.
PRAGMA foreign_keys=off;

CREATE TABLE urls_temp (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_url TEXT NOT NULL,
    short_code TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO urls_temp (id, original_url, short_code, created_at)
SELECT id, original_url, short_code, created_at FROM urls;

DROP TABLE urls;

ALTER TABLE urls_temp RENAME TO urls;

PRAGMA foreign_keys=on;
