use crate::data::config::GLOBAL_CONFIG;

use super::super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct IntervalReq {
    from: f32,
    to: f32,
    force: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntervalRes {
    interval: f32,
}

pub async fn get_interval() -> core::result::Result<Res<IntervalRes>, Res<String>> {
    let interval = GLOBAL_CONFIG.interval().await;
    Ok(Res::success(IntervalRes { interval }))
}

pub async fn post_interval_handler(req: IntervalReq) -> anyhow::Result<f32> {
    let mut do_change = false;
    if let Some(t) = req.force {
        if t {
            do_change = true;
        }
    }

    let current_interval = GLOBAL_CONFIG.interval().await;
    if current_interval == req.from {
        do_change = true;
    }

    if do_change {
        GLOBAL_CONFIG.set_interval(req.to).await?;
    }

    // save changes
    GLOBAL_CONFIG.save().await?;

    Ok(req.to)
}

pub async fn post_interval(
    Json(req): Json<IntervalReq>,
) -> core::result::Result<Res<IntervalRes>, Res<String>> {
    match post_interval_handler(req).await {
        Ok(_) => Ok({
            let interval = GLOBAL_CONFIG.interval().await;
            Res::success(IntervalRes { interval })
        }),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
