# esc_server
- [ADVゲームランチャー](https://github.com/ryoha000/es-client.git)のサーバー
```
sudo apt update
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```
新しいテーブルを作るなりなんなりするとき
```
diesel migration generate <<dir_name>>
```
```
例) diesel migration generate create_games
```