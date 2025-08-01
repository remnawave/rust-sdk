use crate::api::controllers::macros::*;
use crate::api::types::config_profiles::*;
use uuid::Uuid;

api_controller!(ConfigProfilesController);

api_get!(ConfigProfilesController, get_config_profiles, "/api/config-profiles", GetConfigProfilesResponseDto);
api_post!(ConfigProfilesController, create_config_profile, "/api/config-profiles", CreateConfigProfileRequestDto, CreateConfigProfileResponseDto);
api_patch!(ConfigProfilesController, update_config_profile, "/api/config-profiles", UpdateConfigProfileRequestDto, UpdateConfigProfileResponseDto);
api_get!(ConfigProfilesController, get_all_inbounds, "/api/config-profiles/inbounds", GetAllInboundsResponseDto);
api_get_with_path!(ConfigProfilesController, get_inbounds_by_profile_uuid, "/api/config-profiles/{}", GetInboundsByProfileUuidResponseDto, uuid: Uuid);
api_get_with_path!(ConfigProfilesController, get_config_profile_by_uuid, "/api/config-profiles/{}", GetConfigProfileByUuidResponseDto, uuid: Uuid);
api_delete!(ConfigProfilesController, delete_config_profile_by_uuid, "/api/config-profiles/{}", DeleteConfigProfileResponseDto, uuid: Uuid);
