-- Your SQL goes here
CREATE TABLE follows (
  id VARCHAR NOT NULL PRIMARY KEY,
  followee_id VARCHAR NOT NULL,
  FOREIGN KEY (followee_id) REFERENCES users(id),
  follower_id VARCHAR NOT NULL,
  FOREIGN KEY (follower_id) REFERENCES users(id),
  allowed BOOLEAN NOT NULL DEFAULT FALSE,
  mutual BOOLEAN NOT NULL DEFAULT FALSE,
  comment text,
  created_at TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP
)