use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistorySubscribeAddReq {
    pub name: String,
    pub interval: i64,
}

pub async fn post_history_subscribe_add(
    Json(req): Json<HistorySubscribeAddReq>,
) -> core::result::Result<Res<Vec<HistoryItem>>, Res<String>> {
    match post_history_subscribe_add_handler(req).await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

pub async fn post_history_subscribe_add_handler(
    req: HistorySubscribeAddReq,
) -> anyhow::Result<Vec<HistoryItem>> {
    // add subscribe
    GLOBAL_HISTORY.add(req.name, req.interval).await?;

    let nodes = GLOBAL_HISTORY.get().await;

    Ok(nodes)
}
