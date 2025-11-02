use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetConfigProfilesResponseDto {
    pub response: GetConfigProfilesResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigProfilesResponseData {
    pub total: usize,
    pub config_profiles: Vec<ConfigProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllInboundsResponseDto {
    pub response: GetAllInboundsResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllInboundsResponseData {
    pub total: usize,
    pub inbounds: Vec<Inbound>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInboundsByProfileUuidResponseDto {
    pub response: GetInboundsByProfileUuidResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInboundsByProfileUuidResponseData {
    pub total: usize,
    pub inbounds: Vec<Inbound>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetConfigProfileByUuidResponseDto {
    pub response: ConfigProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetComputedConfigProfileByUuidResponseDto {
    pub response: ConfigProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteConfigProfileResponseDto {
    pub response: DeleteConfigProfileResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteConfigProfileResponseData {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateConfigProfileRequestDto {
    pub name: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateConfigProfileResponseDto {
    pub response: ConfigProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateConfigProfileRequestDto {
    pub uuid: Uuid,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateConfigProfileResponseDto {
    pub response: ConfigProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigProfile {
    pub uuid: Uuid,
    pub name: String,
    pub config: serde_json::Value,
    pub inbounds: Vec<Inbound>,
    pub nodes: Vec<Node>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Inbound {
    pub uuid: Uuid,
    pub profile_uuid: Uuid,
    pub tag: String,
    #[serde(rename = "type")]
    pub inbound_type: String,
    pub network: Option<String>,
    pub security: Option<String>,
    pub port: Option<u16>,
    pub raw_inbound: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub uuid: Uuid,
    pub name: String,
    pub country_code: String,
}
