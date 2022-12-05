use std::sync::Arc;

use chrono::Utc;
use lazy_static::lazy_static;
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

// history subscribe item
#[derive(Savefile, Clone, Serialize, Deserialize, Debug)]
pub struct HistoryItem {
    pub name: String,
    pub interval: f32,
    pub add_time: i64,
}

// file dir to save locally
const DEFAULT_HISTORY_FILE: &str = "history.bin";
lazy_static! {
    pub static ref HISTORY_FILE: String = {
        option_env!("HISTORY_FILE")
            .unwrap_or(DEFAULT_HISTORY_FILE)
            .to_string()
    };
}

// define data structure
pub struct GlobalHistory {
    pub history: RwLock<Vec<HistoryItem>>,
}

impl GlobalHistory {
    pub async fn get(&self) -> Vec<HistoryItem> {
        self.history.read().await.clone()
    }

    pub async fn add(&self, name: String, interval: f32) -> anyhow::Result<()> {
        let current_timestamp = Utc::now().timestamp();

        let item = HistoryItem {
            name,
            interval,
            add_time: current_timestamp,
        };

        {
            self.history.write().await.push(item);
        }

        // save add subscribe
        self.save().await?;

        Ok(())
    }

    pub async fn delete(&self, names: Vec<String>) -> anyhow::Result<()> {
        let items = self.get().await;
        let mut new_items = vec![];

        for item in items {
            if names.contains(&item.name) {
                continue;
            }
            new_items.push(item)
        }

        {
            *self.history.write().await = new_items;
        }

        // save add subscribe
        self.save().await?;

        Ok(())
    }

    pub async fn get_history(&self, name: String) -> anyhow::Result<HistoryItem> {
        let items = self.get().await;

        for item in items {
            if item.name == name {
                return Ok(item);
            }
        }

        Err(anyhow::anyhow!("history item not found!"))
    }
}

#[derive(Savefile)]
pub struct History {
    // history
    pub history: Vec<HistoryItem>,
}
impl From<History> for GlobalHistory {
    fn from(n: History) -> Self {
        Self {
            history: n.history.into(),
        }
    }
}

impl GlobalHistory {
    pub async fn history(&self) -> History {
        History {
            history: self.history.read().await.clone(),
        }
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let n: History = self.history().await;
        save_config(&n);
        Ok(())
    }

    fn load_config() -> anyhow::Result<GlobalHistory> {
        let config: GlobalHistory = load_config()?.into();
        Ok(config)
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let config: GlobalHistory = load_config()?.into();
        *self = config;

        Ok(())
    }
}

lazy_static! {
    pub static ref GLOBAL_HISTORY: Arc<GlobalHistory> = {
        let config = match GlobalHistory::load_config() {
            Ok(c) => c,
            Err(_) => GlobalHistory {
                history: RwLock::new(vec![]),
            },
        };

        Arc::new(config)
    };
}

fn save_config(config: &History) {
    save_file(&*HISTORY_FILE, 0, config).unwrap();
}

fn load_config() -> anyhow::Result<History> {
    Ok(load_file(&*HISTORY_FILE, 0)?)
}
