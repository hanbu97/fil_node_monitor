use chrono::{DateTime, Local, SecondsFormat};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::data::history::db::{DealDbType, DealDbTypeFull};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinerInfo {
    pub id: String,
    pub pledge: f64,
    pub power: f64,
    pub blocks: u64,
    pub rewards: f64,
}

impl MinerInfo {
    pub fn new() -> Self {
        Self {
            id: "all".to_string(),
            pledge: 0.,
            power: 0.,
            blocks: 0,
            rewards: 0.,
        }
    }
}

impl Default for MinerInfo {
    fn default() -> Self {
        Self::new()
    }
}

// String, // 0    name
// i64,    // 1    timestamp
// f32,    // 2    pledge
// f32,    // 3    power
// i64,    // 4    blocks
// f32,    // 5    rewards
impl From<DealDbType> for MinerInfo {
    fn from(value: DealDbType) -> Self {
        Self {
            id: "all".to_string(),
            pledge: value.2,
            power: value.3,
            blocks: value.4 as u64,
            rewards: value.5,
        }
    }
}

impl From<DealDbTypeFull> for MinerInfo {
    fn from(value: DealDbTypeFull) -> Self {
        Self {
            id: "all".to_string(),
            pledge: value.3,
            power: value.4,
            blocks: value.5 as u64,
            rewards: value.6,
        }
    }
}

impl From<FilfoxMinerInfo> for MinerInfo {
    fn from(value: FilfoxMinerInfo) -> Self {
        let pledge: f64 = value.miner.initial_pledge_requirement.parse().unwrap_or(0.);
        let pledge = pledge / (1.0e18);

        let rewards = value.miner.total_rewards.parse().unwrap_or(0.);
        let rewards = rewards / (1.0e18);

        let power = value.miner.quality_adj_power.parse().unwrap_or(0.);
        let power = power / 1024. / 1024. / 1024. / 1024.;

        let blocks = value.miner.weighted_blocks_mined;

        Self {
            id: value.id,
            pledge,
            power,
            blocks: blocks as u64,
            rewards,
        }
    }
}

pub struct MinerInfos {
    pub last_update: RwLock<DateTime<Local>>,
    pub infos: RwLock<Vec<FilfoxMinerInfo>>,
}

impl MinerInfos {
    pub async fn info(&self) -> anyhow::Result<Vec<MinerInfo>> {
        let data = { self.infos.read().await.clone() };
        let out: Vec<MinerInfo> = data.into_iter().map(MinerInfo::from).collect();
        Ok(out)
    }

    pub async fn last_update(&self) -> anyhow::Result<String> {
        Ok(self
            .last_update
            .read()
            .await
            .to_rfc3339_opts(SecondsFormat::Millis, false))
    }
}

impl Default for MinerInfos {
    fn default() -> Self {
        Self::new()
    }
}

impl MinerInfos {
    pub fn new() -> Self {
        Self {
            last_update: RwLock::new(Local::now()),
            infos: RwLock::new(vec![]),
        }
    }
}

impl From<Vec<FilfoxMinerInfo>> for MinerInfos {
    fn from(value: Vec<FilfoxMinerInfo>) -> Self {
        Self {
            last_update: RwLock::new(Local::now()),
            infos: RwLock::new(value),
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_MINER_INFOS: MinerInfos = MinerInfos::new();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilfoxMinerInfo {
    pub actor: String,
    pub address: String,
    pub balance: String,
    #[serde(rename = "createHeight")]
    pub create_height: i64,
    #[serde(rename = "createTimestamp")]
    pub create_timestamp: i64,
    pub id: String,
    #[serde(rename = "lastSeenHeight")]
    pub last_seen_height: i64,
    #[serde(rename = "lastSeenTimestamp")]
    pub last_seen_timestamp: i64,
    #[serde(rename = "messageCount")]
    pub message_count: i64,
    pub miner: Miner,
    #[serde(rename = "ownedMiners")]
    pub owned_miners: Vec<String>,
    pub robust: String,
    pub timestamp: i64,
    #[serde(rename = "workerMiners")]
    pub worker_miners: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Miner {
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    #[serde(rename = "blocksMined")]
    pub blocks_mined: i64,
    #[serde(rename = "controlAddresses")]
    pub control_addresses: Vec<ControlAddress>,
    #[serde(rename = "initialPledgeRequirement")]
    pub initial_pledge_requirement: String,
    #[serde(rename = "multiAddresses")]
    pub multi_addresses: Vec<String>,
    #[serde(rename = "networkQualityAdjPower")]
    pub network_quality_adj_power: String,
    #[serde(rename = "networkRawBytePower")]
    pub network_raw_byte_power: String,
    pub owner: Owner,
    #[serde(rename = "peerId")]
    pub peer_id: String,
    #[serde(rename = "pledgeBalance")]
    pub pledge_balance: String,
    #[serde(rename = "preCommitDeposits")]
    pub pre_commit_deposits: String,
    #[serde(rename = "qualityAdjPower")]
    pub quality_adj_power: String,
    #[serde(rename = "qualityAdjPowerRank")]
    pub quality_adj_power_rank: i64,
    #[serde(rename = "rawBytePower")]
    pub raw_byte_power: String,
    #[serde(rename = "rawBytePowerRank")]
    pub raw_byte_power_rank: i64,
    #[serde(rename = "sectorPledgeBalance")]
    pub sector_pledge_balance: String,
    pub sectors: Sectors,
    #[serde(rename = "sectorSize")]
    pub sector_size: i64,
    #[serde(rename = "totalRewards")]
    pub total_rewards: String,
    #[serde(rename = "vestingFunds")]
    pub vesting_funds: String,
    #[serde(rename = "weightedBlocksMined")]
    pub weighted_blocks_mined: i64,
    pub worker: Worker,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlAddress {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Owner {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sectors {
    pub active: i64,
    pub faulty: i64,
    pub live: i64,
    pub recovering: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Worker {
    pub address: String,
    pub balance: String,
}
