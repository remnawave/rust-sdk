use crate::api::controllers::macros::*;
use crate::api::types::hosts::*;
use uuid::Uuid;

api_controller!(HostsController);

api_post!(HostsController, create_host, "/api/hosts", CreateHostRequestDto, CreateHostResponseDto);
api_patch!(HostsController, update_host, "/api/hosts", UpdateHostRequestDto, UpdateHostResponseDto);
api_get!(HostsController, get_all_hosts, "/api/hosts", GetAllHostsResponseDto);
api_get_with_path!(HostsController, get_one_host, "/api/hosts/{}", GetOneHostResponseDto, uuid: Uuid);
api_delete!(HostsController, delete_host, "/api/hosts/{}", DeleteHostResponseDto, uuid: Uuid);

api_post!(HostsController, reorder_hosts, "/api/hosts/actions/reorder", ReorderHostRequestDto, ReorderHostResponseDto);

api_post!(HostsController, bulk_delete_hosts, "/api/hosts/bulk/delete", BulkDeleteHostsRequestDto, BulkDeleteHostsResponseDto);
api_post!(HostsController, bulk_disable_hosts, "/api/hosts/bulk/disable", BulkDisableHostsRequestDto, BulkDisableHostsResponseDto);
api_post!(HostsController, bulk_enable_hosts, "/api/hosts/bulk/enable", BulkEnableHostsRequestDto, BulkEnableHostsResponseDto);
api_post!(HostsController, bulk_set_inbound_to_hosts, "/api/hosts/bulk/set-inbound", SetInboundToManyHostsRequestDto, SetInboundToManyHostsResponseDto);
api_post!(HostsController, bulk_set_port_to_hosts, "/api/hosts/bulk/set-port", SetPortToManyHostsRequestDto, SetPortToManyHostsResponseDto);
