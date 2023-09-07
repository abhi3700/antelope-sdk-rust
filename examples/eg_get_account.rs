use antelope_sdk_rust::get_account::*;

#[tokio::main]
/// main entry point
/// So, tokio runtime attribute is used here.
/// All the values are received here as we await the response and then we further
/// extract the required values from `response`
async fn main() {
    let acc_name = "abhieosindia";

    // Usage of `get account` as example
    match get_response_account(acc_name).await {
        Ok(response) => match get_account(response).await {
            Ok(acc) => println!("Account info: {:?}", acc),
            Err(error) => eprintln!("Error getting chain info: {:?}", error),
        },
        Err(error) => eprintln!("Error getting response: {:?}", error),
    }
}
