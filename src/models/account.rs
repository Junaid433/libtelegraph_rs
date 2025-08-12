use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub short_name: String,
    pub author_name: Option<String>,
    pub author_url: Option<String> ,
    pub access_token: Option<String>, 
    pub auth_url: Option<String> ,
    pub page_count: Option<u32>
}