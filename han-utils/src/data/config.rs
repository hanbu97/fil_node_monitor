use std::sync::Arc;

use lazy_static::lazy_static;
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref GLOBAL_CONFIG: Arc<GlobalConfig> = {
        let config = match GlobalConfig::load_config() {
            Ok(c) => c,
            Err(_) => GlobalConfig {
                elapse: RwLock::new(DEFAULT_ELAPSE),
                save_dir: RwLock::new(match std::env::var("SAVE_DIR") {
                    Ok(t) => t,
                    Err(_) => "./".to_string(),
                }),
            },
        };

        Arc::new(config)
    };
}

const DEFAULT_ELAPSE: f32 = 60. * 5.;
const DEFAULT_CONFIG_FILE: &str = "nodes.bin";
lazy_static! {
    pub static ref CONFIG_FILE: String = {
        option_env!("CONFIG_FILE")
            .unwrap_or(DEFAULT_CONFIG_FILE)
            .to_string()
    };
}

#[derive(Savefile)]
pub struct Config {
    // pan sync sleep time between two requests
    pub elapse: f32,
    // file saving dir
    // from config > env > default
    pub save_dir: String,
}

pub struct GlobalConfig {
    pub elapse: RwLock<f32>,
    pub save_dir: RwLock<String>,
}

impl From<Config> for GlobalConfig {
    fn from(config: Config) -> Self {
        Self {
            elapse: config.elapse.into(),
            save_dir: config.save_dir.into(),
        }
    }
}

impl GlobalConfig {
    pub async fn config(&self) -> Config {
        Config {
            elapse: *self.elapse.read().await,
            save_dir: self.save_dir.read().await.clone(),
        }
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

fn save_config(config: &Config) {
    save_file(&*CONFIG_FILE, 0, config).unwrap();
}

fn load_config() -> anyhow::Result<Config> {
    Ok(load_file(&*CONFIG_FILE, 0)?)
}
