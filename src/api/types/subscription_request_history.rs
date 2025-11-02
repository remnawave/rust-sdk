use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionRequestHistoryResponseDto {
    pub response: SubscriptionRequestHistoryData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequestHistoryData {
    pub records: Vec<SubscriptionRequestRecord>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequestRecord {
    pub id: i64,
    pub user_uuid: Uuid,
    pub request_ip: Option<String>,
    pub user_agent: Option<String>,
    pub request_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSubscriptionRequestHistoryStatsResponseDto {
    pub response: SubscriptionRequestHistoryStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequestHistoryStats {
    pub by_parsed_app: Vec<SubscriptionRequestHistoryAppStat>,
    pub hourly_request_stats: Vec<SubscriptionRequestHistoryHourlyStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequestHistoryAppStat {
    pub app: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRequestHistoryHourlyStat {
    pub date_time: DateTime<Utc>,
    pub request_count: usize,
}
