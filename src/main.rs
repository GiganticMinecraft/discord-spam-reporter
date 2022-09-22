use std::env;
use std::fs::File;
use std::io::BufReader;

use fancy_regex::Regex;
use once_cell::sync::OnceCell;
use serde::{self, Deserialize};

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, GuildId, RoleId},
    },
    prelude::*,
    utils::MessageBuilder,
};

mod parse_channel_id;
mod parse_guild_id;
mod parse_regexp;
mod parse_role_id;

#[derive(Debug, Deserialize)]
struct EnvConfig {
    token: String,
    #[serde(with = "parse_channel_id")]
    report_channel: ChannelId,
    #[serde(with = "parse_guild_id")]
    guild: GuildId,
    #[serde(with = "parse_role_id")]
    role: RoleId,
}

#[derive(Debug, Deserialize)]
struct Config {
    rules: Vec<Filter>,
}

#[derive(Debug, Deserialize)]
struct Filter {
    #[serde(with = "parse_regexp")]
    pattern: Regex,
    note: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

static ENV_CONFIG: OnceCell<EnvConfig> = OnceCell::new();

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let config = CONFIG.get().unwrap();
        let env_config = ENV_CONFIG.get().unwrap();

        if (&msg.guild_id).filter(|v| v == &env_config.guild).is_none() {
            return;
        }

        let notes: Vec<&str> = (&config.rules)
            .iter()
            .filter(|s| s.pattern.is_match(&msg.content).unwrap_or(false))
            .map(|s| s.note.as_str())
            .collect();

        if notes.is_empty() {
            return;
        }

        let notes = notes
            .iter()
            .map(|s| format!("- {}", s))
            .collect::<Vec<String>>()
            .join("\n");

        // NOTE:
        // あまりに長いSPAMを送られるとそれ自身をメッセージに含むのでレポートできない可能性がある
        // 色は6桁続いていたほうが読みやすい
        #[allow(clippy::unreadable_literal)]
        let msg_s = (&env_config.report_channel)
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(":x: violation detected")
                        .colour(0xee0000)
                        .field(
                            "user",
                            MessageBuilder::new().mention(&msg.author.id).build(),
                            true,
                        )
                        .field(
                            "in",
                            MessageBuilder::new().mention(&msg.channel_id).build(),
                            true,
                        )
                        .field(
                            "violation(s)",
                            MessageBuilder::new().push_codeblock_safe(&notes, None),
                            false,
                        )
                        .field(
                            "original message",
                            MessageBuilder::new().push_codeblock_safe(&msg.content, None),
                            false,
                        )
                })
            })
            .await;

        if let Err(why) = msg_s {
            println!("Error sending message: {:?}", why);
        }

        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Error deleting message: {:?}", why);
        };

        let mut member = env_config.guild.member(&ctx.http, &msg.author.id).await.unwrap();
        if let Err(why) = member.add_role(&ctx.http, &env_config.role).await {
            println!("Error adding a role: {:?}", why);
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    CONFIG
        .set(
            serde_yaml::from_reader(BufReader::new(
                File::open(env::var("CONFIG").expect("Failed to lookup CONFIG environment"))
                    .expect("Failed to open CONFIG"),
            ))
            .expect("Failed to parse CONFIG"),
        )
        .unwrap();
    ENV_CONFIG.set(envy::from_env::<EnvConfig>().expect("Failed to parse CONFIG from env variables")).unwrap();

    let mut client = Client::builder(
        &ENV_CONFIG.get().unwrap().token,
        GatewayIntents::non_privileged().union(GatewayIntents::MESSAGE_CONTENT),
    )
    .event_handler(Handler)
    .await
    .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
