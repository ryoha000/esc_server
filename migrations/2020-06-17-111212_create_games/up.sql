-- Your SQL goes here
CREATE TABLE games (
  id INTEGER NOT NULL PRIMARY KEY,
  gamename TEXT,
  furigana TEXT,
  sellday	date, --- 発売日、発売日が決まっていない場合は2030-01-01
  brand_id	integer NOT NULL, --- brandlistテーブルのidを参照する外部キー
  FOREIGN KEY (brand_id) REFERENCES brands(id),
  comike	integer, --- Getchu.comのID
  shoukai	text, --- ゲームのOHPのURL
  model	text, --- ゲームの機種
  erogame	boolean, --- エロゲか否か、tはエロゲー、fは非エロゲー
  banner_url	text, --- バナーのURL
  gyutto_id	integer, --- Gyutto.comのID
  dmm	text, --- FANZAのID
  dmm_genre	text, --- FANZAのURLの一部
  dmm_genre_2	text, --- FANZAのURLの一部
  erogametokuten integer,
  total_play_time_median integer,	
  time_before_understanding_fun_median integer,	
  dlsite_id	text,
  dlsite_domain	text,
  trial_url	text,
  okazu	boolean,
  genre	text,
  twitter	text,
  twitter_data_widget_id INTEGER,
  masterup date,
  steam INTEGER,
  dlsite_rental boolean,
  dmm_subsc text,
  surugaya_1 INTEGER,
  median INTEGER,
  stdev INTEGER,
  count2 INTEGER
)
