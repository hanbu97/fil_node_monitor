use crate::data::history::subscribe::{HistoryItem, GLOBAL_HISTORY};

use super::super::*;
use axum::extract::Query;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryGetReq {
    pub name: String,
}

pub async fn get_history(
    Query(req): Query<HistoryGetReq>,
) -> core::result::Result<Res<HistoryItem>, Res<String>> {
    match GLOBAL_HISTORY.get_history(req.name).await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
