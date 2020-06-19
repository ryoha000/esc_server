-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR NOT NULL PRIMARY KEY,
  es_user_id VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  comment TEXT,
  show_all_users BOOLEAN DEFAULT TRUE,
  show_detail_all_users BOOLEAN DEFAULT FALSE,
  show_followers BOOLEAN DEFAULT TRUE,
  show_followers_okazu BOOLEAN DEFAULT FALSE,
  twitter_id VARCHAR
)