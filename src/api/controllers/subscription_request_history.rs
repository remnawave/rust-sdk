use crate::api::controllers::macros::*;
use crate::api::types::subscription_request_history::*;

api_controller!(SubscriptionRequestHistoryController);

api_get_with_query!(
    SubscriptionRequestHistoryController,
    get_all,
    "/api/subscription-request-history",
    GetSubscriptionRequestHistoryResponseDto,
    size: Option<usize>,
    start: Option<usize>
);
api_get!(SubscriptionRequestHistoryController, get_stats, "/api/subscription-request-history/stats", GetSubscriptionRequestHistoryStatsResponseDto);
