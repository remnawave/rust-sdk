use crate::api::{
    ApiClient, ApiTokensController, AuthController, ConfigProfilesController, ExternalSquadsController, HostsController, HwidUserDevicesController, InfraBillingController,
    InternalSquadsController, KeygenController, NodesController, NodesUsageController, PasskeysController, RemnawaveSettingsController, SnippetsController,
    SubscriptionRequestHistoryController, SubscriptionSettingsController, SubscriptionTemplateController, SubscriptionsController, SystemController, UsersController,
};
use anyhow::Result;
use std::sync::Arc;

pub struct RemnawaveApiClient {
    client: Arc<ApiClient>,
    pub auth: AuthController,
    pub users: UsersController,
    pub subscriptions: SubscriptionsController,
    pub subscription_templates: SubscriptionTemplateController,
    pub subscription_settings: SubscriptionSettingsController,
    pub subscription_request_history: SubscriptionRequestHistoryController,
    pub external_squads: ExternalSquadsController,
    pub passkeys: PasskeysController,
    pub snippets: SnippetsController,
    pub remnawave_settings: RemnawaveSettingsController,
    pub nodes: NodesController,
    pub nodes_usage: NodesUsageController,
    pub hosts: HostsController,
    pub system: SystemController,
    pub tokens: ApiTokensController,
    pub config_profiles: ConfigProfilesController,
    pub internal_squads: InternalSquadsController,
    pub hwid: HwidUserDevicesController,
    pub billing: InfraBillingController,
    pub keygen: KeygenController,
}

impl RemnawaveApiClient {
    pub fn new(base_url: String, token: Option<String>) -> Result<Self> {
        Self::with_caddy_token(base_url, token, None)
    }

    pub fn with_caddy_token(base_url: String, token: Option<String>, caddy_token: Option<String>) -> Result<Self> {
        let client = Arc::new(ApiClient::with_caddy_token(base_url, token, caddy_token));

        Ok(Self {
            auth: AuthController::new(client.clone()),
            users: UsersController::new(client.clone()),
            subscriptions: SubscriptionsController::new(client.clone()),
            subscription_templates: SubscriptionTemplateController::new(client.clone()),
            subscription_settings: SubscriptionSettingsController::new(client.clone()),
            subscription_request_history: SubscriptionRequestHistoryController::new(client.clone()),
            external_squads: ExternalSquadsController::new(client.clone()),
            passkeys: PasskeysController::new(client.clone()),
            snippets: SnippetsController::new(client.clone()),
            remnawave_settings: RemnawaveSettingsController::new(client.clone()),
            nodes: NodesController::new(client.clone()),
            nodes_usage: NodesUsageController::new(client.clone()),
            hosts: HostsController::new(client.clone()),
            system: SystemController::new(client.clone()),
            tokens: ApiTokensController::new(client.clone()),
            config_profiles: ConfigProfilesController::new(client.clone()),
            internal_squads: InternalSquadsController::new(client.clone()),
            hwid: HwidUserDevicesController::new(client.clone()),
            billing: InfraBillingController::new(client.clone()),
            keygen: KeygenController::new(client.clone()),
            client,
        })
    }

    pub fn set_token(&mut self, token: Option<String>) {
        let new_client = Arc::new(ApiClient::with_caddy_token(self.client.base_url().to_string(), token, self.client.caddy_token.clone()));

        self.client = new_client.clone();
        self.auth = AuthController::new(new_client.clone());
        self.users = UsersController::new(new_client.clone());
        self.subscriptions = SubscriptionsController::new(new_client.clone());
        self.subscription_templates = SubscriptionTemplateController::new(new_client.clone());
        self.subscription_settings = SubscriptionSettingsController::new(new_client.clone());
        self.subscription_request_history = SubscriptionRequestHistoryController::new(new_client.clone());
        self.external_squads = ExternalSquadsController::new(new_client.clone());
        self.passkeys = PasskeysController::new(new_client.clone());
        self.snippets = SnippetsController::new(new_client.clone());
        self.remnawave_settings = RemnawaveSettingsController::new(new_client.clone());
        self.nodes = NodesController::new(new_client.clone());
        self.nodes_usage = NodesUsageController::new(new_client.clone());
        self.hosts = HostsController::new(new_client.clone());
        self.system = SystemController::new(new_client.clone());
        self.tokens = ApiTokensController::new(new_client.clone());
        self.config_profiles = ConfigProfilesController::new(new_client.clone());
        self.internal_squads = InternalSquadsController::new(new_client.clone());
        self.hwid = HwidUserDevicesController::new(new_client.clone());
        self.billing = InfraBillingController::new(new_client.clone());
        self.keygen = KeygenController::new(new_client.clone());
    }

    pub fn set_caddy_token(&mut self, caddy_token: Option<String>) {
        let new_client = Arc::new(ApiClient::with_caddy_token(self.client.base_url().to_string(), self.client.token.clone(), caddy_token));

        self.client = new_client.clone();
        self.auth = AuthController::new(new_client.clone());
        self.users = UsersController::new(new_client.clone());
        self.subscriptions = SubscriptionsController::new(new_client.clone());
        self.subscription_templates = SubscriptionTemplateController::new(new_client.clone());
        self.subscription_settings = SubscriptionSettingsController::new(new_client.clone());
        self.subscription_request_history = SubscriptionRequestHistoryController::new(new_client.clone());
        self.external_squads = ExternalSquadsController::new(new_client.clone());
        self.passkeys = PasskeysController::new(new_client.clone());
        self.snippets = SnippetsController::new(new_client.clone());
        self.remnawave_settings = RemnawaveSettingsController::new(new_client.clone());
        self.nodes = NodesController::new(new_client.clone());
        self.nodes_usage = NodesUsageController::new(new_client.clone());
        self.hosts = HostsController::new(new_client.clone());
        self.system = SystemController::new(new_client.clone());
        self.tokens = ApiTokensController::new(new_client.clone());
        self.config_profiles = ConfigProfilesController::new(new_client.clone());
        self.internal_squads = InternalSquadsController::new(new_client.clone());
        self.hwid = HwidUserDevicesController::new(new_client.clone());
        self.billing = InfraBillingController::new(new_client.clone());
        self.keygen = KeygenController::new(new_client.clone());
    }

    pub fn base_url(&self) -> &str {
        self.client.base_url()
    }
}
