use antelope_sdk_rust::get_block::*;

#[tokio::main]
/// main entry point
/// So, tokio runtime attribute is used here.
/// All the values are received here as we await the response and then we further
/// extract the required values from `response`
async fn main() {
    let block_num = "329938745";
    let block_id = "13a1609cafb79cda7ba6f739dea81ee36d56c7ca6733e087b5df13afa9afae75";

    // Usage of `get block` as example
    match get_response_block(block_id).await {
        Ok(response) => match get_block(response).await {
            Ok(block) => println!("Block info: {:?}", block),
            Err(error) => eprintln!("Error getting chain info: {:?}", error),
        },
        Err(error) => eprintln!("Error getting response: {:?}", error),
    }
}
