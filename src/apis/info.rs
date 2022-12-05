use super::*;
use crate::data::filfox::models::{MinerInfo, GLOBAL_MINER_INFOS};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInfoRes {
    pub total: MinerInfo,
    pub info: Vec<MinerInfo>,
    pub last_update: String,
}

pub async fn get_info() -> core::result::Result<Res<GetInfoRes>, Res<String>> {
    match get_info_handler().await {
        Ok(d) => Ok(Res::success(d)),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

async fn get_info_handler() -> anyhow::Result<GetInfoRes> {
    let info = GLOBAL_MINER_INFOS.info().await?;
    let last_update = GLOBAL_MINER_INFOS.last_update().await?;

    let mut total = MinerInfo::new();
    for i in &info {
        total.pledge += i.pledge;
        total.blocks += i.blocks;
        total.power += i.power;
        total.rewards += i.rewards;
    }

    Ok(GetInfoRes {
        info,
        last_update,
        total,
    })
}
