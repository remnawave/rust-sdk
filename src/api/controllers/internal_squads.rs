use crate::api::controllers::macros::*;
use crate::api::types::internal_squads::*;
use uuid::Uuid;

api_controller!(InternalSquadsController);

api_get!(InternalSquadsController, get_all, "/api/internal-squads", GetInternalSquadsResponseDto);
api_get_with_path!(InternalSquadsController, get_by_uuid, "/api/internal-squads/{}", GetInternalSquadByUuidResponseDto,uuid: Uuid);
api_post!(InternalSquadsController, create, "/api/internal-squads", CreateInternalSquadRequestDto, CreateInternalSquadResponseDto);
api_patch!(InternalSquadsController, update, "/api/internal-squads", UpdateInternalSquadRequestDto, UpdateInternalSquadResponseDto);
api_delete!(InternalSquadsController, delete, "/api/internal-squads/{}", DeleteInternalSquadResponseDto, uuid: Uuid);

api_post_with_path_no_body!(InternalSquadsController, add_users_to_internal_squad, "/api/internal-squads/{}/bulk-actions/add-users", AddUsersToInternalSquadResponseDto, uuid: Uuid);
api_delete!(InternalSquadsController, remove_users_from_internal_squad, "/api/internal-squads/{}/bulk-actions/remove-users", RemoveUsersFromInternalSquadResponseDto, uuid: Uuid);
api_get_with_path!(InternalSquadsController, get_accessible_nodes, "/api/internal-squads/{}/accessible-nodes", GetInternalSquadAccessibleNodesResponseDto, uuid: Uuid);

api_post_with_path_no_body!(InternalSquadsController, bulk_add_users_to_internal_squad, "/api/internal-squads/{}/bulk-actions/add-users", AddUsersToInternalSquadResponseDto, deprecate: "Use add_users_to_internal_squad", uuid: Uuid);
api_post!(InternalSquadsController, create_internal_squad, "/api/internal-squads", CreateInternalSquadRequestDto, CreateInternalSquadResponseDto, deprecate: "Use create");
api_patch!(InternalSquadsController, update_internal_squad, "/api/internal-squads", UpdateInternalSquadRequestDto, UpdateInternalSquadResponseDto, deprecate: "Use update");
api_delete!(InternalSquadsController, delete_internal_squad, "/api/internal-squads/{}", DeleteInternalSquadResponseDto, deprecate: "Use delete", uuid: Uuid);
api_get_with_path!(InternalSquadsController, get_internal_squad_by_uuid, "/api/internal-squads/{}", GetInternalSquadByUuidResponseDto, deprecate: "Use get_by_uuid", uuid: Uuid);
api_get!(InternalSquadsController, get_internal_squads, "/api/internal-squads", GetInternalSquadsResponseDto, deprecate: "Use get_all");
