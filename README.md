# Remnawave Rust SDK

![Crates.io Version](https://img.shields.io/crates/v/remnawave) ![GitHub Tag](https://img.shields.io/github/v/tag/remnawave/rust-sdk) ![GitHub Repo stars](https://img.shields.io/github/stars/remnawave/rust-sdk)

![Build Status](https://img.shields.io/github/actions/workflow/status/remnawave/rust-sdk/.github/workflows/publish.yml) ![Crates.io Last Update](https://img.shields.io/crates/last-update/remnawave)
![Downloads](https://img.shields.io/crates/d/remnawave) ![License](https://img.shields.io/crates/l/remnawave)

![Known Vulnerabilities](https://snyk.io/test/github/remnawave/rust-sdk/badge.svg) ![Coverage Status](https://img.shields.io/codecov/c/github/remnawave/rust-sdk)

A ~~🚀 blazingly fast~~ high-performance Rust SDK for interacting with the **[Remnawave API](https://remna.st)**. This library provides a type-safe, async interface for managing and monitoring your Remnawave server, including user management, subscription handling, node monitoring, and comprehensive statistics.

## Features

- **Type-safe Rust API** - Leveraging Rust's type system for compile-time guarantees
- **Async/Await Support** - Built on `tokio` and `reqwest` for high-performance async operations
- **Controller-based Architecture** - Organized API endpoints into logical controllers
- **Comprehensive Management** - Users, subscriptions, nodes, hosts, billing, and more
- **Error Handling** - Detailed error types with context and debugging information
- **Rich Type Definitions** - Complete request/response DTOs with serde support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
remnawave = "2.1.11" # { version = "2.1.11", features = ["native-tls"] }
tokio = { version = "1.0", features = ["full"] }
```

Or install via cargo:

```bash
cargo add remnawave
cargo add tokio --features full
```

## Quick Start

```rust
use remnawave::RemnawaveApiClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the API client
    let client = RemnawaveApiClient::new(
        "https://your-panel.com".to_string(),
        Some("your-bearer-token".to_string())
    )?;

    // Get all users
    let users_response = client.users.get_all_users(Some(10), Some(0)).await?;
    println!("Total users: {}", users_response.total);

    // Get system statistics
    let stats = client.system.get_stats().await?;
    println!("System stats: {:?}", stats);

    Ok(())
}
```

## Core Features

### User Management

```rust
use remnawave::types::{CreateUserRequestDto, UpdateUserRequestDto};
use uuid::Uuid;

// Create a new user
let new_user = CreateUserRequestDto {
    username: "john_doe".to_string(),
    email: Some("john@example.com".to_string()),
    // ... other fields
};
let user = client.users.create_user(new_user).await?;

// Get user by UUID
let user_uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
let user = client.users.get_user_by_uuid(user_uuid).await?;

// Update user
let update_data = UpdateUserRequestDto {
    // ... update fields
};
let updated_user = client.users.update_user(update_data).await?;

// Bulk operations
let bulk_delete = client.users.bulk_delete_users(delete_request).await?;
let bulk_update = client.users.bulk_update_users(update_request).await?;
```

### Subscription Management

```rust
// Get all subscriptions
let subscriptions = client.subscriptions.get_all_subscriptions(Some(20), Some(0)).await?;

// Get subscription by username
let subscription = client.subscriptions.get_subscription_by_username("john_doe").await?;

// Get subscription settings
let settings = client.subscription_settings.get_subscription_settings().await?;

// Update subscription settings
let new_settings = UpdateSubscriptionSettingsRequestDto {
    // ... settings
};
client.subscription_settings.update_subscription_settings(new_settings).await?;
```

### Node and Host Management

```rust
// Get all nodes
let nodes = client.nodes.get_all_nodes().await?;

// Get node statistics
let node_stats = client.nodes.get_nodes_statistics().await?;

// Get real-time usage
let realtime_usage = client.nodes.get_nodes_realtime_usage().await?;

// Restart a node
let node_uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
client.nodes.restart_node(node_uuid).await?;

// Host management
let hosts = client.hosts.get_all_hosts().await?;
let host = client.hosts.get_one_host(host_uuid).await?;
```

### Authentication

```rust
use remnawave::types::{LoginRequestDto, RegisterRequestDto};

// Login
let login_request = LoginRequestDto {
    username: "admin".to_string(),
    password: "secure_password".to_string(),
};
let login_response = client.auth.login(login_request).await?;

// Register (if enabled)
let register_request = RegisterRequestDto {
    username: "new_user".to_string(),
    password: "secure_password".to_string(),
    email: "user@example.com".to_string(),
};
let register_response = client.auth.register(register_request).await?;
```

## API Reference

### RemnawaveApiClient

Main client struct for interacting with the Remnawave API.

```rust
impl RemnawaveApiClient {
    pub fn new(base_url: String, token: Option<String>) -> Result<Self>
    pub fn set_token(&mut self, token: Option<String>)
    pub fn base_url(&self) -> &str
}
```

### Available Controllers

| Controller                   | Description                      |
|------------------------------|----------------------------------|
| `auth`                       | Authentication operations        |
| `users`                      | User management                  |
| `subscriptions`              | Subscription handling            |
| `subscription_templates`     | Subscription template management |
| `subscription_settings`      | Subscription settings            |
| `nodes`                      | Node management                  |
| `nodes_usage`                | Node usage statistics            |
| `hosts`                      | Host management                  |
| `system`                     | System operations                |
| `tokens`                     | API token management             |
| `config_profiles`            | Configuration profiles           |
| `internal_squads`            | Internal squad management        |
| `hwid`                       | Hardware ID devices              |
| `billing`                    | Infrastructure billing           |
| `keygen`                     | Key generation                   |

### Bulk Operations

The SDK supports efficient bulk operations for user management:

```rust
// Bulk delete users by status
let bulk_delete_request = BulkDeleteUsersByStatusRequestDto {
    statuses: vec![UserStatus::Disabled],
};
client.users.bulk_delete_users_by_status(bulk_delete_request).await?;

// Bulk update users
let bulk_update_request = BulkUpdateUsersRequestDto {
    user_uuids: vec![user_uuid1, user_uuid2],
    update_data: UpdateUserRequestDto {
        // ... update fields
    },
};
client.users.bulk_update_users(bulk_update_request).await?;

// Bulk reset traffic
let reset_request = BulkResetTrafficUsersRequestDto {
    user_uuids: vec![user_uuid1, user_uuid2],
};
client.users.bulk_reset_user_traffic(reset_request).await?;
```

### Usage Statistics

```rust
// Get node usage by date range
let usage = client.nodes_usage.get_nodes_usage_by_range(
    start_date,
    end_date,
    Some(node_uuid)
).await?;

// Get user usage by range
let user_usage = client.nodes_usage.get_user_usage_by_range(
    user_uuid,
    start_date,
    end_date
).await?;

// Get node user usage
let node_user_usage = client.nodes_usage.get_node_user_usage_by_range(
    node_uuid,
    start_date,
    end_date
).await?;
```

## Error Handling

The SDK provides comprehensive error handling through the `ApiError` type:

```rust
use remnawave::ApiError;

match client.users.get_user_by_uuid(user_uuid).await {
    Ok(user) => println!("User: {:?}", user),
    Err(e) => {
        eprintln!("API Error [{}]: {}", e.status_code, e);
        eprintln!("Response: {}", e.response_body);
    }
} 
```

## Configuration

### Client Configuration

```rust
// With token
let client = RemnawaveApiClient::new(
    "https://your-panel.com".to_string(),
    Some("your-token".to_string())
)?;

// Without token
let client = RemnawaveApiClient::new(
    "https://your-panel.com".to_string(),
    None
)?;

// With token and Caddy API key
let client = RemnawaveApiClient::with_caddy_token(
    "https://your-panel.com".to_string(),
    Some("your-token".to_string()),
    Some("your-caddy-api-key".to_string())
)?;

// Update token later
client.set_token(Some("new-token".to_string()));

// Update Caddy token later
client.set_caddy_token(Some("new-caddy-api-key".to_string()));
```

## Examples

Check out the `tests/` directory for comprehensive examples:

- `integration_tests.rs` - Real API integration examples
- `unit_tests.rs` - Unit test examples and mocking

## Compatible Versions

| SDK Version   | Remnawave Panel Version |
|---------------|-------------------------|
| 2.1.11        | >=2.1.8                 |
| 2.1.8         | >=2.1.4                 |
| Not supported | >=2.1.0,<2.1.4          |
| 2.0.0         | >=2.0.0,<2.1.0          |
| Not supported | <2.0.0                  |

# Contributors

We ❤️‍🔥 contributors! If you'd like to contribute, feel free to submit a pull request or open an issue.

Check [open issues](https://github.com/remnawave/rust-sdk/issues) to help the progress of this project.

<p align="center">
Thanks to all contributors who have helped improve the Remnawave Rust SDK:
</p>
<p align="center">
<a href="https://github.com/remnawave/rust-sdk/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=remnawave/rust-sdk" />
</a>
</p>

## License

MIT License
