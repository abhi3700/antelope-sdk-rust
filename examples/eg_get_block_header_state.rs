use antelope_sdk_rust::{get_block_header_state::*, get_info::*};

#[tokio::main]
/// main entry point
/// So, tokio runtime attribute is used here.
/// All the values are received here as we await the response and then we further
/// extract the required values from `response`
async fn main() {
    // get latest block num or id
    match get_response_chain_info().await {
        Ok(response) => match get_chain_info(response).await {
            Ok(chain_info) => {
                let block_num = &chain_info.head_block_num.to_string();
                let block_id = &chain_info.head_block_id.to_string();
                println!("Block num: {}", block_num);

                // Usage of `get block header state` as example
                match get_response_block_header_state(block_num).await {
                    Ok(response) => match get_block_header_state(response).await {
                        Ok(block) => println!("Block info: {:?}", block),
                        Err(error) => {
                            eprintln!("BlockHeaderInfo | Error getting chain info: {:?}", error)
                        }
                    },
                    Err(error) => {
                        eprintln!("BlockHeaderInfo | Error getting response: {:?}", error)
                    }
                }
            }
            Err(error) => eprintln!("ChainInfo | Error getting chain info: {:?}", error),
        },
        Err(error) => eprintln!("ChainInfo | Error getting response: {:?}", error),
    }
}
