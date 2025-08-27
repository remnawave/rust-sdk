use crate::api::controllers::macros::*;
use crate::api::types::hosts::*;
use uuid::Uuid;

api_controller!(HostsController);

api_post!(HostsController, create, "/api/hosts", CreateHostRequestDto, CreateHostResponseDto);
api_patch!(HostsController, update, "/api/hosts", UpdateHostRequestDto, UpdateHostResponseDto);
api_get!(HostsController, get_all, "/api/hosts", GetAllHostsResponseDto);
api_get_with_path!(HostsController, get, "/api/hosts/{}", GetOneHostResponseDto, uuid: Uuid);
api_delete!(HostsController, delete, "/api/hosts/{}", DeleteHostResponseDto, uuid: Uuid);

api_post!(HostsController, reorder, "/api/hosts/actions/reorder", ReorderHostRequestDto, ReorderHostResponseDto);

api_post!(HostsController, bulk_delete, "/api/hosts/bulk/delete", BulkDeleteHostsRequestDto, BulkDeleteHostsResponseDto);
api_post!(HostsController, bulk_disable, "/api/hosts/bulk/disable", BulkDisableHostsRequestDto, BulkDisableHostsResponseDto);
api_post!(HostsController, bulk_enable, "/api/hosts/bulk/enable", BulkEnableHostsRequestDto, BulkEnableHostsResponseDto);
api_post!(HostsController, bulk_set_inbound, "/api/hosts/bulk/set-inbound", SetInboundToManyHostsRequestDto, SetInboundToManyHostsResponseDto);
api_post!(HostsController, bulk_set_port, "/api/hosts/bulk/set-port", SetPortToManyHostsRequestDto, SetPortToManyHostsResponseDto);
api_get!(HostsController, get_all_tags, "/api/hosts/tags", GetAllHostTagsResponseDto);

api_post!(HostsController, create_host, "/api/hosts", CreateHostRequestDto, CreateHostResponseDto, deprecate: "Use create");
api_patch!(HostsController, update_host, "/api/hosts", UpdateHostRequestDto, UpdateHostResponseDto, deprecate: "Use update");
api_get!(HostsController, get_all_hosts, "/api/hosts", GetAllHostsResponseDto, deprecate: "Use get_all");
api_get_with_path!(HostsController, get_one_host, "/api/hosts/{}", GetOneHostResponseDto, deprecate: "Use get", uuid: Uuid);
api_delete!(HostsController, delete_host, "/api/hosts/{}", DeleteHostResponseDto, deprecate: "Use delete", uuid: Uuid);
api_post!(HostsController, reorder_hosts, "/api/hosts/actions/reorder", ReorderHostRequestDto, ReorderHostResponseDto, deprecate: "Use reorder");
api_post!(HostsController, bulk_delete_hosts, "/api/hosts/bulk/delete", BulkDeleteHostsRequestDto, BulkDeleteHostsResponseDto, deprecate: "Use bulk_delete");
api_post!(HostsController, bulk_disable_hosts, "/api/hosts/bulk/disable", BulkDisableHostsRequestDto, BulkDisableHostsResponseDto, deprecate: "Use bulk_disable");
api_post!(HostsController, bulk_enable_hosts, "/api/hosts/bulk/enable", BulkEnableHostsRequestDto, BulkEnableHostsResponseDto, deprecate: "Use bulk_enable");
api_post!(HostsController, bulk_set_inbound_to_hosts, "/api/hosts/bulk/set-inbound", SetInboundToManyHostsRequestDto, SetInboundToManyHostsResponseDto, deprecate: "Use bulk_set_inbound");
api_post!(HostsController, bulk_set_port_to_hosts, "/api/hosts/bulk/set-port", SetPortToManyHostsRequestDto, SetPortToManyHostsResponseDto, deprecate: "Use bulk_set_port");
api_get!(HostsController, get_all_host_tags, "/api/hosts/tags", GetAllHostTagsResponseDto, deprecate: "Use get_all_tags");
