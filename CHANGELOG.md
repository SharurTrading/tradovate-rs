<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

All notable changes to `tradovate-client` are recorded here. The project uses
[Semantic Versioning](https://semver.org/) for releases and follows the
structure of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Establish repository governance, security, and contribution boundaries.
- Define stable `TV-*` architecture, exact-value, authentication, mutation,
  realtime, validation, and maintainability rules.
- Record the initial single-crate capability-oriented architecture decision.
- Add deterministic quality, dependency, license, advisory, file-size, and
  full-history secret gates.
- Add exact-decimal JSON boundaries and validated provider identity types.
- Add redacted direct authentication, expiry/revision-fenced single-flight renewal,
  and client/request-bound delayed penalty-ticket retry.
- Add bounded REST execution, shared rate admission, typed business failures, and
  the complete current Partner REST contract inventory and model surface.
- Enforce the current Partner all-request and failed-only endpoint quotas with
  cancellation-safe reservations, aggregate plus account-scoped demo-balance
  admission, global one-hour 429 stops, and exact penalty-ticket cooldowns.
- Pin the official Partner OpenAPI snapshot retrieved 2026-08-21 at SHA-256
  `37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769`.
- Generate and check in all 278 current component schemas and an exhaustive
  350-operation manifest grouped into capability modules: 271 queries, 73 mutations,
  and 6 lifecycle operations.
- Publish 334 callable typed operations: 333 direct REST methods plus
  `/user/syncrequest` owned by the realtime lifecycle, through 263 generated query
  methods and 71 reviewed specialized methods; leave no operation modeled-only.
- Record the remaining 16 operations as explicit documentation-blocked rows instead
  of guessing request, response, or completion contracts. The pinned responses are
  338 typed, 11 unspecified, and 1 incomplete.
- Keep lifecycle-sensitive and state-changing calls behind dedicated reviewed
  implementations instead of treating schema generation as authority to transmit.
- Preserve every unspecified or incomplete response as a non-raw documentation
  boundary with its operation-specific provider blocker.
- Bound generated request encoding and repeated-key query construction, validate
  public identities, decode financial numbers directly to `Decimal`, redact generated
  secret fields, and preserve unknown response enum values without allowing them to
  be transmitted.
- Add deterministic `tools/generate_openapi.py --check` validation; generation is
  hash-fenced to the reviewed current snapshot and generated Rust is never hand-edited.
- Fail closed after an admitted mutation is cancelled or receives an uncertain
  result; client clones reject further mutations until callers acknowledge completed
  provider-state reconciliation.
- Add production-public single-generation realtime authorization, exact
  SockJS-derived framing, heartbeat, request correlation, configurable validated
  unsplit user synchronization, and overflow-driven resynchronization state.
- Add public typed user bootstrap and `props` deltas for all 31 pinned `SyncMessage`
  collections plus the documented `OtherEnvAdminAlertSignal`, reusing current REST
  entity models with required-bootstrap readiness checks.
- Add public quote, DOM, and histogram subscriptions by `Symbol` or `ContractId`,
  exact-decimal payloads, validated chart request/cancellation, regular bars, and
  checked compact-tick reconstruction.
- Bound decoded messages per frame and aggregate queue memory; preflight requests
  before enqueue; stage bootstrap-era deltas behind the snapshot; and require
  resynchronization after every unexpected active-connection loss.
- Poison a realtime generation when an admitted request future is abandoned, reject
  responses observed at or after their stored deadline, and fence every writer send
  with the next monotonic heartbeat deadline.

### Changed

- Relicense repository-authored source and documentation under the MIT No
  Attribution License (`MIT-0`), replace proprietary notices with SPDX headers,
  and prepare the GitHub repository for public visibility. Third-party contract
  material retains its own terms.
- Match the current Partner `AccessTokenRequest` contract: only `name` and
  `password` are required, while `hibpCheck`, `appId`, `appVersion`, `cid`, `sec`,
  and `deviceId` are validated when supplied and otherwise omitted from JSON;
  accept the documented 512-character password ceiling.
- Redact application, client, secret, device, and HIBP credential metadata from
  both builder and built-credential debug output.
- Make CI's strict Rust policy explicit with all-target checking, formatting,
  `clippy::all`, `clippy::pedantic`, unsafe-code forbiddance, and an HTTP
  proxy-isolation invariant alongside the existing test and supply-chain gates.
- Render README examples as directly copyable Rust without visible rustdoc-only
  hidden-line markers.

### Known limitations

- Realtime connections do not automatically reconnect, retain/replay subscriptions,
  or acknowledge recovery fences; callers own replacement generations and
  snapshot-before-delta recovery.
- User synchronization uses validated unsplit profiles, explicitly requests all 31
  current entity families by default, and does not automatically retry provider
  penalties. B2B split-response completion is documentation blocked.
- Replay socket lifecycle is available, but the current Partner documentation does
  not publish replay control or `clock` payload contracts, so those typed capabilities
  remain withheld.
- Sixteen current REST operations remain documentation blocked. No invented fields,
  success evidence, or public raw JSON escape hatch is exposed.
- Live probes are tracked independently from deterministic contract coverage and must
  be invoked deliberately.

### Security

- Require redacted secret ownership, encrypted remote transport, bounded remote
  input, and explicit ambiguous outcomes for money-moving operations.
- Add a CI-proven inactive-feature guard for the lockfile-only
  `RUSTSEC-2026-0235` exception; vulnerable `rkyv` 0.7 must never enter the compiled
  dependency graph.

No release has been cut yet.
