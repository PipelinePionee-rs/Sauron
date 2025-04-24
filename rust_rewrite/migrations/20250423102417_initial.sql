CREATE TABLE IF NOT EXISTS pages (
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    language TEXT NOT NULL,
    last_updated TIMESTAMP NOT NULL,
    content TEXT NOT NULL
);

-- Add indexes for better search performance
CREATE INDEX IF NOT EXISTS idx_pages_language ON pages(language);
CREATE INDEX IF NOT EXISTS idx_pages_content ON pages(content);