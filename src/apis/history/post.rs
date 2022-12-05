use std::sync::Arc;

use crate::data::{
    filfox::models::MinerInfo,
    history::db::{get_db, init_history_db},
};

use super::super::*;
use axum::{extract::Query, Extension};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryGetReq {
    pub name: String,
    pub from: i64,
    pub to: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryGetRes {
    pub name: String,
    pub time: Vec<i64>,
    pub info: Vec<MinerInfo>,
}

pub async fn post_history(
    Json(req): Json<HistoryGetReq>,
) -> core::result::Result<Res<HistoryGetRes>, Res<String>> {
    match post_history_handler(req).await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
// .on(MethodFilter::POST, apis::history::post::post_history)
pub async fn post_history_handler(
    req: HistoryGetReq,
    // db: SqlitePool,
) -> anyhow::Result<HistoryGetRes> {
    tracing::info!("{:?}", &req);
    let db = init_history_db().await?;
    let (time_vec, info_vec) = get_db(db, req.name.clone(), req.from, req.to).await?;

    Ok(HistoryGetRes {
        name: req.name,
        time: time_vec,
        info: info_vec,
    })
}
