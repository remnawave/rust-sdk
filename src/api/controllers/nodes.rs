use crate::api::controllers::macros::*;
use crate::api::types::nodes::*;
use uuid::Uuid;

api_controller!(NodesController);

api_post!(NodesController, create_node, "/api/nodes", CreateNodeRequestDto, CreateNodeResponseDto);
api_get!(NodesController, get_all_nodes, "/api/nodes", GetAllNodesResponseDto);
api_patch!(NodesController, update_node, "/api/nodes", UpdateNodeRequestDto, UpdateNodeResponseDto);
api_get_with_path!(NodesController, get_one_node, "/api/nodes/{}", GetOneNodeResponseDto, uuid: Uuid);
api_delete!(NodesController, delete_node, "/api/nodes/{}", DeleteNodeResponseDto, uuid: Uuid);

api_post_with_path_no_body!(NodesController, enable_node, "/api/nodes/{}/actions/enable", EnableNodeResponseDto, uuid: Uuid);
api_post_with_path_no_body!(NodesController, disable_node, "/api/nodes/{}/actions/disable", DisableNodeResponseDto, uuid: Uuid);
api_post_with_path_no_body!(NodesController, restart_node, "/api/nodes/{}/actions/restart", RestartNodeResponseDto, uuid: Uuid);
api_post_no_body!(NodesController, restart_all_nodes, "/api/nodes/actions/restart-all", RestartAllNodesResponseDto);
api_post!(NodesController, reorder_nodes, "/api/nodes/actions/reorder", ReorderNodeRequestDto, ReorderNodeResponseDto);

api_controller!(NodesUsageController);

api_get_with_query!(NodesUsageController, get_nodes_usage_by_range, "/api/nodes/usage/range", GetNodesUsageByRangeResponseDto, start: Option<String>, end: Option<String>);
api_get_with_path_and_query!(NodesUsageController, get_node_user_usage, "/api/nodes/usage/{}/users/range", GetNodeUserUsageByRangeResponseDto, path_params: [uuid: String], query_params: [start: Option<String>, end: Option<String>]);
api_get!(NodesUsageController, get_nodes_realtime_usage, "/api/nodes/usage/realtime", GetNodesRealtimeUsageResponseDto);
