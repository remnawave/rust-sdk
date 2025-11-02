use crate::api::controllers::macros::*;
use crate::api::types::snippets::*;

api_controller!(SnippetsController);

api_get!(SnippetsController, get_all, "/api/snippets", GetSnippetsResponseDto);
api_post!(SnippetsController, create, "/api/snippets", CreateSnippetRequestDto, CreateSnippetResponseDto);
api_patch!(SnippetsController, update, "/api/snippets", UpdateSnippetRequestDto, UpdateSnippetResponseDto);
api_delete_with_body!(SnippetsController, delete, "/api/snippets", DeleteSnippetRequestDto, DeleteSnippetResponseDto);
