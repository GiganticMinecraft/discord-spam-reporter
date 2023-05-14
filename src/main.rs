use discord_spam_reporter::{
    config::EnvConfig,
    handler::Handler,
    vars::{CONFIG, ENV_CONFIG},
};
use serenity::{prelude::GatewayIntents, Client};
use std::{fs::File, io::BufReader};

#[tokio::main]
async fn main() {
    ENV_CONFIG
        .set(envy::from_env::<EnvConfig>().expect("Failed to parse ENV_CONFIG from env variables"))
        .unwrap();
    CONFIG
        .set(
            serde_yaml::from_reader(BufReader::new(
                File::open(&ENV_CONFIG.get().unwrap().config_file_path)
                    .expect("Failed to open CONFIG"),
            ))
            .expect("Failed to parse CONFIG"),
        )
        .unwrap();

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
