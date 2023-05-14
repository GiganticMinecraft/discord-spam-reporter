use crate::parsers::*;

use fancy_regex::Regex;
use serde::{self, Deserialize};
use serenity::model::id::{ChannelId, GuildId, RoleId};

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub config_file_path: String,
    pub token: String,
    #[serde(with = "parse_channel_id")]
    pub report_channel: ChannelId,
    #[serde(with = "parse_guild_id")]
    pub guild: GuildId,
    #[serde(with = "parse_role_id")]
    pub role: RoleId,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Filter>,
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(with = "parse_regexp")]
    pub pattern: Regex,
    pub note: String,
}
