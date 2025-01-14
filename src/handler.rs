use crate::vars::{CONFIG, ENV_CONFIG};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

pub struct Handler;

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

        let mut member = env_config
            .guild
            .member(&ctx.http, &msg.author.id)
            .await
            .unwrap();
        if let Err(why) = member.add_role(&ctx.http, &env_config.role).await {
            println!("Error adding a role: {:?}", why);
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
