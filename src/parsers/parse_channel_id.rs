use serde::{self, Deserialize, Deserializer};
use serenity::model::id::ChannelId;

pub fn deserialize<'de, D>(deserializer: D) -> Result<ChannelId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ChannelId(u64::deserialize(deserializer)?))
}
