-- Add migration script here
ALTER TABLE users
ADD COLUMN verified BOOLEAN DEFAULT FALSE;

ALTER TABLE users
ADD COLUMN verification_token VARCHAR(255);
