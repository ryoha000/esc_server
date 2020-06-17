-- Your SQL goes here
CREATE TABLE brands (
  id integer NOT NULL PRIMARY KEY, --- 主キー
  brandname	text NOT NULL, --- ブランドの名前
  brandfurigana	text, --- ブランドの名前のフリガナ
  makername	text, --- 未使用
  makerfurigana	text, --- 未使用
  url	text, --- ブランドのOHPのURL
  checked	boolean, --- 管理用
  kind text, --- CORPORATION(企業)かCIRCLE(同人)か
  lost boolean, --- TRUEなら解散
  directlink	boolean, --- TRUEならトップページ以外へのリンク不可
  median integer, --- ブランドが作ったゲームの中央値、1日1回計算されて格納される
  http_response_code integer, --- 管理用
  twitter	text, --- ブランド公式twitterのID
  twitter_data_widget_id INTEGER,
  notes text, ---例) このページはコンチェルトの許諾を得て使用しております。(C) 2014 Concerto All Rights Reserved.
  erogetrailers INTEGER,
  cien INTEGER,
  scheduled_date DATE NOT NULL
)
