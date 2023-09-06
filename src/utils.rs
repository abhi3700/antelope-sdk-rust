use reqwest::Result;
use std::env::VarError;

/// Get API URL
pub(crate) fn get_api_url() -> Result<String> {
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
