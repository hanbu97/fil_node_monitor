use crate::data::nodes::GLOBAL_NODES;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeDeleteReq {
    pub ids: Vec<String>,
}

pub async fn post_subscribe_delete(
    Json(req): Json<SubscribeDeleteReq>,
) -> core::result::Result<Res<Vec<String>>, Res<String>> {
    match post_subscribe_delete_handler(req).await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

pub async fn post_subscribe_delete_handler(req: SubscribeDeleteReq) -> anyhow::Result<Vec<String>> {
    let nodes = { GLOBAL_NODES.nodes.read().await.clone() };
    let delete_ids = req.ids;

    let nodes = nodes
        .into_iter()
        .filter(|x| !delete_ids.contains(x))
        .collect();

    {
        *GLOBAL_NODES.nodes.write().await = nodes;
    }

    GLOBAL_NODES.save().await?;
    let nodes = { GLOBAL_NODES.nodes.read().await.clone() };

    Ok(nodes)
}
