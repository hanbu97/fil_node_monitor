use std::sync::Arc;

use lazy_static::lazy_static;
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use tokio::sync::RwLock;

// file dir to save locally
const DEFAULT_NODES_FILE: &str = "nodes.bin";
lazy_static! {
    pub static ref NODES_FILE: String = {
        option_env!("NODES_FILE")
            .unwrap_or(DEFAULT_NODES_FILE)
            .to_string()
    };
}

// define data structure
pub struct GlobalNodes {
    pub nodes: RwLock<Vec<String>>,
}
#[derive(Savefile)]
pub struct Nodes {
    // nodes
    pub nodes: Vec<String>,
}
impl From<Nodes> for GlobalNodes {
    fn from(n: Nodes) -> Self {
        Self {
            nodes: n.nodes.into(),
        }
    }
}

impl GlobalNodes {
    pub async fn nodes(&self) -> Nodes {
        Nodes {
            nodes: self.nodes.read().await.clone(),
        }
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let n: Nodes = self.nodes().await;
        save_config(&n);
        Ok(())
    }

    fn load_config() -> anyhow::Result<GlobalNodes> {
        let config: GlobalNodes = load_config()?.into();
        Ok(config)
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let config: GlobalNodes = load_config()?.into();
        *self = config;

        Ok(())
    }
}

lazy_static! {
    pub static ref GLOBAL_NODES: Arc<GlobalNodes> = {
        let config = match GlobalNodes::load_config() {
            Ok(c) => c,
            Err(_) => GlobalNodes {
                nodes: RwLock::new(vec![]),
            },
        };

        Arc::new(config)
    };
}

fn save_config(config: &Nodes) {
    save_file(&*NODES_FILE, 0, config).unwrap();
}

fn load_config() -> anyhow::Result<Nodes> {
    Ok(load_file(&*NODES_FILE, 0)?)
}
