<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

This project is pre-release. No release entries have been published.

## 0.4.0 (unreleased)

- Continuity loss is reported at its FIFO source position without discarding valid
  suffix records. Remove recovery acknowledgement and batch publication epochs.
- Add `RealtimeConnection::retire(&mut self)` to stop producers while preserving
  the received bounded batch, correlated replies and drainable terminal evidence.
- Initial authentication/bootstrap validation, provider rate limits and actual
  transport retirement stay unchanged. Registry publication remains disabled.
