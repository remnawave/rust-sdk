use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseData {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponseDto {
    pub response: LoginResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterRequestDto {
    pub username: String,
    #[serde(with = "password_validation")]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponseData {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterResponseDto {
    pub response: RegisterResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetStatusResponseData {
    pub is_login_allowed: bool,
    pub is_register_allowed: bool,
    pub authentication: Option<AuthenticationStatus>,
    pub branding: BrandingSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg_auth: Option<TgAuthStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<OAuth2Providers>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetStatusResponseDto {
    pub response: GetStatusResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationStatus {
    pub passkey: PasskeyAuthStatus,
    pub tg_auth: TgAuthStatus,
    pub oauth2: OAuth2Providers,
    pub password: PasswordAuthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyAuthStatus {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TgAuthStatus {
    pub enabled: bool,
    pub bot_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PasswordAuthStatus {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuth2Providers {
    pub providers: std::collections::HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BrandingSettings {
    pub title: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelegramCallbackRequestDto {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TelegramCallbackResponseData {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelegramCallbackResponseDto {
    pub response: TelegramCallbackResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OAuth2Provider {
    Github,
    Pocketid,
    Yandex,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuth2AuthorizeRequestDto {
    pub provider: OAuth2Provider,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2AuthorizeResponseData {
    pub authorization_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuth2AuthorizeResponseDto {
    pub response: OAuth2AuthorizeResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuth2CallbackRequestDto {
    pub provider: OAuth2Provider,
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2CallbackResponseData {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuth2CallbackResponseDto {
    pub response: OAuth2CallbackResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPasskeyAuthenticationOptionsResponseDto {
    pub response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifyPasskeyAuthenticationRequestDto {
    pub response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPasskeyAuthenticationResponseData {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyPasskeyAuthenticationResponseDto {
    pub response: VerifyPasskeyAuthenticationResponseData,
}

mod password_validation {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(password: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        password.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let password = String::deserialize(deserializer)?;

        if password.len() < 24 {
            return Err(serde::de::Error::custom("Password must be at least 24 characters long"));
        }

        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());

        if !has_upper || !has_lower || !has_digit {
            return Err(serde::de::Error::custom("Password must contain at least one uppercase letter, one lowercase letter, and one digit"));
        }

        Ok(password)
    }
}
