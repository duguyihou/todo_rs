-- Add migration script here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    task_name TEXT NOT NULL,
    task_status task_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
