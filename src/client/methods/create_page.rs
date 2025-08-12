use crate::models::{CreatePageRequest, Node, Page, ApiResponse};
use std::error::Error;

pub async fn create_page(
    client: &reqwest::Client,
    access_token: &str,
    title: &str,
    author_name: Option<&str>,
    author_url: Option<&str>,
    content: &[Node],
    return_content: Option<bool>,
) -> Result<Page, Box<dyn Error>> {
    let req = CreatePageRequest {
        access_token: Some(access_token),
        title,
        author_name,
        author_url,
        content,
        return_content,
    };

    let resp = client
        .post("https://api.telegra.ph/createPage")
        .json(&req)
        .send()
        .await?;

    let api_resp: ApiResponse<Page> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
