use crate::data::{config::GLOBAL_CONFIG, nodes::GLOBAL_NODES};

use super::{miner_info::download_from_downloadinfo, models::GLOBAL_MINER_INFOS};

pub async fn update_miner_info() -> anyhow::Result<()> {
    let nodes = GLOBAL_NODES.nodes().await.nodes;
    let interval = { *GLOBAL_CONFIG.interval.read().await };

    let gap = interval / nodes.len() as f32;
    let mut infos = vec![];

    for node in nodes {
        tokio::time::sleep(std::time::Duration::from_secs_f32(gap)).await;

        if let Ok(info) = download_from_downloadinfo(&node).await {
            infos.push(info);
        };
    }
    {
        *GLOBAL_MINER_INFOS.infos.write().await = infos;
    }

    Ok(())
}

pub async fn miner_info_updater() {
    loop {
        if let Err(e) = update_miner_info().await {
            tracing::error!("miner_info_updater error: {}", e)
        }
    }
}
