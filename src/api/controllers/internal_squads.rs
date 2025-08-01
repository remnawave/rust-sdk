use crate::api::controllers::macros::*;
use crate::api::types::internal_squads::*;
use uuid::Uuid;

api_controller!(InternalSquadsController);

api_get!(InternalSquadsController, get_internal_squads, "/api/internal-squads", GetInternalSquadsResponseDto);
api_get_with_path!(InternalSquadsController, get_internal_squad_by_uuid, "/api/internal-squads/{}", GetInternalSquadByUuidResponseDto,uuid: Uuid);
api_post!(InternalSquadsController, create_internal_squad, "/api/internal-squads", CreateInternalSquadRequestDto, CreateInternalSquadResponseDto);
api_patch!(InternalSquadsController, update_internal_squad, "/api/internal-squads", UpdateInternalSquadRequestDto, UpdateInternalSquadResponseDto);
api_delete!(InternalSquadsController, delete_internal_squad, "/api/internal-squads/{}", DeleteInternalSquadResponseDto, uuid: Uuid);

api_post_with_path_no_body!(InternalSquadsController, bulk_add_users_to_internal_squad, "/api/internal-squads/{}/bulk-actions/add-users", AddUsersToInternalSquadResponseDto, uuid: Uuid);
api_delete!(InternalSquadsController, bulk_remove_users_from_internal_squad, "/api/internal-squads/{}/bulk-actions/remove-users", RemoveUsersFromInternalSquadResponseDto, uuid: Uuid);
