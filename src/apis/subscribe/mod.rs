use axum::http::StatusCode;
use axum::Json;
use han_utils::res::Res;
use serde::{Deserialize, Serialize};

use crate::data::nodes::GLOBAL_NODES;

pub mod add;
pub mod delete;

pub async fn get_subscribe() -> core::result::Result<Res<Vec<String>>, Res<String>> {
    match get_subscribe_handler().await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

pub async fn get_subscribe_handler() -> anyhow::Result<Vec<String>> {
    let nodes = GLOBAL_NODES.nodes.read().await.clone();

    Ok(nodes)
}
