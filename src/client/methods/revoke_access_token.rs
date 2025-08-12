use crate::models::{Account, ApiResponse};
use reqwest::Url;
use std::error::Error;

pub async fn revoke_access_token(
    client: &reqwest::Client,
    access_token: &str,
) -> Result<Account, Box<dyn Error>> {
    let mut url = Url::parse("https://api.telegra.ph/revokeAccessToken")?;
    url.query_pairs_mut().append_pair("access_token", access_token);

    let resp = client.get(url).send().await?;
    let api_resp: ApiResponse<Account> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
