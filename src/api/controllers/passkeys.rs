use crate::api::controllers::macros::*;
use crate::api::types::passkeys::*;

api_controller!(PasskeysController);

api_get!(PasskeysController, get_registration_options, "/api/passkeys/registration/options", GetPasskeyRegistrationOptionsResponseDto);
api_post!(PasskeysController, verify_registration, "/api/passkeys/registration/verify", VerifyPasskeyRegistrationRequestDto, VerifyPasskeyRegistrationResponseDto);
api_get!(PasskeysController, get_all, "/api/passkeys", GetAllPasskeysResponseDto);
api_delete_with_body!(PasskeysController, delete, "/api/passkeys", DeletePasskeyRequestDto, DeletePasskeyResponseDto);
