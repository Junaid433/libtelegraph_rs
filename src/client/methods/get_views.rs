use crate::models::{ApiResponse, PageViews};
use reqwest::Url;
use std::error::Error;

pub async fn get_views(
    client: &reqwest::Client,
    path: &str,
    year: Option<u32>,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
) -> Result<PageViews, Box<dyn Error>> {
    let mut url = Url::parse(&format!("https://api.telegra.ph/getViews/{}", path))?;

    if let Some(y) = year {
        url.query_pairs_mut().append_pair("year", &y.to_string());
    }
    if let Some(m) = month {
        url.query_pairs_mut().append_pair("month", &m.to_string());
    }
    if let Some(d) = day {
        url.query_pairs_mut().append_pair("day", &d.to_string());
    }
    if let Some(h) = hour {
        url.query_pairs_mut().append_pair("hour", &h.to_string());
    }

    let resp = client.get(url).send().await?;
    let api_resp: ApiResponse<PageViews> = resp.json().await?;

    if api_resp.ok {
        Ok(api_resp.result.expect("Missing result"))
    } else {
        Err(api_resp.error.unwrap_or_else(|| "Unknown error".to_string()).into())
    }
}
