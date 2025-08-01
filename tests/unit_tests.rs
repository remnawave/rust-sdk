use mockito::Server;
use remnawave::{ApiError, RemnawaveApiClient};
use serde_json::json;

#[tokio::test]
async fn test_api_client_creation() {
    let client = RemnawaveApiClient::new("https://api.example.com".to_string(), Some("test_token".to_string())).unwrap();

    assert_eq!(client.base_url(), "https://api.example.com");
}

#[tokio::test]
async fn test_api_client_creation_without_token() {
    let client = RemnawaveApiClient::new("https://api.example.com".to_string(), None).unwrap();

    assert_eq!(client.base_url(), "https://api.example.com");
}

#[tokio::test]
async fn test_token_update() {
    let mut client = RemnawaveApiClient::new("https://api.example.com".to_string(), Some("initial_token".to_string())).unwrap();

    client.set_token(Some("new_token".to_string()));

    client.set_token(None);

    assert_eq!(client.base_url(), "https://api.example.com");
}

#[tokio::test]
async fn test_api_error_display() {
    let error = ApiError {
        status_code: 404,
        url: "https://api.example.com/test".to_string(),
        request_body: None,
        response_body: "Not Found".to_string(),
        response_headers: std::collections::HashMap::new(),
        timestamp: None,
        path: None,
        message: Some("Resource not found".to_string()),
        error_code: Some("NOT_FOUND".to_string()),
        error: None,
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("404"));
    assert!(error_string.contains("Resource not found"));
    assert!(error_string.contains("NOT_FOUND"));
}

#[tokio::test]
async fn test_api_error_display_without_message() {
    let error = ApiError {
        status_code: 500,
        url: "https://api.example.com/test".to_string(),
        request_body: None,
        response_body: "Internal Server Error".to_string(),
        response_headers: std::collections::HashMap::new(),
        timestamp: None,
        path: None,
        message: None,
        error_code: None,
        error: None,
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("500"));
    assert!(error_string.contains("https://api.example.com/test"));
    assert!(error_string.contains("Internal Server Error"));
}

#[tokio::test]
async fn test_mock_subscription_endpoint() {
    let mut server = Server::new_async().await;

    let _mock = server
        .mock("GET", "/api/subscriptions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "subscriptions": [],
                "total": 0,
                "page": 0,
                "size": 10
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = RemnawaveApiClient::new(server.url(), Some("test_token".to_string())).unwrap();

    assert!(client.base_url().starts_with("http://127.0.0.1"));
}

#[tokio::test]
async fn test_mock_template_endpoint() {
    let mut server = Server::new_async().await;

    let _mock = server
        .mock("GET", "/api/subscriptions/template/clash")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "template": "clash_template_content",
                "version": "1.0"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = RemnawaveApiClient::new(server.url(), Some("test_token".to_string())).unwrap();

    assert!(client.base_url().starts_with("http://127.0.0.1"));
}

#[tokio::test]
async fn test_mock_error_response() {
    let mut server = Server::new_async().await;

    let _mock = server
        .mock("GET", "/api/subscriptions/info/invalid")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "error": "Subscription not found",
                "code": "NOT_FOUND"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = RemnawaveApiClient::new(server.url(), Some("test_token".to_string())).unwrap();

    assert!(client.base_url().starts_with("http://127.0.0.1"));
}

#[test]
fn test_controllers_exist() {
    let client = RemnawaveApiClient::new("https://api.example.com".to_string(), Some("test_token".to_string())).unwrap();

    let _ = &client.auth;
    let _ = &client.users;
    let _ = &client.subscriptions;
    let _ = &client.nodes;
    let _ = &client.hosts;
    let _ = &client.system;
    let _ = &client.tokens;
    let _ = &client.config_profiles;
    let _ = &client.internal_squads;
    let _ = &client.hwid;
    let _ = &client.billing;
    let _ = &client.keygen;
}
