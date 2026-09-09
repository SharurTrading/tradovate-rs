<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Advisory exceptions

Any active exception must be limited to a lockfile-scanner false positive, never
an active vulnerability. CI must independently prove that the affected package is
absent from the compiled dependency graph under every crate feature.

There are no active exceptions. CI runs `cargo audit` without advisory ignores.

## Retired: RUSTSEC-2026-0235 — `rkyv` 0.7

Retired on 2026-09-09 after upgrading to `rust_decimal` 1.43.0. Upstream removed
its optional `rkyv` 0.7 feature bridge, and the package is absent from the updated
lockfile. The removal condition below is satisfied; both the audit ignore and
its associated feature-graph guard have been removed. The historical evidence
is retained for auditability.

Source: [upstream 1.43.0 release](https://github.com/paupino/rust-decimal/releases/tag/1.43.0),
reviewed 2026-09-09.

- Reviewed: 2026-08-21.
- Advisory: <https://rustsec.org/advisories/RUSTSEC-2026-0235.html>.
- Former lockfile path: the optional `rust_decimal` 1.42.1 `rkyv` feature records `rkyv`
  0.7.46 in `Cargo.lock` even though the feature is not enabled.
- Historical runtime/build exposure: none. `rust_decimal` has default features disabled and
  enables only `std`. The repository defines no feature that enables
  `rust_decimal/rkyv` or `rust_decimal/rkyv-safe`.
- Former enforced evidence: CI ran `cargo tree --locked --all-features --edges
  normal,build,dev --prefix none` and failed if any `rkyv` 0.7 package is active
  before passing this advisory ID to `cargo audit --ignore`.
- Removal condition: remove the exception as soon as `rust_decimal` no longer
  resolves vulnerable `rkyv` into the lockfile, `cargo audit` becomes
  feature-graph-aware, or any crate feature activates `rkyv` (the last condition
  must fail CI rather than extending this exception).

The former optional compatibility feature is documented in the pinned
[1.42.1 README](https://github.com/paupino/rust-decimal/blob/1.42.1/README.md#rkyv).
