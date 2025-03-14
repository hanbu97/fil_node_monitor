use std::sync::Arc;

use lazy_static::lazy_static;
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use serde::Serialize;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref GLOBAL_CONFIG: Arc<GlobalConfig> = {
        let config = match GlobalConfig::load_config() {
            Ok(c) => c,
            Err(_) => GlobalConfig {
                timeouts: GlobalTimeouts::default(),
                interval: RwLock::new(DEFAULT_INTERVAL),
            },
        };

        Arc::new(config)
    };
}

const DEFAULT_TIMEOUT: f32 = 10.;
const DEFAULT_INTERVAL: f32 = 10.;
const DEFAULT_CONFIG_FILE: &str = "config.bin";
lazy_static! {
    pub static ref CONFIG_FILE: String = {
        option_env!("CONFIG_FILE")
            .unwrap_or(DEFAULT_CONFIG_FILE)
            .to_string()
    };
}

#[derive(Savefile)]
pub struct Config {
    pub timeouts: Timeouts,
    pub interval: f32,
}

pub struct GlobalConfig {
    pub timeouts: GlobalTimeouts,
    // interval between two requests
    pub interval: RwLock<f32>,
}

impl From<Config> for GlobalConfig {
    fn from(config: Config) -> Self {
        Self {
            timeouts: config.timeouts.into(),
            interval: config.interval.into(),
        }
    }
}

impl GlobalConfig {
    async fn config(&self) -> Config {
        Config {
            timeouts: self.timeouts.config().await,
            interval: *self.interval.read().await,
        }
    }

    pub async fn interval(&self) -> f32 {
        *self.interval.read().await
    }

    pub async fn set_interval(&self, interval: f32) -> anyhow::Result<()> {
        *self.interval.write().await = interval;
        Ok(())
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let config: Config = self.config().await;
        save_config(&config);
        Ok(())
    }

    fn load_config() -> anyhow::Result<GlobalConfig> {
        let config: GlobalConfig = load_config()?.into();
        Ok(config)
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let config: GlobalConfig = load_config()?.into();
        *self = config;

        Ok(())
    }
}

pub struct GlobalTimeouts {
    /// filfox timeout
    pub filfox: RwLock<f32>,
}

#[derive(Serialize, Savefile)]
pub struct Timeouts {
    /// filfox timeout
    pub filfox: f32,
}

impl GlobalTimeouts {
    pub async fn filfox(&self) -> f32 {
        *self.filfox.read().await
    }
}

impl GlobalTimeouts {
    async fn config(&self) -> Timeouts {
        Timeouts {
            filfox: *self.filfox.read().await,
        }
    }
}

impl Default for GlobalTimeouts {
    fn default() -> Self {
        Self {
            filfox: RwLock::new(DEFAULT_TIMEOUT),
        }
    }
}

impl From<Timeouts> for GlobalTimeouts {
    fn from(t: Timeouts) -> Self {
        Self {
            filfox: RwLock::new(t.filfox),
        }
    }
}

impl From<GlobalTimeouts> for Timeouts {
    fn from(t: GlobalTimeouts) -> Self {
        Self {
            filfox: *t.filfox.blocking_read(),
        }
    }
}

fn save_config(config: &Config) {
    save_file(&*CONFIG_FILE, 0, config).unwrap();
}

fn load_config() -> anyhow::Result<Config> {
    Ok(load_file(&*CONFIG_FILE, 0)?)
}
