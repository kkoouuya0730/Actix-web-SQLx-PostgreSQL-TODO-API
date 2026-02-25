-- Add migration script here
ALTER TABLE todos
ADD COLUMN deleted_at TIMESTAMPTZ NULL;

CREATE INDEX idx_todos_deleted_at ON todos(deleted_at);
