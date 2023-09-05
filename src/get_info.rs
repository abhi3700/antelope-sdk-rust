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
