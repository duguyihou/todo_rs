-- Add migration script here
ALTER TABLE users
ALTER COLUMN verified TYPE BOOLEAN;

ALTER TABLE users
ALTER COLUMN verified
SET DEFAULT FALSE;

ALTER TABLE users
ALTER COLUMN verification_token TYPE TEXT;
