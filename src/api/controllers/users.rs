use crate::api::controllers::macros::*;
use crate::api::types::*;
use uuid::Uuid;

api_controller!(UsersController);

api_post!(UsersController, create, "/api/users", CreateUserRequestDto, CreateUserResponseDto);
api_patch!(UsersController, update, "/api/users", UpdateUserRequestDto, UpdateUserResponseDto);
api_get_with_query!(UsersController, get_all, "/api/users", GetAllUsersResponseDto, size: Option<u32>, start: Option<u32>);
api_delete!(UsersController, delete, "/api/users/{uuid}", DeleteUserResponseDto, uuid: Uuid);
api_get_with_path!(UsersController, get_by_uuid, "/api/users/{}", GetUserByUuidResponseDto, uuid: Uuid);

api_get!(UsersController, get_all_tags, "/api/users/tags", GetAllTagsResponseDto);

api_get_with_path!(UsersController, get_accessible_nodes, "/api/users/{}/accessible-nodes", GetUserAccessibleNodesResponseDto, uuid: Uuid);
api_get_with_path!(UsersController, get_subscription_request_history, "/api/users/{}/subscription-request-history", GetUserSubscriptionRequestHistoryResponseDto, uuid: Uuid);
api_get_with_path!(UsersController, get_by_short_uuid, "/api/users/by-short-uuid/{}", GetUserByShortUuidResponseDto, short_uuid: String);
api_get_with_path!(UsersController, get_by_username, "/api/users/by-username/{}", GetUserByUsernameResponseDto, username: String);
api_get_with_path!(UsersController, get_by_telegram_id, "/api/users/by-telegram-id/{}", GetUserByTelegramIdResponseDto, telegram_id: String);
api_get_with_path!(UsersController, get_by_email, "/api/users/by-email/{}", GetUserByEmailResponseDto, email: String);
api_get_with_path!(UsersController, get_by_tag, "/api/users/by-tag/{}", GetUserByTagResponseDto, tag: String);

api_post_with_path!(UsersController, revoke_subscription, "/api/users/{}/actions/revoke", RevokeUserSubscriptionBodyDto, RevokeUserSubscriptionResponseDto, uuid: Uuid);
api_post_with_path_no_body!(UsersController, disable, "/api/users/{}/actions/disable", DisableUserResponseDto, uuid: Uuid);
api_post_with_path_no_body!(UsersController, enable, "/api/users/{}/actions/enable", EnableUserResponseDto, uuid: Uuid);
api_post_with_path_no_body!(UsersController, reset_traffic, "/api/users/{}/actions/reset-traffic", ResetUserTrafficResponseDto, uuid: Uuid);

api_post!(UsersController, bulk_delete_by_status, "/api/users/bulk/delete-by-status", BulkDeleteUsersByStatusRequestDto, BulkDeleteUsersByStatusResponseDto);
api_post!(UsersController, bulk_delete, "/api/users/bulk/delete", BulkDeleteUsersRequestDto, BulkDeleteUsersResponseDto);
api_post!(UsersController, bulk_revoke_subscription, "/api/users/bulk/revoke-subscription", BulkRevokeUsersSubscriptionRequestDto, BulkRevokeUsersSubscriptionResponseDto);
api_post!(UsersController, bulk_reset_traffic, "/api/users/bulk/reset-traffic", BulkResetTrafficUsersRequestDto, BulkResetTrafficUsersResponseDto);
api_post!(UsersController, bulk_update, "/api/users/bulk/update", BulkUpdateUsersRequestDto, BulkUpdateUsersResponseDto);
api_post!(UsersController, bulk_update_internal_squads, "/api/users/bulk/update-squads", BulkUpdateUsersSquadsRequestDto, BulkUpdateUsersSquadsResponseDto);
api_post!(UsersController, bulk_update_all, "/api/users/bulk/all/update", BulkAllUpdateUsersRequestDto, BulkAllUpdateUsersResponseDto);
api_post_no_body!(UsersController, bulk_all_reset_traffic, "/api/users/bulk/all/reset-traffic", BulkAllResetTrafficUsersResponseDto);

api_get_with_path_and_query!(
	UsersController,
	get_usage_by_range,
	"/api/users/stats/usage/{}/range",
	GetUserUsageByRangeResponseDto,
	path_params: [uuid: Uuid],
	query_params: [start: Option<String>, end: Option<String>]
);

api_post!(UsersController, create_user, "/api/users", CreateUserRequestDto, CreateUserResponseDto, deprecate: "Use create");
api_patch!(UsersController, update_user, "/api/users", UpdateUserRequestDto, UpdateUserResponseDto, deprecate: "Use update");
api_get_with_query!(UsersController, get_all_users, "/api/users", GetAllUsersResponseDto, deprecate: "Use get_all", size: Option<u32>, start: Option<u32>);
api_delete!(UsersController, delete_user, "/api/users/{uuid}", DeleteUserResponseDto, deprecate: "Use delete", uuid: Uuid);
api_get_with_path!(UsersController, get_user_by_uuid, "/api/users/{}", GetUserByUuidResponseDto, deprecate: "Use get_by_uuid", uuid: Uuid);
api_get!(UsersController, get_all_tags_deprecated, "/api/users/tags", GetAllTagsResponseDto, deprecate: "Use get_all_tags");
api_get_with_path!(UsersController, get_user_accessible_nodes, "/api/users/{}/accessible-nodes", GetUserAccessibleNodesResponseDto, deprecate: "Use get_accessible_nodes", uuid: Uuid);
api_get_with_path!(UsersController, get_user_subscription_request_history, "/api/users/{}/subscription-request-history", GetUserSubscriptionRequestHistoryResponseDto, deprecate: "Use get_subscription_request_history", uuid: Uuid);
api_get_with_path!(UsersController, get_user_by_short_uuid, "/api/users/by-short-uuid/{}", GetUserByShortUuidResponseDto, deprecate: "Use get_by_short_uuid", short_uuid: String);
api_get_with_path!(UsersController, get_user_by_username, "/api/users/by-username/{}", GetUserByUsernameResponseDto, deprecate: "Use get_by_username", username: String);
api_get_with_path!(UsersController, get_user_by_telegram_id, "/api/users/by-telegram-id/{}", GetUserByTelegramIdResponseDto, deprecate: "Use get_by_telegram_id", telegram_id: String);
api_get_with_path!(UsersController, get_users_by_email, "/api/users/by-email/{}", GetUserByEmailResponseDto, deprecate: "Use get_by_email", email: String);
api_get_with_path!(UsersController, get_users_by_tag, "/api/users/by-tag/{}", GetUserByTagResponseDto, deprecate: "Use get_by_tag", tag: String);
api_post_with_path!(UsersController, revoke_user_subscription, "/api/users/{}/actions/revoke", RevokeUserSubscriptionBodyDto, RevokeUserSubscriptionResponseDto, deprecate: "Use revoke_subscription", uuid: Uuid);
api_post_with_path_no_body!(UsersController, disable_user, "/api/users/{}/actions/disable", DisableUserResponseDto, deprecate: "Use disable", uuid: Uuid);
api_post_with_path_no_body!(UsersController, enable_user, "/api/users/{}/actions/enable", EnableUserResponseDto, deprecate: "Use enable", uuid: Uuid);
api_post_with_path_no_body!(UsersController, reset_user_traffic, "/api/users/{}/actions/reset-traffic", ResetUserTrafficResponseDto, deprecate: "Use reset_traffic", uuid: Uuid);
api_post!(UsersController, bulk_delete_users_by_status, "/api/users/bulk/delete-by-status", BulkDeleteUsersByStatusRequestDto, BulkDeleteUsersByStatusResponseDto, deprecate: "Use bulk_delete_by_status");
api_post!(UsersController, bulk_delete_users, "/api/users/bulk/delete", BulkDeleteUsersRequestDto, BulkDeleteUsersResponseDto, deprecate: "Use bulk_delete");
api_post!(UsersController, bulk_revoke_users_subscription, "/api/users/bulk/revoke-subscription", BulkRevokeUsersSubscriptionRequestDto, BulkRevokeUsersSubscriptionResponseDto, deprecate: "Use bulk_revoke_subscription");
api_post!(UsersController, bulk_reset_user_traffic, "/api/users/bulk/reset-traffic", BulkResetTrafficUsersRequestDto, BulkResetTrafficUsersResponseDto, deprecate: "Use bulk_reset_traffic");
api_post!(UsersController, bulk_update_users, "/api/users/bulk/update", BulkUpdateUsersRequestDto, BulkUpdateUsersResponseDto, deprecate: "Use bulk_update");
api_post!(UsersController, bulk_update_users_internal_squads, "/api/users/bulk/update-squads", BulkUpdateUsersSquadsRequestDto, BulkUpdateUsersSquadsResponseDto, deprecate: "Use bulk_update_internal_squads");
api_post!(UsersController, bulk_update_all_users, "/api/users/bulk/all/update", BulkAllUpdateUsersRequestDto, BulkAllUpdateUsersResponseDto, deprecate: "Use bulk_update_all");
api_post_no_body!(UsersController, bulk_all_reset_user_traffic, "/api/users/bulk/all/reset-traffic", BulkAllResetTrafficUsersResponseDto, deprecate: "Use bulk_all_reset_traffic");
api_get_with_path_and_query!(
	UsersController,
	get_user_usage_by_range,
	"/api/users/stats/usage/{}/range",
	GetUserUsageByRangeResponseDto,
	deprecate: "Use get_usage_by_range",
	path_params: [uuid: Uuid],
	query_params: [start: Option<String>, end: Option<String>]
);
