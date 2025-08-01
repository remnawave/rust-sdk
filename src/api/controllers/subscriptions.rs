use crate::api::controllers::macros::*;
use crate::api::types::subscriptions::*;

api_controller!(SubscriptionsController);

api_get_with_path!(SubscriptionsController, get_subscription_info_by_short_uuid, "/api/sub/{}/info", GetSubscriptionInfoResponseDto, short_uuid: String);
api_get_with_path!(SubscriptionsController, get_raw_subscription_by_short_uuid, "/api/sub/{}/raw", GetRawSubscriptionByShortUuidResponseDto, short_uuid: String);

api_get_with_path!(SubscriptionsController, get_subscription, "/api/sub/{}", String, short_uuid: String);
api_get_with_path!(SubscriptionsController, get_subscription_by_client_type, "/api/sub/{}/{}", String, short_uuid: String, client_type: SubscriptionClientType);

impl SubscriptionsController {
    #[doc = "GET /api/sub/outline/{}/{}/{} - SubscriptionsController"]
    pub async fn get_subscription_with_type(&self, short_uuid: String, encoded_tag: String, subscription_type: Option<String>) -> Result<String, crate::ApiError> {
        let subscription_type = subscription_type.unwrap_or_else(|| "ss".to_string());
        let url = format!("{}/api/sub/outline/{}/{}/{}", self.client.base_url(), short_uuid, subscription_type, encoded_tag);
        let response = api_request_common!(self, get, url, None::<()>)?;
        self.handle_response(response, url).await
    }
}

api_get_with_query!(SubscriptionsController, get_all_subscriptions, "/api/subscriptions", GetAllSubscriptionsResponseDto, size: Option<usize>, start: Option<usize>);
api_get_with_path!(SubscriptionsController, get_subscription_by_username, "/api/subscriptions/by-username/{}", GetSubscriptionByUsernameResponseDto, username: String);

api_controller!(SubscriptionTemplateController);

api_get_with_path!(SubscriptionTemplateController, get_template, "/api/subscription-templates/{}", GetTemplateResponseDto, template_type: SubscriptionTemplateType);
api_patch!(SubscriptionTemplateController, update_template, "/api/subscription-templates", UpdateTemplateRequestDto, UpdateTemplateResponseDto);

api_controller!(SubscriptionSettingsController);

api_get!(SubscriptionSettingsController, get_settings, "/api/subscription-settings", GetSubscriptionSettingsResponseDto);
api_patch!(SubscriptionSettingsController, update_settings, "/api/subscription-settings", UpdateSubscriptionSettingsRequestDto, UpdateSubscriptionSettingsResponseDto);
