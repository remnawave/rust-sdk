use crate::api::controllers::macros::*;
use crate::api::types::external_squads::*;
use uuid::Uuid;

api_controller!(ExternalSquadsController);

api_get!(ExternalSquadsController, get_all, "/api/external-squads", GetExternalSquadsResponseDto);
api_get_with_path!(ExternalSquadsController, get_by_uuid, "/api/external-squads/{}", GetExternalSquadByUuidResponseDto, uuid: Uuid);
api_post!(ExternalSquadsController, create, "/api/external-squads", CreateExternalSquadRequestDto, CreateExternalSquadResponseDto);
api_patch!(ExternalSquadsController, update, "/api/external-squads", UpdateExternalSquadRequestDto, UpdateExternalSquadResponseDto);
api_delete!(ExternalSquadsController, delete, "/api/external-squads/{}", DeleteExternalSquadResponseDto, uuid: Uuid);

api_post_with_path_no_body!(ExternalSquadsController, add_users_to_external_squad, "/api/external-squads/{}/bulk-actions/add-users", AddUsersToExternalSquadResponseDto, uuid: Uuid);
api_delete!(ExternalSquadsController, remove_users_from_external_squad, "/api/external-squads/{}/bulk-actions/remove-users", RemoveUsersFromExternalSquadResponseDto, uuid: Uuid);
