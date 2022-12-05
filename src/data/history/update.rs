use chrono::Utc;
use sqlx::SqlitePool;

use crate::apis::info::get_info_handler;

use super::{db::DealDbType, *};

// update history when global node info changes
pub async fn update_history(conn: SqlitePool) -> anyhow::Result<()> {
    let histories = GLOBAL_HISTORY.get().await;
    let last_updates = GLOBAL_HISTORY.last_update().await;
    let current_timestamp = Utc::now().timestamp();

    for ((idx, history), last) in histories.into_iter().enumerate().zip(last_updates) {
        if current_timestamp - last > history.interval {
            // String, // 0    name
            // i64,    // 1    timestamp
            // f64,    // 2    pledge
            // f64,    // 3    power
            // i64,    // 4    blocks
            // f64,    // 5    rewards
            let info = get_info_handler().await?;
            let data: DealDbType = (
                history.name,
                current_timestamp,
                info.total.pledge,
                info.total.power,
                info.total.blocks as i64,
                info.total.rewards,
            );

            // insert current item to db
            super::db::insert_db(conn.clone(), data).await?;
            // update last update timestamp
            GLOBAL_HISTORY.update_time(idx, current_timestamp).await?;
        }
    }

    // let mut update_tasks = vec![];

    Ok(())
}

// pub async fn history_updater() {
//     loop {
//         if let Err(e) = update_history().await {
//             tracing::error!("update_history error: {}", e)
//         }
//     }
// }
