# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [2.1.5] - 2025-08-27

Compatibility:
- Contract/SDK 2.1.5 requires Remnawave Panel >= 2.1.4

### Added
- nodes: Include `nodeCountryCode` in nodes usage/statistics DTOs. ([6df3614])
- hosts: API to retrieve all host tags. ([7160a23])
- hosts: New host configuration fields in DTOs (muxParams, sockoptParams, tag, isHidden, overrideSniFromAddress, vlessRouteId). ([a20d08d])
- subscriptions: Enhanced subscription DTOs with extended host and traffic data. ([a18a834])
- core: Deprecation support added to API macros. ([e9bcce4])

### Changed
- api-controllers: Standardized API client method names; older names marked deprecated. ([d25e13b])
- internal-squads: Simplified method names and updated API for user removal. ([ab3dc5b])
- nodes: "Restart all nodes" endpoint updated to POST with request body. ([bf3cd16])
- build/deps: Reuse features from reqwest in this crate. ([9940951], [1cd0b93])
- subscriptions: Updated raw subscription response DTO and structure. ([51d72fc])

### Tests
- Updated integration tests for API refactoring and new subscription data. ([1dda94d])

### Chore
- Bump package version to 2.1.5. ([c0ab958])
- Bump dependencies in Cargo.toml. ([3bfb4f9])
- Add '/versions' to .gitignore. ([4c34703])

### Merges
- Merge pull request "Reuse features from reqwest". ([9940951])
- Merge remote-tracking branch 'origin/main'. ([1cd0b93])

[2.1.5]: https://github.com/remnawave/rust-sdk/releases/tag/v2.1.5

[51d72fc]: https://github.com/remnawave/rust-sdk/commit/51d72fc8804294f7ec666871366d3ce6684283b4
[1cd0b93]: https://github.com/remnawave/rust-sdk/commit/1cd0b932ff23ce66e0bee866c5277adce12b1a30
[c0ab958]: https://github.com/remnawave/rust-sdk/commit/c0ab958a0626b86652ecf289cb2a12ca22553224
[1dda94d]: https://github.com/remnawave/rust-sdk/commit/1dda94dfe083d6bd4a95ae7ec822788fd32ea466
[a18a834]: https://github.com/remnawave/rust-sdk/commit/a18a834162d9570fd3581d39e4ce21e54c725d1b
[d25e13b]: https://github.com/remnawave/rust-sdk/commit/d25e13b0e92fa7539cc0dbeee5c4701c8d5940b4
[ab3dc5b]: https://github.com/remnawave/rust-sdk/commit/ab3dc5b55bcca0616357c66fd59c3bf7a31bee4a
[e9bcce4]: https://github.com/remnawave/rust-sdk/commit/e9bcce491b42b411e540090422c3bda65f21fd6f
[61cadca]: https://github.com/remnawave/rust-sdk/commit/61cadca6197e230ec6ed7516703a40994c939741
[6df3614]: https://github.com/remnawave/rust-sdk/commit/6df36145413704654be8e67db29100bd8f8b1ba4
[bf3cd16]: https://github.com/remnawave/rust-sdk/commit/bf3cd16281de42358b5de12bc45b3fc235594a04
[561fc66]: https://github.com/remnawave/rust-sdk/commit/561fc66ef5f39964609193a72c554072c680644b
[7160a23]: https://github.com/remnawave/rust-sdk/commit/7160a23ad70182f791daf01ee8aefd14417754e1
[a20d08d]: https://github.com/remnawave/rust-sdk/commit/a20d08dfe4524996109488847f2be0c2c9197279
[4c34703]: https://github.com/remnawave/rust-sdk/commit/4c347034cfb35151d3be53e9303ab55003a257c8
[3bfb4f9]: https://github.com/remnawave/rust-sdk/commit/3bfb4f92db46e9c64ee352218553f1a55a728ade
[9940951]: https://github.com/remnawave/rust-sdk/commit/9940951e7a30c0ee5fa098103e3ed1a7fe113175
