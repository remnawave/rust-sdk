use crate::api::controllers::macros::*;
use crate::api::types::remnawave_settings::*;

api_controller!(RemnawaveSettingsController);

api_get!(RemnawaveSettingsController, get, "/api/remnawave-settings", GetRemnawaveSettingsResponseDto);
api_patch!(RemnawaveSettingsController, update, "/api/remnawave-settings", UpdateRemnawaveSettingsRequestDto, UpdateRemnawaveSettingsResponseDto);
