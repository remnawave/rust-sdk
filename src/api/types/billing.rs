use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetInfraProvidersResponseDto {
    pub response: GetInfraProvidersData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetInfraProvidersData {
    pub total: usize,
    pub providers: Vec<InfraProviderDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInfraProviderByUuidResponseDto {
    pub response: InfraProviderDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteInfraProviderByUuidResponseDto {
    pub response: DeleteInfraProviderData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInfraProviderData {
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateInfraProviderRequestDto {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateInfraProviderResponseDto {
    pub response: InfraProviderDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfraProviderRequestDto {
    pub uuid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateInfraProviderResponseDto {
    pub response: InfraProviderDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InfraProviderDto {
    pub uuid: Uuid,
    pub name: String,
    pub favicon_link: Option<String>,
    pub login_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub billing_history: BillingHistorySummary,
    pub billing_nodes: Vec<BillingNodeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingHistorySummary {
    pub total_amount: usize,
    pub total_bills: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingNodeSummary {
    pub node_uuid: String,
    pub name: String,
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateInfraBillingHistoryRecordRequestDto {
    pub provider_uuid: String,
    pub amount: usize,
    pub billed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateInfraBillingHistoryRecordResponseDto {
    pub response: BillingHistoryResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetInfraBillingHistoryRecordsResponseDto {
    pub response: BillingHistoryResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteInfraBillingHistoryRecordByUuidResponseDto {
    pub response: BillingHistoryResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BillingHistoryResponseData {
    pub records: Vec<BillingHistoryRecordDto>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingHistoryRecordDto {
    pub uuid: Uuid,
    pub provider_uuid: String,
    pub amount: usize,
    pub billed_at: DateTime<Utc>,
    pub provider: BillingProviderInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingProviderInfo {
    pub uuid: Uuid,
    pub name: String,
    pub favicon_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetInfraBillingNodesResponseDto {
    pub response: BillingNodesResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateInfraBillingNodeRequestDto {
    pub provider_uuid: String,
    pub node_uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateInfraBillingNodeResponseDto {
    pub response: BillingNodesResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfraBillingNodeRequestDto {
    pub uuid: Uuid,
    pub next_billing_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateInfraBillingNodeResponseDto {
    pub response: BillingNodesResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteInfraBillingNodeByUuidResponseDto {
    pub response: BillingNodesResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BillingNodesResponseData {
    pub total_billing_nodes: usize,
    pub billing_nodes: Vec<BillingNodeDto>,
    pub available_billing_nodes: Vec<AvailableBillingNodeDto>,
    pub total_available_billing_nodes: usize,
    pub stats: BillingNodesStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingNodeDto {
    pub uuid: Uuid,
    pub node_uuid: String,
    pub provider_uuid: String,
    pub provider: BillingNodeProviderInfo,
    pub node: BillingNodeInfo,
    pub next_billing_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingNodeProviderInfo {
    pub uuid: Uuid,
    pub name: String,
    pub login_url: Option<String>,
    pub favicon_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BillingNodeInfo {
    pub uuid: Uuid,
    pub name: String,
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableBillingNodeDto {
    pub uuid: Uuid,
    pub name: String,
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BillingNodesStats {
    pub upcoming_nodes_count: usize,
    pub current_month_payments: usize,
    pub total_spent: usize,
}
