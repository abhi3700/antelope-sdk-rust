use antelope_sdk_rust::get_info::*;

#[tokio::main]
/// main entry point
/// So, tokio runtime attribute is used here.
/// All the values are received here as we await the response and then we further
/// extract the required values from `response`
async fn main() {
    // Usage of `get chain info` as example
    match get_response_chain_info().await {
        Ok(response) => match get_chain_info(response).await {
            Ok(chain_info) => println!("Chain info: {:?}", chain_info),
            Err(error) => eprintln!("Error getting chain info: {:?}", error),
        },
        Err(error) => eprintln!("Error getting response: {:?}", error),
    }
}
