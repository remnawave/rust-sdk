use crate::api::controllers::macros::*;
use crate::api::types::config_profiles::*;
use uuid::Uuid;

api_controller!(ConfigProfilesController);

api_get!(ConfigProfilesController, get_all, "/api/config-profiles", GetConfigProfilesResponseDto);
api_post!(ConfigProfilesController, create, "/api/config-profiles", CreateConfigProfileRequestDto, CreateConfigProfileResponseDto);
api_patch!(ConfigProfilesController, update, "/api/config-profiles", UpdateConfigProfileRequestDto, UpdateConfigProfileResponseDto);
api_get!(ConfigProfilesController, get_all_inbounds, "/api/config-profiles/inbounds", GetAllInboundsResponseDto);
api_get_with_path!(ConfigProfilesController, get_inbounds_by_profile_uuid, "/api/config-profiles/{}/inbounds", GetInboundsByProfileUuidResponseDto, uuid: Uuid);
api_get_with_path!(ConfigProfilesController, get_by_uuid, "/api/config-profiles/{}", GetConfigProfileByUuidResponseDto, uuid: Uuid);
api_get_with_path!(
    ConfigProfilesController,
    get_computed_config_by_uuid,
    "/api/config-profiles/{}/computed-config",
    GetComputedConfigProfileByUuidResponseDto,
    uuid: Uuid
);
api_delete!(ConfigProfilesController, delete_by_uuid, "/api/config-profiles/{}", DeleteConfigProfileResponseDto, uuid: Uuid);

api_get!(ConfigProfilesController, get_config_profiles, "/api/config-profiles", GetConfigProfilesResponseDto, deprecate: "Use get_all");
api_post!(ConfigProfilesController, create_config_profile, "/api/config-profiles", CreateConfigProfileRequestDto, CreateConfigProfileResponseDto, deprecate: "Use create");
api_patch!(ConfigProfilesController, update_config_profile, "/api/config-profiles", UpdateConfigProfileRequestDto, UpdateConfigProfileResponseDto, deprecate: "Use update");
api_get_with_path!(ConfigProfilesController, get_config_profile_by_uuid, "/api/config-profiles/{}", GetConfigProfileByUuidResponseDto, deprecate: "Use get_by_uuid", uuid: Uuid);
api_delete!(ConfigProfilesController, delete_config_profile_by_uuid, "/api/config-profiles/{}", DeleteConfigProfileResponseDto, deprecate: "Use delete_by_uuid", uuid: Uuid);
