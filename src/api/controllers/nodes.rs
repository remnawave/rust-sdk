use crate::api::controllers::macros::*;
use crate::api::types::nodes::*;
use uuid::Uuid;

api_controller!(NodesController);

api_post!(NodesController, create, "/api/nodes", CreateNodeRequestDto, CreateNodeResponseDto);
api_get!(NodesController, get_all, "/api/nodes", GetAllNodesResponseDto);
api_patch!(NodesController, update, "/api/nodes", UpdateNodeRequestDto, UpdateNodeResponseDto);
api_get_with_path!(NodesController, get, "/api/nodes/{}", GetOneNodeResponseDto, uuid: Uuid);
api_delete!(NodesController, delete, "/api/nodes/{}", DeleteNodeResponseDto, uuid: Uuid);

api_post_with_path_no_body!(NodesController, enable, "/api/nodes/{}/actions/enable", EnableNodeResponseDto, uuid: Uuid);
api_post_with_path_no_body!(NodesController, disable, "/api/nodes/{}/actions/disable", DisableNodeResponseDto, uuid: Uuid);
api_post_with_path_no_body!(NodesController, restart, "/api/nodes/{}/actions/restart", RestartNodeResponseDto, uuid: Uuid);
api_post_with_path_no_body!(NodesController, reset_traffic, "/api/nodes/{}/actions/reset-traffic", ResetNodeTrafficResponseDto, uuid: Uuid);
api_post!(NodesController, restart_all, "/api/nodes/actions/restart-all", RestartAllNodesRequestBodyDto, RestartAllNodesResponseDto);
api_post!(NodesController, reorder, "/api/nodes/actions/reorder", ReorderNodeRequestDto, ReorderNodeResponseDto);

api_post!(NodesController, create_node, "/api/nodes", CreateNodeRequestDto, CreateNodeResponseDto, deprecate: "Use create");
api_get!(NodesController, get_all_nodes, "/api/nodes", GetAllNodesResponseDto, deprecate: "Use get_all");
api_patch!(NodesController, update_node, "/api/nodes", UpdateNodeRequestDto, UpdateNodeResponseDto, deprecate: "Use update");
api_get_with_path!(NodesController, get_one_node, "/api/nodes/{}", GetOneNodeResponseDto, deprecate: "Use get", uuid: Uuid);
api_delete!(NodesController, delete_node, "/api/nodes/{}", DeleteNodeResponseDto, deprecate: "Use delete", uuid: Uuid);
api_post_with_path_no_body!(NodesController, enable_node, "/api/nodes/{}/actions/enable", EnableNodeResponseDto, deprecate: "Use enable", uuid: Uuid);
api_post_with_path_no_body!(NodesController, disable_node, "/api/nodes/{}/actions/disable", DisableNodeResponseDto, deprecate: "Use disable", uuid: Uuid);
api_post_with_path_no_body!(NodesController, restart_node, "/api/nodes/{}/actions/restart", RestartNodeResponseDto, deprecate: "Use restart", uuid: Uuid);
api_post_with_path_no_body!(NodesController, reset_node_traffic, "/api/nodes/{}/actions/reset-traffic", ResetNodeTrafficResponseDto, deprecate: "Use reset_traffic", uuid: Uuid);
api_post!(NodesController, restart_all_nodes, "/api/nodes/actions/restart-all", RestartAllNodesRequestBodyDto, RestartAllNodesResponseDto, deprecate: "Use restart_all");
api_post!(NodesController, reorder_nodes, "/api/nodes/actions/reorder", ReorderNodeRequestDto, ReorderNodeResponseDto, deprecate: "Use reorder");

api_controller!(NodesUsageController);

api_get_with_query!(NodesUsageController, get_usage_by_range, "/api/nodes/usage/range", GetNodesUsageByRangeResponseDto, start: Option<String>, end: Option<String>);
api_get_with_path_and_query!(NodesUsageController, get_user_usage, "/api/nodes/usage/{}/users/range", GetNodeUserUsageByRangeResponseDto, path_params: [uuid: String], query_params: [start: Option<String>, end: Option<String>]);
api_get!(NodesUsageController, get_realtime_usage, "/api/nodes/usage/realtime", GetNodesRealtimeUsageResponseDto);

api_get_with_query!(NodesUsageController, get_nodes_usage_by_range, "/api/nodes/usage/range", GetNodesUsageByRangeResponseDto, deprecate: "Use get_usage_by_range", start: Option<String>, end: Option<String>);
api_get_with_path_and_query!(NodesUsageController, get_node_user_usage, "/api/nodes/usage/{}/users/range", GetNodeUserUsageByRangeResponseDto, deprecate: "Use get_user_usage", path_params: [uuid: String], query_params: [start: Option<String>, end: Option<String>]);
api_get!(NodesUsageController, get_nodes_realtime_usage, "/api/nodes/usage/realtime", GetNodesRealtimeUsageResponseDto, deprecate: "Use get_realtime_usage");
