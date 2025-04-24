-- Migration to make columns NOT NULL
ALTER TABLE pages ALTER COLUMN title SET NOT NULL;
ALTER TABLE pages ALTER COLUMN url SET NOT NULL;
ALTER TABLE pages ALTER COLUMN language SET NOT NULL;
ALTER TABLE pages ALTER COLUMN last_updated SET NOT NULL;
ALTER TABLE pages ALTER COLUMN content SET NOT NULL;

-- Update any existing NULL values
UPDATE pages SET title = '' WHERE title IS NULL;
UPDATE pages SET url = '' WHERE url IS NULL;
UPDATE pages SET language = '' WHERE language IS NULL;
UPDATE pages SET content = '' WHERE content IS NULL;
-- For timestamp, set a default date
UPDATE pages SET last_updated = NOW() WHERE last_updated IS NULL;