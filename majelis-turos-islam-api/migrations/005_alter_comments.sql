ALTER TABLE comments
ADD COLUMN name TEXT;

UPDATE comments
SET name = 'Anonymous'
WHERE name IS NULL;