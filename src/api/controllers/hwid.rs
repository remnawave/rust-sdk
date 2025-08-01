use uuid::Uuid;

use crate::api::controllers::macros::*;
use crate::api::types::hwid::*;

api_controller!(HwidUserDevicesController);

api_post!(HwidUserDevicesController, create_user_hwid_device, "/api/hwid/devices", CreateUserHwidDeviceRequestDto, CreateUserHwidDeviceResponseDto);
api_post!(HwidUserDevicesController, delete_user_hwid_device, "/api/hwid/devices/delete", DeleteUserHwidDeviceRequestDto, DeleteUserHwidDeviceResponseDto);
api_get_with_path!(HwidUserDevicesController, get_user_hwid_devices, "/api/hwid/devices/{}", GetUserHwidDevicesResponseDto, user_uuid: Uuid);
