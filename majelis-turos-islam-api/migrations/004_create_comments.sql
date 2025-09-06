-- migrations/001_create_kajian.sql
CREATE TABLE
IF NOT EXISTS comments
(
    comment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    stars INTEGER NOT NULL,
    email TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
