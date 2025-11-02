use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SnippetItem {
    pub name: String,
    pub snippet: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SnippetsCollection {
    pub total: usize,
    pub snippets: Vec<SnippetItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetSnippetsResponseDto {
    pub response: SnippetsCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteSnippetRequestDto {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteSnippetResponseDto {
    pub response: SnippetsCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateSnippetRequestDto {
    pub name: String,
    pub snippet: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSnippetResponseDto {
    pub response: SnippetsCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnippetRequestDto {
    pub name: String,
    pub snippet: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateSnippetResponseDto {
    pub response: SnippetsCollection,
}
