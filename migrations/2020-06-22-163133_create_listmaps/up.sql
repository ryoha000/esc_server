-- Your SQL goes here
CREATE TABLE listmaps (
  id VARCHAR NOT NULL PRIMARY KEY,
  list_id VARCHAR NOT NULL,
  FOREIGN KEY (list_id) REFERENCES lists(id),
  game_id INTEGER NOT NULL,
  FOREIGN KEY (game_id) REFERENCES games(id)
)