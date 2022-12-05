use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistorySubscribeDeleteReq {
    pub names: Vec<String>,
}

pub async fn post_history_subscribe_delete(
    Json(req): Json<HistorySubscribeDeleteReq>,
) -> core::result::Result<Res<Vec<HistoryItem>>, Res<String>> {
    match post_history_subscribe_delete_handler(req).await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

pub async fn post_history_subscribe_delete_handler(
    req: HistorySubscribeDeleteReq,
) -> anyhow::Result<Vec<HistoryItem>> {
    GLOBAL_HISTORY.delete(req.names).await?;

    let data = { GLOBAL_HISTORY.get().await };

    Ok(data)
}
