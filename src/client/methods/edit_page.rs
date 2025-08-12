use crate::models::{ApiResponse, Node, Page};
use reqwest::Url;
use serde_json::json;
use std::error::Error;

pub async fn edit_page(
    client: &reqwest::Client,
    access_token: &str,
    path: &str,
    title: &str,
    author_name: Option<&str>,
    author_url: Option<&str>,
    content: &[Node],
    return_content: Option<bool>,
) -> Result<Page, Box<dyn Error>> {
    let mut url = Url::parse(&format!("https://api.telegra.ph/editPage/{}", path))?;
    url.query_pairs_mut().append_pair("access_token", access_token);

    let mut json_body = json!({
        "title": title,
        "content": content,
    });
    if let Some(name) = author_name {
        json_body["author_name"] = json!(name);
    }
    if let Some(url_str) = author_url {
        json_body["author_url"] = json!(url_str);
    }
    if let Some(ret) = return_content {
        json_body["return_content"] = json!(ret);
    }

    let resp = client.post(url).json(&json_body).send().await?;
    let api_resp: ApiResponse<Page> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
