-- Your SQL goes here
CREATE TABLE reviews (
  id VARCHAR NOT NULL PRIMARY KEY,
  game_id integer NOT NULL, --- gamesテーブルのidを参照する外部キー
  FOREIGN KEY (game_id) REFERENCES games(id),
  user_id VARCHAR NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  es_user_id text NOT NULL, --- usersテーブルのuidを参照する外部キー
  tokuten integer, --- ゲームの得点
  tourokubi timestamp, --- with time zone	一言感想を登録した登録日時
  hitokoto text, --- 一言感想
  memo text, --- 長文感想
  netabare boolean, --- 長文感想のネタバレ具合 t:ネタバレ f:ネタバレなし
  giveup boolean, --- あきらめたか否か t: あきらめた f:クリアした
  possession boolean, --- 所持しているか否か t: 所持 f:所持していない
  play boolean, --- プレイしたか否か t: プレイした f:プレイしていない
  before_hitokoto text, --- 発売前一言
  before_tokuten integer, --- 発売前得点
  before_tourokubi timestamp, --- 発売前一言感想を登録した登録日時
  display boolean, --- データ登録画面の表示有無 t:表示 f:非表示
  play_tourokubi timestamp, --- play列がtとなった日時
  display_unique_count integer, --- 長文感想の参照数
  sage boolean, --- 一言感想をトップページに表示するか否か t:表示 f:表示しない
  before_purchase_will text, --- 購入予定 0_必ず購入 多分購入 様子見
  before_sage boolean, --- 発売前一言をトップページに表示するか否か t:表示 f:表示しない
  total_play_time integer, --- 総プレイ時間
  time_before_understanding_fun integer, --- 面白さが分かるまでのプレイ時間
  okazu_tokuten integer, --- どの程度おかずに使えたか 4:とても使えた 3:かなり使えた 2:だいぶ使えた 1:それなりに使えた -1:あまり使えなかった -2:まったく使えなかった -998:そもそも趣向があわなかった -999:未登録
  trial_version_hitokoto text, --- 体験版一言
  trial_version_hitokoto_sage boolean, --- 体験版一言をトップページに表示するか否か t:表示 f:表示しない
  trial_version_hitokoto_tourokubi timestamp, --- 体験版一言を登録した登録日時
  created_at timestamp --- データが登録された日時
)
