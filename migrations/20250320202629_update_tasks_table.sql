-- Add migration script here
ALTER TABLE tasks
ADD COLUMN user_id INT;

ALTER TABLE tasks ADD CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id);
