use crate::api::types::users::{InternalSquad, LastConnectedNode};
use crate::types::{TrafficLimitStrategy, UserStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionClientType {
    Stash,
    SingBox,
    #[serde(rename = "singbox-legacy")]
    SingBoxLegacy,
    Mihomo,
    Json,
    #[serde(rename = "v2ray-Json")]
    V2RayJson,
    Clash,
}

impl fmt::Display for SubscriptionClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubscriptionTemplateType {
    Stash,
    SingBox,
    #[serde(rename = "SINGBOX_LEGACY")]
    SingBoxLegacy,
    Mihomo,
    #[serde(rename = "XRAY_JSON")]
    XrayJson,
    Clash,
}

impl fmt::Display for SubscriptionTemplateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionUser {
    pub short_uuid: String,
    pub days_left: usize,
    pub traffic_used: String,
    pub traffic_limit: String,
    pub lifetime_traffic_used: String,
    pub lifetime_traffic_used_bytes: String,
    pub traffic_limit_bytes: String,
    pub traffic_used_bytes: String,
    pub username: String,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub user_status: UserStatus,
    pub traffic_limit_strategy: TrafficLimitStrategy,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HappConfig {
    pub crypto_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawHost {
    pub address: Option<String>,
    pub alpn: Option<String>,
    pub fingerprint: Option<String>,
    pub host: Option<String>,
    pub network: Option<String>,
    pub password: HostPasswords,
    pub path: Option<String>,
    pub public_key: Option<String>,
    pub port: Option<u16>,
    pub protocol: Option<String>,
    pub remark: Option<String>,
    pub short_id: Option<String>,
    pub sni: Option<String>,
    pub spider_x: Option<String>,
    pub tls: Option<String>,
    pub header_type: Option<String>,
    pub additional_params: Option<AdditionalParams>,
    pub x_http_extra_params: Option<HashMap<String, serde_json::Value>>,
    pub server_description: Option<String>,
    pub flow: Option<String>,
    pub protocol_options: Option<ProtocolOptions>,
    #[serde(rename = "muxParams")]
    pub mux_params: Option<serde_json::Value>,
    #[serde(rename = "sockoptParams")]
    pub sockopt_params: Option<serde_json::Value>,
    #[serde(rename = "dbData")]
    pub db_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalParams {
    pub mode: Option<String>,
    pub heartbeat_period: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolOptions {
    pub ss: Option<SsOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SsOptions {
    pub method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostPasswords {
    #[serde(rename = "trojanPassword")]
    pub trojan: String,
    #[serde(rename = "vlessPassword")]
    pub vless: String,
    #[serde(rename = "ssPassword")]
    pub ss: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawSubscriptionUser {
    pub uuid: Uuid,
    pub short_uuid: String,
    pub username: String,
    pub status: UserStatus,
    pub used_traffic_bytes: u64,
    pub lifetime_used_traffic_bytes: u64,
    pub traffic_limit_bytes: i32,
    pub traffic_limit_strategy: TrafficLimitStrategy,
    pub sub_last_user_agent: Option<String>,
    pub sub_last_opened_at: Option<DateTime<Utc>>,
    pub expire_at: DateTime<Utc>,
    pub online_at: Option<DateTime<Utc>>,
    pub sub_revoked_at: Option<DateTime<Utc>>,
    pub last_traffic_reset_at: Option<DateTime<Utc>>,
    pub trojan_password: String,
    pub vless_uuid: Uuid,
    pub ss_password: String,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub telegram_id: Option<i64>,
    pub email: Option<String>,
    pub hwid_device_limit: Option<i32>,
    pub first_connected_at: Option<DateTime<Utc>>,
    pub last_triggered_threshold: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active_internal_squads: Vec<InternalSquad>,
    pub subscription_url: String,
    pub last_connected_node: Option<LastConnectedNode>,
    pub happ: HappConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConvertedUserInfo {
    pub days_left: f64,
    pub traffic_limit: String,
    pub traffic_used: String,
    pub lifetime_traffic_used: String,
    pub is_hwid_limited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub is_found: bool,
    pub user: SubscriptionUser,
    pub links: Vec<String>,
    pub ss_conf_links: HashMap<String, String>,
    pub subscription_url: String,
    pub happ: Option<HappConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionInfoResponseDto {
    pub response: Subscription,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRawSubscriptionByShortUuidResponseDto {
    pub response: RawSubscriptionResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawSubscriptionResponse {
    pub user: RawSubscriptionUser,
    pub converted_user_info: ConvertedUserInfo,
    pub headers: HashMap<String, String>,
    pub raw_hosts: Vec<RawHost>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllSubscriptionsResponseDto {
    pub response: AllSubscriptionsResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllSubscriptionsResponse {
    pub subscriptions: Vec<BasicSubscription>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BasicSubscription {
    pub is_found: bool,
    pub user: BasicSubscriptionUser,
    pub links: Vec<String>,
    pub ss_conf_links: HashMap<String, String>,
    pub subscription_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BasicSubscriptionUser {
    pub short_uuid: String,
    pub days_left: usize,
    pub traffic_used: String,
    pub traffic_limit: String,
    pub username: String,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub user_status: UserStatus,
    pub traffic_limit_strategy: TrafficLimitStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionByUsernameResponseDto {
    pub response: UsernameSubscriptionResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionByShortUuidResponseDto {
    pub response: ShortUuidSubscriptionResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ShortUuidSubscriptionResponse {
    pub is_found: bool,
    pub user: SubscriptionUser,
    pub links: Vec<String>,
    pub ss_conf_links: HashMap<String, String>,
    pub subscription_url: String,
    pub happ: HappConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionByUuidResponseDto {
    pub response: UuidSubscriptionResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UuidSubscriptionResponse {
    pub is_found: bool,
    pub user: SubscriptionUser,
    pub links: Vec<String>,
    pub ss_conf_links: HashMap<String, String>,
    pub subscription_url: String,
    pub happ: HappConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsernameSubscriptionResponse {
    pub is_found: bool,
    pub user: SubscriptionUser,
    pub links: Vec<String>,
    pub ss_conf_links: HashMap<String, String>,
    pub subscription_url: String,
    pub happ: HappConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetTemplateResponseDto {
    pub response: TemplateResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateRequestDto {
    pub template_type: SubscriptionTemplateType,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TemplateResponse {
    pub uuid: Uuid,
    pub template_type: SubscriptionTemplateType,
    pub template_json: Option<serde_json::Value>,
    pub encoded_template_yaml: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateTemplateResponseDto {
    pub response: TemplateResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionSettingsResponseDto {
    pub response: SubscriptionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubscriptionSettingsRequestDto {
    pub uuid: Uuid,
    pub profile_title: String,
    pub support_link: String,
    pub profile_update_interval: usize,
    pub is_profile_webpage_url_enabled: bool,
    pub serve_json_at_base_subscription: bool,
    pub add_username_to_base_subscription: bool,
    pub is_show_custom_remarks: bool,
    pub happ_announce: Option<String>,
    pub happ_routing: Option<String>,
    pub expired_users_remarks: Vec<String>,
    pub limited_users_remarks: Vec<String>,
    pub disabled_users_remarks: Vec<String>,
    pub custom_response_headers: Option<std::collections::HashMap<String, String>>,
    pub randomize_hosts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateSubscriptionSettingsResponseDto {
    pub response: SubscriptionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionSettings {
    pub uuid: Uuid,
    pub profile_title: String,
    pub support_link: String,
    pub profile_update_interval: usize,
    pub is_profile_webpage_url_enabled: bool,
    pub serve_json_at_base_subscription: bool,
    pub add_username_to_base_subscription: bool,
    pub is_show_custom_remarks: bool,
    pub happ_announce: Option<String>,
    pub happ_routing: Option<String>,
    pub expired_users_remarks: Vec<String>,
    pub limited_users_remarks: Vec<String>,
    pub disabled_users_remarks: Vec<String>,
    pub custom_response_headers: Option<std::collections::HashMap<String, String>>,
    pub randomize_hosts: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
