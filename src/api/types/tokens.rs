use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiTokenRequestDto {
    pub token_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiTokenResponseDto {
    pub response: CreateApiTokenResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiTokenResponse {
    pub token: String,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApiTokenResponseDto {
    pub response: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindAllApiTokensResponseDto {
    pub response: FindAllApiTokensResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindAllApiTokensResponse {
    pub api_keys: Vec<ApiTokenInfo>,
    pub docs: DocsInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTokenInfo {
    pub uuid: Uuid,
    pub token: String,
    pub token_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocsInfo {
    pub is_docs_enabled: bool,
    pub scalar_path: Option<String>,
    pub swagger_path: Option<String>,
}
