//! Get block
//!
//! Using curl:
//! ```sh
//! $ source .env
//! $ curl -X POST -H "Content-Type: application/json" -d '{"block_num_or_id": "13a1609cafb79cda7ba6f739dea81ee36d56c7ca6733e087b5df13afa9afae75"}' $API_URL/v1/chain/get_block`
//! $ curl -X POST -H "Content-Type: application/json" -d '{"block_num_or_id": "329343132"}' $API_URL/v1/chain/get_block`
//! ```
//!
//! RPC API: /v1/chain/get_block
//! Sample response:
//! ```json
//! {
//!   "timestamp": "2023-09-04T20:36:31.500",
//!   "producer": "blockpooleos",
//!   "confirmed": 0,
//!   "previous": "13a1609b317378a30e7d286cc0b70837e1e4f2c37023200154b7936aceb1ff2b",
//!   "transaction_mroot": "44ceff918372b381202915711498da1562a2e5baa61d60928b11a68885926faa",
//!   "action_mroot": "52d79f93397be4ea0d182abdf618a11766d3bfe4b865d3ab8eac045a34bc716c",
//!   "schedule_version": 2084,
//!   "new_producers": null,
//!   "producer_signature": "SIG_K1_KiHzt3Z2aN5rcMLnW5EvpVHVNznKkYzr3SKsYNd74V4xWbvm5kpm7wLTm6uxTQgQJRuEnJ9fqTgvC3bWPt2uHRsGVgdxBp",
//!   "transactions": [
//!     {
//!       "status": "executed",
//!       "cpu_usage_us": 2070,
//!       "net_usage_words": 70,
//!       "trx": {
//!         "id": "d9e094d91979dde2716cc09a4e1808d037d9a4b6ed10cf54e2316560e9f0ca38",
//!         "signatures": [
//!           "SIG_K1_K6QpgCJxdMhiYFpZKFbDgX6ezLh9WnU5JzjT4i1cqCGbtmumwmM2iGunAXQHHSPNkwP5jz7Ff8YhFAJtWuFoYGLQGRY1zc",
//!           "SIG_K1_KcDvpwd4tXQJpgfsmHdhPqeMrsXtCJbpsAbeXgEqndMG7A6k8RDLFiH5G2M6PyiG9CQeMHNmAZgZNu2JwghuNKCrt1hftF"
//!         ],
//!         "compression": "none",
//!         "packed_context_free_data": "",
//!         "context_free_data": [],
//!         "packed_trx": "0540f6648f60218940110000000002a0649a2656ed4dac0000d0155dbabca901a0649a2656ed4dac0000d0155dbabca908d0ee91ed9b37dcb1a0649a2656ed4dac000000000000c29801d0ee91ed9b37dcb100000000a46962d59903d0ee91ed9b37dcb13235fb26029d4a00000ffa262f9f4a00001cfb26289f4a000077fb26e09d4a000045fc264da04a000050de19075c48000006f726e3a14a00008cf72631a24a0000d4fa26389c4a00006af7261ca24a000029f726c4a14a0000eeea26b7a64a0000c45c47b5fd2500003dde191e5b480000a2f926e19c4a0000e2f72673a24a000028f726eea14a000088f826c1a24a00005df726fca14a000094f926cb9c4a00001b404597a22500004cfc26ce9e4a00000dfb26de9d4a0000b2f82601a24a00009ef72660a24a00009ef926c19a4a00007fbf19be53480000403145ba9c250000f9f626bea14a000088f826d5a24a00000eeb260ba74a000027f7260aa24a0000cef72670a24a000061ed26c6a74a0000b3eb26c6a74a00009cf926059b4a00004af726dda14a0000861c2cb4e943000050202cf7f2430000961849fe414a000031ec2649a94a0000c7ea26afa74a0000c7ea26b9a74a000028eb263da84a0000b9ea26aca74a0000870d1d8a1646000031ec2653a94a000019eb2678a84a0000dfea26caa74a0000302b1c2ff646000000",
//!         "transaction": {
//!           "expiration": "2023-09-04T20:37:25",
//!           "ref_block_num": 24719,
//!           "ref_block_prefix": 289442081,
//!           "max_net_usage_words": 0,
//!           "max_cpu_usage_ms": 0,
//!           "delay_sec": 0,
//!           "context_free_actions": [],
//!           "actions": [
//!             {
//!               "account": "playuplandme",
//!               "name": "payforcpu",
//!               "authorization": [
//!                 { "actor": "playuplandme", "permission": "payforcpu" }
//!               ],
//!               "data": { "user_name": "qbi3jazhmbrh" },
//!               "hex_data": "d0ee91ed9b37dcb1"
//!             },
//!             {
//!               "account": "playuplandme",
//!               "name": "n31",
//!               "authorization": [
//!                 { "actor": "qbi3jazhmbrh", "permission": "upland" }
//!               ],
//!               "data": {
//!                 "a54": "qbi3jazhmbrh",
//!                 "p55": [
//!                   "82038206430005",
//!                   "82047551339023",
//!                   "82047433898780",
//!                   "82041930972023",
//!                   "82052349623365",
//!                   "79560093326928",
//!                   "82059161171718",
//!                   "82060469794700",
//!                   "82034817432276",
//!                   "82060117473130",
//!                   "82058641078057",
//!                   "82079897807598",
//!                   "41771598306500",
//!                   "79556184235581",
//!                   "82037652781474",
//!                   "82061577091042",
//!                   "82059345721128",
//!                   "82062885714056",
//!                   "82059580602205",
//!                   "82037283682708",
//!                   "41380252827675",
//!                   "82045923949644",
//!                   "82041897417485",
//!                   "82059664488626",
//!                   "82061258323870",
//!                   "82028525975966",
//!                   "79524508843903",
//!                   "41355070222656",
//!                   "82058540414713",
//!                   "82063221258376",
//!                   "82081307093774",
//!                   "82059815483175",
//!                   "82061526759374",
//!                   "82084444433761",
//!                   "82084444433331",
//!                   "82029666826652",
//!                   "82059060508490",
//!                   "74671029230726",
//!                   "74710808010832",
//!                   "81647299532950",
//!                   "82090937216049",
//!                   "82084058557127",
//!                   "82084226329287",
//!                   "82086440921896",
//!                   "82084008225465",
//!                   "77062620384647",
//!                   "82091104988209",
//!                   "82087430777625",
//!                   "82084511541983",
//!                   "78023166274352"
//!                 ]
//!               },
//!               "hex_data": "d0ee91ed9b37dcb13235fb26029d4a00000ffa262f9f4a00001cfb26289f4a000077fb26e09d4a000045fc264da04a000050de19075c48000006f726e3a14a00008cf72631a24a0000d4fa26389c4a00006af7261ca24a000029f726c4a14a0000eeea26b7a64a0000c45c47b5fd2500003dde191e5b480000a2f926e19c4a0000e2f72673a24a000028f726eea14a000088f826c1a24a00005df726fca14a000094f926cb9c4a00001b404597a22500004cfc26ce9e4a00000dfb26de9d4a0000b2f82601a24a00009ef72660a24a00009ef926c19a4a00007fbf19be53480000403145ba9c250000f9f626bea14a000088f826d5a24a00000eeb260ba74a000027f7260aa24a0000cef72670a24a000061ed26c6a74a0000b3eb26c6a74a00009cf926059b4a00004af726dda14a0000861c2cb4e943000050202cf7f2430000961849fe414a000031ec2649a94a0000c7ea26afa74a0000c7ea26b9a74a000028eb263da84a0000b9ea26aca74a0000870d1d8a1646000031ec2653a94a000019eb2678a84a0000dfea26caa74a0000302b1c2ff6460000"
//!             }
//!           ]
//!         }
//!       }
//!     },
//!     {
//!       "status": "executed",
//!       "cpu_usage_us": 503,
//!       "net_usage_words": 14,
//!       "trx": {
//!         "id": "c63e115601b5149ea37e24ef4262282bebc55c24a5d011615a5bca1b58d34298",
//!         "signatures": [
//!           "SIG_K1_KjvZTAbBhhW6vGZRuRVo6fMQi3kCJYVKQuoTaZdzbintdtUrenbXraGjsZK9LPdpeXihyysVAG6TNDzhtMjjzYFzHy9fbw"
//!         ],
//!         "compression": "none",
//!         "packed_context_free_data": "",
//!         "context_free_data": [],
//!         "packed_trx": "f53ff664936054dbbd260000000001000000a063d0b0ae0000000000a0a69301e0359b7aa996315500000000a8ed323210e0359b7aa99631558a1a00000000000000",
//!         "transaction": {
//!           "expiration": "2023-09-04T20:37:09",
//!           "ref_block_num": 24723,
//!           "ref_block_prefix": 649976660,
//!           "max_net_usage_words": 0,
//!           "max_cpu_usage_ms": 0,
//!           "delay_sec": 0,
//!           "context_free_actions": [],
//!           "actions": [
//!             {
//!               "account": "push.sx",
//!               "name": "mine",
//!               "authorization": [
//!                 { "actor": "eosthefunguy", "permission": "active" }
//!               ],
//!               "data": { "executor": "eosthefunguy", "nonce": 6794 },
//!               "hex_data": "e0359b7aa99631558a1a000000000000"
//!             }
//!           ]
//!         }
//!       }
//!     },
//!     {
//!       "status": "executed",
//!       "cpu_usage_us": 506,
//!       "net_usage_words": 33,
//!       "trx": {
//!         "id": "3ef5939bf8254d6ee173ce3cc39abc2269170afd62ddc2dfc2742b11b5b75237",
//!         "signatures": [
//!           "SIG_K1_K3AQGZz8YdRtdJXyQU5aR9WdYKkcrqxeRSmcQdUoSgbx8sewxZe1Ffi72YuhFAszxp64DHC52ibGR8KVhsL8fxdnUW28Fm",
//!           "SIG_K1_Jup5twVXR9n6uX1M63kEqoSLjxbCVnNHXf35rtSp7aEib2Eu3ja9Xxvdd6xBDTCs9fe29rVtKzVgUuEg1gGSWcan9v1gid",
//!           "SIG_K1_KAzNwrf4uizF4vzcqjNYuvaYgSSQP8yQ1sPxE5pUcg22F3Zbf4bwDcd7QU1JJKCy99VyucE5wQD2GhHLGFEuDvhfmgA42z"
//!         ],
//!         "compression": "none",
//!         "packed_context_free_data": "",
//!         "context_free_data": [],
//!         "packed_trx": "ad4df66436600d5e5f780000000003a0649a2656ed4dac0000d0155dbabca901a0649a2656ed4dac0000d0155dbabca908c074d591cc9309e790113253419a7bd5000000572d3ccdcd01c074d591cc9309e700000000a46962d521c074d591cc9309e7a0649a2656ed4dacc409000000000000025550580000000000a0649a2656ed4dac000000000000029901a0649a2656ed4dac000000c067175dd620c074d591cc9309e76e543f1f22500000c409000000000000025550580000000000",
//!         "transaction": {
//!           "expiration": "2023-09-04T21:35:41",
//!           "ref_block_num": 24630,
//!           "ref_block_prefix": 2019515917,
//!           "max_net_usage_words": 0,
//!           "max_cpu_usage_ms": 0,
//!           "delay_sec": 0,
//!           "context_free_actions": [],
//!           "actions": [
//!             {
//!               "account": "playuplandme",
//!               "name": "payforcpu",
//!               "authorization": [
//!                 { "actor": "playuplandme", "permission": "payforcpu" }
//!               ],
//!               "data": { "user_name": "ww4tbn4lupug" },
//!               "hex_data": "c074d591cc9309e7"
//!             },
//!             {
//!               "account": "upxtokenacct",
//!               "name": "transfer",
//!               "authorization": [
//!                 { "actor": "ww4tbn4lupug", "permission": "upland" }
//!               ],
//!               "data": {
//!                 "from": "ww4tbn4lupug",
//!                 "to": "playuplandme",
//!                 "quantity": "25.00 UPX",
//!                 "memo": ""
//!               },
//!               "hex_data": "c074d591cc9309e7a0649a2656ed4dacc409000000000000025550580000000000"
//!             },
//!             {
//!               "account": "playuplandme",
//!               "name": "n41",
//!               "authorization": [
//!                 { "actor": "playuplandme", "permission": "utility" }
//!               ],
//!               "data": {
//!                 "p51": "ww4tbn4lupug",
//!                 "a45": "88107483354222",
//!                 "p54": "25.00 UPX"
//!               },
//!               "hex_data": "c074d591cc9309e76e543f1f22500000c4090000000000000255505800000000"
//!             }
//!           ]
//!         }
//!       }
//!     }
//!   ],
//!   "id": "13a1609cafb79cda7ba6f739dea81ee36d56c7ca6733e087b5df13afa9afae75",
//!   "block_num": 329343132,
//!   "ref_block_prefix": 972531323
//! }
//! ```

use crate::types::Block;
use crate::utils::get_api_url;
use reqwest::{Client, Response, Result, StatusCode};
use serde_json::json;

/// Get response of block
pub async fn get_response_block(block_num_or_id: &str) -> Result<Response> {
    let url = get_api_url()?;
    let url = format!("{}/v1/chain/get_block", url);

    let client = Client::new();

    // Define your JSON payload
    let payload = json!({
        "block_num_or_id": block_num_or_id
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

/// Get block
pub async fn get_block(res: Response) -> Result<Block> {
    let block = res.json::<Block>().await?;

    Ok(block)
}

// TODO: create more isolated functions

// TODO: tests: for `block_num` & `block_id` as input
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_block_num() {
        todo!()
    }

    #[test]
    fn test_with_block_id() {
        todo!()
    }
}
