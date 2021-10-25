use serde::{self, Deserialize, Deserializer};
use serenity::model::id::RoleId;

pub fn deserialize<'de, D>(deserializer: D) -> Result<RoleId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RoleId(u64::deserialize(deserializer)?))
}