-- migrations/001_create_kajian.sql
CREATE TABLE
IF NOT EXISTS kajian
(
    kajian_id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    yt_link TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
