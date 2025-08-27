use crate::types::InboundDto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInternalSquadsResponseDto {
    pub response: GetInternalSquadsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetInternalSquadsData {
    pub total: usize,
    pub internal_squads: Vec<InternalSquadDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInternalSquadByUuidResponseDto {
    pub response: InternalSquadDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateInternalSquadRequestDto {
    pub name: String,
    pub inbounds: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateInternalSquadResponseDto {
    pub response: InternalSquadDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateInternalSquadRequestDto {
    pub uuid: Uuid,
    pub inbounds: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateInternalSquadResponseDto {
    pub response: InternalSquadDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteInternalSquadResponseDto {
    pub response: DeleteInternalSquadData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInternalSquadData {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddUsersToInternalSquadResponseDto {
    pub response: BulkActionResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoveUsersFromInternalSquadResponseDto {
    pub response: BulkActionResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulkActionResponseData {
    pub event_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InternalSquadDto {
    pub uuid: Uuid,
    pub name: String,
    pub info: InternalSquadInfo,
    pub inbounds: Vec<InboundDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InternalSquadInfo {
    pub members_count: usize,
    pub inbounds_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInternalSquadAccessibleNodesResponseDto {
    pub response: InternalSquadAccessibleNodesData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InternalSquadAccessibleNodesData {
    pub squad_uuid: Uuid,
    pub accessible_nodes: Vec<AccessibleNodeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccessibleNodeDto {
    pub uuid: Uuid,
    pub node_name: String,
    pub country_code: String,
    pub config_profile_uuid: Uuid,
    pub config_profile_name: String,
    pub active_inbounds: Vec<Uuid>,
}
