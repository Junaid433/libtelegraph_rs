use crate::models::page::Page;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageList {
    pub total_count: u32,
    pub pages: Vec<Page>,
}