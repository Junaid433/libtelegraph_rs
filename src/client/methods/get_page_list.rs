use crate::models::{ApiResponse, PageList};
use reqwest::Url;
use std::error::Error;

pub async fn get_page_list(
    client: &reqwest::Client,
    access_token: &str,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<PageList, Box<dyn Error>> {
    let mut url = Url::parse("https://api.telegra.ph/getPageList")?;
    url.query_pairs_mut().append_pair("access_token", access_token);
    if let Some(o) = offset {
        url.query_pairs_mut().append_pair("offset", &o.to_string());
    }
    if let Some(l) = limit {
        url.query_pairs_mut().append_pair("limit", &l.to_string());
    }

    let resp = client.get(url).send().await?;
    let api_resp: ApiResponse<PageList> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
