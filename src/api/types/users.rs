use crate::types::{TrafficLimitStrategy, UserStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InternalSquad {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LastConnectedNode {
    pub connected_at: DateTime<Utc>,
    pub node_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Happ {
    pub crypto_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveNode {
    pub uuid: Uuid,
    pub node_name: String,
    pub country_code: String,
    pub config_profile_uuid: Uuid,
    pub config_profile_name: String,
    pub active_squads: Vec<ActiveSquad>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveSquad {
    pub squad_name: String,
    pub active_inbounds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequestDto {
    pub username: String,
    #[serde(default)]
    pub status: UserStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trojan_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vless_uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_limit_bytes: Option<usize>,
    #[serde(default)]
    pub traffic_limit_strategy: TrafficLimitStrategy,
    pub expire_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_traffic_reset_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_device_limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_internal_squads: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateUserResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequestDto {
    pub uuid: Uuid,
    #[serde(default)]
    pub status: UserStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_limit_bytes: Option<usize>,
    #[serde(default)]
    pub traffic_limit_strategy: TrafficLimitStrategy,
    pub expire_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_device_limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_internal_squads: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateUserResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub uuid: Uuid,
    pub short_uuid: String,
    pub username: String,
    pub status: UserStatus,
    pub used_traffic_bytes: i64,
    pub lifetime_used_traffic_bytes: i64,
    pub traffic_limit_bytes: i64,
    #[serde(default)]
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
    pub hwid_device_limit: Option<usize>,
    pub first_connected_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_triggered_threshold: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active_internal_squads: Vec<InternalSquad>,
    pub subscription_url: String,
    pub last_connected_node: Option<LastConnectedNode>,
    pub happ: Happ,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteUserResponseDto {
    pub response: DeleteUserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserResponse {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllUsersResponseDto {
    pub response: GetAllUsersResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllUsersResponse {
    pub users: Vec<UserData>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllTagsResponseDto {
    pub response: GetAllTagsResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllTagsResponse {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserAccessibleNodesResponseDto {
    pub response: GetUserAccessibleNodesResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetUserAccessibleNodesResponse {
    pub user_uuid: Uuid,
    pub active_nodes: Vec<ActiveNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserByShortUuidResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserByUuidResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserByUsernameResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserByTelegramIdResponseDto {
    pub response: Vec<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserByEmailResponseDto {
    pub response: Vec<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserByTagResponseDto {
    pub response: Vec<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RevokeUserSubscriptionBodyDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RevokeUserSubscriptionResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DisableUserResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnableUserResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResetUserTrafficResponseDto {
    pub response: UserData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDeleteUsersByStatusRequestDto {
    pub status: UserStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDeleteUsersByStatusResponseDto {
    pub response: BulkOperationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDeleteUsersRequestDto {
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDeleteUsersResponseDto {
    pub response: BulkOperationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkRevokeUsersSubscriptionRequestDto {
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkRevokeUsersSubscriptionResponseDto {
    pub response: BulkOperationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkResetTrafficUsersRequestDto {
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkResetTrafficUsersResponseDto {
    pub response: BulkOperationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkUpdateUsersRequestDto {
    pub uuids: Vec<Uuid>,
    pub fields: BulkUpdateFields,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkUpdateUsersResponseDto {
    pub response: BulkOperationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulkUpdateUsersSquadsRequestDto {
    pub uuids: Vec<Uuid>,
    pub active_internal_squads: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkUpdateUsersSquadsResponseDto {
    pub response: BulkOperationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulkAllUpdateUsersRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_limit_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_limit_strategy: Option<TrafficLimitStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_device_limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkAllUpdateUsersResponseDto {
    pub response: BulkEventResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkAllResetTrafficUsersResponseDto {
    pub response: BulkEventResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulkOperationResponse {
    pub affected_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulkEventResponse {
    pub event_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulkUpdateFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_limit_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_limit_strategy: Option<TrafficLimitStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwid_device_limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetUserUsageByRangeResponseDto {
    pub response: Vec<UsageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsageData {
    pub user_uuid: Uuid,
    pub node_uuid: Uuid,
    pub node_name: String,
    pub total: usize,
    pub date: String,
}
