# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [2.1.7] - 2025-08-31

Compatibility:
- Contract/SDK 2.1.7 targets Remnawave API v217

### Added
- subscriptions/controllers: Implement endpoints from API v217:
  - GET `/api/subscriptions/by-short-uuid/{shortUuid}`
  - GET `/api/subscriptions/by-uuid/{uuid}`
- subscriptions/types: Introduced `BasicSubscriptionUser` used by `GetAllSubscriptionsResponseDto`.

### Changed
- subscriptions/types: Response DTOs updated to match API v217:
  - `GetSubscriptionByUsernameResponseDto.response.happ` added
  - `GetSubscriptionByShortUuidResponseDto.response.happ` added
  - `GetSubscriptionByUuidResponseDto.response.happ` added
  - `GetAllSubscriptionsResponseDto.response.subscriptions[].user` now uses simplified `BasicSubscriptionUser`
