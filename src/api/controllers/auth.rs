use crate::api::controllers::macros::*;
use crate::api::types::*;

api_controller!(AuthController);

api_post!(AuthController, login, "/api/auth/login", LoginRequestDto, LoginResponseDto);
api_post!(AuthController, register, "/api/auth/register", RegisterRequestDto, RegisterResponseDto);
api_get!(AuthController, get_status, "/api/auth/status", GetStatusResponseDto);
api_post!(AuthController, telegram_callback, "/api/auth/oauth2/tg/callback", TelegramCallbackRequestDto, TelegramCallbackResponseDto);
api_post!(AuthController, oauth2_authorize, "/api/auth/oauth2/authorize", OAuth2AuthorizeRequestDto, OAuth2AuthorizeResponseDto);
api_post!(AuthController, oauth2_callback, "/api/auth/oauth2/callback", OAuth2CallbackRequestDto, OAuth2CallbackResponseDto);
