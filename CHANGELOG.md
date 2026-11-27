<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

This project is pre-release. No release entries have been published.

## 0.4.0 (unreleased)

- Use Jiff 1.0.3 for provider timestamps. Callers who use the exposed Jiff types
  use Jiff 1.x. Request and response timestamp encodings stay the same.
- Add `ClientBuilder::wall_clock` to supply the UTC source for authentication
  expiry checks and OAuth expiry calculation. Realtime timers remain monotonic.

- Add `RealtimeConnection::queued_event_count()` for an exact local retained FIFO
  prefix, including control and pending terminal events; keep `backlog()` as the
  separate venue-event lag observation.
- Continuity loss is reported at its FIFO source position without discarding valid
  suffix records. Remove recovery acknowledgement and batch publication epochs.
- Add `RealtimeConnection::retire(&mut self)` to stop producers while preserving
  the received bounded batch, correlated replies and drainable terminal evidence.
- Initial authentication/bootstrap validation, provider rate limits and actual
  transport retirement stay unchanged. Registry publication remains disabled.
