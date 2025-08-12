pub mod account;
pub mod node;
pub mod page;
pub mod pagelist;
pub mod pageviews;

pub use account::Account;
pub use node::{Node, NodeElement};
pub use page::{CreatePageRequest, Page};
pub use pagelist::PageList;
pub use pageviews::PageViews;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error: Option<String>,
}
