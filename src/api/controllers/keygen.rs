use crate::api::controllers::macros::*;
use crate::api::types::keygen::GetPubKeyResponseDto;

api_controller!(KeygenController);

api_get!(KeygenController, generate_key, "/api/keygen", GetPubKeyResponseDto);
