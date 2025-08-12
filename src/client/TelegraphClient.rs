use crate::models::{
    Account, ApiResponse, Node, Page, PageList, PageViews,
};
use reqwest::{blocking::Client, Url};
use serde_json::json;
use std::error::Error;
use crate::models::CreatePageRequest;


pub struct TelegraphClient {
    client: Client,
}

impl TelegraphClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn create_account(
        &self,
        short_name: &str,
        author_name: Option<&str>,
        author_url: Option<&str>,
    ) -> Result<Account, Box<dyn Error>> {
        let mut url = Url::parse("https://api.telegra.ph/createAccount")?;
        url.query_pairs_mut().append_pair("short_name", short_name);
        if let Some(name) = author_name {
            url.query_pairs_mut().append_pair("author_name", name);
        }
        if let Some(url_str) = author_url {
            url.query_pairs_mut().append_pair("author_url", url_str);
        }

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<Account> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn create_page(
        &self,
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

        let resp = self
            .client
            .post("https://api.telegra.ph/createPage")
            .json(&req)
            .send()?;

        let api_resp: ApiResponse<Page> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn edit_account_info(
        &self,
        access_token: &str,
        short_name: Option<&str>,
        author_name: Option<&str>,
        author_url: Option<&str>,
    ) -> Result<Account, Box<dyn Error>> {
        let mut url = Url::parse("https://api.telegra.ph/editAccountInfo")?;
        url.query_pairs_mut()
            .append_pair("access_token", access_token);

        if let Some(name) = short_name {
            url.query_pairs_mut().append_pair("short_name", name);
        }
        if let Some(author) = author_name {
            url.query_pairs_mut().append_pair("author_name", author);
        }
        if let Some(url_str) = author_url {
            url.query_pairs_mut().append_pair("author_url", url_str);
        }

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<Account> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn edit_page(
        &self,
        access_token: &str,
        path: &str,
        title: &str,
        author_name: Option<&str>,
        author_url: Option<&str>,
        content: &[Node],
        return_content: Option<bool>,
    ) -> Result<Page, Box<dyn Error>> {
        let mut url = Url::parse(&format!("https://api.telegra.ph/editPage/{}", path))?;
        url.query_pairs_mut()
            .append_pair("access_token", access_token);

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

        let resp = self.client.post(url).json(&json_body).send()?;
        let api_resp: ApiResponse<Page> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn get_account_info(
        &self,
        access_token: &str,
        fields: Option<&[&str]>,
    ) -> Result<Account, Box<dyn Error>> {
        let mut url = Url::parse("https://api.telegra.ph/getAccountInfo")?;
        url.query_pairs_mut()
            .append_pair("access_token", access_token);

        if let Some(fields_list) = fields {
            let fields_json = serde_json::to_string(fields_list)?;
            url.query_pairs_mut().append_pair("fields", &fields_json);
        }

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<Account> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn get_page(
        &self,
        path: &str,
        return_content: Option<bool>,
    ) -> Result<Page, Box<dyn Error>> {
        let mut url = Url::parse(&format!("https://api.telegra.ph/getPage/{}", path))?;
        if let Some(ret) = return_content {
            url.query_pairs_mut()
                .append_pair("return_content", if ret { "true" } else { "false" });
        }

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<Page> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn get_page_list(
        &self,
        access_token: &str,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<PageList, Box<dyn Error>> {
        let mut url = Url::parse("https://api.telegra.ph/getPageList")?;
        url.query_pairs_mut()
            .append_pair("access_token", access_token);
        if let Some(o) = offset {
            url.query_pairs_mut().append_pair("offset", &o.to_string());
        }
        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        }

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<PageList> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn get_views(
        &self,
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

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<PageViews> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }

    pub fn revoke_access_token(&self, access_token: &str) -> Result<Account, Box<dyn Error>> {
        let mut url = Url::parse("https://api.telegra.ph/revokeAccessToken")?;
        url.query_pairs_mut()
            .append_pair("access_token", access_token);

        let resp = self.client.get(url).send()?;
        let api_resp: ApiResponse<Account> = resp.json()?;

        if api_resp.ok {
            Ok(api_resp.result.expect("Missing result"))
        } else {
            Err(api_resp
                .error
                .unwrap_or_else(|| "Unknown error".to_string())
                .into())
        }
    }
}