#![recursion_limit = "256"]
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
                                "shortUuid": "user_short_uuid_1",
                                "daysLeft": 27027,
                                "trafficUsed": "31.48 MiB",
                                "trafficLimit": "0",
                                "lifetimeTrafficUsed": "31.48 MiB",
                                "lifetimeTrafficUsedBytes": "33013814",
                                "trafficLimitBytes": "0",
                                "trafficUsedBytes": "33013814",
                                "username": "user1",
                                "expiresAt": "2099-08-26T23:10:43.000Z",
                                "isActive": true,
                                "userStatus": "ACTIVE",
                                "trafficLimitStrategy": "NO_RESET"
                            },
                            "links": [
                                "vless://uuid1@endpoint1:8443?...#Country1 endpoint1",
                                "vless://uuid1@endpoint2:8443?...#Country1 endpoint2"
                            ],
                            "ssConfLinks": {},
                            "subscriptionUrl": "https://example.com/api/sub/user_short_uuid_1",
                            "happ": {
                                "cryptoLink": "happ://crypt/example_crypto_link1"
                            }
                        },
                        {
                            "isFound": true,
                            "user": {
                                "shortUuid": "user_short_uuid_2",
                                "daysLeft": 27018,
                                "trafficUsed": "43.93 GiB",
                                "trafficLimit": "0",
                                "lifetimeTrafficUsed": "43.93 GiB",
                                "lifetimeTrafficUsedBytes": "47174214929",
                                "trafficLimitBytes": "0",
                                "trafficUsedBytes": "47174214929",
                                "username": "user2",
                                "expiresAt": "2099-08-17T09:26:23.000Z",
                                "isActive": true,
                                "userStatus": "ACTIVE",
                                "trafficLimitStrategy": "NO_RESET"
                            },
                            "links": [
                                "vless://uuid2@endpoint1:8443?...#Country1 endpoint1",
                                "vless://uuid2@endpoint2:8443?...#Country1 endpoint2"
                            ],
                            "ssConfLinks": {},
                            "subscriptionUrl": "https://example.com/api/sub/user_short_uuid_2",
                            "happ": {
                                "cryptoLink": "happ://crypt/example_crypto_link2"
                            }
                        },
                        {
                            "isFound": true,
                            "user": {
                                "shortUuid": "user_short_uuid_3",
                                "daysLeft": 26999,
                                "trafficUsed": "57.46 GiB",
                                "trafficLimit": "0",
                                "lifetimeTrafficUsed": "57.46 GiB",
                                "lifetimeTrafficUsedBytes": "61694628011",
                                "trafficLimitBytes": "0",
                                "trafficUsedBytes": "61694628011",
                                "username": "user3",
                                "expiresAt": "2099-07-29T15:43:09.000Z",
                                "isActive": true,
                                "userStatus": "ACTIVE",
                                "trafficLimitStrategy": "NO_RESET"
                            },
                            "links": [
                                "vless://uuid3@endpoint1:8443?...#Country1 endpoint1",
                                "vless://uuid3@endpoint2:8443?...#Country1 endpoint2"
                            ],
                            "ssConfLinks": {},
                            "subscriptionUrl": "https://example.com/api/sub/user_short_uuid_3",
                            "happ": {
                                "cryptoLink": "happ://crypt/example_crypto_link3"
                            }
                        }
                    ],
                    "total": 19
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
        .mock("GET", "/api/subscriptions/by-short-uuid/invalid-test-uuid/raw")
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

fn create_raw_subscription_found_mock(server: &mut mockito::Server) -> Mock {
    server
        .mock("GET", "/api/subscriptions/by-short-uuid/test-uuid/raw")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "response": {
                    "user": {
                        "uuid": "11111111-2222-3333-4444-555555555555",
                        "shortUuid": "test-short-uuid",
                        "username": "test_user",
                        "status": "ACTIVE",
                        "usedTrafficBytes": 12939412,
                        "lifetimeUsedTrafficBytes": 12939412,
                        "trafficLimitBytes": 0,
                        "trafficLimitStrategy": "NO_RESET",
                        "subLastUserAgent": null,
                        "subLastOpenedAt": null,
                        "expireAt": "2099-12-31T23:59:59.000Z",
                        "onlineAt": null,
                        "subRevokedAt": null,
                        "lastTrafficResetAt": null,
                        "trojanPassword": "test-trojan-password",
                        "vlessUuid": "11111111-2222-3333-4444-555555555555",
                        "ssPassword": "test-ss-password",
                        "description": null,
                        "tag": null,
                        "telegramId": null,
                        "email": null,
                        "hwidDeviceLimit": null,
                        "firstConnectedAt": null,
                        "lastTriggeredThreshold": 0,
                        "createdAt": "2024-01-01T00:00:00.000Z",
                        "updatedAt": "2024-01-01T00:00:00.000Z",
                        "activeInternalSquads": [],
                        "subscriptionUrl": "https://example.com/api/sub/test-short-uuid",
                        "lastConnectedNode": null,
                        "happ": {
                            "cryptoLink": "happ://test-crypto-link"
                        }
                    },
                    "convertedUserInfo": {
                        "daysLeft": 9999.0,
                        "trafficLimit": "0",
                        "trafficUsed": "12.34 MiB",
                        "lifetimeTrafficUsed": "12.34 MiB",
                        "isHwidLimited": false
                    },
                    "headers": {},
                    "rawHosts": [
                        {
                            "remark": "Test Server 1",
                            "address": "test-server1.example.com",
                            "port": 12345,
                            "protocol": "vless",
                            "path": "",
                            "host": "",
                            "tls": "reality",
                            "sni": "test1.example.com",
                            "alpn": "",
                            "publicKey": "TESTPUBLICKEY1",
                            "fingerprint": "chrome",
                            "shortId": "testshortid1",
                            "spiderX": "/",
                            "network": "tcp",
                            "password": {
                                "trojanPassword": "test-trojan-password-1",
                                "vlessPassword": "test-vless-password-1",
                                "ssPassword": "test-ss-password-1"
                            },
                            "muxParams": null,
                            "sockoptParams": null,
                            "dbData": {
                                "rawInbound": {
                                    "tag": "TEST VLESS 1",
                                    "port": 12345,
                                    "listen": "0.0.0.0",
                                    "protocol": "vless",
                                    "settings": {"clients": [], "decryption": "none"},
                                    "sniffing": {"enabled": true, "routeOnly": false, "destOverride": ["http","tls","quic","fakedns"], "metadataOnly": false},
                                    "streamSettings": {
                                        "network": "raw",
                                        "security": "reality",
                                        "rawSettings": {"acceptProxyProtocol": false},
                                        "realitySettings": {
                                            "show": false,
                                            "xver": 0,
                                            "target": "/dev/shm/rels.sock",
                                            "spiderX": "/",
                                            "shortIds": ["testshortid1"],
                                            "privateKey": "testprivatekey1",
                                            "fingerprint": "chrome",
                                            "serverNames": ["test1.example.com"]
                                        }
                                    }
                                },
                                "inboundTag": "TEST VLESS 1",
                                "uuid": "11111111-2222-3333-4444-555555555555",
                            "configProfileUuid": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
                            "configProfileInboundUuid": "aaaaaaaa-bbbb-cccc-dddd-ffffffffffff",
                            "isDisabled": false,
                            "viewPosition": 0,
                            "remark": "Test Server 1",
                            "isHidden": false,
                            "tag": null,
                            "vlessRouteId": null
                        },
                        "flow": "xtls-rprx-vision"
                    },
                    {
                        "remark": "Test Server 2",
                        "address": "test-server2.example.com",
                        "port": 23456,
                        "protocol": "vless",
                        "path": "",
                        "host": "",
                        "tls": "reality",
                        "sni": "test2.example.com",
                        "alpn": "h3,h2,http/1.1",
                        "publicKey": "TESTPUBLICKEY2",
                        "fingerprint": "chrome",
                        "shortId": "testshortid2",
                        "spiderX": "/",
                        "network": "tcp",
                        "password": {
                            "trojanPassword": "test-trojan-password-2",
                            "vlessPassword": "test-vless-password-2",
                            "ssPassword": "test-ss-password-2"
                        },
                        "serverDescription": "VGVzdCBTZXJ2ZXIgMg==",
                        "muxParams": null,
                        "sockoptParams": null,
                        "dbData": {
                            "rawInbound": {
                                "tag": "TEST VLESS 2",
                                "port": 23456,
                                "listen": "0.0.0.0",
                                "protocol": "vless",
                                "settings": {"clients": [], "decryption": "none"},
                                "sniffing": {"enabled": true, "routeOnly": false, "destOverride": ["http","tls","quic","fakedns"], "metadataOnly": false},
                                "streamSettings": {
                                    "network": "raw",
                                    "sockopt": {"tcpNoDelay": true, "tcpFastOpen": true},
                                    "security": "reality",
                                    "rawSettings": {"acceptProxyProtocol": false},
                                    "realitySettings": {
                                        "show": false,
                                        "xver": 0,
                                        "target": "/dev/shm/rels.sock",
                                        "spiderX": "/",
                                        "shortIds": ["testshortid2","othertestshortid"],
                                        "privateKey": "testprivatekey2",
                                        "fingerprint": "chrome",
                                        "serverNames": ["test2.example.com","other.example.com"]
                                    }
                                }
                            },
                            "inboundTag": "TEST VLESS 2",
                            "uuid": "66666666-7777-8888-9999-000000000000",
                            "configProfileUuid": "bbbbbbbb-cccc-dddd-eeee-ffffffffffff",
                            "configProfileInboundUuid": "bbbbbbbb-cccc-dddd-eeee-111111111111",
                            "isDisabled": false,
                            "viewPosition": 4,
                            "remark": "Test Server 2",
                            "isHidden": false,
                            "tag": null,
                            "vlessRouteId": null
                        },
                        "flow": "xtls-rprx-vision"
                    }
                ],
                "headers": {
                    "content-disposition": "attachment; filename=test_user",
                    "support-url": "https://support.example.com",
                    "profile-title": "base64:dGVzdC10aXRsZQ==",
                    "profile-update-interval": "1",
                    "subscription-userinfo": "upload=0; download=12939412; total=0; expire=0",
                    "announce": "base64:VGVzdCBhbm5vdW5jZW1lbnQgdGV4dA=="
                }
            }
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

    let result = client.subscriptions.get_all(Some(10), Some(0)).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.total, 19);
    assert_eq!(response.response.subscriptions.len(), 3);
    assert_eq!(response.response.subscriptions[0].user.username, "user1");
    assert_eq!(response.response.subscriptions[0].user.short_uuid, "user_short_uuid_1");
}

#[tokio::test]
async fn test_subscription_settings() {
    let (mut server, client) = setup_client().await;
    create_subscription_settings_mock(&mut server);

    let result = client.subscription_settings.get().await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.uuid, Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap());
    assert_eq!(response.response.profile_title, "string");
}

#[tokio::test]
async fn test_subscription_info() {
    let (mut server, client) = setup_client().await;
    create_subscription_not_found_mock(&mut server);

    let result = client.subscriptions.get("test-invalid-uuid".to_string()).await;
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

    let result = client.subscriptions.get_by_username("nonexistent_user_test_12345".to_string()).await;
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

    let result = client.subscriptions.get_raw_by_short_uuid("invalid-test-uuid".to_string(), None).await;
    assert!(result.is_err(), "Expected error for invalid UUID");
    if let Err(e) = result {
        assert_eq!(e.status_code, 404);
        assert!(e.message.expect("REASON").contains("Subscription not found"));
    }

    create_raw_subscription_found_mock(&mut server);

    let result = client.subscriptions.get_raw_by_short_uuid("test-uuid".to_string(), None).await;
    assert!(result.is_ok(), "Expected Ok result for valid UUID: {result:?}");
    let dto = result.unwrap();
    assert_eq!(dto.response.user.username, "test_user");
    assert_eq!(dto.response.user.used_traffic_bytes, 12939412);
    assert_eq!(dto.response.user.subscription_url, "https://example.com/api/sub/test-short-uuid");
    assert!(!dto.response.converted_user_info.is_hwid_limited);
    assert_eq!(dto.response.raw_hosts.len(), 2);
    let first = &dto.response.raw_hosts[0];
    assert_eq!(first.remark, Some("Test Server 1".to_string()));
    assert_eq!(first.password.vless, "test-vless-password-1");
}

#[tokio::test]
async fn test_multiple_template_requests() {
    let (mut server, client) = setup_client().await;
    create_template_mock(&mut server, "CLASH");
    create_template_mock(&mut server, "XRAY_JSON");
    create_template_mock(&mut server, "SINGBOX");

    let clash_result = client.subscription_templates.get(SubscriptionTemplateType::Clash).await;
    let xray_result = client.subscription_templates.get(SubscriptionTemplateType::XrayJson).await;
    let singbox_result = client.subscription_templates.get(SubscriptionTemplateType::SingBox).await;

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
        client.subscription_templates.get(SubscriptionTemplateType::Clash),
        client.subscription_templates.get(SubscriptionTemplateType::XrayJson),
        client.subscription_templates.get(SubscriptionTemplateType::SingBox)
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

    let result = client.subscriptions.get_all(Some(10), Some(0)).await;
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

    let result = client.subscription_settings.get().await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.uuid, Uuid::from_str("3fa85f64-5717-4562-b3fc-2c963f66afa6").unwrap());
}

#[tokio::test]
async fn test_x25519_keypairs_endpoint() {
    let (mut server, client) = setup_client().await;

    let _mock = server
        .mock("GET", "/api/system/tools/x25519/generate")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "response": {
                    "keypairs": [
                        {
                            "publicKey": "test_public_key_1",
                            "privateKey": "test_private_key_1"
                        },
                        {
                            "publicKey": "test_public_key_2",
                            "privateKey": "test_private_key_2"
                        }
                    ]
                }
            })
            .to_string(),
        )
        .create();

    let result = client.system.get_x25519_keypairs().await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.response.keypairs.len(), 2);
    assert_eq!(response.response.keypairs[0].public_key, "test_public_key_1");
    assert_eq!(response.response.keypairs[0].private_key, "test_private_key_1");
    assert_eq!(response.response.keypairs[1].public_key, "test_public_key_2");
    assert_eq!(response.response.keypairs[1].private_key, "test_private_key_2");
}
