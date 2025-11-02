use crate::api::types::subscriptions::{SubscriptionResponseRule, SubscriptionResponseRuleType, SubscriptionResponseRulesConfig};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetStatsResponseDto {
    pub response: SystemStatsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetBandwidthStatsResponseDto {
    pub response: BandwidthStatsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetNodesStatisticsResponseDto {
    pub response: NodesStatisticsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetRemnawaveHealthResponseDto {
    pub response: RemnawaveHealthData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetNodesMetricsResponseDto {
    pub response: NodesMetricsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateX25519ResponseDto {
    pub response: GenerateX25519Data,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EncryptHappCryptoLinkRequestDto {
    pub link_to_encrypt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptHappCryptoLinkResponseDto {
    pub response: EncryptHappCryptoLinkResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EncryptHappCryptoLinkResponse {
    pub encrypted_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatsData {
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub uptime: f64,
    pub timestamp: f64,
    pub users: UsersStats,
    pub online_stats: OnlineStats,
    pub nodes: NodesStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CpuStats {
    pub cores: usize,
    pub physical_cores: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryStats {
    pub total: f64,
    pub free: f64,
    pub used: f64,
    pub active: f64,
    pub available: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsersStats {
    pub status_counts: std::collections::HashMap<String, usize>,
    pub total_users: usize,
    pub total_traffic_bytes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OnlineStats {
    pub last_day: usize,
    pub last_week: usize,
    pub never_online: usize,
    pub online_now: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodesStats {
    pub total_online: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BandwidthStatsData {
    #[serde(rename = "bandwidthLastTwoDays")]
    pub last_two_days: BandwidthPeriod,
    #[serde(rename = "bandwidthLastSevenDays")]
    pub last_seven_days: BandwidthPeriod,
    #[serde(rename = "bandwidthLast30Days")]
    pub last_30_days: BandwidthPeriod,
    #[serde(rename = "bandwidthCalendarMonth")]
    pub calendar_month: BandwidthPeriod,
    #[serde(rename = "bandwidthCurrentYear")]
    pub current_year: BandwidthPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BandwidthPeriod {
    pub current: String,
    pub previous: String,
    pub difference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodesStatisticsData {
    pub last_seven_days: Vec<NodeStatisticItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatisticItem {
    pub node_name: String,
    pub date: String,
    pub total_bytes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RemnawaveHealthData {
    pub pm2_stats: Vec<Pm2Stat>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pm2Stat {
    pub name: String,
    pub memory: String,
    pub cpu: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodesMetricsData {
    pub nodes: Vec<NodeMetricItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NodeMetricItem {
    pub node_uuid: Uuid,
    pub node_name: String,
    pub country_emoji: String,
    pub provider_name: String,
    pub users_online: usize,
    pub inbounds_stats: Vec<InboundStat>,
    pub outbounds_stats: Vec<OutboundStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InboundStat {
    pub tag: String,
    pub upload: String,
    pub download: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutboundStat {
    pub tag: String,
    pub upload: String,
    pub download: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateX25519Data {
    pub keypairs: Vec<X25519Keypair>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugSrrMatcherRequestDto {
    #[serde(rename = "responseRules")]
    pub response_rules: SubscriptionResponseRulesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugSrrMatcherResponseDto {
    pub response: DebugSrrMatcherResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DebugSrrMatcherResponse {
    pub matched: bool,
    pub response_type: SubscriptionResponseRuleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_rule: Option<SubscriptionResponseRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct X25519Keypair {
    pub public_key: String,
    pub private_key: String,
}
