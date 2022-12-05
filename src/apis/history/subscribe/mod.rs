use crate::data::history::subscribe::{HistoryItem, GLOBAL_HISTORY};

use super::super::*;

pub mod add;
pub mod delete;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHistorySubscribeRes {
    subscribe: Vec<HistoryItem>,
}

pub async fn get_history_subscribe(
) -> core::result::Result<Res<GetHistorySubscribeRes>, Res<String>> {
    match get_history_subscribe_handler().await {
        Ok(d) => Ok(Res::success(GetHistorySubscribeRes { subscribe: d })),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

pub async fn get_history_subscribe_handler() -> anyhow::Result<Vec<HistoryItem>> {
    let histories = GLOBAL_HISTORY.get().await;

    Ok(histories)
}
