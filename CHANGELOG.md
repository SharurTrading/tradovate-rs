<!--
Copyright (C) 2026 Kevin Monaghan. All rights reserved.

This file is proprietary and confidential.
Unauthorized copying, use, modification, distribution, or disclosure of this file,
via any medium, is strictly prohibited except under a written agreement with the
copyright owner.
-->

# Changelog

All notable internal changes to `tradovate-client` are recorded here. The project
uses [Semantic Versioning](https://semver.org/) for private releases and follows the
structure of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Establish the private proprietary repository governance and security boundary.
- Define stable `TV-*` architecture, exact-value, authentication, mutation,
  realtime, validation, and maintainability rules.
- Record the initial single-crate capability-oriented architecture decision.
- Add deterministic quality, dependency, license, advisory, file-size, and
  full-history secret gates.
- Add exact-decimal JSON boundaries and validated provider identity types.
- Add redacted direct authentication, expiry/revision-fenced single-flight renewal,
  and client/request-bound delayed penalty-ticket retry.
- Add bounded REST execution, shared rate admission, typed business failures, and
  account, contract, position, and order capabilities.
- Fail closed after an admitted mutation is cancelled or receives an uncertain
  result; client clones reject further mutations until callers acknowledge completed
  provider-state reconciliation.
- Add single-generation realtime authorization, exact SockJS-derived framing,
  heartbeat, request correlation, fixed user synchronization, typed market-data
  command encoding, and overflow-driven resynchronization state in a test-only
  architecture harness pending typed production event models.
- Bound decoded messages per frame and aggregate queue memory; preflight requests
  before enqueue; stage bootstrap-era deltas behind the snapshot; and require
  resynchronization after every unexpected active-connection loss.
- Poison a realtime generation when an admitted request future is abandoned, reject
  responses observed at or after their stored deadline, and fence every writer send
  with the next monotonic heartbeat deadline.

### Known limitations

- The realtime harness is not part of production builds and does not automatically
  reconnect, retain/replay subscriptions, or acknowledge recovery fences.
- User synchronization uses one fixed entity profile with unsplit responses and
  does not automatically retry provider penalties.
- Remaining REST endpoints, typed realtime event payloads, charts, replay commands,
  and live probes are not yet implemented.

### Security

- Require redacted secret ownership, encrypted remote transport, bounded remote
  input, and explicit ambiguous outcomes for money-moving operations.
- Add a CI-proven inactive-feature guard for the lockfile-only
  `RUSTSEC-2026-0235` exception; vulnerable `rkyv` 0.7 must never enter the compiled
  dependency graph.

No release has been cut yet.
