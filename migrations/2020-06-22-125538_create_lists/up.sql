-- Your SQL goes here
CREATE TABLE lists (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  name TEXT NOT NULL,
  comment TEXT NOT NULL,
  priority INTEGER NOT NULL,
  url TEXT,
  is_public BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP
)
