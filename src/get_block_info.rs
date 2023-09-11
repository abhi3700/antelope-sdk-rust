//! Get block info
//!
//! Using curl:
//! ```sh
//! $ source .env
//! $ curl -X POST -H "Content-Type: application/json" -d '{"block_num": 329938745}' $API_URL/v1/chain/get_block_info`
//! ```
//!
//! RPC API: /v1/chain/get_info
//! Sample response:
//! ```json
//! {
//!   "block_num": 329938745,
//!   "ref_block_num": 30521,
//!   "id": "13aa77399ede88d6d60fc79666e6f560e4e65d3afb17ba2531881084d6b6cc03",
//!   "timestamp": "2023-09-08T07:20:20.500",
//!   "producer": "whaleex.com",
//!   "confirmed": 0,
//!   "previous": "13aa7738fb038d4cf2401b21cbbdc47936e58cb85189433dfd8b5468cfd80078",
//!   "transaction_mroot": "8a19c9bca6fd5db5160d408fd38d00ae08e87495495bb93f9787643ca904437c",
//!   "action_mroot": "aa18e60096cbf4700afc0f12bfe9355772407074975e29e40ef6d4143787a2a1",
//!   "schedule_version": 2084,
//!   "producer_signature": "SIG_K1_Ki8aJ7qXhVzgXcTrFAMGuVXWYxyW1ny4awdvHYcU2YbY6wHyNdgWfsWSKyzFXdjuwP4STdsp7BYDSAg42Hsva4Gu1B4EyN",
//!   "ref_block_prefix": 2529628118
//! }
//! ```

use crate::types::BlockInfo;
use crate::utils::get_api_url;
use reqwest::{Client, Response, Result, StatusCode};
use serde_json::json;

/// Get response of block info
pub async fn get_response_block_info(block_num: u32) -> Result<Response> {
    let url = get_api_url()?;
    let url = format!("{}/v1/chain/get_block_info", url);

    let client = Client::new();

    // Define your JSON payload
    let payload = json!({
        "block_num": block_num
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

/// Get block info
pub async fn get_block_info(res: Response) -> Result<BlockInfo> {
    let block_info = res.json::<BlockInfo>().await?;

    Ok(block_info)
}

// TODO: create more isolated functions
