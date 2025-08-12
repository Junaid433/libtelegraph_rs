use crate::models::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreatePageRequest<'a> {
    pub access_token: Option<&'a str>,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_url: Option<&'a str>,
    pub content: &'a [Node],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_content: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    pub path: String,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub image_url: Option<String>,
    pub content: Option<Vec<Node>>,
    pub views: Option<u32>,
    pub can_edit: Option<bool>,
}
