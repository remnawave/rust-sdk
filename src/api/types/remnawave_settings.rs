use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemnawaveSettingsDto {
    pub passkey_settings: Option<PasskeySettings>,
    pub oauth2_settings: Option<Oauth2Settings>,
    pub tg_auth_settings: Option<TgAuthSettings>,
    pub password_settings: Option<PasswordSettings>,
    pub branding_settings: Option<RemnawaveBrandingSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRemnawaveSettingsResponseDto {
    pub response: RemnawaveSettingsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRemnawaveSettingsRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passkey_settings: Option<PasskeySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_settings: Option<Oauth2Settings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg_auth_settings: Option<TgAuthSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_settings: Option<PasswordSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding_settings: Option<RemnawaveBrandingSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRemnawaveSettingsResponseDto {
    pub response: RemnawaveSettingsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PasskeySettings {
    pub enabled: bool,
    pub rp_id: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GithubOAuthSettings {
    pub enabled: bool,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub allowed_emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PocketIdOAuthSettings {
    pub enabled: bool,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub plain_domain: Option<String>,
    pub allowed_emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct YandexOAuthSettings {
    pub enabled: bool,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub allowed_emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Oauth2Settings {
    pub github: GithubOAuthSettings,
    pub pocketid: PocketIdOAuthSettings,
    pub yandex: YandexOAuthSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TgAuthSettings {
    pub enabled: bool,
    pub bot_token: Option<String>,
    pub admin_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PasswordSettings {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemnawaveBrandingSettings {
    pub title: Option<String>,
    pub logo_url: Option<String>,
}
