-- Your SQL goes here
CREATE TABLE timelines (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_id VARCHAR NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  game_id INTEGER NOT NULL,
  FOREIGN KEY (game_id) REFERENCES games(id),
  log_type INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL
)