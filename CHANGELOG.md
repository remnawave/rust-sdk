# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [2.1.11] - 2025-09-10

Compatibility:
- Contract/SDK 2.1.11 targets Remnawave API v2111

### Changed
- Updated API compatibility from v2.1.8 to v2.1.11
- OpenAPI specification updated from 3.0.0 to 3.1.1
- Schema format changes for nullable types (internal - no changes for sdk)

## [2.1.8] - 2025-09-04

Compatibility:
- Contract/SDK 2.1.8 targets Remnawave API v218

### Added
- system/controllers: Implement new X25519 keypair generation endpoint:
  - GET `/api/system/tools/x25519/generate`
- system/types: Introduced X25519 keypair generation types:
  - `GenerateX25519ResponseDto`
  - `GenerateX25519Data`
  - `X25519Keypair`
- subscriptions/types: New comprehensive raw subscription data types:
  - `RawSubscriptionUser` - Full user details with all fields
  - `ConvertedUserInfo` - Backward-compatible display values
- users/types: Added HWID device limit support:
  - `hwid_device_limit: Option<i32>` field in `BulkUpdateFields`
  - `hwid_device_limit: Option<i32>` field in `BulkAllUpdateUsersRequestDto`

### Changed
- subscriptions/controllers: Moved raw subscription endpoint for better organization:
  - `/api/sub/{shortUuid}/raw` → `/api/subscriptions/by-short-uuid/{shortUuid}/raw`
- subscriptions/types: Complete redesign of `GetRawSubscriptionByShortUuidResponseDto`:
  - `RawSubscriptionResponse` now includes `user`, `converted_user_info`, `headers`, `raw_hosts`
  - `HostPasswords` structure made required (no longer optional)
  - Enhanced user information with full profile data, internal squads, and connection history
- API organization: Clear separation between public and protected subscription endpoints