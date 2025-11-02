# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [2.2.4] - 2025-11-02

Compatibility:
- Contract/SDK 2.2.4 targets Remnawave API v2.2.4

### Added
- New controllers and endpoints:
  - External Squads: CRUD and bulk operations
    - GET `/api/external-squads`
    - GET `/api/external-squads/{uuid}`
    - POST `/api/external-squads`
    - PATCH `/api/external-squads`
    - DELETE `/api/external-squads/{uuid}`
    - POST `/api/external-squads/{uuid}/bulk-actions/add-users`
    - DELETE `/api/external-squads/{uuid}/bulk-actions/remove-users`
  - Passkeys: registration/authn flow and management
    - GET `/api/passkeys/registration/options`
    - POST `/api/passkeys/registration/verify`
    - GET `/api/passkeys`
    - DELETE `/api/passkeys`
  - Remnawave Settings: read/update server-wide auth/branding settings
    - GET `/api/remnawave-settings`
    - PATCH `/api/remnawave-settings`
  - Snippets: simple JSON snippets CRUD
    - GET `/api/snippets`
    - POST `/api/snippets`
    - PATCH `/api/snippets`
    - DELETE `/api/snippets`
  - Subscription Request History (global):
    - GET `/api/subscription-request-history?size=&start=`
    - GET `/api/subscription-request-history/stats`
  - Users: subscription request history by user
    - GET `/api/users/{uuid}/subscription-request-history`
- System tools and testers:
  - POST `/api/system/tools/happ/encrypt` — encrypt Happ crypto links
  - POST `/api/system/testers/srr-matcher` — debug subscription response rules matcher
- Subscriptions (plaintext responses):
  - GET `/api/sub/{short}` returns plaintext subscription
  - GET `/api/sub/{short}/{clientType}` returns plaintext subscription for a specific client
  - GET `/api/sub/{short}/info` returns structured subscription info
  - GET `/api/subscriptions/by-short-uuid/{short}/raw` returns rich raw subscription payload
- Integration tests:
  - Rewrote end-to-end integration test to exercise all GET endpoints with tolerant logging (401/403 do not fail the run); uses env-driven base URL and tokens.
- Types for new features:
  - `external_squads` DTOs for squads CRUD and bulk actions
  - `passkeys` DTOs for registration/verification and listing
  - `remnawave_settings` DTOs for auth providers and branding settings
  - `snippets` DTOs for snippets CRUD
  - `subscription_request_history` DTOs for records and stats

### Changed
- Authentication types:
  - `GetStatusResponseData` restructured: adds `authentication`, `branding`; `tg_auth` and `oauth2` are optional; provider map for OAuth2.
  - `OAuth2AuthorizeResponseData.authorization_url` is now required (`String`).
- Billing types:
  - `UpdateInfraBillingNodeRequestDto` now supports bulk via `uuids: Vec<Uuid>` (was a single `uuid`).
  - Monetary fields use `f64` (`amount`, `total_amount`, `current_month_payments`, `total_spent`).
- Users types:
  - Broad nullable support: many string-like fields use `Option<Option<T>>` for “clear” semantics in create/update/bulk DTOs.
  - Counters widened: several traffic byte counters now `i64`.
- Subscriptions:
  - Plaintext subscription endpoints now return `String`; SDK routes use a text-response handler.
  - Added `/api/sub/{short}/info` and `/api/subscriptions/by-short-uuid/{short}/raw` responses.
- Nodes/Usage and Users/Usage:
  - Range endpoints accept optional `start`/`end` query parameters as RFC3339 DateTime strings (seconds precision, Z).
- System types:
  - `SystemStatsData.uptime` and `timestamp` changed to `f64`.
  - `MemoryStats` values changed to `f64`.
- Config Profiles:
  - Fixed inbounds path to `/api/config-profiles/{uuid}/inbounds`.
- Macros/Controllers:
  - Controller macro gained `handle_text_response` to support plaintext endpoints.
  - Deprecated several old method aliases to standardize naming (e.g., `get_*` vs older variants).

### Breaking Changes
- billing: `UpdateInfraBillingNodeRequestDto.uuid` renamed to `uuids` and type changed from `Uuid` to `Vec<Uuid>`.
- Public DTOs adjusted for floats (`f64`) where server returns non-integer values; some derives dropped from `Eq` as a result.
