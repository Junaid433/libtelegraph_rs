use crate::models::{Account, ApiResponse};
use reqwest::Url;
use serde_json::to_string;
use std::error::Error;

pub async fn get_account_info(
    client: &reqwest::Client,
    access_token: &str,
    fields: Option<&[&str]>,
) -> Result<Account, Box<dyn Error>> {
    let mut url = Url::parse("https://api.telegra.ph/getAccountInfo")?;
    url.query_pairs_mut().append_pair("access_token", access_token);

    if let Some(fields_list) = fields {
        let fields_json = to_string(fields_list)?;
        url.query_pairs_mut().append_pair("fields", &fields_json);
    }

    let resp = client.get(url).send().await?;
    let api_resp: ApiResponse<Account> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
