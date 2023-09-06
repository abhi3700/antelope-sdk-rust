//! Get account
//!
//! Using curl: `curl -X GET -H "Content-Type: application/json" -d '{"account_name": "abhieosindia"}' https://eos.greymass.com/v1/chain/get_account`
//!
//! RPC API: /v1/chain/get_account
//! Sample response:
//! ```json
//! {
//!   "account_name": "abhieosindia",
//!   "head_block_num": 329625518,
//!   "head_block_time": "2023-09-06T11:50:00.500",
//!   "privileged": false,
//!   "last_code_update": "1970-01-01T00:00:00.000",
//!   "created": "2018-12-19T04:21:33.000",
//!   "core_liquid_balance": "0.0010 EOS",
//!   "ram_quota": 5473,
//!   "net_weight": 100,
//!   "cpu_weight": 1000,
//!   "net_limit": {
//!     "used": 474,
//!     "available": 0,
//!     "max": 188,
//!     "last_usage_update_time": "2019-02-13T21:22:47.500",
//!     "current_used": 0
//!   },
//!   "cpu_limit": {
//!     "used": 774,
//!     "available": 0,
//!     "max": 0,
//!     "last_usage_update_time": "2019-02-13T21:22:47.500",
//!     "current_used": 0
//!   },
//!   "ram_usage": 3446,
//!   "permissions": [
//!     {
//!       "perm_name": "active",
//!       "parent": "owner",
//!       "required_auth": {
//!         "threshold": 1,
//!         "keys": [
//!           {
//!             "key": "EOS7yrz7wmtcQwjGqmHUNuudmFoZUfqSt53ugeRbb75ogEmnz9y6n",
//!             "weight": 1
//!           }
//!         ],
//!         "accounts": [],
//!         "waits": []
//!       },
//!       "linked_actions": []
//!     },
//!     {
//!       "perm_name": "owner",
//!       "parent": "",
//!       "required_auth": {
//!         "threshold": 1,
//!         "keys": [
//!           {
//!             "key": "EOS8WrnTR5CadqpcpKJHQFjATfwaQ3B9xN7x5f8KWrhBKvwCSASr5",
//!             "weight": 1
//!           }
//!         ],
//!         "accounts": [],
//!         "waits": []
//!       },
//!       "linked_actions": []
//!     }
//!   ],
//!   "total_resources": {
//!     "owner": "abhieosindia",
//!     "net_weight": "0.0100 EOS",
//!     "cpu_weight": "0.1000 EOS",
//!     "ram_bytes": 4073
//!   },
//!   "self_delegated_bandwidth": {
//!     "from": "abhieosindia",
//!     "to": "abhieosindia",
//!     "net_weight": "0.0100 EOS",
//!     "cpu_weight": "0.1000 EOS"
//!   },
//!   "refund_request": null,
//!   "voter_info": {
//!     "owner": "abhieosindia",
//!     "proxy": "",
//!     "producers": [],
//!     "staked": 1100,
//!     "last_vote_weight": "0.00000000000000000",
//!     "proxied_vote_weight": "0.00000000000000000",
//!     "is_proxy": 0,
//!     "flags1": 0,
//!     "reserved2": 0,
//!     "reserved3": "0.0000 EOS"
//!   },
//!   "rex_info": null,
//!   "subjective_cpu_bill_limit": {
//!     "used": 0,
//!     "available": 0,
//!     "max": 0,
//!     "last_usage_update_time": "2000-01-01T00:00:00.000",
//!     "current_used": 0
//!   },
//!   "eosio_any_linked_actions": []
//! }
//! ```

use reqwest::{Client, Response, Result, StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::utils::get_api_url;

#[derive(Debug, Deserialize)]
pub struct ResourceLimits {
    used: u32,
    available: u32,
    max: u32,
    last_usage_update_time: String,
    current_used: u32,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    key: String,
    weight: u64,
}

#[derive(Debug, Deserialize)]
pub struct Wait {
    wait_sec: u64,
    weight: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccountPermission {
    actor: String,
    permission: String,
}

#[derive(Debug, Deserialize)]
pub struct Account2 {
    weight: u64,
    permission: AccountPermission,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    threshold: u32,
    keys: Vec<Key>,
    accounts: Vec<Account2>,
    waits: Vec<Wait>,
}

#[derive(Debug, Deserialize)]
pub struct Permission {
    // TODO: add enum with Active, Owner type instead of string
    perm_name: String,
    parent: String,
    required_auth: Auth,
    linked_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceOverview {
    owner: String,
    net_weight: String,
    cpu_weight: String,
    ram_bytes: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResourceDelegation {
    from: String,
    to: String,
    net_weight: String,
    cpu_weight: String,
}

#[derive(Debug, Deserialize)]
pub struct RefundRequest;

#[derive(Debug, Deserialize)]
pub struct RexInfo;

#[derive(Debug, Deserialize)]
pub struct VoterInfo {
    owner: String,
    proxy: String,
    producers: Vec<String>,
    staked: u64,
    last_vote_weight: String,
    proxied_vote_weight: String,
    is_proxy: u32,
    flags1: u32,
    reserved2: u32,
    reserved3: String,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    account_name: String,
    head_block_num: u32,
    head_block_time: String,
    privileged: bool,
    last_code_update: String,
    created: String,
    core_liquid_balance: String,
    ram_quota: u32,
    net_weight: u32,
    cpu_weight: u32,
    net_limit: ResourceLimits,
    cpu_limit: ResourceLimits,
    ram_usage: u32,
    permissions: Vec<Permission>,
    total_resources: ResourceOverview,
    self_delegated_bandwidth: Option<ResourceDelegation>,
    refund_request: Option<RefundRequest>,
    voter_info: Option<VoterInfo>,
    rex_info: RexInfo,
    subjective_cpu_bill_limit: ResourceLimits,
    eosio_any_linked_actions: Vec<String>,
}

/// Get response of account
pub async fn get_response_account(account_name: &str) -> Result<Response> {
    let url = get_api_url()?;
    let url = format!("{}/v1/chain/get_account", url);

    let client = Client::new();

    // Define your JSON payload
    let payload = json!({
        "account_name": account_name
    });

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        return Err(res.status().as_u16()).unwrap();
    }

    Ok(res)
}

/// Get account
pub async fn get_account(res: Response) -> Result<Account> {
    let account = res.json::<Account>().await?;

    Ok(account)
}

// TODO: add functions for getting individual params
