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

See README migration guidance and ADR 0002. This pre-1.0 minor version denotes the
changed public recovery contract. `publish = false` remains; no tag or registry release.
