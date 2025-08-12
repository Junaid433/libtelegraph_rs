# Telegraph.rs

A Rust wrapper for the [Telegraph API](https://telegra.ph/api). This library provides a simple and easy-to-use interface for interacting with Telegraph.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
telegraph_rs = "0.1.0"
```

## Usage

### Creating a Client

To get started, create a new `TelegraphClient`:

```rust
use telegraph_rs::TelegraphClient;

let client = TelegraphClient::new();
```

### Creating an Account

Here's an example of how to create a new Telegraph account:

```rust
use telegraph_rs::TelegraphClient;

let client = TelegraphClient::new();
let account = client.create_account(
    "my-short-name",
    Some("My Author Name"),
    Some("https://example.com"),
);

match account {
    Ok(acc) => println!("Account created: {:?}", acc),
    Err(err) => eprintln!("Error creating account: {}", err),
}
```

## API Reference

The `TelegraphClient` provides the following methods:

*   `create_account(short_name: &str, author_name: Option<&str>, author_url: Option<&str>) -> Result<Account, Box<dyn Error>>`
*   `create_page(access_token: &str, title: &str, author_name: Option<&str>, author_url: Option<&str>, content: &[Node], return_content: Option<bool>) -> Result<Page, Box<dyn Error>>`
*   `edit_account_info(access_token: &str, short_name: Option<&str>, author_name: Option<&str>, author_url: Option<&str>) -> Result<Account, Box<dyn Error>>`
*   `edit_page(access_token: &str, path: &str, title: &str, author_name: Option<&str>, author_url: Option<&str>, content: &[Node], return_content: Option<bool>) -> Result<Page, Box<dyn Error>>`
*   `get_account_info(access_token: &str, fields: Option<&[&str]>) -> Result<Account, Box<dyn Error>>`
*   `get_page(path: &str, return_content: Option<bool>) -> Result<Page, Box<dyn Error>>`
*   `get_page_list(access_token: &str, offset: Option<u32>, limit: Option<u32>) -> Result<PageList, Box<dyn Error>>`
*   `get_views(path: &str, year: Option<u32>, month: Option<u8>, day: Option<u8>, hour: Option<u8>) -> Result<PageViews, Box<dyn Error>>`
*   `revoke_access_token(access_token: &str) -> Result<Account, Box<dyn Error>>`

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License. Please see the `LICENSE` file for details.
