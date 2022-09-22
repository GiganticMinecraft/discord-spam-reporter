# discord-spam-reporter

![Build](https://img.shields.io/github/workflow/status/GiganticMinecraft/discord-spam-reporter/CI?logo=github&style=for-the-badge)
![License](https://img.shields.io/github/license/GiganticMinecraft/discord-spam-reporter.svg?style=for-the-badge&color=blue)
![Top Language](https://img.shields.io/github/languages/top/GiganticMinecraft/discord-spam-reporter.svg?style=for-the-badge)

## 導入

以下のものを事前に用意する必要があります。

* DiscordBotのトークン
* Botを導入するサーバーのID
* Botが通知を送信するチャンネルのID
* Botが付与するロールのID

### バイナリを直接使う場合

1. `cargo build --release`
2. `target/release/discord-spam-reporter` を取り出す
3. `config.yml` を作成する
4. `config.yml` を以下の内容で作成する (内容は適宜書き換える)

```yml
# patternには正規表現を指定する。
# 正規表現の書式は https://docs.rs/fancy-regex/latest/fancy_regex/#syntax を参照。
rules:
  - pattern: ".+"
    note: "何らかのメッセージ"
  - pattern: "free +nitro"
    note: "free nitroを検知"
```

5. `.env`ファイルもしくは環境変数に直接以下の変数を指定する

```env
# `config.yml`の存在する場所をフルパスまたはワーキングディレクトリからの相対パスで指定
CONFIG=/path/to/your/config.yml
# DiscordBotのトークン
TOKEN=YourDiscordBotToken
# Botを導入するサーバーのID
GUILD=123456789101112131
# Botが通知を送信するチャンネルのID
REPORT_CHANNEL=123456789101112131
# Botが付与するロールのID
ROLE=123456789101112131
```

### Dockerを使う場合

1. `config.yml`と`.env`を[上記](#バイナリを直接使う場合)と同様に作成する
2. `docker-compose.yml`を以下の内容で作成する（内容は適宜書き換える）  
`volumes`の`./config.yml`のパスは[上記](#バイナリを直接使う場合)で`CONFIG`を指定する場合のものと同様です。

```yml
version: '3'
services:
  bot:
    image: ghcr.io/giganticminecraft/discord-spam-reporter:latest
    restart: always
    volumes:
      - ./config.yml:/config.yml
    env_file:
      - ./.env
```

3. `docker-compose up`
