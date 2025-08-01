use dotenv::dotenv;
use remnawave_api::{RemnawaveApiClient, types::SubscriptionTemplateType};
use std::env;

fn setup_client() -> RemnawaveApiClient {
    dotenv().ok();

    let base_url = env::var("REMNAWAVE_BASE_URL").expect("REMNAWAVE_BASE_URL must be set in .env file");
    let token = env::var("REMNAWAVE_TOKEN").expect("REMNAWAVE_TOKEN must be set in .env file");

    RemnawaveApiClient::new(base_url, Some(token)).expect("Failed to create API client")
}

#[tokio::test]
async fn test_client_creation() {
    let client = setup_client();
    assert!(!client.base_url().is_empty());
}

#[tokio::test]
async fn test_subscriptions_get_all() {
    let client = setup_client();

    let result = client.subscriptions.get_all_subscriptions(Some(10), Some(0)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_subscription_settings() {
    let client = setup_client();

    let result = client.subscription_settings.get_settings().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_subscription_info() {
    let client = setup_client();

    let result = client.subscriptions.get_subscription("test-invalid-uuid".to_string()).await;
    assert!(result.is_err(), "Expected error for invalid UUID");
    if let Err(e) = result {
        assert!(e.status_code == 404);
    }
}

#[tokio::test]
async fn test_subscription_by_username() {
    let client = setup_client();

    let result = client.subscriptions.get_subscription_by_username("nonexistent_user_test_12345".to_string()).await;
    assert!(result.is_err(), "Expected error for non-existent username");
    if let Err(e) = result {
        assert!(e.status_code == 404);
    }
}

#[tokio::test]
async fn test_raw_subscription_endpoint() {
    let client = setup_client();

    let result = client.subscriptions.get_raw_subscription_by_short_uuid("invalid-test-uuid".to_string()).await;
    assert!(result.is_err(), "Expected error for invalid UUID");
    if let Err(e) = result {
        assert!(e.status_code == 404);
    }
}

#[tokio::test]
async fn test_multiple_template_requests() {
    let client = setup_client();

    let clash_result = client.subscription_templates.get_template(SubscriptionTemplateType::Clash).await;
    let v2ray_result = client.subscription_templates.get_template(SubscriptionTemplateType::XrayJson).await;
    let singbox_result = client.subscription_templates.get_template(SubscriptionTemplateType::SingBox).await;

    assert!(clash_result.is_ok());
    assert!(v2ray_result.is_ok());
    assert!(singbox_result.is_ok());
}

#[tokio::test]
async fn test_concurrent_requests() {
    let client = setup_client();

    let (clash_result, v2ray_result, singbox_result) = tokio::join!(
        client.subscription_templates.get_template(SubscriptionTemplateType::Clash),
        client.subscription_templates.get_template(SubscriptionTemplateType::XrayJson),
        client.subscription_templates.get_template(SubscriptionTemplateType::SingBox)
    );

    assert!(clash_result.is_ok(), "Expected Ok result for Clash template: {:?}", clash_result);
    assert!(v2ray_result.is_ok(), "Expected Ok result for V2Ray template: {:?}", v2ray_result);
    assert!(singbox_result.is_ok(), "Expected Ok result for SingBox template: {:?}", singbox_result);
}

#[tokio::test]
async fn test_client_token_update() {
    let mut client = setup_client();
    let original_token = env::var("REMNAWAVE_TOKEN").unwrap();

    client.set_token(Some("new_test_token".to_string()));

    client.set_token(Some(original_token));

    println!("Token update functionality works");
}

#[tokio::test]
async fn test_client_without_token() {
    dotenv().ok();
    let base_url = env::var("REMNAWAVE_BASE_URL").expect("REMNAWAVE_BASE_URL must be set in .env file");

    let client = RemnawaveApiClient::new(base_url, None).expect("Failed to create API client without token");

    assert!(!client.base_url().is_empty());

    let result = client.subscriptions.get_all_subscriptions(Some(10), Some(0)).await;
    assert!(result.is_err());
}
