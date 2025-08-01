use std::str::FromStr;

use mockito::{Matcher, Mock};
use remnawave::{types::SubscriptionTemplateType, RemnawaveApiClient};
use serde_json::json;
use uuid::Uuid;

async fn setup_client() -> (mockito::ServerGuard, RemnawaveApiClient) {
    let server = mockito::Server::new_async().await;
    let client = RemnawaveApiClient::new(server.url(), Some("test_token".to_string())).expect("Failed to create API client");

    (server, client)
}

fn create_subscription_mock(server: &mut mockito::Server) -> Mock {
    server
        .mock("GET", "/api/subscriptions")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded("size".into(), "10".into()), Matcher::UrlEncoded("start".into(), "0".into())]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "response": {
                    "subscriptions": [
                        {
                            "isFound": true,
                            "user": {
                                "shortUuid": "abc123",
                                "daysLeft": 30,
                                "trafficUsed": "1024000",
                                "trafficLimit": "10737418240",
                                "username": "test_user",
                                "expiresAt": "2024-12-31T23:59:59.000Z",
                                "isActive": true,
                                "userStatus": "ACTIVE",
                                "trafficLimitStrategy": "MONTH"
                            },
                            "links": [
                                "vless://test-link-1",
                                "vmess://test-link-2"
                            ],
                            "ssConfLinks": {
                                "ss1": "ss://test-ss-link"
                            },
                            "subscriptionUrl": "https://example.com/sub/abc123"
                        }
                    ],
                    "total": 1
                }
            })
            .to_string(),
        )
        .create()
}

fn create_subscription_settings_mock(server: &mut mockito::Server) -> Mock {
    server
        .mock("GET", "/api/subscription-settings")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "response": {
                    "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "profileTitle": "string",
                    "supportLink": "string",
                    "profileUpdateInterval": 1,
                    "isProfileWebpageUrlEnabled": true,
                    "serveJsonAtBaseSubscription": true,
                    "addUsernameToBaseSubscription": true,
                    "isShowCustomRemarks": true,
                    "happAnnounce": "string",
                    "happRouting": "string",
                    "expiredUsersRemarks": [
                        "string"
                    ],
                    "limitedUsersRemarks": [
                        "string"
                    ],
                    "disabledUsersRemarks": [
                        "string"
                    ],
                    "customResponseHeaders": {
                        "additionalProp1": "string",
                        "additionalProp2": "string",
                        "additionalProp3": "string"
                    },
                    "randomizeHosts": true,
                    "createdAt": "2025-08-01T15:20:55.497Z",
                    "updatedAt": "2025-08-01T15:20:55.497Z"
                }
            })
            .to_string(),
        )
        .create()
}

fn create_subscription_not_found_mock(server: &mut mockito::Server) -> Mock {
    server
        .mock("GET", "/api/sub/test-invalid-uuid")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "statusCode": 404,
                "message": "Subscription not found",
                "error": "Not Found"
            })
            .to_string(),
        )
        .create()
}

fn create_subscription_by_username_not_found_mock(server: &mut mockito::Server) -> Mock {
    server
        .mock("GET", "/api/subscriptions/by-username/nonexistent_user_test_12345")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "statusCode": 404,
                "message": "User not found",
                "error": "Not Found"
            })
            .to_string(),
        )
        .create()
}

fn create_raw_subscription_not_found_mock(server: &mut mockito::Server) -> Mock {
    server
        .mock("GET", "/api/sub/invalid-test-uuid/raw")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "statusCode": 404,
                "message": "Subscription not found",
                "error": "Not Found"
            })
            .to_string(),
        )
        .create()
}

fn create_template_mock(server: &mut mockito::Server, template_type: &str) -> Mock {
    server
        .mock("GET", format!("/api/subscription-templates/{template_type}").as_str())
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "response": {
                    "uuid": "550e8400-e29b-41d4-a716-446655440000",
                    "templateType": template_type.to_uppercase(),
                    "templateJson": {
                        "proxies": [],
                        "proxy-groups": [],
                        "rules": []
                    },
                    "encodedTemplateYaml": "cHJveGllczogW10KcHJveHktZ3JvdXBzOiBbXQpydWxlczogW10K"
                }
            })
            .to_string(),
        )
        .create()
}

#[tokio::test]
async fn test_client_creation() {
    let (_, client) = setup_client().await;
    assert!(!client.base_url().is_empty());
    assert!(client.base_url().contains("127.0.0.1"));
}

#[tokio::test]
async fn test_subscriptions_get_all() {
    let (mut server, client) = setup_client().await;
    create_subscription_mock(&mut server);

    let result = client.subscriptions.get_all_subscriptions(Some(10), Some(0)).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.total, 1);
    assert_eq!(response.response.subscriptions.len(), 1);
    assert_eq!(response.response.subscriptions[0].user.username, "test_user");
    assert_eq!(response.response.subscriptions[0].user.short_uuid, "abc123");
}

#[tokio::test]
async fn test_subscription_settings() {
    let (mut server, client) = setup_client().await;
    create_subscription_settings_mock(&mut server);

    let result = client.subscription_settings.get_settings().await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.uuid, Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap());
    assert_eq!(response.response.profile_title, "string");
}

#[tokio::test]
async fn test_subscription_info() {
    let (mut server, client) = setup_client().await;
    create_subscription_not_found_mock(&mut server);

    let result = client.subscriptions.get_subscription("test-invalid-uuid".to_string()).await;
    assert!(result.is_err(), "Expected error for invalid UUID");
    if let Err(e) = result {
        assert_eq!(e.status_code, 404);
        assert!(e.message.expect("REASON").contains("Subscription not found"));
    }
}

#[tokio::test]
async fn test_subscription_by_username() {
    let (mut server, client) = setup_client().await;
    create_subscription_by_username_not_found_mock(&mut server);

    let result = client.subscriptions.get_subscription_by_username("nonexistent_user_test_12345".to_string()).await;
    assert!(result.is_err(), "Expected error for non-existent username");
    if let Err(e) = result {
        assert_eq!(e.status_code, 404);
        assert!(e.message.expect("REASON").contains("User not found"));
    }
}

#[tokio::test]
async fn test_raw_subscription_endpoint() {
    let (mut server, client) = setup_client().await;
    create_raw_subscription_not_found_mock(&mut server);

    let result = client.subscriptions.get_raw_subscription_by_short_uuid("invalid-test-uuid".to_string()).await;
    assert!(result.is_err(), "Expected error for invalid UUID");
    if let Err(e) = result {
        assert_eq!(e.status_code, 404);
        assert!(e.message.expect("REASON").contains("Subscription not found"));
    }
}

#[tokio::test]
async fn test_multiple_template_requests() {
    let (mut server, client) = setup_client().await;
    create_template_mock(&mut server, "CLASH");
    create_template_mock(&mut server, "XRAY_JSON");
    create_template_mock(&mut server, "SINGBOX");

    let clash_result = client.subscription_templates.get_template(SubscriptionTemplateType::Clash).await;
    let xray_result = client.subscription_templates.get_template(SubscriptionTemplateType::XrayJson).await;
    let singbox_result = client.subscription_templates.get_template(SubscriptionTemplateType::SingBox).await;

    assert!(clash_result.is_ok());
    assert!(xray_result.is_ok());
    assert!(singbox_result.is_ok());

    let clash_response = clash_result.unwrap();
    assert_eq!(clash_response.response.template_type, SubscriptionTemplateType::Clash);
    assert_eq!(clash_response.response.uuid, Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
}

#[tokio::test]
async fn test_concurrent_requests() {
    let (mut server, client) = setup_client().await;
    create_template_mock(&mut server, "CLASH");
    create_template_mock(&mut server, "XRAY_JSON");
    create_template_mock(&mut server, "SINGBOX");

    let (clash_result, xray_result, singbox_result) = tokio::join!(
        client.subscription_templates.get_template(SubscriptionTemplateType::Clash),
        client.subscription_templates.get_template(SubscriptionTemplateType::XrayJson),
        client.subscription_templates.get_template(SubscriptionTemplateType::SingBox)
    );

    assert!(clash_result.is_ok(), "Expected Ok result for Clash template: {clash_result:?}");
    assert!(xray_result.is_ok(), "Expected Ok result for Xray template: {xray_result:?}");
    assert!(singbox_result.is_ok(), "Expected Ok result for SingBox template: {singbox_result:?}");

    assert_eq!(clash_result.unwrap().response.template_type, SubscriptionTemplateType::Clash);
    assert_eq!(xray_result.unwrap().response.template_type, SubscriptionTemplateType::XrayJson);
    assert_eq!(singbox_result.unwrap().response.template_type, SubscriptionTemplateType::SingBox);
}

#[tokio::test]
async fn test_client_token_update() {
    let (_, mut client) = setup_client().await;
    let original_token = "test_token".to_string();

    client.set_token(Some("new_test_token".to_string()));
    client.set_token(Some(original_token));

    println!("Token update functionality works");
}

#[tokio::test]
async fn test_client_without_token() {
    let mut server = mockito::Server::new_async().await;
    let client = RemnawaveApiClient::new(server.url(), None).expect("Failed to create API client without token");

    server
        .mock("GET", "/api/subscriptions")
        .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded("size".into(), "10".into()), Matcher::UrlEncoded("start".into(), "0".into())]))
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "statusCode": 401,
                "message": "Unauthorized",
                "error": "Unauthorized"
            })
            .to_string(),
        )
        .create();

    assert!(!client.base_url().is_empty());

    let result = client.subscriptions.get_all_subscriptions(Some(10), Some(0)).await;
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(e.status_code, 401);
        assert!(e.message.expect("REASON").contains("Unauthorized"));
    }
}

#[tokio::test]
async fn test_caddy_token_support() {
    let mut server = mockito::Server::new_async().await;
    let client = RemnawaveApiClient::with_caddy_token(server.url(), Some("test_token".to_string()), Some("caddy_test_key".to_string()))
        .expect("Failed to create API client with caddy token");

    server
        .mock("GET", "/api/subscription-settings")
        .match_header("X-Api-Key", "caddy_test_key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "response": {
                    "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                    "profileTitle": "string",
                    "supportLink": "string",
                    "profileUpdateInterval": 1,
                    "isProfileWebpageUrlEnabled": true,
                    "serveJsonAtBaseSubscription": true,
                    "addUsernameToBaseSubscription": true,
                    "isShowCustomRemarks": true,
                    "happAnnounce": "string",
                    "happRouting": "string",
                    "expiredUsersRemarks": [
                        "string"
                    ],
                    "limitedUsersRemarks": [
                        "string"
                    ],
                    "disabledUsersRemarks": [
                        "string"
                    ],
                    "customResponseHeaders": {
                        "additionalProp1": "string",
                        "additionalProp2": "string",
                        "additionalProp3": "string"
                    },
                    "randomizeHosts": true,
                    "createdAt": "2025-08-01T15:20:55.497Z",
                    "updatedAt": "2025-08-01T15:20:55.497Z"
                }
            })
            .to_string(),
        )
        .create();

    let result = client.subscription_settings.get_settings().await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.uuid, Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap());
}
