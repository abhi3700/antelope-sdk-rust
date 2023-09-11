use antelope_sdk_rust::get_block_info::*;

#[tokio::main]
/// main entry point
/// So, tokio runtime attribute is used here.
/// All the values are received here as we await the response and then we further
/// extract the required values from `response`
async fn main() {
    let block_num = 329938745;

    // Usage of `get block info` as example
    match get_response_block_info(block_num).await {
        Ok(response) => match get_block(response).await {
            Ok(block) => println!("Block info: {:?}", block),
            Err(error) => eprintln!("Error getting chain info: {:?}", error),
        },
        Err(error) => eprintln!("Error getting response: {:?}", error),
    }
}
