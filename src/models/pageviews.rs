use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageViews {
    pub views: u32,
}
