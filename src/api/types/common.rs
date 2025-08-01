use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub timestamp: Option<String>,
    pub path: Option<String>,
    pub message: String,
    pub error_code: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InboundDto {
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrafficLimitStrategy {
    NoReset,
    Day,
    Week,
    Month,
}

impl Default for TrafficLimitStrategy {
    fn default() -> Self {
        TrafficLimitStrategy::NoReset
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserStatus {
    Active,
    Disabled,
    Limited,
    Expired,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Active
    }
}

impl fmt::Display for TrafficLimitStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{}", s)
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_plain::to_string(self).unwrap();
        write!(f, "{}", s)
    }
}
