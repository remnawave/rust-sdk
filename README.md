# Remnawave API Client

A Rust client library for the Remnawave API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
remnawave = "1.6.15"
```

## Usage

```rust
use remnawave::{RemnawaveApiClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RemnawaveApiClient::new(
        "https://api.example.com".to_string(),
        "your_token_here".to_string()
    )?;

    let status = client.auth.get_status().await?;
    println!("Auth status: {:?}", status);

    Ok(())
}
```

## Features

This library provides access to all Remnawave API endpoints through organized controllers:

- **Auth Controller** - Authentication and status
- **Users Controller** - User management operations
- **Subscription Controller** - Subscription information
- **API Tokens Controller** - API token management
- **Nodes Controller** - Node management
- **Hosts Controller** - Host management
- **Inbounds Controller** - Inbound configuration
- **System Controller** - System statistics and health

## Examples

See the `examples/` directory for more usage examples.

## Error Handling

The library provides detailed error information through the `ApiError` type, including:

- HTTP status codes
- Response body
- Request details
- Parsed error messages from the API

## License

This project is licensed under the MIT License.
