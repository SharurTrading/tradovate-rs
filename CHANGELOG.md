<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

This project is pre-release. No release entries have been published.

## [Unreleased] — 0.2.0

### Breaking recovery changes

- Preserve healthy realtime sockets across request timeout/cancellation, malformed
  application records, event overflow, ordinary silence, and subscription refusal.
- Deliver the accepted event prefix, a retained `ContinuityGap`, and resumed data
  after exact-marker acknowledgement. Retain `GenerationEnded` separately with the
  original error, only after socket producers stop.
- Add cloneable `RealtimeSession` handles for generation-bound concurrent admission,
  `wait_ended`, and allocating-generation identity on chart IDs. Reject stale work
  before enqueueing. Replacements remain caller-owned; no automatic reconnect is added.
- Remove the obsolete `DisconnectReason::RequestTimeout` category; request
  deadlines no longer terminate a socket.
- Return uncertain outcomes for admitted timeouts; atomically prevent cancelled queued
  requests from transmitting. Never reuse request identities or retry unknown operations.
- Track teardown after cancelled setup/shutdown and final-owner drop outside Tokio.
- Make collection ceilings configurable, remove speculative aggregate memory charges,
  and increase command/pending defaults to 4,096 and event/record defaults to 65,536.
  Finite frame/collection memory controls remain configurable; no subscription cap is added.
- Remove the unsupported 4,096-ID cap on caller-owned user-sync filters; preserve
  uniqueness, cross-field validation, and the configured outbound byte limit.
- Yield between coalesced records and index pending deadlines. Keep Tradovate's 2.5-second
  heartbeat, use active ping/pong liveness, and separate socket-write/request deadlines.
  Buffered decoding cannot expire a pong that the reader has not had a chance to observe.

### Security

- Update `rustls` from 0.23.43 to 0.23.45 to clear the active
  [RUSTSEC-2026-0285](https://rustsec.org/advisories/RUSTSEC-2026-0285) advisory
  (GHSA-2mjx-qc3c-rqvc): a vulnerable peer could send handshake messages that should
  use encryption in plaintext. `cargo deny` and `cargo audit` previously failed on
  every run and now pass without ignores. This lockfile-only change applies to both
  the REST and realtime TLS stacks and introduces no API change.

### Dependency and CI maintenance

- Drive the unanswered-ping burst regression with an explicit test clock so real
  TCP delivery delays cannot race its liveness deadline. Preserve repeated-batch
  coverage and the original `LivenessTimeout` assertion; production timing is unchanged.
- Update `rust_decimal` from 1.42.1 to 1.43.0, retaining `std`-only features
  and the exact-decimal wire contract ([#10](https://github.com/SharurTrading/tradovate-rs/pull/10)).
- Retire the inactive `RUSTSEC-2026-0235` exception and its feature-graph guard:
  `rkyv` 0.7 no longer appears in the lockfile. CI now runs `cargo audit` without ignores.
- Update the locked `reqwest` from 0.13.4 to 0.13.5, retaining the existing
  manifest constraint and the `default-features = false` set of `query`, `rustls`,
  and `stream` ([#15](https://github.com/SharurTrading/tradovate-rs/pull/15)).
- Update the SHA-pinned `taiki-e/install-action` from 2.87.0 to 2.87.4
  ([#9](https://github.com/SharurTrading/tradovate-rs/pull/9)).
- Update the SHA-pinned `taiki-e/install-action` from 2.87.4 to 2.87.10
  ([#14](https://github.com/SharurTrading/tradovate-rs/pull/14)).
- Update the SHA-pinned `taiki-e/install-action` from 2.87.10 to 2.87.14
  ([#18](https://github.com/SharurTrading/tradovate-rs/pull/18)).
- Update the SHA-pinned `dtolnay/rust-toolchain` action from `6c977a6` to `02cb101`
  on its tracked `master` ref
  ([#19](https://github.com/SharurTrading/tradovate-rs/pull/19)).
- Update the locked `jiff` from 0.2.35 to 0.2.37, retaining the existing manifest
  constraint and refreshing its locked `getrandom` transitive dependency
  ([#20](https://github.com/SharurTrading/tradovate-rs/pull/20)).

See README migration guidance and ADR 0002. This pre-1.0 minor version denotes the
changed public recovery contract. `publish = false` remains; no tag or registry release.
