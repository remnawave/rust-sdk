use crate::api::controllers::macros::*;
use crate::api::types::subscriptions::*;

api_controller!(SubscriptionsController);

api_get_with_path!(SubscriptionsController, get_info_by_short_uuid, "/api/sub/{}/info", GetSubscriptionInfoResponseDto, short_uuid: String);
api_get_with_path_and_query!(SubscriptionsController, get_raw_by_short_uuid, "/api/sub/{}/raw", RawSubscriptionResponse, path_params: [short_uuid: String], query_params: [with_disabled_hosts: Option<bool>]);

api_get_with_path!(SubscriptionsController, get, "/api/sub/{}", String, short_uuid: String);
api_get_with_path!(SubscriptionsController, get_by_client_type, "/api/sub/{}/{}", String, short_uuid: String, client_type: SubscriptionClientType);

impl SubscriptionsController {
    #[doc = "GET /api/sub/outline/{}/{}/{} - SubscriptionsController"]
    pub async fn get_with_type(&self, short_uuid: String, encoded_tag: String, subscription_type: Option<String>) -> Result<String, crate::ApiError> {
        let subscription_type = subscription_type.unwrap_or_else(|| "ss".to_string());
        let url = format!("{}/api/sub/outline/{}/{}/{}", self.client.base_url(), short_uuid, subscription_type, encoded_tag);
        let response = api_request_common!(self, get, url, None::<()>)?;
        self.handle_response(response, url).await
    }

    #[deprecated(note = "Use get_with_type")]
    pub async fn get_subscription_with_type(&self, short_uuid: String, encoded_tag: String, subscription_type: Option<String>) -> Result<String, crate::ApiError> {
        self.get_with_type(short_uuid, encoded_tag, subscription_type).await
    }
}

api_get_with_query!(SubscriptionsController, get_all, "/api/subscriptions", GetAllSubscriptionsResponseDto, size: Option<usize>, start: Option<usize>);
api_get_with_path!(SubscriptionsController, get_by_username, "/api/subscriptions/by-username/{}", GetSubscriptionByUsernameResponseDto, username: String);

api_get_with_path!(SubscriptionsController, get_subscription_info_by_short_uuid, "/api/sub/{}/info", GetSubscriptionInfoResponseDto, deprecate: "Use get_info_by_short_uuid", short_uuid: String);
api_get_with_path_and_query!(SubscriptionsController, get_raw_subscription_by_short_uuid, "/api/sub/{}/raw", RawSubscriptionResponse, deprecate: "Use get_raw_by_short_uuid", path_params: [short_uuid: String], query_params: [with_disabled_hosts: Option<bool>]);
api_get_with_path!(SubscriptionsController, get_subscription, "/api/sub/{}", String, deprecate: "Use get", short_uuid: String);
api_get_with_path!(SubscriptionsController, get_subscription_by_client_type, "/api/sub/{}/{}", String, deprecate: "Use get_by_client_type", short_uuid: String, client_type: SubscriptionClientType);
api_get_with_query!(SubscriptionsController, get_all_subscriptions, "/api/subscriptions", GetAllSubscriptionsResponseDto, deprecate: "Use get_all", size: Option<usize>, start: Option<usize>);
api_get_with_path!(SubscriptionsController, get_subscription_by_username, "/api/subscriptions/by-username/{}", GetSubscriptionByUsernameResponseDto, deprecate: "Use get_by_username", username: String);

api_controller!(SubscriptionTemplateController);

api_get_with_path!(SubscriptionTemplateController, get, "/api/subscription-templates/{}", GetTemplateResponseDto, template_type: SubscriptionTemplateType);
api_patch!(SubscriptionTemplateController, update, "/api/subscription-templates", UpdateTemplateRequestDto, UpdateTemplateResponseDto);

api_get_with_path!(SubscriptionTemplateController, get_template, "/api/subscription-templates/{}", GetTemplateResponseDto, deprecate: "Use get", template_type: SubscriptionTemplateType);
api_patch!(SubscriptionTemplateController, update_template, "/api/subscription-templates", UpdateTemplateRequestDto, UpdateTemplateResponseDto, deprecate: "Use update");

api_controller!(SubscriptionSettingsController);

api_get!(SubscriptionSettingsController, get, "/api/subscription-settings", GetSubscriptionSettingsResponseDto);
api_patch!(SubscriptionSettingsController, update, "/api/subscription-settings", UpdateSubscriptionSettingsRequestDto, UpdateSubscriptionSettingsResponseDto);

api_get!(SubscriptionSettingsController, get_settings, "/api/subscription-settings", GetSubscriptionSettingsResponseDto, deprecate: "Use get");
api_patch!(SubscriptionSettingsController, update_settings, "/api/subscription-settings", UpdateSubscriptionSettingsRequestDto, UpdateSubscriptionSettingsResponseDto, deprecate: "Use update");
