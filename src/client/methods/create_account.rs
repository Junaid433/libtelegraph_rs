use crate::models::{Account, ApiResponse};
use reqwest::Url;
use std::error::Error;

pub async fn create_account(
    client: &reqwest::Client,
    short_name: &str,
    author_name: Option<&str>,
    author_url: Option<&str>,
) -> Result<Account, Box<dyn Error>> {
    let mut url = Url::parse("https://api.telegra.ph/createAccount")?;
    url.query_pairs_mut().append_pair("short_name", short_name);
    if let Some(name) = author_name {
        url.query_pairs_mut().append_pair("author_name", name);
    }
    if let Some(url_str) = author_url {
        url.query_pairs_mut().append_pair("author_url", url_str);
    }

    let resp = client.get(url).send().await?;
    let api_resp: ApiResponse<Account> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
