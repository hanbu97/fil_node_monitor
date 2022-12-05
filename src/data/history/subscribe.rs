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
    pub interval: i64,
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
    pub last_update: RwLock<Vec<i64>>,
}

impl GlobalHistory {
    pub async fn get(&self) -> Vec<HistoryItem> {
        self.history.read().await.clone()
    }

    pub async fn last_update(&self) -> Vec<i64> {
        self.last_update.read().await.clone()
    }

    pub async fn update_time(&self, idx: usize, timestamp: i64) -> anyhow::Result<()> {
        let mut times = self.last_update.write().await;
        times[idx] = timestamp;
        Ok(())
    }

    pub async fn add(&self, name: String, interval: i64) -> anyhow::Result<()> {
        let current_timestamp = Utc::now().timestamp();

        let item = HistoryItem {
            name,
            interval,
            add_time: current_timestamp,
        };

        {
            self.history.write().await.push(item);
        }
        {
            self.last_update.write().await.push(0);
        }

        // save add subscribe
        self.save().await?;

        Ok(())
    }

    pub async fn delete(&self, names: Vec<String>) -> anyhow::Result<()> {
        let items = self.get().await;
        let last_updates = self.last_update().await;
        let mut new_items = vec![];
        let mut new_last_updates = vec![];

        for (item, last) in items.into_iter().zip(last_updates) {
            if names.contains(&item.name) {
                continue;
            }
            new_items.push(item);
            new_last_updates.push(last);
        }

        {
            *self.history.write().await = new_items;
        }
        {
            *self.last_update.write().await = new_last_updates;
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
    // last_update timestamp
    pub last_update: Vec<i64>,
}
impl From<History> for GlobalHistory {
    fn from(n: History) -> Self {
        Self {
            history: n.history.into(),
            last_update: n.last_update.into(),
        }
    }
}

impl GlobalHistory {
    pub async fn history(&self) -> History {
        History {
            history: self.history.read().await.clone(),
            last_update: self.last_update.read().await.clone(),
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
                last_update: RwLock::new(vec![]),
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
