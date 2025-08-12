# Telegraph.rs

[![Crates.io](https://img.shields.io/crates/v/telegraph_rs.svg)](https://crates.io/crates/telegraph_rs)
[![Docs.rs](https://docs.rs/telegraph_rs/badge.svg)](https://docs.rs/telegraph_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust wrapper for the [Telegraph API](https://telegra.ph/api). This library provides a simple and easy-to-use interface for interacting with Telegraph.

## Features

*   A simple and intuitive API.
*   Supports all Telegraph API methods.
*   Asynchronous requests using `reqwest`.
*   Strongly typed models for API responses.

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

## API Reference

### `create_account`

Creates a new Telegraph account.

**Parameters:**

*   `short_name` (required): Account name, displayed on the user's profile page.
*   `author_name` (optional): Default author name used when creating new articles.
*   `author_url` (optional): Default author URL used when creating new articles.

**Example:**

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

### `edit_account_info`

Updates the information of an existing Telegraph account.

**Parameters:**

*   `access_token` (required): Access token of the account to edit.
*   `short_name` (optional): New account name.
*   `author_name` (optional): New default author name.
*   `author_url` (optional): New default author URL.

### `get_account_info`

Gets information about a Telegraph account.

**Parameters:**

*   `access_token` (required): Access token of the account.
*   `fields` (optional): A list of fields to return. Available fields: `short_name`, `author_name`, `author_url`, `auth_url`, `page_count`.

### `revoke_access_token`

Revokes the access token of a Telegraph account, logging out the user.

**Parameters:**

*   `access_token` (required): Access token to revoke.

### `create_page`

Creates a new Telegraph page.

**Parameters:**

*   `access_token` (required): Access token of the account.
*   `title` (required): Page title.
*   `author_name` (optional): Author name, displayed on the page.
*   `author_url` (optional): URL to be opened when the author name is clicked.
*   `content` (required): Content of the page, as a JSON-serialized array of Node objects.
*   `return_content` (optional): If `true`, the created page will be returned in the response.

### `edit_page`

Edits an existing Telegraph page.

**Parameters:**

*   `access_token` (required): Access token of the account.
*   `path` (required): Path to the page to edit.
*   `title` (required): New page title.
*   `author_name` (optional): New author name.
*   `author_url` (optional): New author URL.
*   `content` (required): New content of the page.
*   `return_content` (optional): If `true`, the edited page will be returned in the response.

### `get_page`

Gets a Telegraph page.

**Parameters:**

*   `path` (required): Path to the page.
*   `return_content` (optional): If `true`, the page content will be returned in the response.

### `get_page_list`

Gets a list of pages belonging to a Telegraph account.

**Parameters:**

*   `access_token` (required): Access token of the account.
*   `offset` (optional): Sequential number of the first page to be returned.
*   `limit` (optional): Number of pages to be returned.

### `get_views`

Gets the number of views for a Telegraph page.

**Parameters:**

*   `path` (required): Path to the page.
*   `year` (optional): Year to get views for.
*   `month` (optional): Month to get views for.
*   `day` (optional): Day to get views for.
*   `hour` (optional): Hour to get views for.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.