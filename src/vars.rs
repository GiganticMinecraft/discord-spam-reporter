use crate::config::{Config, EnvConfig};

use once_cell::sync::OnceCell;

pub static CONFIG: OnceCell<Config> = OnceCell::new();
pub static ENV_CONFIG: OnceCell<EnvConfig> = OnceCell::new();
