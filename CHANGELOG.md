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
- Update `rustls` from 0.23.45 to 0.23.46 to clear four active advisories:
  [RUSTSEC-2026-0346](https://rustsec.org/advisories/RUSTSEC-2026-0346) (derived
  `Debug` implementations could expose TLS 1.2 master secrets or TLS 1.3 resumption
  PSKs to custom session-store logging),
  [RUSTSEC-2026-0347](https://rustsec.org/advisories/RUSTSEC-2026-0347) (application
  data buffered before an ECH rejection could be sent to the outer server),
  [RUSTSEC-2026-0348](https://rustsec.org/advisories/RUSTSEC-2026-0348) (the optional
  Brotli certificate decompressor accepted a peer-selected window large enough to
  request a 1 GiB allocation), and
  [RUSTSEC-2026-0349](https://rustsec.org/advisories/RUSTSEC-2026-0349) (an
  unauthenticated ServerHello could pin a classical key share for later
  connections). This lockfile-only change applies to both the REST and realtime
  TLS stacks and introduces no API change.
- Update `quinn-proto` from 0.11.17 to 0.11.19 to clear three advisories now
  flagged by `cargo audit` in the HTTP/3 dependency tree carried in the lockfile:
  [RUSTSEC-2026-0338](https://rustsec.org/advisories/RUSTSEC-2026-0338) (zero-length
  STREAM frames during unordered reads),
  [RUSTSEC-2026-0340](https://rustsec.org/advisories/RUSTSEC-2026-0340) (peer minimum
  ACK delay with `ACK_FREQUENCY` transmission), and
  [RUSTSEC-2026-0343](https://rustsec.org/advisories/RUSTSEC-2026-0343) (outgoing
  datagram buffer accounting). A remote peer could panic the QUIC connection handler
  in each case. Together with the `rustls` upgrade above, `cargo deny` and
  `cargo audit` now pass without ignores. The `http3` reqwest feature remains
  disabled; this is a lockfile-only change with no API change.

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
- Update the SHA-pinned `taiki-e/install-action` from 2.87.14 to 2.87.26
  ([#25](https://github.com/SharurTrading/tradovate-rs/pull/25)).
- Update the locked `cargo-minor-patch` group: `jiff` 0.2.37 to 0.2.38 (refreshing
  the bundled IANA time zone database to 2026e), `serde_json` 1.0.151 to 1.0.154,
  `thiserror` 2.0.20 to 2.0.21, `tokio` 1.53.1 to 1.53.3, `tokio-util` 0.7.19 to
  0.7.20, and `proptest` 1.11.0 to 1.12.0. The manifest constraints are unchanged;
  `proptest` swaps `rand_chacha` for `chacha20` in the dev-only test graph
  ([#34](https://github.com/SharurTrading/tradovate-rs/pull/34)).

See README migration guidance and ADR 0002. This pre-1.0 minor version denotes the
changed public recovery contract. `publish = false` remains; no tag or registry release.
