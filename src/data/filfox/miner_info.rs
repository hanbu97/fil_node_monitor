use crate::data::config::GLOBAL_CONFIG;

use super::models::FilfoxMinerInfo;

const FILFOX_MINER_URL: &str = "https://filfox.info/api/v1/address/";

pub async fn download_from_downloadinfo(id: &str) -> anyhow::Result<FilfoxMinerInfo> {
    let url = format!("{}{}", FILFOX_MINER_URL, id);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs_f32(
            GLOBAL_CONFIG.timeouts.filfox().await,
        ))
        .build()?;

    let res: FilfoxMinerInfo = client.get(url).send().await?.json().await?;

    Ok(res)
}

#[tokio::test]
async fn test_download_from_downloadinfo() -> anyhow::Result<()> {
    let id = "f0123261";
    let info = download_from_downloadinfo(id).await?;
    dbg!(info);

    Ok(())
}
