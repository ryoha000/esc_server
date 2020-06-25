-- Your SQL goes here
CREATE TABLE messages (
  id VARCHAR NOT NULL PRIMARY KEY,
  from_user_id VARCHAR NOT NULL,
  FOREIGN KEY (from_user_id) REFERENCES users(id),
  to_user_id VARCHAR NOT NULL,
  FOREIGN KEY (to_user_id) REFERENCES users(id),
  message text NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOt NULL,
  deleted_at TIMESTAMP
)
