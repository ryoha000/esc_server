-- Your SQL goes here
CREATE TABLE randomids (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  purpose INTEGER NOT NULL --- 0 => through, 1 => timeline, 2 => direct, 3 => name, 4 => play, 5 => review, 6 => list
)
