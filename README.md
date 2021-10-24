# discord-spam-reporter

## 導入
(※DiscordのBotトークンが必要です)

1. `cargo build --release`
2. `target/release/discord-spam-reporter` を取り出す
3. `config.yml` を作成する
4. `config.yml` を以下の内容で作成する (内容は適宜書き換える)
```yml
# 通知するチャンネルのID
report_channel: 42
# 通知するサーバーのID
guild: 42
# Botのトークン
token: "Your token here"
# patternには正規表現を指定する。
# 正規表現の書式は https://docs.rs/regex/1.5.4/regex/#syntax を参照。
filters:
  - pattern: ".+"
    note: "何らかのメッセージ"
  - pattern: "free +nitro"
    note: "free nitroを検知"
```

5. `CONFIG`環境変数に`config.yml`の存在する場所をフルパスで指定する<br>\(例 `/home/kisaragi/projects/discord-spam-reporter/config.yml`\)