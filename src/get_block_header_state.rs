//! Get block header state
//! NOTE: take the block status from irreversible ones i.e. the block that's not irreversible yet.
//! Take the latest block via `$ curl $API_URL/v1/chain/get_info`
//!
//! Using curl:
//! ```sh
//! $ source .env
//! $ curl -X POST -H "Content-Type: application/json" -d '{"block_num_or_id": "13a1609cafb79cda7ba6f739dea81ee36d56c7ca6733e087b5df13afa9afae75"}' $API_URL/v1/chain/get_block_header_state
//! $ curl -X POST -H "Content-Type: application/json" -d '{"block_num_or_id": "329343132"}' $API_URL/v1/chain/get_block_header_state
//! ```
//!
//! RPC API: /v1/chain/get_block_header_state
//! Sample response:
//! ```json
//! {
//!   "block_num": 330510957,
//!   "dpos_proposed_irreversible_blocknum": 330510794,
//!   "dpos_irreversible_blocknum": 330510626,
//!   "active_schedule": {
//!     "version": 2085,
//!     "producers": [
//!       {
//!         "producer_name": "atticlabeosb",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS7PfA3A4UdfMu2wKbuXdbHn8EWAxbMnFoFWui4X2zsr2oPwdQJP",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "aus1genereos",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS6on4KsoQ3cjhXLixNQxB3jwYhmUhNK9rKTABdmBBZWbfNmhTeU",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "big.one",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS8MpYyXwn3DLqk9Y9XTHYcd6wGGijNqJefFoQEwEoXTq1awZ42w",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "binancestake",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS7unwwVJfmKonrT6Gj46LDiNUPpFhpPALpTe2eofmFeoG74bKKn",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "bitfinexeos1",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS4tkw7LgtURT3dvG3kQ4D1sg3aAtPDymmoatpuFkQMc7wzZdKxc",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "blockpooleos",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS61FDJz3GC42GhaPSsmKh7SxuesyZhjm7hBwBKqN52v1HukEqBu",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "bp.defi",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS5BoXgRJwL7JFvKnV64Q3Ha3ux6x2cP8nnhU9NVrRkyrhPC3m5b",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eosasia11111",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS76gG6ATpqfVf5KrVjh3f4JAa4EKzAwWabTucNQ4Xv2TmVAj9bN",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eoscannonchn",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS73cTi9V7PNg4ujW5QzoTfRSdhH44MPiUJkUV6m3oGwj7RX7kML",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eoseouldotio",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS6SSA4gYCSZ3q9NWpxGsYDv5MWjSwKseyq25RRZexwj8EM6YHDa",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eosflytomars",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS6Agpfp38bTyRjJDmB4Qb1EpQSq7wnEAsALXgXE7KFSzKjokkFD",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eosinfstones",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS6CSvGzNhNxVYbcnWSuheNcfzjGeGBY9trR4YAJ4Yvakq4oCh6y",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eosiosg11111",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS7zVBQMhV7dZ5zRQwBgDmmbFCHA6YcmwW6Dq5CePGpqLR1ZsVAc",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eoslaomaocom",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS8QgURqo875qu3a8vgZ58qBeu2cTehe9zAWRfpdCXAQipicu1Fi",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eosnationftw",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS8L12yBrtx7mpewHmjwgJeNb2aLaeQdoDgMW82dzDSu17ec2XNL",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "eosphereiobp",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS5FYRrG3ThNU56d3uZ8d4bo4MsN5Fig9WPY3xYSpH3RqPhpoH3A",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "hashfineosio",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS7jSfvStvbKDmGvQdtrQsCyNkWczXfvh6CHmBVmeypJyHsUrMqj",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "ivote4eosusa",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS6KzD4YVbuXV5uBH5d4Ay4sTzuQk88ivmnWfJPLoo6SFrX6iyqj",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "newdex.bp",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS688SnH8tQ7NiyhamiCzWXAGPDLF9S7K8ga79UBHKFgjS1MhqhB",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "starteosiobp",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS4wZZXm994byKANLuwHD6tV3R3Mu3ktc41aSVXCBaGnXJZJ4pwF",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       },
//!       {
//!         "producer_name": "whaleex.com",
//!         "authority": [
//!           0,
//!           {
//!             "threshold": 1,
//!             "keys": [
//!               {
//!                 "key": "EOS88EGcFghfQJER1mDaEe4kDJ7MGDoPmXQfA7q2QMTLLqiYP1UQR",
//!                 "weight": 1
//!               }
//!             ]
//!           }
//!         ]
//!       }
//!     ]
//!   },
//!   "blockroot_merkle": {
//!     "_active_nodes": [
//!       "f7ca92cede8bd5f10eaa18a6956471e3ae917ea531c77a8f468a43b44452838d",
//!       "4985d6f083806612b19ca73b16e9473e12953ff590253212707e5da69b13c9ab",
//!       "0cd7462fc3d07c49c88a7154ea6191911ca278a7a7baa9c7e0bfe0a41112484a",
//!       "bf65d49089ebdf2a1bd6ec3503562f2f69918f144a06033b3d174dff0ad7672b",
//!       "7a7717b3dc6bddb8f4031c664f326eede97fb40171cb32c9c0e9325d4cea40d2",
//!       "18552ea5c60a6ef314c7bfca940a4238ad0c0553658d705655b9ce90e99a56f4",
//!       "87c900e630f9c6977052516b0d9b7b507aaeaf7d8a73ba734af0489baf793cb8",
//!       "19d7e54e47b02d2cd2948c786e7ac4f400a5d23df4a8beaea76464ec8c64dc53",
//!       "69bee88feb5e8a20e2c2cf4708700c81c3830ba74a4d4d008609ed536a3cb8b8",
//!       "cf77fdb4e8688ec08847aa8aef1a03618d211dd2059a86ff58e0b6c94079afbb",
//!       "deb7e05ba7c30f421664ade178d8dac9992926746851914559d160b88c0aed2b",
//!       "b74d36dd75f8c3b9dcb735959751d61ad8ab903c611a52ae5ce25309906ac553",
//!       "d35daf5afbda7394e07768377c151b9505d605edd3e066df0ef3f2ad29ce4956",
//!       "d9a29e3bf09ab99bb4c23c2824ee71c11c94ea493abce7c106e3b0ee68d1256f",
//!       "5a4c2d8587c853d2d433d75dbebba03275bcea54c7ed0bec018e47909dbd4624",
//!       "d2793244d9885ad9bfe064762db5397d6b90c61d50ee9ee83c84fa788de5786d"
//!     ],
//!     "_node_count": 330510956
//!   },
//!   "producer_to_last_produced": [
//!     ["atticlabeosb", 330510890],
//!     ["aus1genereos", 330510902],
//!     ["big.one", 330510914],
//!     ["binancestake", 330510926],
//!     ["bitfinexeos1", 330510938],
//!     ["blockpooleos", 330510950],
//!     ["bp.defi", 330510957],
//!     ["eosasia11111", 330510722],
//!     ["eoscannonchn", 330510734],
//!     ["eoseouldotio", 330510746],
//!     ["eosflytomars", 330510758],
//!     ["eosinfstones", 330510770],
//!     ["eosiosg11111", 330510782],
//!     ["eoslaomaocom", 330510794],
//!     ["eosnationftw", 330510806],
//!     ["eosphereiobp", 330510818],
//!     ["hashfineosio", 330510830],
//!     ["ivote4eosusa", 330510842],
//!     ["newdex.bp", 330510854],
//!     ["starteosiobp", 330510866],
//!     ["whaleex.com", 330510878]
//!   ],
//!   "producer_to_last_implied_irb": [
//!     ["atticlabeosb", 330510722],
//!     ["aus1genereos", 330510734],
//!     ["big.one", 330510746],
//!     ["binancestake", 330510758],
//!     ["bitfinexeos1", 330510770],
//!     ["blockpooleos", 330510782],
//!     ["bp.defi", 330510794],
//!     ["eosasia11111", 330510554],
//!     ["eoscannonchn", 330510566],
//!     ["eoseouldotio", 330510578],
//!     ["eosflytomars", 330510590],
//!     ["eosinfstones", 330510602],
//!     ["eosiosg11111", 330510614],
//!     ["eoslaomaocom", 330510626],
//!     ["eosnationftw", 330510638],
//!     ["eosphereiobp", 330510650],
//!     ["hashfineosio", 330510662],
//!     ["ivote4eosusa", 330510674],
//!     ["newdex.bp", 330510686],
//!     ["starteosiobp", 330510698],
//!     ["whaleex.com", 330510710]
//!   ],
//!   "valid_block_signing_authority": [
//!     0,
//!     {
//!       "threshold": 1,
//!       "keys": [
//!         {
//!           "key": "EOS5BoXgRJwL7JFvKnV64Q3Ha3ux6x2cP8nnhU9NVrRkyrhPC3m5b",
//!           "weight": 1
//!         }
//!       ]
//!     }
//!   ],
//!   "confirm_count": [
//!     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3,
//!     3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5,
//!     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7,
//!     7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9,
//!     9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11,
//!     11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12,
//!     12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 14, 14, 14,
//!     14, 14, 14, 14
//!   ],
//!   "id": "13b3326d941fe4d0e4d479b83a12e252b32d20d77c4f12a71e3519c1a4916eea",
//!   "header": {
//!     "timestamp": "2023-09-11T14:52:15.000",
//!     "producer": "bp.defi",
//!     "confirmed": 0,
//!     "previous": "13b3326c1435c2ba4663715adc05353af96ce2edf4c9cf1462002201dbae715b",
//!     "transaction_mroot": "1e39f1cf8478d60cb864a41487b944520eadd8893c76aa6593527999a01ce613",
//!     "action_mroot": "870175bb0cd7a04aea4655211605e7a86d3b7aea086d023f23dc2d58f232e3b3",
//!     "schedule_version": 2085,
//!     "header_extensions": [],
//!     "producer_signature": "SIG_K1_KiSZ6Tdy7GHNEVYND7XJY6KNEeGE3wLpwxXW1j76tbQsqmx1hEzB1yhiuv2czMBJVrrrnAhz2mfEk8aCJH9rWwQo8b9bjp"
//!   },
//!   "pending_schedule": {
//!     "schedule_lib_num": 330323099,
//!     "schedule_hash": "41b3a669a9696478dfbe0d90daa5be984bb415c8f09b00a7e473fa1e449a89c0",
//!     "schedule": { "version": 2085, "producers": [] }
//!   },
//!   "activated_protocol_features": {
//!     "protocol_features": [
//!       "0ec7e080177b2c02b278d5088611686b49d739925a92d9bfcacd7fc6b74053bd",
//!       "1a99a59d87e06e09ec5b028a9cbb7749b4a5ad8819004365d02dc4379a8b7241",
//!       "2652f5f96006294109b3dd0bbde63693f55324af452b799ee137a81a905eed25",
//!       "299dcb6af692324b899b39f16d5a530a33062804e41f09dc97e9f156b4476707",
//!       "35c2186cc36f7bb4aeaf4487b36e57039ccf45a9136aa856a5d569ecca55ef2b",
//!       "4a90c00d55454dc5b059055ca213579c6ea856967712a56017487886a4d4cc0f",
//!       "4e7bf348da00a945489b2a681749eb56f5de00b900014e137ddae39f48f69d67",
//!       "4fca8bd82bbd181e714e283f83e1b45d95ca5af40fb89ad3977b653c448f78c2",
//!       "5443fcf88330c586bc0e5f3dee10e7f63c76c00249c87fe4fbf7f38c082006b4",
//!       "68dcaa34c0517d19666e6b33add67351d8c5f69e999ca1e37931bc410a297428",
//!       "6bcb40a24e49c26d0a60513b6aeb8551d264e4717f306b81a37a5afb3b47cedc",
//!       "8ba52fe7a3956c5cd3a656a3174b931d3bb2abb45578befc59f283ecd816a405",
//!       "ad9e3d8f650687709fd68f4b90b41f7d825a365b02c23a636cef88ac2ac00c43",
//!       "bcd2a26394b36614fd4894241d3c451ab0f6fd110958c3423073621a70826e99",
//!       "c3a6138c5061cf291310887c0b5c71fcaffeab90d5deb50d3b9e687cead45071",
//!       "d528b9f6e9693f45ed277af93474fd473ce7d831dae2180cca35d907bd10cb40",
//!       "e0fb64b1085cc5538970158d05a009c24e276fb94e1a0bf6a528b48fbc4ff526",
//!       "ef43112c6543b88db2283a2e077278c315ae2c84719a8b25f25cc88565fbea99",
//!       "f0af56d2c5a48d60a4a5b5c903edfb7db3a736a94ed589d0b797df33ff9d3e1d"
//!     ]
//!   },
//!   "additional_signatures": []
//! }
//! ```

use crate::types::BlockHeaderState;
use crate::utils::get_api_url;
use reqwest::{Client, Response, Result, StatusCode};
use serde_json::json;

/// Get response of block header state
pub async fn get_response_block_header_state(block_num_or_id: &str) -> Result<Response> {
    let url = get_api_url()?;
    let url = format!("{}/v1/chain/get_block_header_state", url);

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

/// Get block header state
pub async fn get_block_header_state(res: Response) -> Result<BlockHeaderState> {
    let block_header_state = res.json::<BlockHeaderState>().await?;

    Ok(block_header_state)
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
