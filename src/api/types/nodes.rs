use crate::types::InboundDto;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateNodeRequestDto {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub is_traffic_tracking_active: bool,
    pub traffic_limit_bytes: usize,
    pub notify_percent: u8,
    pub traffic_reset_day: u8,
    #[serde(default = "default_country_code")]
    pub country_code: String,
    pub consumption_multiplier: f32,
    pub config_profile: ConfigProfileRequest,
    pub provider_uuid: Option<Uuid>,
}

fn default_country_code() -> String {
    "XX".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNodeRequestDto {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub is_traffic_tracking_active: Option<bool>,
    pub traffic_limit_bytes: Option<usize>,
    pub notify_percent: Option<u8>,
    pub traffic_reset_day: Option<u8>,
    pub country_code: Option<String>,
    pub consumption_multiplier: Option<f32>,
    pub config_profile: Option<ConfigProfileRequest>,
    pub provider_uuid: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReorderNodeRequestDto {
    pub nodes: Vec<NodeOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateNodeResponseDto {
    pub response: NodeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAllNodesResponseDto {
    pub response: Vec<NodeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetOneNodeResponseDto {
    pub response: NodeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateNodeResponseDto {
    pub response: NodeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteNodeResponseDto {
    pub response: DeleteNodeData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnableNodeResponseDto {
    pub response: NodeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisableNodeResponseDto {
    pub response: NodeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestartNodeResponseDto {
    pub response: RestartNodeData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestartAllNodesResponseDto {
    pub response: RestartAllNodesData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReorderNodeResponseDto {
    pub response: Vec<NodeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetNodesUsageByRangeResponseDto {
    pub response: Vec<NodesUsageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetNodeUserUsageByRangeResponseDto {
    pub response: Vec<NodeUserUsageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetNodesRealtimeUsageResponseDto {
    pub response: Vec<NodeRealtimeUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeDto {
    pub uuid: Uuid,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub is_connected: bool,
    pub is_disabled: bool,
    pub is_connecting: bool,
    pub is_node_online: bool,
    pub is_xray_running: bool,
    pub last_status_change: Option<String>,
    pub last_status_message: Option<String>,
    pub xray_version: Option<String>,
    pub node_version: Option<String>,
    pub xray_uptime: String,
    pub is_traffic_tracking_active: bool,
    pub traffic_reset_day: Option<i32>,
    pub traffic_limit_bytes: Option<usize>,
    pub traffic_used_bytes: Option<usize>,
    pub notify_percent: Option<i32>,
    pub users_online: Option<i32>,
    pub view_position: u8,
    pub country_code: String,
    pub consumption_multiplier: f32,
    pub cpu_count: Option<i32>,
    pub cpu_model: Option<String>,
    pub total_ram: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub config_profile: NodesConfigProfile,
    pub provider_uuid: Option<Uuid>,
    pub provider: Option<Provider>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigProfileRequest {
    pub active_config_profile_uuid: Uuid,
    pub active_inbounds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodesConfigProfile {
    pub active_config_profile_uuid: Option<Uuid>,
    pub active_inbounds: Vec<InboundDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub uuid: Uuid,
    pub name: String,
    pub favicon_link: Option<String>,
    pub login_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeOrderItem {
    pub view_position: u8,
    pub uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNodeData {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RestartNodeData {
    pub event_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RestartAllNodesData {
    pub event_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodesUsageData {
    pub node_uuid: Uuid,
    pub node_name: String,
    pub total: usize,
    pub total_download: usize,
    pub total_upload: usize,
    pub human_readable_total: String,
    pub human_readable_total_download: String,
    pub human_readable_total_upload: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeUserUsageData {
    pub user_uuid: Uuid,
    pub username: String,
    pub node_uuid: Uuid,
    pub total: usize,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeRealtimeUsage {
    pub node_uuid: Uuid,
    pub node_name: String,
    pub country_code: String,
    pub download_bytes: usize,
    pub upload_bytes: usize,
    pub total_bytes: usize,
    pub download_speed_bps: usize,
    pub upload_speed_bps: usize,
    pub total_speed_bps: usize,
}
