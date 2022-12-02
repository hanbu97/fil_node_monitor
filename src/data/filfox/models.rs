

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub struct MinerInfos {
    pub infos: RwLock<Vec<FilfoxMinerInfo>>,
}

impl MinerInfos {
    pub fn new() -> Self {
        Self {
            infos: RwLock::new(vec![]),
        }
    }
}

impl From<Vec<FilfoxMinerInfo>> for MinerInfos {
    fn from(value: Vec<FilfoxMinerInfo>) -> Self {
        Self {
            infos: RwLock::new(value),
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_MINER_INFOS: MinerInfos = MinerInfos::new();
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlAddress {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sectors {
    pub active: i64,
    pub faulty: i64,
    pub live: i64,
    pub recovering: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    pub address: String,
    pub balance: String,
}
