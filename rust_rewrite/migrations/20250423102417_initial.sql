CREATE TABLE IF NOT EXISTS pages (
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    language TEXT NOT NULL,
    last_updated TIMESTAMP NOT NULL,
    content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

-- Add indexes for better search performance
CREATE INDEX IF NOT EXISTS idx_pages_title ON pages(title);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);