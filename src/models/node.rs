use serde::{Deserialize, Serialize};
use std::collections::HashMap; 

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeElement {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
    Text(String),
    Element(NodeElement),
}