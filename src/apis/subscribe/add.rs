use crate::data::nodes::GLOBAL_NODES;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeAddReq {
    pub id: String,
}

pub async fn post_subscribe_add(
    Json(req): Json<SubscribeAddReq>,
) -> core::result::Result<Res<Vec<String>>, Res<String>> {
    match post_subscribe_add_handler(req).await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

pub async fn post_subscribe_add_handler(req: SubscribeAddReq) -> anyhow::Result<Vec<String>> {
    {
        GLOBAL_NODES.nodes.write().await.push(req.id);
    }
    GLOBAL_NODES.save().await?;
    let nodes = GLOBAL_NODES.nodes.read().await.clone();

    Ok(nodes)
}
