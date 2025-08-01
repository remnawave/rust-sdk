use crate::api::controllers::macros::*;
use crate::api::types::tokens::*;
use uuid::Uuid;

api_controller!(ApiTokensController);

api_post!(ApiTokensController, create, "/api/tokens", CreateApiTokenRequestDto, CreateApiTokenResponseDto);
api_get!(ApiTokensController, find_all, "/api/tokens", FindAllApiTokensResponseDto);
api_delete!(ApiTokensController, delete, "/api/tokens/{}", DeleteApiTokenResponseDto, uuid: Uuid);
