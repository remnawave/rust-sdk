use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum AlpnType {
    H3,
    H2,
    #[serde(rename = "http/1.1")]
    HTTP_1_1,
    #[serde(rename = "h2,http/1.1")]
    H_COMBINED,
    #[serde(rename = "h3,h2,http/1.1")]
    H3_H2_H1_COMBINED,
    #[serde(rename = "h3,h2")]
    H3_H2_COMBINED,
}

impl fmt::Display for AlpnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FingerprintType {
    CHROME,
    FIREFOX,
    SAFARI,
    IOS,
    ANDROID,
    EDGE,
    QQ,
    RANDOM,
    RANDOMIZED,
}

impl fmt::Display for FingerprintType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum SecurityLayerType {
    #[default]
    DEFAULT,
    TLS,
    NONE,
}

impl fmt::Display for SecurityLayerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostInboundRequest {
    pub config_profile_uuid: Uuid,
    pub config_profile_inbound_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostInboundDto {
    pub config_profile_uuid: Option<Uuid>,
    pub config_profile_inbound_uuid: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostOrderItem {
    pub view_position: i32,
    pub uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteHostData {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReorderHostData {
    pub is_updated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostDto {
    pub uuid: Uuid,
    pub view_position: i32,
    pub remark: String,
    pub address: String,
    pub port: u16,
    pub path: Option<String>,
    pub sni: Option<String>,
    pub host: Option<String>,
    pub alpn: Option<String>,
    pub fingerprint: Option<String>,
    pub is_disabled: bool,
    pub security_layer: SecurityLayerType,
    pub x_http_extra_params: Option<serde_json::Value>,
    pub mux_params: Option<serde_json::Value>,
    pub sockopt_params: Option<serde_json::Value>,
    pub inbound: HostInboundDto,
    pub server_description: Option<String>,
    pub tag: Option<String>,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub override_sni_from_address: bool,
    pub vless_route_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateHostRequestDto {
    pub inbound: HostInboundRequest,
    pub remark: String,
    pub address: String,
    pub port: u16,
    pub path: Option<String>,
    pub sni: Option<String>,
    pub host: Option<String>,
    pub alpn: Option<AlpnType>,
    pub fingerprint: Option<FingerprintType>,
    pub is_disabled: bool,
    pub security_layer: SecurityLayerType,
    pub x_http_extra_params: Option<serde_json::Value>,
    pub mux_params: Option<serde_json::Value>,
    pub sockopt_params: Option<serde_json::Value>,
    pub server_description: Option<String>,
    pub tag: Option<String>,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub override_sni_from_address: bool,
    pub vless_route_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHostRequestDto {
    pub uuid: Uuid,
    pub inbound: Option<HostInboundRequest>,
    pub remark: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub sni: Option<String>,
    pub host: Option<String>,
    pub alpn: Option<AlpnType>,
    pub fingerprint: Option<FingerprintType>,
    pub is_disabled: Option<bool>,
    pub security_layer: Option<SecurityLayerType>,
    pub x_http_extra_params: Option<serde_json::Value>,
    pub mux_params: Option<serde_json::Value>,
    pub sockopt_params: Option<serde_json::Value>,
    pub server_description: Option<String>,
    pub tag: Option<String>,
    pub is_hidden: Option<bool>,
    pub override_sni_from_address: Option<bool>,
    pub vless_route_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReorderHostRequestDto {
    pub hosts: Vec<HostOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDeleteHostsRequestDto {
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDisableHostsRequestDto {
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkEnableHostsRequestDto {
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetInboundToManyHostsRequestDto {
    pub uuids: Vec<Uuid>,
    pub config_profile_uuid: Uuid,
    pub config_profile_inbound_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetPortToManyHostsRequestDto {
    pub uuids: Vec<Uuid>,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateHostResponseDto {
    pub response: HostDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateHostResponseDto {
    pub response: HostDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllHostsResponseDto {
    pub response: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetOneHostResponseDto {
    pub response: HostDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteHostResponseDto {
    pub response: DeleteHostData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReorderHostResponseDto {
    pub response: ReorderHostData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDeleteHostsResponseDto {
    pub response: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkDisableHostsResponseDto {
    pub response: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BulkEnableHostsResponseDto {
    pub response: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetInboundToManyHostsResponseDto {
    pub response: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetPortToManyHostsResponseDto {
    pub response: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllHostTagsResponseDto {
    pub response: HostTagsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostTagsData {
    pub tags: Vec<String>,
}
