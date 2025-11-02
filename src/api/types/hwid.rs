use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HwidDeviceDto {
    pub hwid: String,
    pub user_uuid: Uuid,
    pub platform: Option<String>,
    pub os_version: Option<String>,
    pub device_model: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserHwidDeviceRequestDto {
    pub hwid: String,
    pub user_uuid: Uuid,
    pub platform: Option<String>,
    pub os_version: Option<String>,
    pub device_model: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserHwidDeviceData {
    pub total: usize,
    pub devices: Vec<HwidDeviceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserHwidDeviceResponseDto {
    pub response: CreateUserHwidDeviceData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserHwidDeviceRequestDto {
    pub user_uuid: Uuid,
    pub hwid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserHwidDeviceData {
    pub total: usize,
    pub devices: Vec<HwidDeviceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserHwidDeviceResponseDto {
    pub response: DeleteUserHwidDeviceData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetUserHwidDevicesData {
    pub total: usize,
    pub devices: Vec<HwidDeviceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetUserHwidDevicesResponseDto {
    pub response: GetUserHwidDevicesData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAllHwidDevicesResponseDto {
    pub response: GetAllHwidDevicesData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetAllHwidDevicesData {
    pub total: usize,
    pub devices: Vec<HwidDeviceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteAllUserHwidDevicesRequestDto {
    pub user_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteAllUserHwidDevicesResponseDto {
    pub response: DeleteAllUserHwidDevicesData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAllUserHwidDevicesData {
    pub total: usize,
    pub devices: Vec<HwidDeviceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetHwidDevicesStatsResponseDto {
    pub response: HwidDevicesStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HwidDevicesStats {
    pub by_platform: Vec<HwidDevicesPlatformStat>,
    pub by_app: Vec<HwidDevicesAppStat>,
    pub stats: HwidDevicesStatsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HwidDevicesPlatformStat {
    pub platform: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HwidDevicesAppStat {
    pub app: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HwidDevicesStatsSummary {
    pub total_unique_devices: usize,
    pub total_hwid_devices: usize,
    pub average_hwid_devices_per_user: f64,
}
