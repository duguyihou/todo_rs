-- Add migration script here
CREATE TYPE task_status AS ENUM ('open', 'inprogress', 'completed');

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    task_name TEXT NOT NULL,
    task_status task_status NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
