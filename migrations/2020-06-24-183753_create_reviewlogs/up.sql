-- Your SQL goes here
CREATE TABLE reviewlogs (
  id VARCHAR NOT NULL PRIMARY KEY,
  timeline_id VARCHAR NOT NULL,
  FOREIGN KEY (timeline_id) REFERENCES timelines(id),
  review_id VARCHAR NOT NULL,
  FOREIGN KEY (review_id) REFERENCES reviews(id),
  created_at TIMESTAMP NOT NULL
)
