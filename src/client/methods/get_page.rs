use crate::models::{ApiResponse, Page};
use reqwest::Url;
use std::error::Error;

pub async fn get_page(
    client: &reqwest::Client,
    path: &str,
    return_content: Option<bool>,
) -> Result<Page, Box<dyn Error>> {
    let mut url = Url::parse(&format!("https://api.telegra.ph/getPage/{}", path))?;
    if let Some(ret) = return_content {
        url.query_pairs_mut()
            .append_pair("return_content", if ret { "true" } else { "false" });
    }

    let resp = client.get(url).send().await?;
    let api_resp: ApiResponse<Page> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
