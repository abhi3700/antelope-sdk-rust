//! Get information of chain
//!
//! RPC API: /v1/chain/get_info
//! Sample response:
//! ```json
//! {
//!   "server_version": "6c1717c9",
//!   "chain_id": "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906",
//!   "head_block_num": 329343132,
//!   "last_irreversible_block_num": 329342804,
//!   "last_irreversible_block_id": "13a15f5461997218f30b12f846cf543d99c1da4bf22888bb0e293fd97e16f080",
//!   "head_block_id": "13a1609cafb79cda7ba6f739dea81ee36d56c7ca6733e087b5df13afa9afae75",
//!   "head_block_time": "2023-09-04T20:36:31.500",
//!   "head_block_producer": "blockpooleos",
//!   "virtual_block_cpu_limit": 200000,
//!   "virtual_block_net_limit": 1048576000,
//!   "block_cpu_limit": 199413,
//!   "block_net_limit": 1048344,
//!   "server_version_string": "v4.0.3-hotfix",
//!   "fork_db_head_block_num": 329343132,
//!   "fork_db_head_block_id": "13a1609cafb79cda7ba6f739dea81ee36d56c7ca6733e087b5df13afa9afae75",
//!   "server_full_version_string": "v4.0.3-hotfix-6c1717c94394a9713d12b1a5a1742598300f6042",
//!   "total_cpu_weight": "383380109085541",
//!   "total_net_weight": "96155790932318",
//!   "earliest_available_block_num": 290131021,
//!   "last_irreversible_block_time": "2023-09-04T20:33:47.500"
//! }
//! ```

use reqwest::{Response, Result, StatusCode};
use serde::Deserialize;
use std::env::VarError;

#[derive(Debug, Deserialize)]
pub struct ChainInfo {
    server_version: String,
    chain_id: String,
    head_block_num: u32,
    last_irreversible_block_num: u32,
    last_irreversible_block_id: String,
    head_block_id: String,
    head_block_time: String,
    head_block_producer: String,
    virtual_block_cpu_limit: u32,
    virtual_block_net_limit: u32,
    block_cpu_limit: u32,
    block_net_limit: u32,
    server_version_string: String,
    fork_db_head_block_num: u32,
    fork_db_head_block_id: String,
    server_full_version_string: String,
    total_cpu_weight: String,
    total_net_weight: String,
    earliest_available_block_num: u32,
    last_irreversible_block_time: String,
}

/// Get API URL
fn get_api_url() -> Result<String> {
    dotenv::from_path("./.env").expect("Failed in loading .env file");
    let url = std::env::var("API_URL")
        .and_then(|x| {
            if x.is_empty() {
                Err(VarError::NotPresent)
            } else {
                Ok(x)
            }
        })
        .expect("Failed to get API URL");
    Ok(url)
}

/// Get response
pub(crate) async fn get_response() -> Result<Response> {
    let url = get_api_url()?;
    let url = format!("{}/v1/chain/get_info", url);

    let res = reqwest::get(&url).await?;

    if res.status() != StatusCode::OK {
        return Err(res.status().as_u16()).unwrap();
    }

    Ok(res)
}

/// Get chain info
pub async fn get_chain_info(res: Response) -> Result<ChainInfo> {
    let chain_info = res.json::<ChainInfo>().await?;

    Ok(chain_info)
}

/// Get server version
pub async fn get_server_version(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let server_version = chain_info.server_version;
    Ok(server_version)
}

/// Get chain id
pub async fn get_chain_id(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let chain_id = chain_info.chain_id;
    Ok(chain_id)
}

/// Get head block num
pub async fn get_head_block_num(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let head_block_num = chain_info.head_block_num;
    Ok(head_block_num)
}

/// Get last irreversible block number
pub async fn get_last_irreversible_block_num(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let last_irreversible_block_num = chain_info.last_irreversible_block_num;
    Ok(last_irreversible_block_num)
}

/// Get last irreversible block id
pub async fn get_last_irreversible_block_id(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let last_irreversible_block_id = chain_info.last_irreversible_block_id;
    Ok(last_irreversible_block_id)
}

/// Get head block id
pub async fn get_head_block_id(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let head_block_id = chain_info.head_block_id;
    Ok(head_block_id)
}

/// Get head block time
pub async fn get_head_block_time(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let head_block_time = chain_info.head_block_time;
    Ok(head_block_time)
}

/// Get head block producer
pub async fn get_head_block_producer(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let head_block_producer = chain_info.head_block_producer;
    Ok(head_block_producer)
}

/// Get virtual block cpu limit
pub async fn get_virtual_block_cpu_limit(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let virtual_block_cpu_limit = chain_info.virtual_block_cpu_limit;
    Ok(virtual_block_cpu_limit)
}

/// Get virtual block net limit
pub async fn get_virtual_block_net_limit(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let virtual_block_net_limit = chain_info.virtual_block_net_limit;
    Ok(virtual_block_net_limit)
}

/// Get block cpu limit
pub async fn get_block_cpu_limit(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let block_cpu_limit = chain_info.block_cpu_limit;
    Ok(block_cpu_limit)
}

/// Get virtual block net limit
pub async fn get_block_net_limit(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let block_net_limit = chain_info.block_net_limit;
    Ok(block_net_limit)
}

/// Get server version string
pub async fn get_server_version_string(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let server_version_string = chain_info.server_version_string;
    Ok(server_version_string)
}

/// Get fork db head block num
pub async fn get_fork_db_head_block_num(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let fork_db_head_block_num = chain_info.fork_db_head_block_num;
    Ok(fork_db_head_block_num)
}

/// Get fork db head block id
pub async fn get_fork_db_head_block_id(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let fork_db_head_block_id = chain_info.fork_db_head_block_id;
    Ok(fork_db_head_block_id)
}

/// Get server full version string
pub async fn get_server_full_version_string(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let server_full_version_string = chain_info.server_full_version_string;
    Ok(server_full_version_string)
}

/// Get total cpu weight
pub async fn get_total_cpu_weight(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let total_cpu_weight = chain_info.total_cpu_weight;
    Ok(total_cpu_weight)
}

/// Get total net weight
pub async fn get_total_net_weight(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let total_net_weight = chain_info.total_net_weight;
    Ok(total_net_weight)
}

/// Get earliest available block num
pub async fn get_earliest_available_block_num(res: Response) -> Result<u32> {
    let chain_info = res.json::<ChainInfo>().await?;
    let earliest_available_block_num = chain_info.earliest_available_block_num;
    Ok(earliest_available_block_num)
}

/// Get last irreversible block time
pub async fn get_last_irreversible_block_time(res: Response) -> Result<String> {
    let chain_info = res.json::<ChainInfo>().await?;
    let last_irreversible_block_time = chain_info.last_irreversible_block_time;
    Ok(last_irreversible_block_time)
}
