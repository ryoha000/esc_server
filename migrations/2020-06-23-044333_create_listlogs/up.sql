-- Your SQL goes here
CREATE TABLE listlogs (
  id VARCHAR NOT NULL PRIMARY KEY,
  timeline_id VARCHAR NOT NULL,
  FOREIGN KEY (timeline_id) REFERENCES timelines(id),
  list_id VARCHAR NOT NULL,
  FOREIGN KEY (list_id) REFERENCES lists(id),
  created_at TIMESTAMP NOT NULL
)
