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
CREATE INDEX IF NOT EXISTS idx_pages_language ON pages(language);
CREATE INDEX IF NOT EXISTS idx_pages_content ON pages(content);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_password ON users(password);