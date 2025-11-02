use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetExternalSquadsResponseDto {
    pub response: ExternalSquadsList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadsList {
    pub total: usize,
    pub external_squads: Vec<ExternalSquadDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetExternalSquadByUuidResponseDto {
    pub response: ExternalSquadDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadDto {
    pub uuid: Uuid,
    pub name: String,
    pub info: ExternalSquadInfo,
    pub templates: Vec<ExternalSquadTemplate>,
    pub subscription_settings: Option<ExternalSquadSubscriptionSettings>,
    pub host_overrides: Option<ExternalSquadHostOverrides>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadInfo {
    pub members_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadTemplate {
    pub template_uuid: Uuid,
    pub template_type: crate::api::types::subscriptions::SubscriptionTemplateType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadSubscriptionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_update_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_profile_webpage_url_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serve_json_at_base_subscription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_username_to_base_subscription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_show_custom_remarks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub happ_announce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub happ_routing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_hosts: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadHostOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vless_route_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalSquadRequestDto {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateExternalSquadResponseDto {
    pub response: ExternalSquadDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExternalSquadRequestDto {
    pub uuid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<ExternalSquadTemplate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_settings: Option<ExternalSquadSubscriptionSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_overrides: Option<ExternalSquadHostOverrides>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateExternalSquadResponseDto {
    pub response: ExternalSquadDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteExternalSquadResponseDto {
    pub response: DeleteExternalSquadResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteExternalSquadResponse {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddUsersToExternalSquadResponseDto {
    pub response: ExternalSquadBulkActionResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoveUsersFromExternalSquadResponseDto {
    pub response: ExternalSquadBulkActionResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSquadBulkActionResponse {
    pub event_sent: bool,
}
